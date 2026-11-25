//! Chat tables: conversations and messages of an account, the allowances' usage, and the site's
//! spend.
//!
//! `chat_spend` is the ledger of the owner's OpenRouter credit. Besides its month rows
//! (`2026-11`) it holds one row for the current day (`2026-11-24`). `chat_usage` counts answers
//! and spend per [`Asker`] key (account and client network, both hashed) and per day; it is not
//! tied to `users`, so deleting an account gives its address no fresh allowance.
//!
//! Every model call is booked before it is made ([`reserve`]: its ceiling cost, and the answer
//! for its first call) and corrected once its cost is known ([`settle`]). A crash or a deploy in
//! between leaves the ceiling booked, never nothing.

use rusqlite::{Connection, OptionalExtension, params};

use super::limits::{Asker, DAY_BUDGET_USD, MONTH_BUDGET_USD, Standing};
use super::openrouter::Usage;

pub struct Conversation {
    pub id: String,
    pub title: String,
}

pub struct Message {
    pub id: i64,
    /// `user` or `assistant`.
    pub role: String,
    pub content: String,
    /// JSON: the tools the answer called ([`super::tools::ToolRecord`]).
    pub tool_calls: Option<String>,
    /// JSON: the sources the answer cites ([`super::tools::Source`]).
    pub citations: Option<String>,
}

/// A stored answer, as [`insert_answer`] takes it.
pub struct NewAnswer {
    pub content: String,
    pub tool_calls: String,
    pub citations: String,
    pub model: String,
    pub tokens_in: i64,
    pub tokens_out: i64,
    pub cost_usd: f64,
}

/// The conversation `id` if it belongs to `user_id`.
pub fn conversation(
    conn: &Connection,
    user_id: &str,
    id: &str,
) -> rusqlite::Result<Option<Conversation>> {
    conn.query_row(
        "SELECT id, title FROM chat_conversations WHERE id = ?1 AND user_id = ?2",
        [id, user_id],
        |r| Ok(Conversation { id: r.get(0)?, title: r.get(1)? }),
    )
    .optional()
}

/// The account's conversations, most recently used first.
pub fn conversations(conn: &Connection, user_id: &str) -> rusqlite::Result<Vec<Conversation>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, title FROM chat_conversations WHERE user_id = ?1
         ORDER BY updated_at DESC, id DESC LIMIT 200",
    )?;
    let rows =
        stmt.query_map([user_id], |r| Ok(Conversation { id: r.get(0)?, title: r.get(1)? }))?;
    rows.collect()
}

pub fn messages(conn: &Connection, conversation_id: &str) -> rusqlite::Result<Vec<Message>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, role, content, tool_calls, citations FROM chat_messages
         WHERE conversation_id = ?1 AND role IN ('user', 'assistant') ORDER BY id",
    )?;
    let rows = stmt.query_map([conversation_id], |r| {
        Ok(Message {
            id: r.get(0)?,
            role: r.get(1)?,
            content: r.get(2)?,
            tool_calls: r.get(3)?,
            citations: r.get(4)?,
        })
    })?;
    rows.collect()
}

/// The last `limit` answered questions of a conversation, oldest first: (question, answer).
pub fn turns(
    conn: &Connection,
    conversation_id: &str,
    limit: usize,
) -> rusqlite::Result<Vec<(String, String)>> {
    let all = messages(conn, conversation_id)?;
    let mut turns: Vec<(String, String)> = all
        .windows(2)
        .filter(|pair| pair[0].role == "user" && pair[1].role == "assistant")
        .map(|pair| (pair[0].content.clone(), pair[1].content.clone()))
        .collect();
    turns.drain(..turns.len().saturating_sub(limit));
    Ok(turns)
}

/// Stores a question, creating the conversation `title` is `Some` for.
pub fn insert_question(
    conn: &mut Connection,
    user_id: &str,
    conversation_id: &str,
    title: Option<&str>,
    question: &str,
    now: i64,
) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;
    match title {
        Some(title) => tx.execute(
            "INSERT INTO chat_conversations (id, user_id, title, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![conversation_id, user_id, title, now],
        )?,
        None => tx.execute(
            "UPDATE chat_conversations SET updated_at = ?2 WHERE id = ?1",
            params![conversation_id, now],
        )?,
    };
    tx.execute(
        "INSERT INTO chat_messages (conversation_id, role, content, created_at)
         VALUES (?1, 'user', ?2, ?3)",
        params![conversation_id, question, now],
    )?;
    tx.commit()
}

/// Stores an answer and returns its message id; `None` when the conversation was deleted while
/// the answer was being written.
pub fn insert_answer(
    conn: &Connection,
    conversation_id: &str,
    answer: &NewAnswer,
    now: i64,
) -> rusqlite::Result<Option<i64>> {
    let touched = conn.execute(
        "UPDATE chat_conversations SET updated_at = ?2 WHERE id = ?1",
        params![conversation_id, now],
    )?;
    if touched == 0 {
        return Ok(None);
    }
    conn.execute(
        "INSERT INTO chat_messages
           (conversation_id, role, content, tool_calls, citations, model, tokens_in, tokens_out, cost_usd, created_at)
         VALUES (?1, 'assistant', ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            conversation_id,
            answer.content,
            answer.tool_calls,
            answer.citations,
            answer.model,
            answer.tokens_in,
            answer.tokens_out,
            answer.cost_usd,
            now
        ],
    )?;
    Ok(Some(conn.last_insert_rowid()))
}

/// Deletes a conversation of `user_id` with its messages; `false` when there was none.
pub fn delete_conversation(conn: &Connection, user_id: &str, id: &str) -> rusqlite::Result<bool> {
    let deleted = conn
        .execute("DELETE FROM chat_conversations WHERE id = ?1 AND user_id = ?2", [id, user_id])?;
    Ok(deleted > 0)
}

/// The account's chat usage by day (this month: earlier rows are deleted), for the data export.
pub fn export_usage(conn: &Connection, account: &str) -> rusqlite::Result<serde_json::Value> {
    let mut stmt = conn.prepare(
        "SELECT day, requests, tokens_in, tokens_out, cost_usd FROM chat_usage
         WHERE subject = ?1 ORDER BY day",
    )?;
    let rows = stmt.query_map([account], |r| {
        Ok(serde_json::json!({
            "day": r.get::<_, String>(0)?,
            "answers": r.get::<_, i64>(1)?,
            "tokens_in": r.get::<_, i64>(2)?,
            "tokens_out": r.get::<_, i64>(3)?,
            "cost_usd": r.get::<_, f64>(4)?,
        }))
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>().map(serde_json::Value::Array)
}

/// The site's spend of the day and month of `day` (`YYYY-MM-DD`), the account's answers and
/// spend, and the answers to the client's network; reservations included.
pub fn standing(conn: &Connection, asker: &Asker, day: &str) -> rusqlite::Result<Standing> {
    let month = &day[..7];
    conn.query_row(
        "SELECT
           (SELECT coalesce(sum(cost_usd), 0.0) FROM chat_spend WHERE month = ?2),
           (SELECT coalesce(sum(cost_usd), 0.0) FROM chat_spend WHERE month = ?3),
           (SELECT coalesce(sum(requests), 0) FROM chat_usage WHERE subject = ?1 AND day = ?2),
           (SELECT coalesce(sum(requests), 0) FROM chat_usage
             WHERE subject = ?1 AND day BETWEEN ?3 || '-01' AND ?3 || '-31'),
           (SELECT coalesce(sum(cost_usd), 0.0) FROM chat_usage WHERE subject = ?1 AND day = ?2),
           (SELECT coalesce(sum(requests), 0) FROM chat_usage WHERE subject = ?4 AND day = ?2)",
        params![asker.account, day, month, asker.address],
        |r| {
            Ok(Standing {
                day_spend: r.get(0)?,
                month_spend: r.get(1)?,
                user_day: r.get(2)?,
                user_month: r.get(3)?,
                user_spend: r.get(4)?,
                address_day: r.get(5)?,
            })
        },
    )
}

/// Books a model call before it is made: `ceiling_usd` on the site's day and month, the account
/// and the client network, and, for the first call of an answer (`first`), one answer against
/// their allowances. `false`, and nothing booked, when the site's day or month budget is already
/// reached (by spend or by reservations of calls in flight or lost).
pub fn reserve(
    conn: &mut Connection,
    asker: &Asker,
    day: &str,
    first: bool,
    ceiling_usd: f64,
) -> rusqlite::Result<bool> {
    let tx = conn.transaction()?;
    let spent = |period: &str| -> rusqlite::Result<f64> {
        tx.query_row("SELECT cost_usd FROM chat_spend WHERE month = ?1", [period], |r| r.get(0))
            .optional()
            .map(|cost| cost.unwrap_or(0.0))
    };
    if spent(day)? >= DAY_BUDGET_USD || spent(&day[..7])? >= MONTH_BUDGET_USD {
        return Ok(false);
    }
    for period in [&day[..7], day] {
        tx.execute(
            "INSERT INTO chat_spend (month, cost_usd) VALUES (?1, ?2)
             ON CONFLICT (month) DO UPDATE SET cost_usd = cost_usd + excluded.cost_usd",
            params![period, ceiling_usd],
        )?;
    }
    // Day rows of earlier days, and usage of earlier months, have served.
    tx.execute("DELETE FROM chat_spend WHERE length(month) = 10 AND month < ?1", [day])?;
    tx.execute("DELETE FROM chat_usage WHERE day < ?1", [format!("{}-01", &day[..7])])?;
    for subject in [&asker.account, &asker.address] {
        tx.execute(
            "INSERT INTO chat_usage (subject, day, requests, cost_usd) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT (subject, day) DO UPDATE SET
               requests = requests + excluded.requests,
               cost_usd = cost_usd + excluded.cost_usd",
            params![subject, day, i64::from(first), ceiling_usd],
        )?;
    }
    tx.commit()?;
    Ok(true)
}

/// Replaces the reservation of a call made on `day` by what it cost. `usage` is `None` when the
/// call was never billed: its reservation is taken back, and so is the answer of a first call.
pub fn settle(
    conn: &mut Connection,
    asker: &Asker,
    day: &str,
    first: bool,
    ceiling_usd: f64,
    usage: Option<Usage>,
) -> rusqlite::Result<()> {
    let billed = usage.unwrap_or_default();
    let correction = billed.cost_usd - ceiling_usd;
    let tx = conn.transaction()?;
    // Rows that are gone (a new day or month began meanwhile) have nothing left to correct.
    for period in [&day[..7], day] {
        tx.execute(
            "UPDATE chat_spend SET cost_usd = max(cost_usd + ?2, 0.0) WHERE month = ?1",
            params![period, correction],
        )?;
    }
    for subject in [&asker.account, &asker.address] {
        tx.execute(
            "UPDATE chat_usage SET
               requests = max(requests - ?3, 0),
               tokens_in = tokens_in + ?4,
               tokens_out = tokens_out + ?5,
               cost_usd = max(cost_usd + ?6, 0.0)
             WHERE subject = ?1 AND day = ?2",
            params![
                subject,
                day,
                i64::from(first && usage.is_none()),
                billed.tokens_in,
                billed.tokens_out,
                correction
            ],
        )?;
    }
    tx.commit()
}
