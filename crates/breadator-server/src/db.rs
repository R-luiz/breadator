//! SQLite: one writer connection behind a mutex, a small pool of read-only connections, and the
//! numbered migrations of `migrations/` embedded in the binary and applied at start-up.
//!
//! rusqlite is synchronous: every call runs on tokio's blocking pool. WAL lets readers proceed
//! while the writer commits; Litestream replicates the WAL (see `docs/OPERATIONS.md`).

use std::fmt;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use include_dir::{Dir, include_dir};
use rusqlite::{Connection, OpenFlags};

static MIGRATIONS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../../migrations");

const READERS: usize = 3;

#[derive(Debug)]
pub enum DbError {
    Sqlite(rusqlite::Error),
    /// The blocking task panicked or was cancelled.
    Task(String),
    Migration(String),
    Io(std::io::Error),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(e) => write!(f, "sqlite: {e}"),
            Self::Task(e) => write!(f, "database task failed: {e}"),
            Self::Migration(e) => write!(f, "migration: {e}"),
            Self::Io(e) => write!(f, "database directory: {e}"),
        }
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e)
    }
}

struct Pool {
    writer: Mutex<Connection>,
    readers: Vec<Mutex<Connection>>,
    next: AtomicUsize,
}

#[derive(Clone)]
pub struct Db(Arc<Pool>);

/// Unix time in seconds.
pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(0))
}

/// Unix time in milliseconds.
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(0))
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    // A panic while holding the lock cannot leave a half-applied transaction behind (SQLite
    // rolls back on drop), so the connection stays usable.
    m.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Db {
    /// Opens (creating it and its directory if needed) the database at `path`, applies the
    /// pending migrations and opens the readers.
    pub fn open(path: &Path) -> Result<Self, DbError> {
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir).map_err(DbError::Io)?;
        }
        let mut writer = Connection::open(path)?;
        writer.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;
             PRAGMA temp_store = MEMORY;",
        )?;
        migrate(&mut writer)?;
        let mut readers = Vec::with_capacity(READERS);
        for _ in 0..READERS {
            let reader = Connection::open_with_flags(
                path,
                OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )?;
            reader.execute_batch("PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;")?;
            readers.push(Mutex::new(reader));
        }
        Ok(Self(Arc::new(Pool { writer: Mutex::new(writer), readers, next: AtomicUsize::new(0) })))
    }

    /// Runs `f` on the writer connection (one at a time, on the blocking pool).
    pub async fn write<T, F>(&self, f: F) -> Result<T, DbError>
    where
        F: FnOnce(&mut Connection) -> rusqlite::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || f(&mut lock(&pool.writer)))
            .await
            .map_err(|e| DbError::Task(e.to_string()))?
            .map_err(DbError::from)
    }

    /// Runs `f` on a read-only connection.
    pub async fn read<T, F>(&self, f: F) -> Result<T, DbError>
    where
        F: FnOnce(&Connection) -> rusqlite::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || {
            let i = pool.next.fetch_add(1, Ordering::Relaxed) % pool.readers.len();
            f(&lock(&pool.readers[i]))
        })
        .await
        .map_err(|e| DbError::Task(e.to_string()))?
        .map_err(DbError::from)
    }

    /// Deletes what has expired: sessions, magic links, OAuth requests, rate-limit windows, chat
    /// usage of earlier months, and tombstones of synced records older than `TOMBSTONE_DAYS`.
    pub async fn purge_expired(&self) -> Result<(), DbError> {
        self.write(|c| {
            let now = now();
            let tx = c.transaction()?;
            tx.execute("DELETE FROM sessions WHERE expires_at <= ?1", [now])?;
            // Used and expired magic links are kept a day for troubleshooting.
            tx.execute("DELETE FROM email_tokens WHERE expires_at <= ?1", [now - 86_400])?;
            tx.execute("DELETE FROM oauth_states WHERE expires_at <= ?1", [now])?;
            tx.execute("DELETE FROM rate_limits WHERE expires_at <= ?1", [now])?;
            // Chat allowances are per day and per month (UTC).
            tx.execute(
                "DELETE FROM chat_usage WHERE day < strftime('%Y-%m-01', ?1, 'unixepoch')",
                [now],
            )?;
            let cutoff = (now - TOMBSTONE_DAYS * 86_400) * 1000;
            for table in ["formulas", "starters", "bakes", "calibrations"] {
                // The account's stored size (its sync quota) gives back what is deleted.
                tx.execute(
                    &format!(
                        "UPDATE users SET sync_bytes = sync_bytes - (
                           SELECT coalesce(sum(bytes), 0) FROM {table}
                           WHERE user_id = users.id AND deleted = 1 AND updated_at < ?1)
                         WHERE id IN (SELECT user_id FROM {table} WHERE deleted = 1 AND updated_at < ?1)"
                    ),
                    [cutoff],
                )?;
                tx.execute(
                    &format!("DELETE FROM {table} WHERE deleted = 1 AND updated_at < ?1"),
                    [cutoff],
                )?;
            }
            tx.commit()
        })
        .await
    }
}

/// How long a deletion is remembered so that a device that was offline can still learn of it.
pub const TOMBSTONE_DAYS: i64 = 365;

/// `NNNN_name.sql` files of `migrations/`, in order. Numbers must run 1, 2, 3… without gaps.
fn migrations() -> Result<Vec<(u32, &'static str)>, DbError> {
    let mut found = Vec::new();
    for file in MIGRATIONS.files() {
        let name = file.path().file_name().and_then(|n| n.to_str()).unwrap_or("");
        let Some(stem) = name.strip_suffix(".sql") else { continue };
        let number = stem
            .split('_')
            .next()
            .and_then(|n| n.parse::<u32>().ok())
            .ok_or_else(|| DbError::Migration(format!("{name}: expected NNNN_name.sql")))?;
        let sql = file
            .contents_utf8()
            .ok_or_else(|| DbError::Migration(format!("{name} is not UTF-8")))?;
        found.push((number, sql));
    }
    found.sort_by_key(|(n, _)| *n);
    for (i, (n, _)) in found.iter().enumerate() {
        if usize::try_from(*n).ok() != Some(i + 1) {
            return Err(DbError::Migration(format!("migration numbers must run 1..n, found {n}")));
        }
    }
    Ok(found)
}

/// Applies the migrations above the database's `user_version`, each in its own transaction.
fn migrate(conn: &mut Connection) -> Result<(), DbError> {
    let all = migrations()?;
    let current: u32 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    let latest = u32::try_from(all.len()).unwrap_or(u32::MAX);
    if current > latest {
        return Err(DbError::Migration(format!(
            "the database is at schema version {current} but this build only knows {latest}: \
             refusing to start on a newer schema"
        )));
    }
    for (number, sql) in all.into_iter().filter(|(n, _)| *n > current) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", number)?;
        tx.commit()?;
        tracing::info!(version = number, "database migrated");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn opens_migrates_and_reopens() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("test.sqlite3");
        let db = Db::open(&path).unwrap();
        let version: u32 =
            db.read(|c| c.query_row("PRAGMA user_version", [], |r| r.get(0))).await.unwrap();
        assert_eq!(usize::try_from(version).unwrap(), migrations().unwrap().len());
        drop(db);
        // A second start finds the schema up to date.
        Db::open(&path).unwrap();
    }

    #[tokio::test]
    async fn pragmas_and_fts5() {
        let dir = tempfile::tempdir().unwrap();
        let db = Db::open(&dir.path().join("t.sqlite3")).unwrap();
        let (mode, fk): (String, i64) = db
            .write(|c| {
                Ok((
                    c.query_row("PRAGMA journal_mode", [], |r| r.get(0))?,
                    c.query_row("PRAGMA foreign_keys", [], |r| r.get(0))?,
                ))
            })
            .await
            .unwrap();
        assert_eq!((mode.as_str(), fk), ("wal", 1));
        // The search feature needs FTS5 in the bundled SQLite.
        db.write(|c| {
            c.execute_batch(
                "CREATE VIRTUAL TABLE fts USING fts5(body);
                 INSERT INTO fts VALUES ('levain and poolish');",
            )?;
            let hits: i64 =
                c.query_row("SELECT count(*) FROM fts WHERE fts MATCH 'poolish'", [], |r| {
                    r.get(0)
                })?;
            assert_eq!(hits, 1);
            Ok(())
        })
        .await
        .unwrap();
        // Readers are read-only.
        assert!(db.read(|c| c.execute("DELETE FROM users", [])).await.is_err());
    }

    #[test]
    fn refuses_a_newer_schema() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.sqlite3");
        Db::open(&path).unwrap();
        let conn = Connection::open(&path).unwrap();
        conn.pragma_update(None, "user_version", 999).unwrap();
        drop(conn);
        assert!(matches!(Db::open(&path), Err(DbError::Migration(_))));
    }
}
