//! Passkeys (WebAuthn): registration from the account page, sign-in with discoverable credentials.
//!
//! The ceremony state (challenge) of a registration or a sign-in lives in memory for five
//! minutes, which is enough on the single machine the app runs on. Library: `webauthn-rs`
//! 0.6, which is pure Rust (RustCrypto): no OpenSSL to build on Windows or in the image.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rusqlite::OptionalExtension;
use serde::Deserialize;
use serde_json::json;
use webauthn_rs::prelude::{
    DiscoverableAuthentication, Passkey, PasskeyRegistration, PublicKeyCredential,
    RegisterPublicKeyCredential, Url, Uuid, Webauthn, WebauthnBuilder,
};

use super::session;
use super::{ApiError, ClientIp, CurrentUser, random_token};
use crate::app::AppState;
use crate::db;

const CEREMONY_TTL: Duration = Duration::from_secs(300);
/// Ceremonies kept at once, per kind; the oldest of the kind go first. Starting a sign-in needs no
/// account, so sign-ins and registrations are capped apart: a flood of one cannot evict the other.
const LOGIN_CAP: usize = 1000;
const REGISTER_CAP: usize = 1000;
const MAX_PASSKEYS: i64 = 10;
/// Sign-ins started, and sign-ins finished, per client address per ten minutes.
const LOGINS_PER_IP: i64 = 30;

enum Ceremony {
    Register { user_id: String, state: PasskeyRegistration },
    Login(DiscoverableAuthentication),
}

impl Ceremony {
    fn is_login(&self) -> bool {
        matches!(self, Self::Login(_))
    }
}

pub struct Passkeys {
    webauthn: Webauthn,
    ceremonies: Mutex<HashMap<String, (Instant, Ceremony)>>,
}

impl Passkeys {
    /// `None` when the public URL cannot be a WebAuthn origin (an IP address, say).
    pub fn new(public_url: &str) -> Option<Self> {
        let origin = Url::parse(public_url).ok()?;
        let rp_id = origin.host_str()?.to_owned();
        let webauthn =
            WebauthnBuilder::new(&rp_id, &origin).ok()?.rp_name("Breadator").build().ok()?;
        Some(Self { webauthn, ceremonies: Mutex::new(HashMap::new()) })
    }

    fn put(&self, ceremony: Ceremony) -> String {
        let id = random_token();
        let login = ceremony.is_login();
        let cap = if login { LOGIN_CAP } else { REGISTER_CAP };
        let mut map = self.ceremonies.lock().unwrap_or_else(|p| p.into_inner());
        let now = Instant::now();
        map.retain(|_, (at, _)| now.duration_since(*at) < CEREMONY_TTL);
        while map.values().filter(|(_, c)| c.is_login() == login).count() >= cap {
            let Some(oldest) = map
                .iter()
                .filter(|(_, (_, c))| c.is_login() == login)
                .min_by_key(|(_, (at, _))| *at)
                .map(|(k, _)| k.clone())
            else {
                break;
            };
            map.remove(&oldest);
        }
        map.insert(id.clone(), (now, ceremony));
        id
    }

    /// A ceremony can be finished once.
    fn take(&self, id: &str) -> Option<Ceremony> {
        let mut map = self.ceremonies.lock().unwrap_or_else(|p| p.into_inner());
        let (at, ceremony) = map.remove(id)?;
        (at.elapsed() < CEREMONY_TTL).then_some(ceremony)
    }
}

fn unavailable() -> ApiError {
    ApiError(StatusCode::NOT_FOUND, "passkeys_unavailable")
}

fn credential_key(cred_id: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(cred_id)
}

/// `POST /api/v1/passkeys/register/start` (signed in).
pub async fn register_start(
    State(state): State<AppState>,
    current: CurrentUser,
) -> Result<Response, ApiError> {
    let passkeys = state.auth.passkeys.as_ref().ok_or_else(unavailable)?;
    let user_id = current.user.id.clone();
    let existing: Vec<String> = state
        .db
        .read({
            let user_id = user_id.clone();
            move |c| {
                let mut stmt = c.prepare("SELECT credential FROM passkeys WHERE user_id = ?1")?;
                stmt.query_map([user_id], |r| r.get(0))?.collect()
            }
        })
        .await?;
    if i64::try_from(existing.len()).unwrap_or(i64::MAX) >= MAX_PASSKEYS {
        return Err(ApiError(StatusCode::CONFLICT, "too_many_passkeys"));
    }
    let exclude: Vec<Vec<u8>> = existing
        .iter()
        .filter_map(|json| serde_json::from_str::<Passkey>(json).ok())
        .map(|p| p.cred_id().clone())
        .collect();
    let handle = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "internal"))?;
    let label = current.user.email.clone().unwrap_or_else(|| current.user.display_name.clone());
    let display = if current.user.display_name.is_empty() {
        label.clone()
    } else {
        current.user.display_name.clone()
    };
    let (options, registration) = passkeys
        .webauthn
        .start_passkey_registration(handle, &label, &display, Some(exclude))
        .map_err(|e| {
            tracing::error!(error = %e, "cannot start a passkey registration");
            ApiError(StatusCode::INTERNAL_SERVER_ERROR, "internal")
        })?;
    let mut options = serde_json::to_value(&options)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "internal"))?;
    // Signing in without typing anything needs a discoverable ("resident") credential; the
    // library asks authenticators to avoid them by default.
    if let Some(selection) = options.pointer_mut("/publicKey/authenticatorSelection") {
        selection["requireResidentKey"] = json!(true);
        selection["residentKey"] = json!("required");
    }
    let ceremony = passkeys.put(Ceremony::Register { user_id, state: registration });
    Ok(Json(json!({ "ceremony": ceremony, "options": options })).into_response())
}

#[derive(Deserialize)]
pub struct RegisterFinish {
    ceremony: String,
    name: Option<String>,
    credential: RegisterPublicKeyCredential,
}

/// `POST /api/v1/passkeys/register/finish` (signed in).
pub async fn register_finish(
    State(state): State<AppState>,
    current: CurrentUser,
    Json(body): Json<RegisterFinish>,
) -> Result<Response, ApiError> {
    let passkeys = state.auth.passkeys.as_ref().ok_or_else(unavailable)?;
    let Some(Ceremony::Register { user_id, state: registration }) = passkeys.take(&body.ceremony)
    else {
        return Err(ApiError(StatusCode::BAD_REQUEST, "ceremony_expired"));
    };
    if user_id != current.user.id {
        return Err(ApiError(StatusCode::BAD_REQUEST, "ceremony_expired"));
    }
    let passkey = passkeys
        .webauthn
        .finish_passkey_registration(&body.credential, &registration)
        .map_err(|e| {
            tracing::warn!(error = %e, "passkey registration refused");
            ApiError(StatusCode::BAD_REQUEST, "registration_failed")
        })?;
    let credential_id = credential_key(passkey.cred_id());
    let serialized = serde_json::to_string(&passkey)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "internal"))?;
    let name = {
        let cleaned = session::clean_name(body.name.as_deref().unwrap_or(""));
        if cleaned.is_empty() { "Passkey".to_owned() } else { cleaned }
    };
    let now = db::now();
    let inserted = state
        .db
        .write(move |c| {
            let count: i64 =
                c.query_row("SELECT count(*) FROM passkeys WHERE user_id = ?1", [&user_id], |r| {
                    r.get(0)
                })?;
            if count >= MAX_PASSKEYS {
                return Ok(Err("too_many_passkeys"));
            }
            match c.execute(
                "INSERT INTO passkeys (user_id, credential_id, credential, name, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![user_id, credential_id, serialized, name, now],
            ) {
                Ok(_) => Ok(Ok(())),
                // The same authenticator credential cannot belong to two accounts.
                Err(rusqlite::Error::SqliteFailure(e, _))
                    if e.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    Ok(Err("already_registered"))
                }
                Err(e) => Err(e),
            }
        })
        .await?;
    match inserted {
        Ok(()) => Ok(Json(json!({ "ok": true })).into_response()),
        Err(code) => Err(ApiError(StatusCode::CONFLICT, code)),
    }
}

/// `POST /api/v1/passkeys/login/start`: a challenge for any discoverable credential.
pub async fn login_start(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
) -> Result<Response, ApiError> {
    let passkeys = state.auth.passkeys.as_ref().ok_or_else(unavailable)?;
    let now = db::now();
    let allowed = state
        .db
        .write(move |c| super::ratelimit::hit(c, &format!("pk:s:{ip}"), 600, LOGINS_PER_IP, now))
        .await?;
    if !allowed {
        return Err(ApiError(StatusCode::TOO_MANY_REQUESTS, "rate_limited"));
    }
    let (options, authentication) =
        passkeys.webauthn.start_discoverable_authentication().map_err(|e| {
            tracing::error!(error = %e, "cannot start a passkey sign-in");
            ApiError(StatusCode::INTERNAL_SERVER_ERROR, "internal")
        })?;
    let ceremony = passkeys.put(Ceremony::Login(authentication));
    Ok(Json(json!({ "ceremony": ceremony, "options": options })).into_response())
}

#[derive(Deserialize)]
pub struct LoginFinish {
    ceremony: String,
    credential: PublicKeyCredential,
}

/// `POST /api/v1/passkeys/login/finish`: verifies the assertion and opens a session.
pub async fn login_finish(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: HeaderMap,
    Json(body): Json<LoginFinish>,
) -> Result<Response, ApiError> {
    let passkeys = state.auth.passkeys.as_ref().ok_or_else(unavailable)?;
    let now = db::now();
    let allowed = state
        .db
        .write(move |c| super::ratelimit::hit(c, &format!("pk:i:{ip}"), 600, LOGINS_PER_IP, now))
        .await?;
    if !allowed {
        return Err(ApiError(StatusCode::TOO_MANY_REQUESTS, "rate_limited"));
    }
    let refused = || ApiError(StatusCode::UNAUTHORIZED, "passkey_refused");
    let Some(Ceremony::Login(authentication)) = passkeys.take(&body.ceremony) else {
        return Err(ApiError(StatusCode::BAD_REQUEST, "ceremony_expired"));
    };
    let (user_handle, cred_id) = passkeys
        .webauthn
        .identify_discoverable_authentication(&body.credential)
        .map_err(|_| refused())?;
    let (user_id, credential_id) = (user_handle.to_string(), credential_key(cred_id));

    let stored: Option<(i64, String)> = state
        .db
        .read({
            let (user_id, credential_id) = (user_id.clone(), credential_id.clone());
            move |c| {
                c.query_row(
                    "SELECT id, credential FROM passkeys WHERE credential_id = ?1 AND user_id = ?2",
                    rusqlite::params![credential_id, user_id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()
            }
        })
        .await?;
    let Some((row_id, json)) = stored else { return Err(refused()) };
    let mut passkey: Passkey = serde_json::from_str(&json).map_err(|_| refused())?;
    let result = passkeys
        .webauthn
        .finish_discoverable_authentication_v2(&body.credential, authentication, &passkey)
        .map_err(|e| {
            tracing::warn!(error = %e, "passkey sign-in refused");
            refused()
        })?;
    // Keep the signature counter (clone detection) and backup flags current.
    let updated = if result.needs_update() && passkey.update_credential(&result) == Some(true) {
        serde_json::to_string(&passkey).ok()
    } else {
        None
    };
    let agent =
        headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("").to_owned();
    let (token, locale) = state
        .db
        .write(move |c| {
            let tx = c.transaction()?;
            match updated {
                Some(json) => tx.execute(
                    "UPDATE passkeys SET credential = ?2, last_used_at = ?3 WHERE id = ?1",
                    rusqlite::params![row_id, json, now],
                )?,
                None => tx.execute(
                    "UPDATE passkeys SET last_used_at = ?2 WHERE id = ?1",
                    rusqlite::params![row_id, now],
                )?,
            };
            let token = session::create(&tx, &user_id, &agent, now)?;
            let locale = session::user(&tx, &user_id)?.map_or(crate::i18n::Lang::En, |u| u.locale);
            tx.commit()?;
            Ok((token, locale))
        })
        .await?;
    let mut response =
        Json(json!({ "ok": true, "redirect": format!("/{}/account", locale.code()) }))
            .into_response();
    response.headers_mut().append(header::SET_COOKIE, super::guard::session_cookie(&state, &token));
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_flood_of_sign_ins_does_not_evict_registrations() {
        let passkeys = Passkeys::new("https://breadator.test").expect("WebAuthn origin");
        let (_, registration) = passkeys
            .webauthn
            .start_passkey_registration(Uuid::from_u128(1), "me@example.com", "Me", None)
            .unwrap();
        let registered =
            passkeys.put(Ceremony::Register { user_id: "me".into(), state: registration });
        let mut first_login = None;
        // More sign-ins than both caps together: the shared cap evicted the registration.
        for _ in 0..=LOGIN_CAP + REGISTER_CAP {
            let (_, login) = passkeys.webauthn.start_discoverable_authentication().unwrap();
            let id = passkeys.put(Ceremony::Login(login));
            first_login.get_or_insert(id);
        }
        let logins = {
            let map = passkeys.ceremonies.lock().unwrap();
            map.values().filter(|(_, c)| c.is_login()).count()
        };
        assert_eq!(logins, LOGIN_CAP, "sign-ins are capped, the oldest going first");
        assert!(passkeys.take(&first_login.unwrap()).is_none());
        assert!(matches!(passkeys.take(&registered), Some(Ceremony::Register { .. })));
    }
}
