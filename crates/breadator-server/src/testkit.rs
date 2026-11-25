//! Test harness: the real router over a temporary database and a small fixed knowledge base
//! ([`Knowledge::fixture`]), driven with `tower::oneshot`, with a small cookie-keeping client.
//! Mail, OAuth providers and the chat model are wiremock servers; no real network.

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use crate::Config;
use crate::app::{self, AppState};
use crate::auth::oauth::OAuthConfig;
use crate::knowledge::Knowledge;

pub const HTTPS_URL: &str = "https://breadator.test";

/// Every log line of the test binary, each with the id of the thread that wrote it.
///
/// One subscriber for the whole process, installed before the first test application exists.
/// A subscriber per test (thread-local) does not work here: installing one makes `tracing`
/// recompute which call sites are enabled while other test threads are registering theirs, and
/// lines go missing at random.
static LOG_LINES: std::sync::Mutex<Vec<u8>> = std::sync::Mutex::new(Vec::new());

struct LogSink;

impl std::io::Write for LogSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        LOG_LINES.lock().unwrap_or_else(std::sync::PoisonError::into_inner).extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn install_logs() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_ansi(false)
            .with_thread_ids(true)
            .with_writer(|| LogSink)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("the test binary has no other subscriber");
    });
}

/// What the calling test has logged, at every level, since [`Logs::start`]. A test runs on one
/// thread (`#[tokio::test]`), so its lines are those of its thread.
pub struct Logs {
    thread: String,
    from: usize,
}

impl Logs {
    pub fn start() -> Self {
        install_logs();
        let from = LOG_LINES.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len();
        Self { thread: format!("{:?} ", std::thread::current().id()), from }
    }

    pub fn text(&self) -> String {
        let all = LOG_LINES.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        String::from_utf8_lossy(&all[self.from..])
            .lines()
            .filter(|line| line.contains(&self.thread))
            .map(|line| format!("{line}\n"))
            .collect()
    }
}

pub struct TestApp {
    pub router: Router,
    pub state: AppState,
    _dir: tempfile::TempDir,
}

pub fn config(dir: &std::path::Path, public_url: &str) -> Config {
    Config {
        port: 0,
        public_url: public_url.to_owned(),
        database_path: dir.join("test.sqlite3"),
        resend_api_key: None,
        resend_api_url: "http://127.0.0.1:1".to_owned(),
        google: None,
        github: None,
        openrouter_api_key: None,
        openrouter_url: "http://127.0.0.1:1".to_owned(),
    }
}

pub fn oauth_config(base: &str, id: &str) -> OAuthConfig {
    OAuthConfig {
        client_id: id.to_owned(),
        client_secret: "secret".to_owned(),
        auth_url: format!("{base}/authorize"),
        token_url: format!("{base}/token"),
        api_url: base.to_owned(),
        issuer: base.to_owned(),
    }
}

impl TestApp {
    pub fn new() -> Self {
        Self::with(HTTPS_URL, |_| {})
    }

    pub fn with(public_url: &str, tweak: impl FnOnce(&mut Config)) -> Self {
        install_logs();
        let dir = tempfile::tempdir().expect("temporary directory");
        let mut config = config(dir.path(), public_url);
        tweak(&mut config);
        let state =
            AppState::with_knowledge(config, Knowledge::fixture()).expect("test application");
        Self { router: app::router(state.clone()), state, _dir: dir }
    }

    pub fn client(&self) -> Client {
        Client::new(self.router.clone(), &self.state.public_url)
    }

    /// A signed-in client for a new user (session inserted directly: no mail round trip).
    pub async fn signed_in(&self, email: &str) -> (Client, String) {
        let email = email.to_owned();
        let (user_id, token) = self
            .state
            .db
            .write(move |c| {
                let now = crate::db::now();
                let id = crate::auth::session::create_user(
                    c,
                    Some(&email),
                    "Test",
                    crate::i18n::Lang::En,
                    now,
                )?;
                let token = crate::auth::session::create(c, &id, "TestAgent/1.0", now)?;
                Ok((id, token))
            })
            .await
            .expect("session");
        let mut client = self.client();
        client.set_cookie(self.state.auth.session_cookie(), &token);
        (client, user_id)
    }

    pub async fn count(&self, sql: &str) -> i64 {
        let sql = sql.to_owned();
        self.state.db.read(move |c| c.query_row(&sql, [], |r| r.get(0))).await.expect("count")
    }
}

pub struct Reply {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl Reply {
    pub fn json(&self) -> Value {
        serde_json::from_str(&self.body).unwrap_or_else(|e| panic!("not JSON ({e}): {}", self.body))
    }

    pub fn location(&self) -> String {
        self.headers.get(header::LOCATION).and_then(|v| v.to_str().ok()).unwrap_or("").to_owned()
    }

    pub fn set_cookies(&self) -> Vec<String> {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok().map(str::to_owned))
            .collect()
    }

    /// The `Set-Cookie` line of cookie `name`.
    pub fn cookie_line(&self, name: &str) -> Option<String> {
        self.set_cookies().into_iter().find(|c| c.starts_with(&format!("{name}=")))
    }
}

pub struct Client {
    router: Router,
    origin: String,
    cookies: Vec<(String, String)>,
    /// `Origin` header sent with non-GET requests (`None`: none).
    pub origin_header: Option<String>,
    pub extra: Vec<(&'static str, String)>,
}

impl Client {
    fn new(router: Router, origin: &str) -> Self {
        Self {
            router,
            origin: origin.to_owned(),
            cookies: Vec::new(),
            origin_header: Some(origin.to_owned()),
            extra: Vec::new(),
        }
    }

    pub fn set_cookie(&mut self, name: &str, value: &str) {
        self.cookies.retain(|(n, _)| n != name);
        self.cookies.push((name.to_owned(), value.to_owned()));
    }

    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.cookies.iter().find(|(n, _)| n == name).map(|(_, v)| v.as_str())
    }

    pub fn without_origin(mut self) -> Self {
        self.origin_header = None;
        self
    }

    pub fn clone_session(&self) -> Self {
        Self {
            router: self.router.clone(),
            origin: self.origin.clone(),
            cookies: self.cookies.clone(),
            origin_header: self.origin_header.clone(),
            extra: self.extra.clone(),
        }
    }

    pub async fn request(
        &mut self,
        method: Method,
        path: &str,
        content_type: Option<&str>,
        body: Vec<u8>,
    ) -> Reply {
        let mut builder = Request::builder().method(method.clone()).uri(path);
        if !self.cookies.is_empty() {
            let cookie =
                self.cookies.iter().map(|(n, v)| format!("{n}={v}")).collect::<Vec<_>>().join("; ");
            builder = builder.header(header::COOKIE, cookie);
        }
        if method != Method::GET
            && method != Method::HEAD
            && let Some(origin) = &self.origin_header
        {
            builder = builder.header(header::ORIGIN, origin);
        }
        if let Some(ct) = content_type {
            builder = builder.header(header::CONTENT_TYPE, ct);
        }
        for (name, value) in &self.extra {
            builder = builder.header(*name, value);
        }
        let request = builder.body(Body::from(body)).expect("request");
        let response = self.router.clone().oneshot(request).await.expect("response");
        let (parts, body) = response.into_parts();
        let bytes = body.collect().await.expect("body").to_bytes();
        let reply = Reply {
            status: parts.status,
            headers: parts.headers,
            body: String::from_utf8_lossy(&bytes).into_owned(),
        };
        // Keep cookies like a browser: set, replace, expire.
        for line in reply.set_cookies() {
            let pair = line.split(';').next().unwrap_or("");
            if let Some((name, value)) = pair.split_once('=') {
                let expired = line.contains("Max-Age=0");
                self.cookies.retain(|(n, _)| n != name);
                if !expired && !value.is_empty() {
                    self.cookies.push((name.to_owned(), value.to_owned()));
                }
            }
        }
        reply
    }

    pub async fn get(&mut self, path: &str) -> Reply {
        self.request(Method::GET, path, None, Vec::new()).await
    }

    pub async fn form(&mut self, path: &str, fields: &[(&str, &str)]) -> Reply {
        let body = fields
            .iter()
            .map(|(k, v)| {
                format!("{}={}", crate::auth::email::urlencode(k), crate::auth::email::urlencode(v))
            })
            .collect::<Vec<_>>()
            .join("&");
        self.request(
            Method::POST,
            path,
            Some("application/x-www-form-urlencoded"),
            body.into_bytes(),
        )
        .await
    }

    pub async fn json(&mut self, method: Method, path: &str, body: &Value) -> Reply {
        self.request(method, path, Some("application/json"), body.to_string().into_bytes()).await
    }
}

/// Where wiremock's received requests of a path went, with their JSON bodies.
pub async fn received_json(server: &wiremock::MockServer, path: &str) -> Vec<Value> {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|r| r.url.path() == path)
        .filter_map(|r| serde_json::from_slice(&r.body).ok())
        .collect()
}

/// Waits (briefly) for `n` requests to `path`: mail is sent from a spawned task.
pub async fn wait_for_requests(server: &wiremock::MockServer, path: &str, n: usize) -> Vec<Value> {
    for _ in 0..100 {
        let got = received_json(server, path).await;
        if got.len() >= n {
            return got;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    received_json(server, path).await
}
