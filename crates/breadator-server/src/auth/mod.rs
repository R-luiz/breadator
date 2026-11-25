//! Optional accounts: sessions, sign-in methods, and the extractors other modules reuse.
//!
//! - [`MaybeUser`] / [`CurrentUser`]: who is calling (the chat uses them).
//! - Sign-in methods: email magic link ([`email`]), Google and GitHub ([`oauth`]), passkeys
//!   ([`passkey`]); the pages that drive them are in [`pages`].
//! - [`guard`]: the middleware that resolves the session cookie and checks the origin of every
//!   state-changing request.

pub mod email;
pub mod guard;
pub mod oauth;
pub mod pages;
pub mod passkey;
pub mod ratelimit;
pub mod session;
#[cfg(test)]
mod tests;

use std::convert::Infallible;

use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::Config;
use crate::app::AppState;
use crate::db::DbError;
use crate::i18n::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Metric,
    Us,
}

impl Units {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "metric" => Some(Self::Metric),
            "us" => Some(Self::Us),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Metric => "metric",
            Self::Us => "us",
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    /// UUID, also the WebAuthn user handle.
    pub id: String,
    /// Verified address; `None` only for passkey-only accounts.
    pub email: Option<String>,
    pub display_name: String,
    pub locale: Lang,
    pub units: Units,
    pub created_at: i64,
}

/// The signed-in caller of a request.
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user: User,
    pub session_id: i64,
}

/// `Some` when the request carries a valid session. Never rejects.
#[derive(Debug, Clone)]
pub struct MaybeUser(pub Option<CurrentUser>);

impl<S: Send + Sync> FromRequestParts<S> for MaybeUser {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Infallible> {
        Ok(parts.extensions.get::<MaybeUser>().cloned().unwrap_or(MaybeUser(None)))
    }
}

impl<S: Send + Sync> FromRequestParts<S> for CurrentUser {
    /// API paths answer `401` JSON; pages redirect to the sign-in page and come back after.
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Response> {
        if let Some(MaybeUser(Some(current))) = parts.extensions.get::<MaybeUser>() {
            return Ok(current.clone());
        }
        let path = parts.uri.path();
        if path.starts_with("/api/") {
            return Err(json_error(StatusCode::UNAUTHORIZED, "unauthorized"));
        }
        let lang = path
            .split('/')
            .nth(1)
            .and_then(Lang::parse)
            .unwrap_or_else(|| Lang::negotiate(&parts.headers));
        let back = parts.uri.path_and_query().map_or(path, |p| p.as_str());
        Err(Redirect::to(&format!(
            "/{}/account/sign-in?next={}",
            lang.code(),
            email::urlencode(back)
        ))
        .into_response())
    }
}

/// The caller's address as used for rate limits (see [`guard::client_ip`]).
#[derive(Debug, Clone)]
pub struct ClientIp(pub String);

impl<S: Send + Sync> FromRequestParts<S> for ClientIp {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Infallible> {
        Ok(parts
            .extensions
            .get::<ClientIp>()
            .cloned()
            .unwrap_or_else(|| ClientIp("unknown".to_owned())))
    }
}

/// What the page header needs to know about the caller.
#[derive(Debug, Clone)]
pub struct Badge {
    pub name: String,
}

tokio::task_local! {
    /// Set by [`guard::guard`] around the whole request, so `Layout::new` can show the account
    /// link without every page handler passing the user along.
    static BADGE: Option<Badge>;
    /// The units the current request's results are shown in (see [`current_units`]).
    static UNITS: Units;
}

/// Header badge of the current request's user, `None` when signed out (or outside a request).
pub fn current_badge() -> Option<Badge> {
    BADGE.try_with(Clone::clone).ok().flatten()
}

/// Units of the current request: the device's choice (the `units` cookie), else the account's,
/// else metric (also outside a request). Calculations are always in grams and °C; with
/// [`Units::Us`] the results also show ounces and °F (`Tr::g`, `Tr::temp`).
pub fn current_units() -> Units {
    UNITS.try_with(|u| *u).unwrap_or(Units::Metric)
}

/// Runs `f` as a request shown in `units` would (tests of the formatting).
#[cfg(test)]
pub fn with_units<T>(units: Units, f: impl FnOnce() -> T) -> T {
    UNITS.sync_scope(units, f)
}

/// Server error of a handler: logged, answered as a bare 500 (never details to the client).
#[derive(Debug)]
pub enum AppError {
    Db(DbError),
    Internal(String),
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        Self::Db(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            Self::Db(e) => tracing::error!(error = %e, "database error"),
            Self::Internal(e) => tracing::error!(error = %e, "internal error"),
        }
        (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
    }
}

/// `{"error": {"code": …}}` with a status, for the JSON API.
pub fn json_error(status: StatusCode, code: &str) -> Response {
    (status, axum::Json(json!({ "error": { "code": code } }))).into_response()
}

/// Server error of a JSON handler.
pub struct ApiError(pub StatusCode, pub &'static str);

impl From<DbError> for ApiError {
    fn from(e: DbError) -> Self {
        tracing::error!(error = %e, "database error");
        Self(StatusCode::INTERNAL_SERVER_ERROR, "internal")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        json_error(self.0, self.1)
    }
}

/// 256 bits from the operating system, base64url (43 characters): session tokens, magic links,
/// OAuth state and PKCE verifiers.
pub fn random_token() -> String {
    let mut bytes = [0u8; 32];
    // A failing system RNG cannot be recovered from: predictable tokens would be worse than
    // stopping.
    getrandom::fill(&mut bytes).expect("operating-system randomness is available");
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn sha256_hex(s: &str) -> String {
    Sha256::digest(s.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

/// Local path to return to after signing in: starts with a single `/`, no scheme or host, not
/// an API path.
pub fn safe_next(next: &str) -> Option<&str> {
    let ok = next.starts_with('/')
        && !next.starts_with("//")
        && !next.starts_with("/api/")
        && next.len() <= 400
        && !next.chars().any(|c| c.is_control() || c == '\\');
    ok.then_some(next)
}

pub struct AuthState {
    /// `https` public URL: cookies are `__Host-` prefixed and `Secure`.
    pub secure_cookies: bool,
    /// Expected `Origin` of state-changing requests (`scheme://host[:port]` of `PUBLIC_URL`).
    pub origin: String,
    pub http: reqwest::Client,
    pub resend_api_key: Option<String>,
    pub resend_api_url: String,
    /// `PUBLIC_URL` is on a loopback host (local development): without a mail key, sign-in
    /// links are logged so that the developer can open them. Never anywhere else.
    pub log_links: bool,
    pub google: Option<oauth::OAuthConfig>,
    pub github: Option<oauth::OAuthConfig>,
    pub passkeys: Option<passkey::Passkeys>,
}

impl AuthState {
    pub fn new(config: &Config) -> Result<Self, String> {
        // rustls needs a process-wide crypto provider; ring builds with a plain C compiler.
        // Already installed (tests start several apps in one process) is fine.
        let _ = rustls::crypto::ring::default_provider().install_default();
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!("breadator/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| format!("cannot build the HTTP client: {e}"))?;
        let secure_cookies = config.public_url.starts_with("https://");
        let log_links = is_loopback(&config.public_url);
        if config.resend_api_key.is_none() && !log_links {
            tracing::warn!("RESEND_API_KEY is not set: sign-in emails cannot be sent");
        }
        let passkeys = passkey::Passkeys::new(&config.public_url);
        if passkeys.is_none() {
            tracing::warn!("PUBLIC_URL is not usable as a WebAuthn origin: passkeys are disabled");
        }
        Ok(Self {
            secure_cookies,
            origin: guard::origin_of(&config.public_url),
            http,
            resend_api_key: config.resend_api_key.clone(),
            resend_api_url: config.resend_api_url.clone(),
            log_links,
            google: config.google.clone(),
            github: config.github.clone(),
            passkeys,
        })
    }

    /// Name of the session cookie: `__Host-session` on HTTPS, plain `session` on a local HTTP
    /// development server (browsers refuse `Secure` cookies there).
    pub fn session_cookie(&self) -> &'static str {
        if self.secure_cookies { "__Host-session" } else { "session" }
    }

    pub fn oauth_cookie(&self) -> &'static str {
        if self.secure_cookies { "__Host-oauth" } else { "oauth" }
    }

    /// Cookie of the browser that asked for a sign-in link (see [`email`]).
    pub fn link_cookie(&self) -> &'static str {
        if self.secure_cookies { "__Host-signin" } else { "signin" }
    }

    /// A sign-in link reaches its reader: by mail, or in the log of a local development server.
    pub fn mail_enabled(&self) -> bool {
        self.resend_api_key.is_some() || self.log_links
    }

    pub fn provider(&self, name: &str) -> Option<&oauth::OAuthConfig> {
        match name {
            "google" => self.google.as_ref(),
            "github" => self.github.as_ref(),
            _ => None,
        }
    }
}

/// Is the host of `url` a loopback one (`localhost`, `127.0.0.0/8`, `[::1]`)?
fn is_loopback(url: &str) -> bool {
    let Some(host) = reqwest::Url::parse(url).ok().and_then(|u| u.host_str().map(str::to_owned))
    else {
        return false;
    };
    host.eq_ignore_ascii_case("localhost")
        || host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback())
}

/// Sign-in pages, magic link, OAuth callbacks and the passkey sign-in endpoints.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{lang}/account/sign-in", get(pages::sign_in_page).post(email::request_link))
        .route("/{lang}/account/sign-in/verify", get(pages::verify_page).post(email::verify))
        .route("/auth/{provider}/start", get(oauth::start))
        .route("/auth/{provider}/callback", get(oauth::callback))
        .route("/api/v1/passkeys/register/start", post(passkey::register_start))
        .route("/api/v1/passkeys/register/finish", post(passkey::register_finish))
        .route("/api/v1/passkeys/login/start", post(passkey::login_start))
        .route("/api/v1/passkeys/login/finish", post(passkey::login_finish))
}
