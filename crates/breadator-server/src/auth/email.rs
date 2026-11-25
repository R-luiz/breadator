//! Sign-in by email magic link.
//!
//! The same page and the same answer for every well-formed address: a link is mailed (new
//! addresses get an account when the link is first used), so nothing tells whether an address is
//! already registered. Tokens are single use, valid 15 minutes, stored as SHA-256 only.
//!
//! **Which browser.** The browser that asks for a link gets a 15-minute cookie whose SHA-256 is
//! stored with the token. Opening the link consumes nothing (mail scanners fetch links): it shows
//! a confirmation button, which the script presses at once only in that browser. Anywhere else the
//! page names the account (address masked) and waits for the visitor to press it, so a link
//! someone else asked for cannot sign a victim into the asker's account, and a scanner that runs
//! scripts cannot use the token up.
//!
//! **Linking.** A link can also confirm a Google or GitHub identity whose verified address already
//! has an account ([`super::oauth`]): same page, same rules; using it links the identity.
//!
//! **Volume.** Limits per address and per client address, and a site-wide daily cap below the
//! mail provider's quota: past it nothing is sent, the page answers as usual (nothing to learn
//! from it) and an error is logged once an hour.

use axum::Form;
use axum::extract::State;
use axum::http::{HeaderValue, header};
use axum::response::{IntoResponse, Redirect, Response};
use rusqlite::OptionalExtension;
use serde::Deserialize;
use serde_json::json;

use super::guard::{clear_cookie, set_cookie};
use super::pages::{self, MessageKind};
use super::ratelimit::hit;
use super::session;
use super::{AppError, ClientIp, random_token, safe_next, sha256_hex};
use crate::app::{AppState, LangPath};
use crate::db;
use crate::i18n::Lang;

const TOKEN_TTL_SECS: i64 = 15 * 60;
/// Magic links mailed to one address per hour, and per client address per hour.
const PER_EMAIL: i64 = 5;
const PER_IP: i64 = 20;
/// Sign-in emails per UTC day for the whole site, below the 100 a day of Resend's free plan.
pub const MAILS_PER_DAY: i64 = 80;

/// Sender of every mail; the domain is verified in Resend.
const FROM: &str = "Breadator <no-reply@robinluiz.fr>";

/// Percent-encoding of everything but RFC 3986 unreserved characters.
pub fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Lowercased, trimmed address if it looks deliverable: one `@`, a dotted domain, no spaces or
/// delimiters, within the SMTP length limits. Not a verification: the mail itself is.
pub fn normalize_email(raw: &str) -> Option<String> {
    let email = raw.trim().to_lowercase();
    let (local, domain) = email.split_once('@')?;
    let ok = email.len() <= 254
        && !local.is_empty()
        && local.len() <= 64
        && domain.contains('.')
        && !domain.starts_with(['.', '-'])
        && !domain.ends_with(['.', '-'])
        && !domain.contains("..")
        && !domain.contains('@')
        && email
            .chars()
            .all(|c| !c.is_whitespace() && !c.is_control() && !"<>,;:\"\\()[]".contains(c));
    ok.then_some(email)
}

/// `a•••@example.com`: enough for the owner to recognise the address, not to learn it.
pub fn mask_email(email: &str) -> String {
    let (local, domain) = email.split_once('@').unwrap_or((email, ""));
    let first: String = local.chars().take(1).collect();
    format!("{first}•••@{domain}")
}

#[derive(Deserialize)]
pub struct SignInForm {
    email: String,
    next: Option<String>,
}

pub async fn request_link(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    ClientIp(ip): ClientIp,
    Form(form): Form<SignInForm>,
) -> Result<Response, AppError> {
    let next = form.next.as_deref().and_then(safe_next).unwrap_or("").to_owned();
    let Some(email) = normalize_email(&form.email) else {
        return Ok(pages::sign_in_with_error(&state, lang, &next, &form.email, "auth-error-email"));
    };
    let Some(cookie) = send_link(&state, lang, &email, &next, &ip, None).await? else {
        return Ok(pages::message(&state, lang, MessageKind::TooManyRequests));
    };
    let mut response = pages::check_email(&state, lang, &email);
    response.headers_mut().append(header::SET_COOKIE, cookie);
    Ok(response)
}

/// A Google or GitHub identity to link to the account of the address when the link is used.
pub struct Identity {
    pub provider: String,
    pub subject: String,
}

enum Mail {
    /// Over the limit of the address or of the client address: nothing stored or sent.
    Limited,
    /// Over the site's daily cap: nothing stored or sent, and the asker is not told. `alert`: the
    /// first time this hour.
    Capped {
        alert: bool,
    },
    Send,
}

/// Stores a link to `email` and mails it in the background. `None` when the address or the
/// client address asked too often; otherwise the `Set-Cookie` that marks the asking browser
/// (also when the daily cap held the mail back, so that the answer stays the same).
pub async fn send_link(
    state: &AppState,
    lang: Lang,
    email: &str,
    next: &str,
    ip: &str,
    identity: Option<Identity>,
) -> Result<Option<HeaderValue>, AppError> {
    let now = db::now();
    let (token, browser) = (random_token(), random_token());
    let provider = identity.as_ref().map(|i| i.provider.clone());
    let row = (sha256_hex(&token), email.to_owned(), sha256_hex(&browser), identity);
    let (email_key, ip_key) = (format!("mail:e:{}", sha256_hex(email)), format!("mail:i:{ip}"));
    let mail = state
        .db
        .write(move |c| {
            // Both counters tick even when the first is already over: no short-circuit.
            let by_email = hit(c, &email_key, 3600, PER_EMAIL, now)?;
            let by_ip = hit(c, &ip_key, 3600, PER_IP, now)?;
            if !(by_email && by_ip) {
                return Ok(Mail::Limited);
            }
            if !hit(c, "mail:day", 86_400, MAILS_PER_DAY, now)? {
                return Ok(Mail::Capped { alert: hit(c, "mail:day:alert", 3600, 1, now)? });
            }
            let (hash, email, browser_hash, identity) = row;
            let purpose = if identity.is_some() { "link" } else { "signin" };
            let (provider, subject) = identity.map(|i| (i.provider, i.subject)).unzip();
            c.execute(
                "INSERT INTO email_tokens (token_hash, email, purpose, provider, subject, browser_hash, created_at, expires_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![hash, email, purpose, provider, subject, browser_hash, now, now + TOKEN_TTL_SECS],
            )?;
            Ok(Mail::Send)
        })
        .await?;
    match mail {
        Mail::Limited => return Ok(None),
        Mail::Capped { alert } => {
            if alert {
                tracing::error!(
                    cap = MAILS_PER_DAY,
                    "daily cap of sign-in emails reached: none is sent until midnight UTC"
                );
            }
        }
        Mail::Send => {
            let mut link = format!(
                "{}/{}/account/sign-in/verify?token={token}",
                state.public_url,
                lang.code()
            );
            if !next.is_empty() {
                link.push_str(&format!("&next={}", urlencode(next)));
            }
            if state.auth.resend_api_key.is_some() {
                // Not awaited: the answer must not depend on the mail provider (or on the address).
                let (task_state, to) = (state.clone(), email.to_owned());
                tokio::spawn(async move {
                    if let Err(e) =
                        deliver(&task_state, &to, lang, &link, provider.as_deref()).await
                    {
                        tracing::error!(error = %e, "cannot send the sign-in email");
                    }
                });
            } else if state.auth.log_links {
                tracing::info!(%email, %link, "RESEND_API_KEY is not set: sign-in link not emailed");
            } else {
                tracing::error!(
                    "RESEND_API_KEY is not set: mail is not configured, sign-in email not sent"
                );
            }
        }
    }
    Ok(Some(set_cookie(
        state.auth.link_cookie(),
        &browser,
        TOKEN_TTL_SECS,
        state.auth.secure_cookies,
    )))
}

async fn deliver(
    state: &AppState,
    to: &str,
    lang: Lang,
    link: &str,
    provider: Option<&str>,
) -> Result<(), String> {
    let key = state.auth.resend_api_key.as_deref().ok_or("no API key")?;
    let tr = lang.tr();
    let (subject, intro, button, ignore) = match provider {
        None => (
            tr.t("email-signin-subject"),
            tr.t("email-signin-intro"),
            tr.t("email-signin-button"),
            tr.t("email-signin-ignore"),
        ),
        Some(provider) => {
            let args = [("provider", super::oauth::provider_label(provider).into())];
            (
                tr.t_args("email-link-subject", &args),
                tr.t_args("email-link-intro", &args),
                tr.t("email-link-button"),
                tr.t_args("email-link-ignore", &args),
            )
        }
    };
    let expiry = tr.t("email-signin-expiry");
    let text = format!("{intro}\n\n{link}\n\n{expiry}\n{ignore}\n");
    let html = format!(
        "<!doctype html><html lang=\"{lang}\"><body style=\"font-family:system-ui,sans-serif;line-height:1.5;color:#2b2118\">\
         <p>{intro}</p>\
         <p><a href=\"{href}\" style=\"display:inline-block;padding:12px 20px;background:#8a5a2b;color:#ffffff;border-radius:999px;text-decoration:none;font-weight:600\">{button}</a></p>\
         <p style=\"color:#6f6152;font-size:14px\">{expiry}<br>{ignore}</p>\
         </body></html>",
        lang = lang.code(),
        intro = html_escape(&intro),
        href = html_escape(link),
        button = html_escape(&button),
        expiry = html_escape(&expiry),
        ignore = html_escape(&ignore),
    );
    let response = state
        .auth
        .http
        .post(format!("{}/emails", state.auth.resend_api_url.trim_end_matches('/')))
        .bearer_auth(key)
        .json(&json!({
            "from": FROM,
            "to": [to],
            "subject": subject,
            "html": html,
            "text": text,
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        let body = response.text().await.unwrap_or_default();
        Err(format!("Resend answered {status}: {}", body.chars().take(300).collect::<String>()))
    }
}

/// A link that can still be used.
pub struct Pending {
    pub email: String,
    /// Provider (`google`, `github`) of the identity the link also links, if any.
    pub provider: Option<String>,
    browser_hash: Option<String>,
}

impl Pending {
    /// Was the link asked for by the browser that sends this link cookie?
    pub fn asked_by(&self, cookie: Option<&str>) -> bool {
        matches!((cookie, &self.browser_hash), (Some(c), Some(h)) if sha256_hex(c) == *h)
    }
}

/// The unused, unexpired link of `token`. Read-only: the confirmation page uses it.
pub async fn pending(state: &AppState, token: &str) -> Result<Option<Pending>, AppError> {
    let hash = sha256_hex(token);
    let now = db::now();
    Ok(state
        .db
        .read(move |c| {
            c.query_row(
                "SELECT email, provider, browser_hash FROM email_tokens
                 WHERE token_hash = ?1 AND used_at IS NULL AND expires_at > ?2",
                rusqlite::params![hash, now],
                |r| Ok(Pending { email: r.get(0)?, provider: r.get(1)?, browser_hash: r.get(2)? }),
            )
            .optional()
        })
        .await?)
}

#[derive(Deserialize)]
pub struct VerifyForm {
    token: String,
    next: Option<String>,
}

enum Used {
    Invalid,
    /// The identity to link belongs to another account by now.
    LinkedElsewhere,
    SignedIn(String),
}

pub async fn verify(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    headers: axum::http::HeaderMap,
    Form(form): Form<VerifyForm>,
) -> Result<Response, AppError> {
    let hash = sha256_hex(form.token.trim());
    let now = db::now();
    let agent =
        headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("").to_owned();
    let used = state
        .db
        .write(move |c| {
            let tx = c.transaction()?;
            // Single use: the first request to flip `used_at` wins, atomically.
            let link: Option<(String, Option<String>, Option<String>)> = tx
                .query_row(
                    "UPDATE email_tokens SET used_at = ?2
                     WHERE token_hash = ?1 AND used_at IS NULL AND expires_at > ?2
                     RETURNING email, provider, subject",
                    rusqlite::params![hash, now],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                )
                .optional()?;
            let Some((email, provider, subject)) = link else { return Ok(Used::Invalid) };
            let identity = provider.zip(subject);
            let owner = match &identity {
                Some((provider, subject)) => session::find_identity(&tx, provider, subject)?,
                None => None,
            };
            let existing = session::find_by_email(&tx, &email)?;
            if owner.is_some() && owner != existing {
                tx.commit()?;
                return Ok(Used::LinkedElsewhere);
            }
            let user_id = match existing {
                Some(id) => id,
                None => {
                    let name = email.split('@').next().unwrap_or("");
                    session::create_user(&tx, Some(&email), name, lang, now)?
                }
            };
            if let Some((provider, subject)) = identity
                && owner.is_none()
            {
                session::add_identity(&tx, &user_id, &provider, &subject, Some(&email), now)?;
            }
            let token = session::create(&tx, &user_id, &agent, now)?;
            tx.commit()?;
            Ok(Used::SignedIn(token))
        })
        .await?;
    let response = match used {
        Used::Invalid => pages::message(&state, lang, MessageKind::LinkInvalid),
        Used::LinkedElsewhere => pages::message(&state, lang, MessageKind::OAuthLinkedElsewhere),
        Used::SignedIn(token) => {
            let target = form
                .next
                .as_deref()
                .and_then(safe_next)
                .map_or_else(|| format!("/{}/account", lang.code()), str::to_owned);
            let mut response = redirect_with_session(&state, &token, &target);
            // The link is used up: the asking browser's mark has no purpose left.
            response.headers_mut().append(
                header::SET_COOKIE,
                clear_cookie(state.auth.link_cookie(), state.auth.secure_cookies),
            );
            response
        }
    };
    Ok(pages::keep_link_private(response))
}

/// Used by the tests and the OAuth/passkey paths to answer with a fresh session.
pub fn redirect_with_session(state: &AppState, token: &str, target: &str) -> Response {
    let mut response = Redirect::to(target).into_response();
    response.headers_mut().append(header::SET_COOKIE, super::guard::session_cookie(state, token));
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses() {
        assert_eq!(normalize_email("  Jane.Doe@Example.COM "), Some("jane.doe@example.com".into()));
        for bad in [
            "",
            "plain",
            "a@b",
            "@example.com",
            "a b@example.com",
            "a@@example.com",
            "a@example..com",
            "a@.example.com",
            "<a@example.com>",
            "a@example.com,b@example.com",
        ] {
            assert_eq!(normalize_email(bad), None, "{bad}");
        }
        assert_eq!(normalize_email(&format!("{}@example.com", "a".repeat(65))), None);
    }

    #[test]
    fn masked_addresses() {
        assert_eq!(mask_email("jane.doe@example.com"), "j•••@example.com");
        assert_eq!(mask_email("a@example.com"), "a•••@example.com");
        assert_eq!(mask_email("éric@example.fr"), "é•••@example.fr");
    }

    #[test]
    fn encoding() {
        assert_eq!(urlencode("/fr/my/formulas?s=a b&c"), "%2Ffr%2Fmy%2Fformulas%3Fs%3Da%20b%26c");
    }
}
