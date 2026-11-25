//! Fixed-window rate limits kept in SQLite, so they survive restarts and need no extra service.

use rusqlite::{Connection, params};

/// Counts one hit on `key` in the current `window_secs` window and says whether it is still
/// within `limit`. Hits above the limit are counted too, but only until the window ends.
pub fn hit(
    conn: &Connection,
    key: &str,
    window_secs: i64,
    limit: i64,
    now: i64,
) -> rusqlite::Result<bool> {
    let window = now / window_secs;
    let count: i64 = conn.query_row(
        "INSERT INTO rate_limits (key, window, count, expires_at) VALUES (?1, ?2, 1, ?3)
         ON CONFLICT (key, window) DO UPDATE SET count = count + 1
         RETURNING count",
        params![key, window, (window + 1) * window_secs],
        |r| r.get(0),
    )?;
    Ok(count <= limit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    #[tokio::test]
    async fn counts_per_key_and_window() {
        let dir = tempfile::tempdir().unwrap();
        let db = Db::open(&dir.path().join("t.sqlite3")).unwrap();
        db.write(|c| {
            for i in 0..3 {
                assert!(hit(c, "k", 60, 3, 1_000)?, "hit {i} is within the limit");
            }
            assert!(!hit(c, "k", 60, 3, 1_001)?);
            // Another key and the next window start from zero.
            assert!(hit(c, "other", 60, 3, 1_001)?);
            assert!(hit(c, "k", 60, 3, 1_060)?);
            Ok(())
        })
        .await
        .unwrap();
    }
}
