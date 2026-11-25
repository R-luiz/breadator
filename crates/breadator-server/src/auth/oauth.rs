//! Sign-in with Google (OpenID Connect) and GitHub (OAuth 2): authorization-code flow with
//! `state`, PKCE (S256) and, for Google, a `nonce`.
//!
//! - The request in flight (state, verifier, nonce) is stored server-side, single use, 10 minutes;
//!   the `state` is also in a short-lived cookie, so a callback can only complete in the browser
//!   that started it.
//! - Only verified addresses count: Google's `email_verified`, GitHub's primary verified entry of
//!   `/user/emails`. A new address gets its account at once. An identity not linked yet whose
//!   address already has an account neither signs in nor links by itself: a link is mailed to that
//!   address ([`super::email`]), and using it links the identity and signs in. Linking from the
//!   account page needs no mail: the signed-in session that started it is the proof.
//! - Google's ID token comes straight from the token endpoint over TLS; per OpenID Connect Core
//!   §3.1.3.7 that is the authentication of its issuer, so its signature is not re-verified; the
//!   issuer, audience, expiry and nonce are.

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, header};
use axum::response::{IntoResponse, Redirect, Response};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::guard::{clear_cookie, read_cookie, set_cookie};
use super::pages::{self, MessageKind};
use super::{AppError, ClientIp, MaybeUser, random_token, safe_next, session, sha256_hex};
use crate::app::AppState;
use crate::db;
use crate::i18n::Lang;

const STATE_TTL_SECS: i64 = 600;
/// Authorization requests started per client address per hour.
const STARTS_PER_IP: i64 = 60;

/// The provider's name as shown to people.
pub fn provider_label(provider: &str) -> &'static str {
    match provider {
        "google" => "Google",
        "github" => "GitHub",
        _ => "OAuth",
    }
}

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    /// GitHub REST API base (unused for Google).
    pub api_url: String,
    /// Issuer expected in Google's ID tokens (unused for GitHub).
    pub issuer: String,
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().map(|v| v.trim().to_owned()).filter(|v| !v.is_empty())
}

impl OAuthConfig {
    /// `None` without both `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET`: the button is hidden.
    /// `GOOGLE_AUTH_URL`, `GOOGLE_TOKEN_URL` and `GOOGLE_ISSUER` exist for tests.
    pub fn google_from_env() -> Option<Self> {
        Some(Self {
            client_id: env("GOOGLE_CLIENT_ID")?,
            client_secret: env("GOOGLE_CLIENT_SECRET")?,
            auth_url: env("GOOGLE_AUTH_URL")
                .unwrap_or_else(|| "https://accounts.google.com/o/oauth2/v2/auth".into()),
            token_url: env("GOOGLE_TOKEN_URL")
                .unwrap_or_else(|| "https://oauth2.googleapis.com/token".into()),
            api_url: String::new(),
            issuer: env("GOOGLE_ISSUER").unwrap_or_else(|| "https://accounts.google.com".into()),
        })
    }

    /// `GITHUB_AUTH_URL`, `GITHUB_TOKEN_URL` and `GITHUB_API_URL` exist for tests.
    pub fn github_from_env() -> Option<Self> {
        Some(Self {
            client_id: env("GITHUB_CLIENT_ID")?,
            client_secret: env("GITHUB_CLIENT_SECRET")?,
            auth_url: env("GITHUB_AUTH_URL")
                .unwrap_or_else(|| "https://github.com/login/oauth/authorize".into()),
            token_url: env("GITHUB_TOKEN_URL")
                .unwrap_or_else(|| "https://github.com/login/oauth/access_token".into()),
            api_url: env("GITHUB_API_URL").unwrap_or_else(|| "https://api.github.com".into()),
            issuer: String::new(),
        })
    }
}

fn redirect_uri(state: &AppState, provider: &str) -> String {
    format!("{}/auth/{provider}/callback", state.public_url)
}

#[derive(Deserialize)]
pub struct StartQuery {
    lang: Option<String>,
    next: Option<String>,
    link: Option<String>,
}

/// `GET /auth/{provider}/start`: remembers the request and sends the browser to the provider.
pub async fn start(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(q): Query<StartQuery>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
) -> Result<Response, AppError> {
    let lang = q.lang.as_deref().and_then(Lang::parse).unwrap_or(Lang::En);
    let Some(config) = state.auth.provider(&provider).cloned() else {
        return Ok(pages::message(&state, lang, MessageKind::NotFound));
    };
    let link_user_id = if q.link.is_some() {
        match &user {
            Some(current) => Some(current.user.id.clone()),
            None => {
                return Ok(
                    Redirect::to(&format!("/{}/account/sign-in", lang.code())).into_response()
                );
            }
        }
    } else {
        None
    };

    let (csrf_state, verifier, nonce) = (random_token(), random_token(), random_token());
    let next = q.next.as_deref().and_then(safe_next).unwrap_or("").to_owned();
    let row =
        (sha256_hex(&csrf_state), provider.clone(), verifier.clone(), nonce.clone(), lang, next);
    let now = db::now();
    let allowed = state
        .db
        .write(move |c| {
            if !super::ratelimit::hit(c, &format!("oauth:i:{ip}"), 3600, STARTS_PER_IP, now)? {
                return Ok(false);
            }
            c.execute(
                "INSERT INTO oauth_states (state_hash, provider, code_verifier, nonce, lang, next, link_user_id, expires_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![row.0, row.1, row.2, row.3, row.4.code(), row.5, link_user_id, now + STATE_TTL_SECS],
            )?;
            Ok(true)
        })
        .await?;
    if !allowed {
        return Ok(pages::message(&state, lang, MessageKind::TooManyRequests));
    }

    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let redirect = redirect_uri(&state, &provider);
    let mut params = vec![
        ("response_type", "code"),
        ("client_id", config.client_id.as_str()),
        ("redirect_uri", redirect.as_str()),
        ("state", csrf_state.as_str()),
        ("code_challenge", challenge.as_str()),
        ("code_challenge_method", "S256"),
    ];
    if provider == "google" {
        params.extend([
            ("scope", "openid email profile"),
            ("nonce", nonce.as_str()),
            ("prompt", "select_account"),
        ]);
    } else {
        params.push(("scope", "user:email"));
    }
    let Ok(url) = reqwest::Url::parse_with_params(&config.auth_url, &params) else {
        return Err(AppError::Internal(format!("{provider}: authorization URL is invalid")));
    };
    let mut response = Redirect::to(url.as_str()).into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        set_cookie(
            state.auth.oauth_cookie(),
            &csrf_state,
            STATE_TTL_SECS,
            state.auth.secure_cookies,
        ),
    );
    Ok(response)
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

/// What the provider vouches for.
struct Profile {
    subject: String,
    email: String,
    name: String,
}

enum Outcome {
    SignedIn(String),
    /// The address already has an account and the identity is not linked to it: its owner
    /// confirms by mail first.
    Confirm,
    Linked,
    LinkedElsewhere,
    /// A link request whose session is gone or belongs to someone else.
    WrongSession,
}

/// `GET /auth/{provider}/callback`.
pub async fn callback(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(q): Query<CallbackQuery>,
    headers: HeaderMap,
    MaybeUser(current): MaybeUser,
    ClientIp(ip): ClientIp,
) -> Result<Response, AppError> {
    let mut response =
        complete(&state, &provider, q, &headers, current.map(|c| c.user.id), &ip).await?;
    // One try per cookie, whatever happened.
    response.headers_mut().append(
        header::SET_COOKIE,
        clear_cookie(state.auth.oauth_cookie(), state.auth.secure_cookies),
    );
    Ok(response)
}

async fn complete(
    state: &AppState,
    provider: &str,
    q: CallbackQuery,
    headers: &HeaderMap,
    current_user: Option<String>,
    ip: &str,
) -> Result<Response, AppError> {
    let fallback = Lang::negotiate(headers);
    let Some(config) = state.auth.provider(provider).cloned() else {
        return Ok(pages::message(state, fallback, MessageKind::NotFound));
    };
    // The state must come back in the browser that started the request, and be one we issued.
    let cookie = read_cookie(headers, state.auth.oauth_cookie());
    let (Some(returned), Some(cookie)) = (q.state.as_deref(), cookie) else {
        return Ok(pages::message(state, fallback, MessageKind::OAuthState));
    };
    if returned != cookie {
        return Ok(pages::message(state, fallback, MessageKind::OAuthState));
    }
    let hash = sha256_hex(returned);
    let provider_name = provider.to_owned();
    let now = db::now();
    let stored = state
        .db
        .write(move |c| {
            use rusqlite::OptionalExtension;
            c.query_row(
                "DELETE FROM oauth_states WHERE state_hash = ?1 AND provider = ?2 AND expires_at > ?3
                 RETURNING code_verifier, nonce, lang, next, link_user_id",
                rusqlite::params![hash, provider_name, now],
                |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                        r.get::<_, Option<String>>(4)?,
                    ))
                },
            )
            .optional()
        })
        .await?;
    let Some((verifier, nonce, lang, next, link_user_id)) = stored else {
        return Ok(pages::message(state, fallback, MessageKind::OAuthState));
    };
    let lang = Lang::parse(&lang).unwrap_or(Lang::En);

    let code = match (&q.code, &q.error) {
        (Some(code), None) if !code.is_empty() => code.clone(),
        (_, Some(_)) => return Ok(pages::message(state, lang, MessageKind::OAuthCancelled)),
        _ => return Ok(pages::message(state, lang, MessageKind::OAuthState)),
    };

    let profile = match fetch_profile(state, provider, &config, &code, &verifier, &nonce).await {
        Ok(profile) => profile,
        Err(failure) => {
            tracing::warn!(provider, reason = %failure.log, "OAuth sign-in refused");
            return Ok(pages::message(state, lang, failure.kind));
        }
    };

    let agent =
        headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("").to_owned();
    let provider_name = provider.to_owned();
    let (email, subject) = (profile.email.clone(), profile.subject.clone());
    let outcome = state
        .db
        .write(move |c| {
            let tx = c.transaction()?;
            let outcome = if let Some(link_to) = link_user_id {
                if current_user.as_deref() != Some(link_to.as_str()) {
                    Outcome::WrongSession
                } else {
                    match session::find_identity(&tx, &provider_name, &profile.subject)? {
                        Some(owner) if owner == link_to => Outcome::Linked,
                        Some(_) => Outcome::LinkedElsewhere,
                        None => {
                            session::add_identity(
                                &tx,
                                &link_to,
                                &provider_name,
                                &profile.subject,
                                Some(&profile.email),
                                now,
                            )?;
                            Outcome::Linked
                        }
                    }
                }
            } else {
                match session::find_identity(&tx, &provider_name, &profile.subject)? {
                    Some(id) => Outcome::SignedIn(session::create(&tx, &id, &agent, now)?),
                    None if session::find_by_email(&tx, &profile.email)?.is_some() => {
                        Outcome::Confirm
                    }
                    None => {
                        let name = if profile.name.is_empty() {
                            profile.email.split('@').next().unwrap_or("")
                        } else {
                            &profile.name
                        };
                        let id = session::create_user(&tx, Some(&profile.email), name, lang, now)?;
                        session::add_identity(
                            &tx,
                            &id,
                            &provider_name,
                            &profile.subject,
                            Some(&profile.email),
                            now,
                        )?;
                        Outcome::SignedIn(session::create(&tx, &id, &agent, now)?)
                    }
                }
            };
            tx.commit()?;
            Ok(outcome)
        })
        .await?;

    Ok(match outcome {
        Outcome::SignedIn(token) => {
            let target =
                safe_next(&next).map_or_else(|| format!("/{}/account", lang.code()), str::to_owned);
            super::email::redirect_with_session(state, &token, &target)
        }
        Outcome::Confirm => {
            let identity = super::email::Identity { provider: provider.to_owned(), subject };
            let next = safe_next(&next).unwrap_or("");
            match super::email::send_link(state, lang, &email, next, ip, Some(identity)).await? {
                Some(cookie) => {
                    let mut response = pages::check_email_to_link(state, lang, &email, provider);
                    response.headers_mut().append(header::SET_COOKIE, cookie);
                    response
                }
                None => pages::message(state, lang, MessageKind::TooManyRequests),
            }
        }
        Outcome::Linked => {
            Redirect::to(&format!("/{}/account?msg=linked", lang.code())).into_response()
        }
        Outcome::LinkedElsewhere => pages::message(state, lang, MessageKind::OAuthLinkedElsewhere),
        Outcome::WrongSession => pages::message(state, lang, MessageKind::OAuthState),
    })
}

struct Failure {
    kind: MessageKind,
    log: String,
}

fn failure(kind: MessageKind, log: impl Into<String>) -> Failure {
    Failure { kind, log: log.into() }
}

async fn fetch_profile(
    state: &AppState,
    provider: &str,
    config: &OAuthConfig,
    code: &str,
    verifier: &str,
    nonce: &str,
) -> Result<Profile, Failure> {
    let redirect = redirect_uri(state, provider);
    let form = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect.as_str()),
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
        ("code_verifier", verifier),
    ];
    let http = &state.auth.http;
    let unreachable =
        |what: &str, e: reqwest::Error| failure(MessageKind::OAuthFailed, format!("{what}: {e}"));
    let reply = http
        .post(&config.token_url)
        .header(header::ACCEPT, "application/json")
        .form(&form)
        .send()
        .await
        .map_err(|e| unreachable("token request", e))?;
    let status = reply.status();
    let body: serde_json::Value =
        reply.json().await.map_err(|e| unreachable("token response", e))?;
    if !status.is_success() || body.get("error").is_some() {
        return Err(failure(
            MessageKind::OAuthFailed,
            format!(
                "token endpoint answered {status}: {}",
                body.get("error").and_then(|e| e.as_str()).unwrap_or("?")
            ),
        ));
    }
    if provider == "google" {
        let id_token = body
            .get("id_token")
            .and_then(|t| t.as_str())
            .ok_or_else(|| failure(MessageKind::OAuthFailed, "no id_token"))?;
        google_profile(id_token, config, nonce, db::now())
    } else {
        let access = body
            .get("access_token")
            .and_then(|t| t.as_str())
            .ok_or_else(|| failure(MessageKind::OAuthFailed, "no access_token"))?;
        github_profile(state, config, access).await
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Audience {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Flag {
    Bool(bool),
    Text(String),
}

#[derive(Deserialize)]
struct Claims {
    iss: String,
    aud: Audience,
    exp: i64,
    sub: String,
    nonce: Option<String>,
    email: Option<String>,
    email_verified: Option<Flag>,
    name: Option<String>,
}

fn google_profile(
    id_token: &str,
    config: &OAuthConfig,
    nonce: &str,
    now: i64,
) -> Result<Profile, Failure> {
    let bad = |why: &str| failure(MessageKind::OAuthFailed, format!("id_token: {why}"));
    let payload = id_token.split('.').nth(1).ok_or_else(|| bad("not a JWT"))?;
    let bytes = URL_SAFE_NO_PAD
        .decode(payload.trim_end_matches('='))
        .map_err(|_| bad("payload is not base64url"))?;
    let claims: Claims =
        serde_json::from_slice(&bytes).map_err(|e| bad(&format!("claims: {e}")))?;
    let issuer_ok = claims.iss == config.issuer || claims.iss == "accounts.google.com";
    let audience_ok = match &claims.aud {
        Audience::One(a) => *a == config.client_id,
        Audience::Many(list) => list.contains(&config.client_id),
    };
    if !issuer_ok {
        return Err(bad("wrong issuer"));
    }
    if !audience_ok {
        return Err(bad("wrong audience"));
    }
    if claims.exp <= now {
        return Err(bad("expired"));
    }
    if claims.nonce.as_deref() != Some(nonce) {
        return Err(bad("nonce mismatch"));
    }
    let verified = match claims.email_verified {
        Some(Flag::Bool(b)) => b,
        Some(Flag::Text(t)) => t == "true",
        None => false,
    };
    let email = claims.email.as_deref().and_then(super::email::normalize_email);
    match (email, verified) {
        (Some(email), true) if !claims.sub.is_empty() => Ok(Profile {
            subject: claims.sub,
            email,
            name: session::clean_name(claims.name.as_deref().unwrap_or("")),
        }),
        _ => Err(failure(MessageKind::OAuthNoEmail, "no verified email in the id_token")),
    }
}

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
    login: String,
    name: Option<String>,
}

#[derive(Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

async fn github_profile(
    state: &AppState,
    config: &OAuthConfig,
    access: &str,
) -> Result<Profile, Failure> {
    let api = config.api_url.trim_end_matches('/');
    let get = |path: &str| {
        state
            .auth
            .http
            .get(format!("{api}{path}"))
            .bearer_auth(access)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
    };
    let fail = |what: &str, e: reqwest::Error| {
        failure(MessageKind::OAuthFailed, format!("GitHub {what}: {e}"))
    };
    let user: GithubUser = get("/user")
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|e| fail("/user", e))?
        .json()
        .await
        .map_err(|e| fail("/user body", e))?;
    let emails: Vec<GithubEmail> = get("/user/emails")
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|e| fail("/user/emails", e))?
        .json()
        .await
        .map_err(|e| fail("/user/emails body", e))?;
    let email = emails
        .iter()
        .find(|e| e.primary && e.verified)
        .and_then(|e| super::email::normalize_email(&e.email))
        .ok_or_else(|| {
            failure(MessageKind::OAuthNoEmail, "no primary verified email on the GitHub account")
        })?;
    let name = user.name.as_deref().filter(|n| !n.trim().is_empty()).unwrap_or(&user.login);
    Ok(Profile { subject: user.id.to_string(), email, name: session::clean_name(name) })
}
