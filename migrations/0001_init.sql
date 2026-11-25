-- Breadator schema, version 1.
-- Times: `*_at` are Unix seconds, except `updated_at` of synced records, which is the client's
-- clock in milliseconds (last writer wins). Every table of a user cascades when the user is deleted.

CREATE TABLE users (
    id           TEXT PRIMARY KEY,                       -- UUID; also the WebAuthn user handle
    email        TEXT UNIQUE COLLATE NOCASE,             -- verified address; NULL for passkey-only
    display_name TEXT NOT NULL DEFAULT '',
    locale       TEXT NOT NULL DEFAULT 'en' CHECK (locale IN ('fr', 'en')),
    units        TEXT NOT NULL DEFAULT 'metric' CHECK (units IN ('metric', 'us')),
    sync_seq     INTEGER NOT NULL DEFAULT 0,             -- last change sequence handed out
    sync_bytes   INTEGER NOT NULL DEFAULT 0,             -- sum of `bytes` of the synced records (quota)
    created_at   INTEGER NOT NULL
) STRICT;

CREATE TABLE identities (
    id         INTEGER PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    provider   TEXT NOT NULL CHECK (provider IN ('google', 'github')),
    subject    TEXT NOT NULL,
    email      TEXT,
    created_at INTEGER NOT NULL,
    UNIQUE (provider, subject)
) STRICT;
CREATE INDEX identities_user ON identities (user_id);

CREATE TABLE passkeys (
    id            INTEGER PRIMARY KEY,
    user_id       TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    credential_id TEXT NOT NULL UNIQUE,                  -- base64url
    credential    TEXT NOT NULL,                         -- serialized webauthn-rs Passkey
    name          TEXT NOT NULL,
    created_at    INTEGER NOT NULL,
    last_used_at  INTEGER
) STRICT;
CREATE INDEX passkeys_user ON passkeys (user_id);

-- Magic links. Only the SHA-256 of the token is stored; single use. `purpose` 'link' also links a
-- Google/GitHub identity (`provider`, `subject`) to the address's account when the link is used.
CREATE TABLE email_tokens (
    token_hash   TEXT PRIMARY KEY,
    email        TEXT NOT NULL,
    purpose      TEXT NOT NULL DEFAULT 'signin' CHECK (purpose IN ('signin', 'link')),
    provider     TEXT,
    subject      TEXT,
    browser_hash TEXT,                                   -- SHA-256 of the requesting browser's cookie
    created_at   INTEGER NOT NULL,
    expires_at   INTEGER NOT NULL,
    used_at      INTEGER
) STRICT;
CREATE INDEX email_tokens_expiry ON email_tokens (expires_at);

CREATE TABLE sessions (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    token_hash   TEXT NOT NULL UNIQUE,                   -- SHA-256 of the cookie value
    user_id      TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at   INTEGER NOT NULL,
    last_seen_at INTEGER NOT NULL,
    expires_at   INTEGER NOT NULL,
    user_agent   TEXT NOT NULL DEFAULT ''
) STRICT;
CREATE INDEX sessions_user ON sessions (user_id);
CREATE INDEX sessions_expiry ON sessions (expires_at);

-- OAuth authorization requests in flight (state, PKCE verifier, nonce); single use.
CREATE TABLE oauth_states (
    state_hash    TEXT PRIMARY KEY,
    provider      TEXT NOT NULL,
    code_verifier TEXT NOT NULL,
    nonce         TEXT NOT NULL,
    lang          TEXT NOT NULL,
    next          TEXT NOT NULL DEFAULT '',
    link_user_id  TEXT REFERENCES users (id) ON DELETE CASCADE,
    expires_at    INTEGER NOT NULL
) STRICT;
CREATE INDEX oauth_states_expiry ON oauth_states (expires_at);

-- Fixed-window counters; `key` never holds a raw address or email (hashed by the caller).
CREATE TABLE rate_limits (
    key        TEXT NOT NULL,
    window     INTEGER NOT NULL,
    count      INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    PRIMARY KEY (key, window)
) STRICT, WITHOUT ROWID;
CREATE INDEX rate_limits_expiry ON rate_limits (expires_at);

-- Synced records. `id` is chosen by the client (UUIDv7; the culture key for calibrations),
-- `seq` is the user's change sequence at the last write: the pull cursor. `bytes` is the size of
-- the record as sent to clients (tombstones included); `users.sync_bytes` is their running sum.
CREATE TABLE formulas (
    user_id    TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    id         TEXT NOT NULL,
    name       TEXT NOT NULL DEFAULT '',
    json       TEXT NOT NULL DEFAULT '{}',               -- engine Formula
    updated_at INTEGER NOT NULL,
    deleted    INTEGER NOT NULL DEFAULT 0,
    seq        INTEGER NOT NULL,
    bytes      INTEGER NOT NULL,
    PRIMARY KEY (user_id, id)
) STRICT, WITHOUT ROWID;
CREATE INDEX formulas_seq ON formulas (user_id, seq);

CREATE TABLE starters (
    user_id    TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    id         TEXT NOT NULL,
    name       TEXT NOT NULL DEFAULT '',
    flour      TEXT NOT NULL DEFAULT '',
    hydration  REAL NOT NULL DEFAULT 100,
    notes      TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL,
    deleted    INTEGER NOT NULL DEFAULT 0,
    seq        INTEGER NOT NULL,
    bytes      INTEGER NOT NULL,
    PRIMARY KEY (user_id, id)
) STRICT, WITHOUT ROWID;
CREATE INDEX starters_seq ON starters (user_id, seq);

CREATE TABLE bakes (
    user_id    TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    id         TEXT NOT NULL,
    formula_id TEXT,                                     -- soft references: the target may be deleted
    starter_id TEXT,
    plan       TEXT NOT NULL DEFAULT '{}',
    actual     TEXT NOT NULL DEFAULT '{}',               -- phases, temperatures, endpoint, times
    outcome    TEXT NOT NULL DEFAULT '',
    notes      TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL,
    deleted    INTEGER NOT NULL DEFAULT 0,
    seq        INTEGER NOT NULL,
    bytes      INTEGER NOT NULL,
    PRIMARY KEY (user_id, id)
) STRICT, WITHOUT ROWID;
CREATE INDEX bakes_seq ON bakes (user_id, seq);

CREATE TABLE calibrations (
    user_id    TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    id         TEXT NOT NULL,                            -- culture key: starter id or yeast kind
    mean       REAL NOT NULL DEFAULT 0,                  -- log-vigour mean
    variance   REAL NOT NULL DEFAULT 0,
    n          INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL,
    deleted    INTEGER NOT NULL DEFAULT 0,
    seq        INTEGER NOT NULL,
    bytes      INTEGER NOT NULL,
    PRIMARY KEY (user_id, id)
) STRICT, WITHOUT ROWID;
CREATE INDEX calibrations_seq ON calibrations (user_id, seq);

-- Chat (code arrives with the chat feature; the tables exist so that it only adds code).
CREATE TABLE chat_conversations (
    id         TEXT PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title      TEXT NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
) STRICT;
CREATE INDEX chat_conversations_user ON chat_conversations (user_id, updated_at DESC);

CREATE TABLE chat_messages (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id TEXT NOT NULL REFERENCES chat_conversations (id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'tool', 'system')),
    content         TEXT NOT NULL DEFAULT '',
    tool_calls      TEXT,                                -- JSON
    citations       TEXT,                                -- JSON
    model           TEXT,
    tokens_in       INTEGER,
    tokens_out      INTEGER,
    cost_usd        REAL,
    created_at      INTEGER NOT NULL
) STRICT;
CREATE INDEX chat_messages_conversation ON chat_messages (conversation_id, id);

-- Answers and spend by day of an account (`a:` + SHA-256 of its lowercased address, or `u:` +
-- SHA-256 of the user id without one) and of a client network (`c:` + SHA-256 of the IPv4 address
-- or IPv6 /48). Not tied to `users`: deleting an account must not give its address fresh
-- allowances. Rows of earlier months are deleted (the allowances are per day and per month).
-- Costs include reservations: a call is booked at its ceiling before it is made.
CREATE TABLE chat_usage (
    subject    TEXT NOT NULL,
    day        TEXT NOT NULL,                            -- YYYY-MM-DD, UTC
    requests   INTEGER NOT NULL DEFAULT 0,               -- answers
    tokens_in  INTEGER NOT NULL DEFAULT 0,
    tokens_out INTEGER NOT NULL DEFAULT 0,
    cost_usd   REAL NOT NULL DEFAULT 0,
    PRIMARY KEY (subject, day)
) STRICT, WITHOUT ROWID;

-- Global spend ledger of the owner's OpenRouter credit, by month.
CREATE TABLE chat_spend (
    month    TEXT PRIMARY KEY,                           -- YYYY-MM, UTC
    cost_usd REAL NOT NULL DEFAULT 0
) STRICT;
