//! Sync of the signed-in user's saved formulas, starters, bake journal and calibrations.
//!
//! `GET /api/v1/sync?since=<cursor>` pulls what changed; `POST /api/v1/sync` pushes changed
//! records and pulls in the same round trip. Rules:
//!
//! - Records have client-chosen ids (UUIDv7; the culture key for calibrations), a client
//!   `updated_at` in milliseconds and a `deleted` flag (tombstones are kept so that deletions
//!   reach every device). Deleting a record the server never stored is ignored: no other device
//!   knows its id.
//! - Last writer wins per record, by `updated_at`: a pushed record is accepted only if it is
//!   strictly newer than the stored one. Otherwise the stored version comes back in the reply, so
//!   the client converges. An `updated_at` ahead of the server clock by more than five minutes is
//!   clamped (a wrong clock cannot win forever).
//! - The cursor is the user's change sequence: every accepted write takes the next number.
//! - Limits: 16 KiB per record, 500 records and 1 MiB per request, a cap of live records per kind,
//!   and [`QUOTA_BYTES`] stored per account, tombstones included. Each row keeps its size
//!   (`bytes`, the record as clients receive it) and `users.sync_bytes` their running sum, updated
//!   and checked in the transaction that applies a push: a push that would take the account over
//!   the quota is refused whole (`quota_exceeded`, nothing applied). Pushes carrying records are
//!   rate-limited per account (`rate_limited`).
//! - Replies stay small whatever is stored: a page holds at most [`PAGE`] records and about
//!   [`PAGE_BYTES`], cut from the stored sizes before any content is read, and records go out as
//!   the JSON text they are written to, never parsed into a tree.

use std::collections::HashSet;
use std::fmt::Write as _;

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_json::{Map, Value, json};

use crate::app::AppState;
use crate::auth::{ApiError, CurrentUser, ratelimit};
use crate::db;

/// A formula is a few hundred bytes (at most 12 KiB: the `?s=` limit of the calculator) and notes
/// are at most 4,000 characters.
pub const MAX_RECORD_BYTES: usize = 16 * 1024;
pub const MAX_BODY_BYTES: usize = 1024 * 1024;
pub const MAX_PUSHED: usize = 500;
/// Stored bytes per account, tombstones included: far above what a baker keeps, far below the
/// 1 GB volume.
pub const QUOTA_BYTES: i64 = 5_000_000;
/// Records per pull reply; `more` tells the client to ask again from the returned cursor.
pub const PAGE: usize = 500;
/// Bytes of records per pull reply (a page always holds at least one record).
pub const PAGE_BYTES: i64 = 1024 * 1024;
/// Pushes carrying records, per account and window. A device pushes after each edit, and a first
/// sign-in uploads its data in batches of 512 KiB at most.
const PUSHES_PER_WINDOW: i64 = 30;
const PUSH_WINDOW_SECS: i64 = 600;
const FUTURE_SLACK_MS: i64 = 5 * 60 * 1000;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/sync", get(pull).post(push))
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Formulas,
    Starters,
    Bakes,
    Calibrations,
}

impl Kind {
    pub const ALL: [Kind; 4] = [Kind::Formulas, Kind::Starters, Kind::Bakes, Kind::Calibrations];

    pub fn table(self) -> &'static str {
        self.name()
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Formulas => "formulas",
            Self::Starters => "starters",
            Self::Bakes => "bakes",
            Self::Calibrations => "calibrations",
        }
    }

    /// Live (not deleted) records one user may keep: more than a person uses.
    fn cap(self) -> i64 {
        match self {
            Self::Formulas => 500,
            Self::Starters => 50,
            Self::Bakes => 2_000,
            Self::Calibrations => 100,
        }
    }

    /// Columns after `id`, `updated_at`, `deleted` in the SELECT of a record.
    fn columns(self) -> &'static str {
        match self {
            Self::Formulas => "name, json",
            Self::Starters => "name, flour, hydration, notes",
            Self::Bakes => "formula_id, starter_id, plan, actual, outcome, notes",
            Self::Calibrations => "mean, variance, n",
        }
    }
}

/// What a record carries besides its id, clock and tombstone flag.
enum Payload {
    Formula {
        name: String,
        json: String,
    },
    Starter {
        name: String,
        flour: String,
        hydration: f64,
        notes: String,
    },
    Bake {
        formula_id: Option<String>,
        starter_id: Option<String>,
        plan: String,
        actual: String,
        outcome: String,
        notes: String,
    },
    Calibration {
        mean: f64,
        variance: f64,
        n: i64,
    },
}

struct Parsed {
    id: String,
    updated_at: i64,
    deleted: bool,
    payload: Payload,
}

fn text<'a>(
    obj: &'a Map<String, Value>,
    key: &str,
    max_chars: usize,
) -> Result<&'a str, &'static str> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(""),
        Some(Value::String(s)) if s.chars().count() <= max_chars => Ok(s),
        _ => Err("invalid"),
    }
}

fn number(obj: &Map<String, Value>, key: &str, min: f64, max: f64) -> Result<f64, &'static str> {
    match obj.get(key).and_then(Value::as_f64) {
        Some(v) if v.is_finite() && (min..=max).contains(&v) => Ok(v),
        _ => Err("invalid"),
    }
}

/// A JSON object (or null → `{}`) re-serialized compactly.
fn object_text(obj: &Map<String, Value>, key: &str) -> Result<String, &'static str> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok("{}".to_owned()),
        Some(v @ Value::Object(_)) => serde_json::to_string(v).map_err(|_| "invalid"),
        _ => Err("invalid"),
    }
}

fn uuid_or_none(obj: &Map<String, Value>, key: &str) -> Result<Option<String>, &'static str> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => canonical_uuid(s).map(Some).ok_or("invalid"),
        _ => Err("invalid"),
    }
}

fn canonical_uuid(s: &str) -> Option<String> {
    uuid::Uuid::parse_str(s).ok().map(|u| u.hyphenated().to_string())
}

fn culture_key(s: &str) -> Option<String> {
    let ok = !s.is_empty()
        && s.len() <= 64
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b':' | b'-'));
    ok.then(|| s.to_owned())
}

fn parse(kind: Kind, value: &Value, now_ms: i64) -> Result<Parsed, &'static str> {
    let obj = value.as_object().ok_or("invalid")?;
    if serde_json::to_vec(value).map_or(usize::MAX, |v| v.len()) > MAX_RECORD_BYTES {
        return Err("too_large");
    }
    let id_text = obj.get("id").and_then(Value::as_str).ok_or("invalid")?;
    let id = match kind {
        Kind::Calibrations => culture_key(id_text),
        _ => canonical_uuid(id_text),
    }
    .ok_or("invalid")?;
    let updated_at = match obj.get("updated_at").and_then(Value::as_i64) {
        Some(t) if t > 0 => t.min(now_ms + FUTURE_SLACK_MS),
        _ => return Err("invalid"),
    };
    let deleted = match obj.get("deleted") {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        _ => return Err("invalid"),
    };
    let payload = if deleted {
        // A tombstone keeps nothing of the content.
        match kind {
            Kind::Formulas => Payload::Formula { name: String::new(), json: "{}".into() },
            Kind::Starters => Payload::Starter {
                name: String::new(),
                flour: String::new(),
                hydration: 100.0,
                notes: String::new(),
            },
            Kind::Bakes => Payload::Bake {
                formula_id: None,
                starter_id: None,
                plan: "{}".into(),
                actual: "{}".into(),
                outcome: String::new(),
                notes: String::new(),
            },
            Kind::Calibrations => Payload::Calibration { mean: 0.0, variance: 0.0, n: 0 },
        }
    } else {
        match kind {
            Kind::Formulas => {
                let name = text(obj, "name", 120)?.trim();
                if name.is_empty() {
                    return Err("invalid");
                }
                if !matches!(obj.get("formula"), Some(Value::Object(_))) {
                    return Err("invalid");
                }
                Payload::Formula { name: name.to_owned(), json: object_text(obj, "formula")? }
            }
            Kind::Starters => Payload::Starter {
                name: text(obj, "name", 80)?.trim().to_owned(),
                flour: text(obj, "flour", 80)?.trim().to_owned(),
                hydration: number(obj, "hydration", 0.0, 1000.0)?,
                notes: text(obj, "notes", 2_000)?.to_owned(),
            },
            Kind::Bakes => Payload::Bake {
                formula_id: uuid_or_none(obj, "formula_id")?,
                starter_id: uuid_or_none(obj, "starter_id")?,
                plan: object_text(obj, "plan")?,
                actual: object_text(obj, "actual")?,
                outcome: text(obj, "outcome", 40)?.to_owned(),
                notes: text(obj, "notes", 4_000)?.to_owned(),
            },
            Kind::Calibrations => {
                let n = obj
                    .get("n")
                    .and_then(Value::as_i64)
                    .filter(|n| (0..=1_000_000).contains(n))
                    .ok_or("invalid")?;
                Payload::Calibration {
                    mean: number(obj, "mean", -100.0, 100.0)?,
                    variance: number(obj, "variance", 0.0, 1_000_000.0)?,
                    n,
                }
            }
        }
    };
    Ok(Parsed { id, updated_at, deleted, payload })
}

/// `s` as a JSON string.
fn quoted(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_owned())
}

/// The record as clients receive it, written from its fields: a tombstone carries only id, clock
/// and the flag; JSON columns (`formula`, `plan`, `actual`) were written by this module and go in
/// as they are. Its length is what the record weighs against the quota and a page's budget.
fn wire(id: &str, updated_at: i64, deleted: bool, payload: &Payload) -> String {
    let number = |v: f64| serde_json::to_string(&v).unwrap_or_else(|_| "null".to_owned());
    let uuid = |v: &Option<String>| v.as_deref().map_or_else(|| "null".to_owned(), quoted);
    let mut out =
        format!("{{\"id\":{},\"updated_at\":{updated_at},\"deleted\":{deleted}", quoted(id));
    if !deleted {
        let fields: Vec<(&str, String)> = match payload {
            Payload::Formula { name, json } => {
                vec![("name", quoted(name)), ("formula", json.clone())]
            }
            Payload::Starter { name, flour, hydration, notes } => vec![
                ("name", quoted(name)),
                ("flour", quoted(flour)),
                ("hydration", number(*hydration)),
                ("notes", quoted(notes)),
            ],
            Payload::Bake { formula_id, starter_id, plan, actual, outcome, notes } => vec![
                ("formula_id", uuid(formula_id)),
                ("starter_id", uuid(starter_id)),
                ("plan", plan.clone()),
                ("actual", actual.clone()),
                ("outcome", quoted(outcome)),
                ("notes", quoted(notes)),
            ],
            Payload::Calibration { mean, variance, n } => {
                vec![("mean", number(*mean)), ("variance", number(*variance)), ("n", n.to_string())]
            }
        };
        for (key, value) in fields {
            let _ = write!(out, ",\"{key}\":{value}");
        }
    }
    out.push('}');
    out
}

enum Applied {
    Accepted,
    /// A deletion of a record the server never stored: nothing to remember.
    Ignored,
    /// The stored record is at least as new.
    Stale,
    LimitReached,
}

fn apply(tx: &Connection, user_id: &str, kind: Kind, rec: &Parsed) -> rusqlite::Result<Applied> {
    let stored: Option<(i64, bool, i64)> = tx
        .query_row(
            &format!(
                "SELECT updated_at, deleted, bytes FROM {} WHERE user_id = ?1 AND id = ?2",
                kind.table()
            ),
            params![user_id, rec.id],
            |r| Ok((r.get(0)?, r.get::<_, i64>(1)? != 0, r.get(2)?)),
        )
        .optional()?;
    if let Some((at, _, _)) = stored
        && rec.updated_at <= at
    {
        return Ok(Applied::Stale);
    }
    if stored.is_none() && rec.deleted {
        return Ok(Applied::Ignored);
    }
    // A record becoming live (new, or a tombstone brought back) must fit under the cap.
    if !rec.deleted && stored.is_none_or(|(_, deleted, _)| deleted) {
        let live: i64 = tx.query_row(
            &format!("SELECT count(*) FROM {} WHERE user_id = ?1 AND deleted = 0", kind.table()),
            [user_id],
            |r| r.get(0),
        )?;
        if live >= kind.cap() {
            return Ok(Applied::LimitReached);
        }
    }
    let old_bytes = stored.map_or(0, |(_, _, bytes)| bytes);
    let bytes = i64::try_from(wire(&rec.id, rec.updated_at, rec.deleted, &rec.payload).len())
        .unwrap_or(i64::MAX);
    let seq: i64 = tx.query_row(
        "UPDATE users SET sync_seq = sync_seq + 1, sync_bytes = sync_bytes + ?2 WHERE id = ?1
         RETURNING sync_seq",
        params![user_id, bytes - old_bytes],
        |r| r.get(0),
    )?;
    let (id, at, del) = (&rec.id, rec.updated_at, i64::from(rec.deleted));
    match &rec.payload {
        Payload::Formula { name, json } => tx.execute(
            "INSERT INTO formulas (user_id, id, name, json, updated_at, deleted, seq, bytes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT (user_id, id) DO UPDATE SET name = ?3, json = ?4, updated_at = ?5, deleted = ?6,
                 seq = ?7, bytes = ?8",
            params![user_id, id, name, json, at, del, seq, bytes],
        )?,
        Payload::Starter { name, flour, hydration, notes } => tx.execute(
            "INSERT INTO starters (user_id, id, name, flour, hydration, notes, updated_at, deleted, seq, bytes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT (user_id, id) DO UPDATE SET name = ?3, flour = ?4, hydration = ?5, notes = ?6,
                 updated_at = ?7, deleted = ?8, seq = ?9, bytes = ?10",
            params![user_id, id, name, flour, hydration, notes, at, del, seq, bytes],
        )?,
        Payload::Bake { formula_id, starter_id, plan, actual, outcome, notes } => tx.execute(
            "INSERT INTO bakes (user_id, id, formula_id, starter_id, plan, actual, outcome, notes, updated_at, deleted, seq, bytes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
             ON CONFLICT (user_id, id) DO UPDATE SET formula_id = ?3, starter_id = ?4, plan = ?5, actual = ?6,
                 outcome = ?7, notes = ?8, updated_at = ?9, deleted = ?10, seq = ?11, bytes = ?12",
            params![user_id, id, formula_id, starter_id, plan, actual, outcome, notes, at, del, seq, bytes],
        )?,
        Payload::Calibration { mean, variance, n } => tx.execute(
            "INSERT INTO calibrations (user_id, id, mean, variance, n, updated_at, deleted, seq, bytes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT (user_id, id) DO UPDATE SET mean = ?3, variance = ?4, n = ?5,
                 updated_at = ?6, deleted = ?7, seq = ?8, bytes = ?9",
            params![user_id, id, mean, variance, n, at, del, seq, bytes],
        )?,
    };
    Ok(Applied::Accepted)
}

/// A stored row (columns: `id`, `updated_at`, `deleted`, then `kind.columns()`) as its id and
/// wire record.
fn row_wire(kind: Kind, r: &rusqlite::Row<'_>) -> rusqlite::Result<(String, String)> {
    let id: String = r.get(0)?;
    let updated_at: i64 = r.get(1)?;
    let deleted = r.get::<_, i64>(2)? != 0;
    let payload = match kind {
        Kind::Formulas => Payload::Formula { name: r.get(3)?, json: r.get(4)? },
        Kind::Starters => Payload::Starter {
            name: r.get(3)?,
            flour: r.get(4)?,
            hydration: r.get(5)?,
            notes: r.get(6)?,
        },
        Kind::Bakes => Payload::Bake {
            formula_id: r.get(3)?,
            starter_id: r.get(4)?,
            plan: r.get(5)?,
            actual: r.get(6)?,
            outcome: r.get(7)?,
            notes: r.get(8)?,
        },
        Kind::Calibrations => {
            Payload::Calibration { mean: r.get(3)?, variance: r.get(4)?, n: r.get(5)? }
        }
    };
    let text = wire(&id, updated_at, deleted, &payload);
    Ok((id, text))
}

/// Wire text as a raw JSON value (checked, not parsed into a tree).
fn raw(text: String) -> rusqlite::Result<Box<RawValue>> {
    RawValue::from_string(text).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn select_sql(kind: Kind, filter: &str) -> String {
    format!(
        "SELECT id, updated_at, deleted, {} FROM {} WHERE user_id = ?1 {filter}",
        kind.columns(),
        kind.table()
    )
}

fn one(conn: &Connection, user_id: &str, kind: Kind, id: &str) -> rusqlite::Result<Option<String>> {
    let sql = select_sql(kind, "AND id = ?2");
    conn.query_row(&sql, params![user_id, id], |r| row_wire(kind, r).map(|(_, text)| text))
        .optional()
}

fn current_seq(conn: &Connection, user_id: &str) -> rusqlite::Result<i64> {
    conn.query_row("SELECT sync_seq FROM users WHERE id = ?1", [user_id], |r| r.get(0))
}

fn stored_bytes(conn: &Connection, user_id: &str) -> rusqlite::Result<i64> {
    conn.query_row("SELECT sync_bytes FROM users WHERE id = ?1", [user_id], |r| r.get(0))
}

#[derive(Default, Serialize)]
struct Records {
    formulas: Vec<Box<RawValue>>,
    starters: Vec<Box<RawValue>>,
    bakes: Vec<Box<RawValue>>,
    calibrations: Vec<Box<RawValue>>,
}

impl Records {
    fn push(&mut self, kind: Kind, value: Box<RawValue>) {
        match kind {
            Kind::Formulas => self.formulas.push(value),
            Kind::Starters => self.starters.push(value),
            Kind::Bakes => self.bakes.push(value),
            Kind::Calibrations => self.calibrations.push(value),
        }
    }
}

/// Every live record of a user as `"formulas":[…],"starters":[…],"bakes":[…],"calibrations":[…]`
/// (one record per line), written straight from the rows: for "download my data". The quota
/// bounds its size.
pub fn export_records(conn: &Connection, user_id: &str, out: &mut String) -> rusqlite::Result<()> {
    for (i, kind) in Kind::ALL.into_iter().enumerate() {
        let _ = write!(out, "{}\n\"{}\":[", if i > 0 { "," } else { "" }, kind.name());
        let sql = select_sql(kind, "AND deleted = 0 ORDER BY updated_at");
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([user_id])?;
        let mut first = true;
        while let Some(row) = rows.next()? {
            out.push_str(if first { "\n" } else { ",\n" });
            out.push_str(&row_wire(kind, row)?.1);
            first = false;
        }
        out.push(']');
    }
    Ok(())
}

#[derive(Serialize)]
struct ExportedMessage {
    role: String,
    content: String,
    tool_calls: Option<Box<RawValue>>,
    citations: Option<Box<RawValue>>,
    model: Option<String>,
    created_at: i64,
}

/// The user's chat conversations with their messages as a JSON array (one message per line),
/// written straight from the rows: for "download my data".
pub fn export_chat(conn: &Connection, user_id: &str, out: &mut String) -> rusqlite::Result<()> {
    let mut conversations = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM chat_conversations WHERE user_id = ?1 ORDER BY created_at",
    )?;
    let mut messages = conn.prepare(
        "SELECT role, content, tool_calls, citations, model, created_at FROM chat_messages
         WHERE conversation_id = ?1 ORDER BY id",
    )?;
    let json = |s: Option<String>| s.and_then(|s| RawValue::from_string(s).ok());
    out.push('[');
    let mut rows = conversations.query([user_id])?;
    let mut first = true;
    while let Some(c) = rows.next()? {
        let id: String = c.get(0)?;
        let title: String = c.get(1)?;
        let _ = write!(
            out,
            "{}\n{{\"id\":{},\"title\":{},\"created_at\":{},\"updated_at\":{},\"messages\":[",
            if first { "" } else { "," },
            quoted(&id),
            quoted(&title),
            c.get::<_, i64>(2)?,
            c.get::<_, i64>(3)?
        );
        first = false;
        let mut message_rows = messages.query([&id])?;
        let mut first_message = true;
        while let Some(m) = message_rows.next()? {
            let message = ExportedMessage {
                role: m.get(0)?,
                content: m.get(1)?,
                tool_calls: json(m.get(2)?),
                citations: json(m.get(3)?),
                model: m.get(4)?,
                created_at: m.get(5)?,
            };
            out.push_str(if first_message { "\n" } else { ",\n" });
            out.push_str(&serde_json::to_string(&message).unwrap_or_else(|_| "null".to_owned()));
            first_message = false;
        }
        out.push_str("]}");
    }
    out.push(']');
    Ok(())
}

#[derive(Deserialize)]
struct SinceQuery {
    since: Option<i64>,
}

/// One reply's worth of changes.
struct Page {
    cursor: i64,
    more: bool,
    reset: bool,
    records: Records,
    /// What `records` holds, as (kind, id).
    sent: HashSet<(Kind, String)>,
    /// Their size.
    bytes: i64,
}

#[derive(Serialize)]
struct Reply<'a> {
    cursor: i64,
    more: bool,
    reset: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rejected: Option<&'a [Value]>,
    #[serde(flatten)]
    records: &'a Records,
}

fn reply(page: &Page, pushed: Option<(usize, &[Value])>) -> Response {
    let body = Reply {
        cursor: page.cursor,
        more: page.more,
        reset: page.reset,
        accepted: pushed.map(|(accepted, _)| accepted),
        rejected: pushed.map(|(_, rejected)| rejected),
        records: &page.records,
    };
    ([(header::CACHE_CONTROL, "no-store")], Json(body)).into_response()
}

/// Changes after `since`, oldest change first: at most `page` records and, past the first, at
/// most `budget` bytes, omitting `skip` (what the caller just pushed). Where the page ends is
/// decided from the stored sizes; only the records inside it are read.
fn pull_page(
    conn: &Connection,
    user_id: &str,
    since: i64,
    skip: &HashSet<(Kind, String)>,
    page: usize,
    budget: i64,
) -> rusqlite::Result<Page> {
    let current = current_seq(conn, user_id)?;
    // A cursor ahead of the server means the database went back in time (a restore): start over.
    let (since, reset) = if since > current { (0, true) } else { (since, false) };
    let limit = i64::try_from(page + 1).unwrap_or(i64::MAX);
    let mut sizes: Vec<(i64, i64)> = Vec::new();
    for kind in Kind::ALL {
        let mut stmt = conn.prepare(&format!(
            "SELECT seq, bytes FROM {} WHERE user_id = ?1 AND seq > ?2 ORDER BY seq LIMIT ?3",
            kind.table()
        ))?;
        for row in stmt.query_map(params![user_id, since, limit], |r| Ok((r.get(0)?, r.get(1)?)))? {
            sizes.push(row?);
        }
    }
    sizes.sort_unstable();
    let (mut upto, mut taken, mut bytes) = (since, 0, 0);
    for &(seq, size) in &sizes {
        if taken == page || (taken > 0 && bytes + size > budget) {
            break;
        }
        (upto, taken, bytes) = (seq, taken + 1, bytes + size);
    }
    let more = taken < sizes.len();
    let mut out = Page {
        cursor: if more { upto } else { current },
        more,
        reset,
        records: Records::default(),
        sent: HashSet::new(),
        bytes,
    };
    if taken == 0 {
        return Ok(out);
    }
    for kind in Kind::ALL {
        let mut stmt =
            conn.prepare(&select_sql(kind, "AND seq > ?2 AND seq <= ?3 ORDER BY seq"))?;
        let mut rows = stmt.query(params![user_id, since, upto])?;
        while let Some(row) = rows.next()? {
            let (id, text) = row_wire(kind, row)?;
            let key = (kind, id);
            if !skip.contains(&key) {
                out.records.push(kind, raw(text)?);
                out.sent.insert(key);
            }
        }
    }
    Ok(out)
}

async fn pull(
    State(state): State<AppState>,
    current: CurrentUser,
    Query(q): Query<SinceQuery>,
) -> Result<Response, ApiError> {
    let since = q.since.unwrap_or(0).max(0);
    let user_id = current.user.id;
    let page = state
        .db
        .read(move |c| pull_page(c, &user_id, since, &HashSet::new(), PAGE, PAGE_BYTES))
        .await?;
    Ok(reply(&page, None))
}

#[derive(Deserialize, Default)]
struct Push {
    since: Option<i64>,
    #[serde(default)]
    formulas: Vec<Value>,
    #[serde(default)]
    starters: Vec<Value>,
    #[serde(default)]
    bakes: Vec<Value>,
    #[serde(default)]
    calibrations: Vec<Value>,
}

enum Pushed {
    Applied {
        page: Page,
        accepted: usize,
        rejected: Vec<Value>,
    },
    /// Applying it would take the account over [`QUOTA_BYTES`]: nothing was applied.
    OverQuota,
    RateLimited,
}

async fn push(
    State(state): State<AppState>,
    current: CurrentUser,
    body: Body,
) -> Result<Response, ApiError> {
    let bytes = axum::body::to_bytes(body, MAX_BODY_BYTES)
        .await
        .map_err(|_| ApiError(StatusCode::PAYLOAD_TOO_LARGE, "too_large"))?;
    let request: Push = serde_json::from_slice(&bytes)
        .map_err(|_| ApiError(StatusCode::BAD_REQUEST, "invalid_json"))?;
    drop(bytes);
    let total = request.formulas.len()
        + request.starters.len()
        + request.bakes.len()
        + request.calibrations.len();
    if total > MAX_PUSHED {
        return Err(ApiError(StatusCode::PAYLOAD_TOO_LARGE, "too_many_records"));
    }
    let since = request.since.unwrap_or(0).max(0);
    let user_id = current.user.id;
    let now_ms = db::now_ms();
    let batches = [
        (Kind::Formulas, request.formulas),
        (Kind::Starters, request.starters),
        (Kind::Bakes, request.bakes),
        (Kind::Calibrations, request.calibrations),
    ];
    let outcome = state
        .db
        .write(move |c| {
            // A push without records is a pull: only writes are limited.
            let key = format!("sync:push:{user_id}");
            if total > 0
                && !ratelimit::hit(c, &key, PUSH_WINDOW_SECS, PUSHES_PER_WINDOW, db::now())?
            {
                return Ok(Pushed::RateLimited);
            }
            let tx = c.transaction()?;
            let before = stored_bytes(&tx, &user_id)?;
            let mut accepted = 0usize;
            let mut rejected: Vec<Value> = Vec::new();
            let mut done: HashSet<(Kind, String)> = HashSet::new();
            let mut stale: Vec<(Kind, String)> = Vec::new();
            for (kind, records) in &batches {
                for value in records {
                    let id_hint = value.get("id").and_then(Value::as_str).unwrap_or("").to_owned();
                    let reject = |reason: &str| json!({ "kind": kind.name(), "id": id_hint, "reason": reason });
                    match parse(*kind, value, now_ms) {
                        Err(reason) => rejected.push(reject(reason)),
                        Ok(rec) => match apply(&tx, &user_id, *kind, &rec)? {
                            Applied::Accepted => {
                                accepted += 1;
                                done.insert((*kind, rec.id));
                            }
                            Applied::Ignored => accepted += 1,
                            Applied::Stale => {
                                rejected.push(reject("stale"));
                                stale.push((*kind, rec.id));
                            }
                            Applied::LimitReached => rejected.push(reject("limit")),
                        },
                    }
                }
            }
            let after = stored_bytes(&tx, &user_id)?;
            if after > QUOTA_BYTES && after > before {
                // Dropping the transaction rolls every record of the push back.
                return Ok(Pushed::OverQuota);
            }
            tx.commit()?;
            let mut page = pull_page(c, &user_id, since, &done, PAGE, PAGE_BYTES)?;
            // The stored version of every record the server kept, so the client converges even
            // when that version is older than its cursor; within the reply's budget (what does
            // not fit stays stale on the client and comes back on its next push).
            for (kind, id) in stale {
                if page.sent.contains(&(kind, id.clone())) {
                    continue;
                }
                if let Some(text) = one(c, &user_id, kind, &id)? {
                    let size = i64::try_from(text.len()).unwrap_or(i64::MAX);
                    if page.bytes + size > PAGE_BYTES {
                        break;
                    }
                    page.bytes += size;
                    page.records.push(kind, raw(text)?);
                    page.sent.insert((kind, id));
                }
            }
            Ok(Pushed::Applied { page, accepted, rejected })
        })
        .await?;
    match outcome {
        Pushed::Applied { page, accepted, rejected } => {
            Ok(reply(&page, Some((accepted, rejected.as_slice()))))
        }
        Pushed::OverQuota => Err(ApiError(StatusCode::PAYLOAD_TOO_LARGE, "quota_exceeded")),
        Pushed::RateLimited => Err(ApiError(StatusCode::TOO_MANY_REQUESTS, "rate_limited")),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::testkit::{Client, TestApp};

    fn id() -> String {
        uuid::Uuid::now_v7().to_string()
    }

    fn formula(id: &str, name: &str, at: i64) -> Value {
        json!({ "id": id, "name": name, "formula": { "flour": 100 }, "updated_at": at })
    }

    async fn push(client: &mut Client, body: Value) -> Value {
        let reply = client.json(Method::POST, "/api/v1/sync", &body).await;
        assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
        reply.json()
    }

    async fn pull(client: &mut Client, since: i64) -> Value {
        let reply = client.get(&format!("/api/v1/sync?since={since}")).await;
        assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
        reply.json()
    }

    fn ids(list: &Value) -> Vec<String> {
        list.as_array().unwrap().iter().map(|r| r["id"].as_str().unwrap().to_owned()).collect()
    }

    #[tokio::test]
    async fn a_session_is_required() {
        let app = TestApp::new();
        let mut anonymous = app.client();
        assert_eq!(anonymous.get("/api/v1/sync").await.status, StatusCode::UNAUTHORIZED);
        let reply = anonymous.json(Method::POST, "/api/v1/sync", &json!({})).await;
        assert_eq!(reply.status, StatusCode::UNAUTHORIZED);
        assert_eq!(reply.json()["error"]["code"], "unauthorized");
    }

    #[tokio::test]
    async fn pushed_records_reach_other_devices_in_order_of_change() {
        let app = TestApp::new();
        let (mut laptop, _) = app.signed_in("me@example.com").await;
        let mut phone = laptop.clone_session();
        let (f, s, b) = (id(), id(), id());
        let now = crate::db::now_ms();

        let first = push(
            &mut laptop,
            json!({ "since": 0, "formulas": [formula(&f, "Country loaf", now)] }),
        )
        .await;
        assert_eq!(first["accepted"], 1);
        assert_eq!(first["cursor"], 1);
        assert_eq!(first["formulas"], json!([]), "what was just pushed is not echoed back");

        let second = push(
            &mut laptop,
            json!({
                "since": 1,
                "starters": [{ "id": s, "name": "Rye", "flour": "rye", "hydration": 80, "notes": "n", "updated_at": now }],
                "bakes": [{ "id": b, "formula_id": f.to_uppercase(), "starter_id": null, "plan": { "a": 1 },
                           "actual": { "b": 2 }, "outcome": "good", "notes": "ok", "updated_at": now }],
            }),
        )
        .await;
        assert_eq!(second["accepted"], 2);
        assert_eq!(second["cursor"], 3);

        // A second device starts from nothing and catches up; the cursor then moves it forward only.
        let all = pull(&mut phone, 0).await;
        assert_eq!(all["cursor"], 3);
        assert_eq!(all["more"], false);
        assert_eq!(all["reset"], false);
        assert_eq!(ids(&all["formulas"]), std::slice::from_ref(&f));
        assert_eq!(all["formulas"][0]["name"], "Country loaf");
        assert_eq!(all["formulas"][0]["formula"], json!({ "flour": 100 }));
        assert_eq!(all["starters"][0]["hydration"], 80.0);
        assert_eq!(all["bakes"][0]["formula_id"], f, "ids are stored in canonical form");
        assert_eq!(all["bakes"][0]["plan"], json!({ "a": 1 }));
        assert_eq!(all["bakes"][0]["actual"], json!({ "b": 2 }));
        let later = pull(&mut phone, 1).await;
        assert_eq!(later["formulas"], json!([]));
        assert_eq!(ids(&later["starters"]), [s]);
        assert_eq!(ids(&later["bakes"]), [b]);
        let nothing = pull(&mut phone, 3).await;
        assert_eq!(
            (nothing["cursor"].as_i64(), nothing["starters"].as_array().map(Vec::len)),
            (Some(3), Some(0))
        );

        // The other device's own changes come back to the laptop in the same round trip.
        let g = id();
        let exchange =
            push(&mut phone, json!({ "since": 3, "formulas": [formula(&g, "Baguette", now + 5)] }))
                .await;
        assert_eq!(exchange["cursor"], 4);
        let pulled = push(&mut laptop, json!({ "since": 3 })).await;
        assert_eq!(ids(&pulled["formulas"]), [g]);
    }

    #[tokio::test]
    async fn last_writer_wins_per_record() {
        let app = TestApp::new();
        let (mut laptop, _) = app.signed_in("me@example.com").await;
        let f = id();
        push(&mut laptop, json!({ "formulas": [formula(&f, "v1", 1_000)] })).await;

        // An older edit (a device that was offline) loses, and learns the stored version.
        let older =
            push(&mut laptop, json!({ "since": 1, "formulas": [formula(&f, "older", 500)] })).await;
        assert_eq!(older["accepted"], 0);
        assert_eq!(older["rejected"], json!([{ "kind": "formulas", "id": f, "reason": "stale" }]));
        assert_eq!(
            older["formulas"][0]["name"], "v1",
            "the stored version comes back even past the cursor"
        );
        assert_eq!(older["cursor"], 1);

        // The same timestamp is not newer: a retried push changes nothing.
        let retry =
            push(&mut laptop, json!({ "since": 1, "formulas": [formula(&f, "same time", 1_000)] }))
                .await;
        assert_eq!(retry["accepted"], 0);
        assert_eq!(retry["formulas"][0]["name"], "v1");

        let newer =
            push(&mut laptop, json!({ "since": 1, "formulas": [formula(&f, "v2", 2_000)] })).await;
        assert_eq!(newer["accepted"], 1);
        assert_eq!(newer["cursor"], 2);
        let stored = pull(&mut laptop, 0).await;
        assert_eq!(stored["formulas"][0]["name"], "v2");
        assert_eq!(stored["formulas"][0]["updated_at"], 2_000);
        // One row per record, whatever happened.
        assert_eq!(app.count("SELECT count(*) FROM formulas").await, 1);

        // Mixed batch: each record is judged alone.
        let g = id();
        let mixed = push(&mut laptop, json!({ "since": 2, "formulas": [formula(&f, "old again", 10), formula(&g, "fresh", 10)] })).await;
        assert_eq!(mixed["accepted"], 1);
        assert_eq!(mixed["rejected"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn deletions_travel_as_tombstones() {
        let app = TestApp::new();
        let (mut laptop, _) = app.signed_in("me@example.com").await;
        let mut phone = laptop.clone_session();
        let f = id();
        push(&mut laptop, json!({ "formulas": [formula(&f, "Doomed", 1_000)] })).await;
        let seen = pull(&mut phone, 0).await;
        assert_eq!(ids(&seen["formulas"]), std::slice::from_ref(&f));

        push(
            &mut laptop,
            json!({ "since": 1, "formulas": [{ "id": f, "deleted": true, "updated_at": 2_000 }] }),
        )
        .await;
        // The content is gone from the server; only the fact of deletion remains.
        let (name, json_text): (String, String) = app
            .state
            .db
            .read(|c| {
                c.query_row("SELECT name, json FROM formulas", [], |r| Ok((r.get(0)?, r.get(1)?)))
            })
            .await
            .unwrap();
        assert_eq!((name.as_str(), json_text.as_str()), ("", "{}"));

        let delta = pull(&mut phone, seen["cursor"].as_i64().unwrap()).await;
        assert_eq!(delta["formulas"], json!([{ "id": f, "updated_at": 2_000, "deleted": true }]));

        // A device that edited the record before the deletion cannot bring it back...
        let stale = push(
            &mut phone,
            json!({ "since": 2, "formulas": [formula(&f, "Edited offline", 1_500)] }),
        )
        .await;
        assert_eq!(stale["accepted"], 0);
        assert_eq!(stale["formulas"][0]["deleted"], true);
        // ...but a later edit deliberately can.
        let revived =
            push(&mut phone, json!({ "since": 2, "formulas": [formula(&f, "Back", 3_000)] })).await;
        assert_eq!(revived["accepted"], 1);
        let back = pull(&mut laptop, 2).await;
        assert_eq!(back["formulas"][0]["name"], "Back");
        assert_eq!(back["formulas"][0]["deleted"], false);

        // Deleting something the server never stored is ignored: no row, no change to pull.
        let ghost = id();
        let pushed = push(
            &mut laptop,
            json!({ "since": 3, "formulas": [{ "id": ghost, "deleted": true, "updated_at": 5 }] }),
        )
        .await;
        assert_eq!(pushed["accepted"], 1, "the client may forget it");
        assert_eq!(pushed["cursor"], 3);
        assert_eq!(app.count("SELECT count(*) FROM formulas").await, 1);
    }

    #[tokio::test]
    async fn users_are_isolated() {
        let app = TestApp::new();
        let (mut alice, _) = app.signed_in("alice@example.com").await;
        let (mut bob, _) = app.signed_in("bob@example.com").await;
        let f = id();
        push(&mut alice, json!({ "formulas": [formula(&f, "Alice's", 1_000)] })).await;
        assert_eq!(pull(&mut bob, 0).await["formulas"], json!([]));
        // The same id in another account is a different record, with its own clock and cursor.
        let theirs = push(&mut bob, json!({ "formulas": [formula(&f, "Bob's", 10)] })).await;
        assert_eq!(theirs["accepted"], 1);
        assert_eq!(theirs["cursor"], 1);
        assert_eq!(pull(&mut alice, 0).await["formulas"][0]["name"], "Alice's");
        assert_eq!(pull(&mut bob, 0).await["formulas"][0]["name"], "Bob's");
    }

    #[tokio::test]
    async fn records_and_requests_are_size_limited() {
        let app = TestApp::new();
        let (mut me, _) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        let good = id();

        let huge = json!({ "id": id(), "name": "Big", "formula": { "blob": "x".repeat(MAX_RECORD_BYTES) }, "updated_at": now });
        let bad_calibration =
            json!({ "id": "has space", "mean": 0, "variance": 0, "n": 1, "updated_at": now });
        let reply = push(
            &mut me,
            json!({
                "formulas": [
                    formula(&good, "Fine", now),
                    huge,
                    formula("not-a-uuid", "Bad id", now),
                    json!({ "id": id(), "formula": {}, "updated_at": now }),
                    json!({ "id": id(), "name": "No formula", "updated_at": now }),
                    json!({ "id": id(), "name": "Zero clock", "formula": {}, "updated_at": 0 }),
                    json!({ "id": id(), "name": "n".repeat(121), "formula": {}, "updated_at": now }),
                ],
                "starters": [{ "id": id(), "name": "Wet", "hydration": 5000, "updated_at": now }],
                "calibrations": [bad_calibration],
            }),
        )
        .await;
        assert_eq!(reply["accepted"], 1);
        let reasons: Vec<&str> = reply["rejected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["reason"].as_str().unwrap())
            .collect();
        assert_eq!(
            reasons,
            [
                "too_large",
                "invalid",
                "invalid",
                "invalid",
                "invalid",
                "invalid",
                "invalid",
                "invalid"
            ]
        );
        assert_eq!(app.count("SELECT count(*) FROM formulas").await, 1);

        // Too many records in one request.
        let many: Vec<Value> = (0..=MAX_PUSHED)
            .map(|i| formula(&id(), "x", now + i64::try_from(i).unwrap()))
            .collect();
        let reply = me.json(Method::POST, "/api/v1/sync", &json!({ "formulas": many })).await;
        assert_eq!(reply.status, StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(reply.json()["error"]["code"], "too_many_records");

        // A body over 1 MiB.
        let chunky: Vec<Value> = (0..20)
            .map(|_| json!({ "id": id(), "name": "Big", "formula": { "blob": "y".repeat(60_000) }, "updated_at": now }))
            .collect();
        let reply = me.json(Method::POST, "/api/v1/sync", &json!({ "formulas": chunky })).await;
        assert_eq!(reply.status, StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(reply.json()["error"]["code"], "too_large");
        assert_eq!(app.count("SELECT count(*) FROM formulas").await, 1);

        let reply = me
            .request(Method::POST, "/api/v1/sync", Some("application/json"), b"{not json".to_vec())
            .await;
        assert_eq!(reply.status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn a_user_can_only_keep_so_many_live_records() {
        let app = TestApp::new();
        let (mut me, _) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        let starters: Vec<Value> = (0..Kind::Starters.cap())
            .map(|i| json!({ "id": id(), "name": format!("s{i}"), "hydration": 100, "updated_at": now }))
            .collect();
        let reply = push(&mut me, json!({ "starters": starters })).await;
        assert_eq!(reply["accepted"], Kind::Starters.cap());

        let cursor = Kind::Starters.cap();
        let extra = id();
        let reply = push(&mut me, json!({ "since": cursor, "starters": [{ "id": extra, "name": "one too many", "hydration": 100, "updated_at": now }] })).await;
        assert_eq!(
            reply["rejected"],
            json!([{ "kind": "starters", "id": extra, "reason": "limit" }])
        );

        // Deleting frees room.
        let first: String = app
            .state
            .db
            .read(|c| c.query_row("SELECT id FROM starters LIMIT 1", [], |r| r.get(0)))
            .await
            .unwrap();
        push(&mut me, json!({ "since": cursor, "starters": [{ "id": first, "deleted": true, "updated_at": now + 1 }] })).await;
        let reply = push(&mut me, json!({ "since": cursor + 1, "starters": [{ "id": extra, "name": "now it fits", "hydration": 100, "updated_at": now }] })).await;
        assert_eq!(reply["accepted"], 1);
        // Bringing a tombstone back to life counts like a new record.
        let reply = push(&mut me, json!({ "since": cursor + 2, "starters": [{ "id": first, "name": "back", "hydration": 100, "updated_at": now + 2 }] })).await;
        assert_eq!(reply["rejected"][0]["reason"], "limit");
    }

    #[tokio::test]
    async fn a_wrong_clock_cannot_win_forever() {
        let app = TestApp::new();
        let (mut me, _) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        let f = id();
        push(
            &mut me,
            json!({ "formulas": [formula(&f, "from the future", now + 10 * 86_400_000)] }),
        )
        .await;
        let stored: i64 = app
            .state
            .db
            .read(|c| c.query_row("SELECT updated_at FROM formulas", [], |r| r.get(0)))
            .await
            .unwrap();
        assert!(
            stored <= crate::db::now_ms() + FUTURE_SLACK_MS,
            "clamped to the server clock plus five minutes"
        );
        // A correct clock six minutes later overrides it.
        let reply = push(
            &mut me,
            json!({ "since": 1, "formulas": [formula(&f, "corrected", now + 6 * 60_000)] }),
        )
        .await;
        assert_eq!(reply["accepted"], 1);
    }

    #[tokio::test]
    async fn pulls_are_paged_and_a_restored_database_resets_clients() {
        let app = TestApp::new();
        let (mut me, user_id) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        let records: Vec<Value> =
            (0..5).map(|i| formula(&id(), &format!("f{i}"), now + i)).collect();
        push(&mut me, json!({ "formulas": records })).await;

        let id_of =
            |raw: &RawValue| serde_json::from_str::<Value>(raw.get()).unwrap()["id"].clone();
        let mut cursor = 0;
        let mut seen = Vec::new();
        loop {
            let uid = user_id.clone();
            let page = app
                .state
                .db
                .read(move |c| pull_page(c, &uid, cursor, &HashSet::new(), 2, PAGE_BYTES))
                .await
                .unwrap();
            assert!(!page.reset);
            seen.extend(page.records.formulas.iter().map(|r| id_of(r)));
            cursor = page.cursor;
            if !page.more {
                break;
            }
            assert!(page.records.formulas.len() == 2, "full pages until the last");
        }
        assert_eq!(seen.len(), 5);
        assert_eq!(cursor, 5);
        let unique: HashSet<_> = seen.iter().map(Value::to_string).collect();
        assert_eq!(unique.len(), 5, "no record twice, none missed");

        // Pages are cut by bytes too: a budget below one record still sends one.
        let uid = user_id.clone();
        let page = app
            .state
            .db
            .read(move |c| pull_page(c, &uid, 0, &HashSet::new(), PAGE, 1))
            .await
            .unwrap();
        assert_eq!((page.records.formulas.len(), page.more, page.cursor), (1, true, 1));

        // A cursor beyond the server's sequence (the database was restored from an older backup):
        // everything is sent again and the client is told to re-upload what it holds.
        let reply = pull(&mut me, 999).await;
        assert_eq!(reply["reset"], true);
        assert_eq!(reply["formulas"].as_array().unwrap().len(), 5);
        assert_eq!(reply["cursor"], 5);
    }

    /// A bake about `bytes` long on the wire.
    fn bake(bytes: usize, at: i64) -> Value {
        json!({ "id": id(), "plan": { "p": "x".repeat(bytes - 160) }, "updated_at": at })
    }

    /// Rows of every synced table, the account's running total, the sum of the stored sizes, and
    /// its change sequence.
    async fn footprint(app: &TestApp, user_id: &str) -> (i64, i64, i64, i64) {
        let tables = ["formulas", "starters", "bakes", "calibrations"];
        let sum = |what: &str| {
            tables.map(|t| format!("(SELECT coalesce({what}, 0) FROM {t})")).join(" + ")
        };
        let rows = app.count(&format!("SELECT {}", sum("count(*)"))).await;
        let summed = app.count(&format!("SELECT {}", sum("sum(bytes)"))).await;
        let (total, seq) = (
            app.count(&format!("SELECT sync_bytes FROM users WHERE id = '{user_id}'")).await,
            app.count(&format!("SELECT sync_seq FROM users WHERE id = '{user_id}'")).await,
        );
        (rows, total, summed, seq)
    }

    #[tokio::test]
    async fn bakes_of_64_kib_are_refused_and_pushes_are_rate_limited() {
        // The review's trigger: bakes with a plan `{"a":[1,1,…]}` just under the former 64 KiB
        // limit, about 1 MiB a push.
        let app = TestApp::new();
        let (mut me, user_id) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        let plan = json!({ "a": vec![1; 32_000] });
        let mut pushes = 0;
        let limited = loop {
            let bakes: Vec<Value> = (0..15)
                .map(|i| json!({ "id": id(), "plan": plan, "updated_at": now + i }))
                .collect();
            let reply = me.json(Method::POST, "/api/v1/sync", &json!({ "bakes": bakes })).await;
            if reply.status != StatusCode::OK || pushes > 40 {
                break reply;
            }
            let reply = reply.json();
            assert_eq!(reply["accepted"], 0);
            assert!(
                reply["rejected"].as_array().unwrap().iter().all(|r| r["reason"] == "too_large")
            );
            pushes += 1;
        };
        assert_eq!(limited.status, StatusCode::TOO_MANY_REQUESTS, "{}", limited.body);
        assert_eq!(limited.json()["error"]["code"], "rate_limited");
        assert_eq!(pushes, PUSHES_PER_WINDOW);
        assert_eq!(footprint(&app, &user_id).await, (0, 0, 0, 0));
        // Pulling (a push without records) is not limited.
        assert_eq!(push(&mut me, json!({ "since": 0 })).await["cursor"], 0);
    }

    #[tokio::test]
    async fn an_account_at_its_quota_is_refused_and_the_database_does_not_grow() {
        let app = TestApp::new();
        let (mut me, user_id) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        let batch = |n: i64| -> Vec<Value> { (0..n).map(|i| bake(15_000, now + i)).collect() };
        // Five pushes of 60 bakes of 15 kB: 4.5 MB.
        let mut cursor = 0;
        for _ in 0..5 {
            let reply = push(&mut me, json!({ "since": cursor, "bakes": batch(60) })).await;
            assert_eq!(reply["accepted"], 60);
            cursor = reply["cursor"].as_i64().unwrap();
        }
        let full = footprint(&app, &user_id).await;
        assert!(full.1 > 4_450_000 && full.1 == full.2, "{full:?}");

        // 60 more would pass 5 MB: the whole push is refused, nothing of it is stored.
        let refused = me
            .json(Method::POST, "/api/v1/sync", &json!({ "since": cursor, "bakes": batch(60) }))
            .await;
        assert_eq!(refused.status, StatusCode::PAYLOAD_TOO_LARGE, "{}", refused.body);
        assert_eq!(refused.json()["error"]["code"], "quota_exceeded");
        assert_eq!(footprint(&app, &user_id).await, full);

        // Unknown tombstones cost nothing; deleting known records frees room.
        let ghosts: Vec<Value> = (0..MAX_PUSHED)
            .map(|_| json!({ "id": id(), "deleted": true, "updated_at": now }))
            .collect();
        let reply = push(&mut me, json!({ "since": cursor, "bakes": ghosts })).await;
        assert_eq!(reply["accepted"], MAX_PUSHED);
        assert_eq!(footprint(&app, &user_id).await, full);
        let stored: Vec<String> = app
            .state
            .db
            .read(|c| {
                let mut stmt = c.prepare("SELECT id FROM bakes LIMIT 100")?;
                stmt.query_map([], |r| r.get(0))?.collect()
            })
            .await
            .unwrap();
        let deletions: Vec<Value> = stored
            .iter()
            .map(|id| json!({ "id": id, "deleted": true, "updated_at": now + 1_000 }))
            .collect();
        let reply = push(&mut me, json!({ "since": cursor, "bakes": deletions })).await;
        assert_eq!(reply["accepted"], 100);
        let freed = footprint(&app, &user_id).await;
        assert!(freed.1 < 3_100_000 && freed.1 == freed.2, "{freed:?}");
        let reply = push(&mut me, json!({ "since": freed.3, "bakes": batch(60) })).await;
        assert_eq!(reply["accepted"], 60);
    }

    #[tokio::test]
    async fn pulls_send_about_a_mebibyte_at_most() {
        let app = TestApp::new();
        let (mut me, _) = app.signed_in("me@example.com").await;
        let now = crate::db::now_ms();
        for round in 0..5 {
            let bakes: Vec<Value> = (0..60).map(|i| bake(15_000, now + round * 60 + i)).collect();
            push(&mut me, json!({ "since": 0, "bakes": bakes })).await;
        }
        let (mut cursor, mut seen, mut pages) = (0, HashSet::new(), 0);
        loop {
            let reply = me.get(&format!("/api/v1/sync?since={cursor}")).await;
            assert!(reply.body.len() < 1024 * 1024 + 1024, "{} bytes", reply.body.len());
            let page = reply.json();
            for id in ids(&page["bakes"]) {
                assert!(seen.insert(id), "no record twice");
            }
            pages += 1;
            cursor = page["cursor"].as_i64().unwrap();
            if page["more"] == false {
                break;
            }
        }
        assert_eq!(seen.len(), 300);
        assert!(pages >= 5, "{pages} pages");
    }

    #[tokio::test]
    async fn expired_tombstones_give_their_bytes_back() {
        let app = TestApp::new();
        let (mut me, user_id) = app.signed_in("me@example.com").await;
        let (old, kept) = (id(), id());
        // A record deleted in 1970, long past `TOMBSTONE_DAYS`, and one alive.
        push(
            &mut me,
            json!({ "formulas": [formula(&old, "Old", 1_000), formula(&kept, "Kept", 1_000)] }),
        )
        .await;
        push(&mut me, json!({ "since": 2, "formulas": [{ "id": old, "deleted": true, "updated_at": 2_000 }] }))
            .await;
        let before = footprint(&app, &user_id).await;
        app.state.db.purge_expired().await.unwrap();
        let after = footprint(&app, &user_id).await;
        assert_eq!(after.0, 1);
        assert!(after.1 < before.1 && after.1 == after.2, "{before:?} → {after:?}");
    }
}
