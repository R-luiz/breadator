//! Sessions and users in SQL. Handlers call these inside `Db::read` / `Db::write` closures.
//!
//! A session is a random 256-bit token; only its SHA-256 is stored. Expiry slides: each use more
//! than a day after the last pushes `expires_at` to 60 days from now.

use rusqlite::{Connection, OptionalExtension, Row, params};

use super::{Units, User, random_token, sha256_hex};
use crate::i18n::Lang;

pub const TTL_SECS: i64 = 60 * 86_400;
/// The expiry is pushed forward at most this often (one write per day per device).
pub const SLIDE_AFTER_SECS: i64 = 86_400;

const USER_COLUMNS: &str = "u.id, u.email, u.display_name, u.locale, u.units, u.created_at";

fn user_from_row(row: &Row<'_>, at: usize) -> rusqlite::Result<User> {
    let locale: String = row.get(at + 3)?;
    let units: String = row.get(at + 4)?;
    Ok(User {
        id: row.get(at)?,
        email: row.get(at + 1)?,
        display_name: row.get(at + 2)?,
        locale: Lang::parse(&locale).unwrap_or(Lang::En),
        units: Units::parse(&units).unwrap_or(Units::Metric),
        created_at: row.get(at + 5)?,
    })
}

pub struct Found {
    pub session_id: i64,
    pub last_seen_at: i64,
    pub user: User,
}

pub fn lookup(conn: &Connection, token_hash: &str, now: i64) -> rusqlite::Result<Option<Found>> {
    conn.query_row(
        &format!(
            "SELECT s.id, s.last_seen_at, {USER_COLUMNS} FROM sessions s
             JOIN users u ON u.id = s.user_id
             WHERE s.token_hash = ?1 AND s.expires_at > ?2"
        ),
        params![token_hash, now],
        |r| {
            Ok(Found { session_id: r.get(0)?, last_seen_at: r.get(1)?, user: user_from_row(r, 2)? })
        },
    )
    .optional()
}

/// Opens a session for `user_id`; returns the cookie value (the only copy of the token).
pub fn create(
    conn: &Connection,
    user_id: &str,
    user_agent: &str,
    now: i64,
) -> rusqlite::Result<String> {
    let token = random_token();
    let agent: String = user_agent.chars().filter(|c| !c.is_control()).take(300).collect();
    conn.execute(
        "INSERT INTO sessions (token_hash, user_id, created_at, last_seen_at, expires_at, user_agent)
         VALUES (?1, ?2, ?3, ?3, ?4, ?5)",
        params![sha256_hex(&token), user_id, now, now + TTL_SECS, agent],
    )?;
    Ok(token)
}

pub fn slide(conn: &Connection, session_id: i64, now: i64) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE sessions SET last_seen_at = ?2, expires_at = ?3 WHERE id = ?1",
        params![session_id, now, now + TTL_SECS],
    )?;
    Ok(())
}

pub fn revoke(conn: &Connection, user_id: &str, session_id: i64) -> rusqlite::Result<bool> {
    Ok(conn.execute(
        "DELETE FROM sessions WHERE id = ?1 AND user_id = ?2",
        params![session_id, user_id],
    )? > 0)
}

pub fn revoke_all(conn: &Connection, user_id: &str) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM sessions WHERE user_id = ?1", [user_id])
}

pub struct Device {
    pub id: i64,
    pub user_agent: String,
    pub created_at: i64,
    pub last_seen_at: i64,
}

pub fn devices(conn: &Connection, user_id: &str, now: i64) -> rusqlite::Result<Vec<Device>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_agent, created_at, last_seen_at FROM sessions
         WHERE user_id = ?1 AND expires_at > ?2 ORDER BY last_seen_at DESC, id DESC",
    )?;
    stmt.query_map(params![user_id, now], |r| {
        Ok(Device {
            id: r.get(0)?,
            user_agent: r.get(1)?,
            created_at: r.get(2)?,
            last_seen_at: r.get(3)?,
        })
    })?
    .collect()
}

// ---- users -------------------------------------------------------------------------------

pub fn find_by_email(conn: &Connection, email: &str) -> rusqlite::Result<Option<String>> {
    conn.query_row("SELECT id FROM users WHERE email = ?1", [email], |r| r.get(0)).optional()
}

pub fn create_user(
    conn: &Connection,
    email: Option<&str>,
    display_name: &str,
    locale: Lang,
    now: i64,
) -> rusqlite::Result<String> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO users (id, email, display_name, locale, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, email, clean_name(display_name), locale.code(), now],
    )?;
    Ok(id)
}

/// Display name as stored: control characters dropped, whitespace trimmed, 60 characters at most.
pub fn clean_name(name: &str) -> String {
    name.chars().filter(|c| !c.is_control()).take(60).collect::<String>().trim().to_owned()
}

pub fn user(conn: &Connection, user_id: &str) -> rusqlite::Result<Option<User>> {
    conn.query_row(&format!("SELECT {USER_COLUMNS} FROM users u WHERE u.id = ?1"), [user_id], |r| {
        user_from_row(r, 0)
    })
    .optional()
}

// ---- identities ----------------------------------------------------------------------------

pub struct Identity {
    pub id: i64,
    pub provider: String,
    pub email: Option<String>,
    pub created_at: i64,
}

pub fn identities(conn: &Connection, user_id: &str) -> rusqlite::Result<Vec<Identity>> {
    let mut stmt = conn.prepare(
        "SELECT id, provider, email, created_at FROM identities WHERE user_id = ?1 ORDER BY id",
    )?;
    stmt.query_map([user_id], |r| {
        Ok(Identity { id: r.get(0)?, provider: r.get(1)?, email: r.get(2)?, created_at: r.get(3)? })
    })?
    .collect()
}

pub fn find_identity(
    conn: &Connection,
    provider: &str,
    subject: &str,
) -> rusqlite::Result<Option<String>> {
    conn.query_row(
        "SELECT user_id FROM identities WHERE provider = ?1 AND subject = ?2",
        params![provider, subject],
        |r| r.get(0),
    )
    .optional()
}

pub fn add_identity(
    conn: &Connection,
    user_id: &str,
    provider: &str,
    subject: &str,
    email: Option<&str>,
    now: i64,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO identities (user_id, provider, subject, email, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![user_id, provider, subject, email, now],
    )?;
    Ok(())
}

// ---- passkeys ------------------------------------------------------------------------------

pub struct PasskeyRow {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub last_used_at: Option<i64>,
}

pub fn passkeys(conn: &Connection, user_id: &str) -> rusqlite::Result<Vec<PasskeyRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, created_at, last_used_at FROM passkeys WHERE user_id = ?1 ORDER BY id",
    )?;
    stmt.query_map([user_id], |r| {
        Ok(PasskeyRow {
            id: r.get(0)?,
            name: r.get(1)?,
            created_at: r.get(2)?,
            last_used_at: r.get(3)?,
        })
    })?
    .collect()
}

/// Number of ways to sign in: the address (magic link) when there is one, each linked provider,
/// each passkey. The last one can never be removed.
pub fn method_count(conn: &Connection, user_id: &str) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COALESCE((SELECT email IS NOT NULL FROM users WHERE id = ?1), 0)
              + (SELECT count(*) FROM identities WHERE user_id = ?1)
              + (SELECT count(*) FROM passkeys WHERE user_id = ?1)",
        [user_id],
        |r| r.get(0),
    )
}
