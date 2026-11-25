//! Pages of the sign-in flow: the sign-in form, "check your inbox", the link confirmation and the
//! message page every failure ends on.

use askama::Template;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;

use super::guard::read_cookie;
use super::oauth::provider_label;
use super::{AppError, MaybeUser, safe_next};
use crate::app::{AppState, LangPath, Layout, render};
use crate::i18n::Lang;

pub struct ProviderButton {
    pub label: String,
    pub href: String,
}

#[derive(Template)]
#[template(path = "auth_sign_in.html")]
struct SignIn {
    layout: Layout,
    next: String,
    email: String,
    error: Option<String>,
    providers: Vec<ProviderButton>,
    passkeys: bool,
    /// Sign-in links can be sent (a mail key, or a local development server that logs them).
    by_email: bool,
    /// Some way to sign in exists; otherwise the page says accounts are not open yet.
    open: bool,
    wiped: bool,
}

fn sign_in_view(
    state: &AppState,
    lang: Lang,
    next: &str,
    email: &str,
    error: Option<&str>,
    wiped: bool,
) -> SignIn {
    let tr = lang.tr();
    let mut query = format!("lang={}", lang.code());
    if !next.is_empty() {
        query.push_str(&format!("&next={}", super::email::urlencode(next)));
    }
    let providers: Vec<ProviderButton> = ["google", "github"]
        .into_iter()
        .filter(|name| state.auth.provider(name).is_some())
        .map(|name| ProviderButton {
            label: tr.t_args("auth-with-provider", &[("provider", provider_label(name).into())]),
            href: format!("/auth/{name}/start?{query}"),
        })
        .collect();
    SignIn {
        layout: Layout::new(
            state,
            lang,
            "/account/sign-in",
            "account",
            tr.t("auth-sign-in-title"),
            tr.t("auth-sign-in-description"),
        )
        .private(),
        next: next.to_owned(),
        email: email.to_owned(),
        error: error.map(|key| tr.t(key)),
        open: state.auth.mail_enabled() || !providers.is_empty(),
        providers,
        passkeys: state.auth.passkeys.is_some(),
        by_email: state.auth.mail_enabled(),
        wiped,
    }
}

/// The sign-in form again, with `error_key` (a catalog key) above it.
pub fn sign_in_with_error(
    state: &AppState,
    lang: Lang,
    next: &str,
    email: &str,
    error_key: &str,
) -> Response {
    (
        StatusCode::BAD_REQUEST,
        render(&sign_in_view(state, lang, next, email, Some(error_key), false)),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct SignInQuery {
    next: Option<String>,
    wiped: Option<String>,
}

pub async fn sign_in_page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    MaybeUser(current): MaybeUser,
    Query(q): Query<SignInQuery>,
) -> Response {
    let next = q.next.as_deref().and_then(safe_next).unwrap_or("");
    if current.is_some() {
        let target =
            if next.is_empty() { format!("/{}/account", lang.code()) } else { next.to_owned() };
        return Redirect::to(&target).into_response();
    }
    render(&sign_in_view(&state, lang, next, "", None, q.wiped.is_some()))
}

#[derive(Template)]
#[template(path = "auth_check_email.html")]
struct CheckEmail {
    layout: Layout,
    text: String,
}

pub fn check_email(state: &AppState, lang: Lang, email: &str) -> Response {
    let text = lang.tr().t_args("auth-check-text", &[("email", email.to_owned().into())]);
    check_page(state, lang, text)
}

/// After a Google or GitHub sign-in whose address already has an account: the identity is linked
/// once the link mailed to that address is used.
pub fn check_email_to_link(state: &AppState, lang: Lang, email: &str, provider: &str) -> Response {
    let text = lang.tr().t_args(
        "auth-check-link-text",
        &[("email", email.to_owned().into()), ("provider", provider_label(provider).into())],
    );
    check_page(state, lang, text)
}

fn check_page(state: &AppState, lang: Lang, text: String) -> Response {
    let tr = lang.tr();
    render(&CheckEmail {
        layout: Layout::new(
            state,
            lang,
            "/account/sign-in",
            "account",
            tr.t("auth-check-title"),
            tr.t("auth-sign-in-description"),
        )
        .private(),
        text,
    })
}

#[derive(Template)]
#[template(path = "auth_verify.html")]
struct Verify {
    layout: Layout,
    token: String,
    next: String,
    /// The account the link signs in to (address masked), and what it links.
    who: String,
    /// This browser asked for the link: the script presses the button.
    asked_here: bool,
}

#[derive(Deserialize)]
pub struct VerifyQuery {
    token: Option<String>,
    next: Option<String>,
}

/// The page the magic link opens: a button that signs in. Nothing is consumed by loading it. The
/// script presses the button only in the browser that asked for the link; anywhere else the
/// visitor sees whose account it is and decides.
pub async fn verify_page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    headers: HeaderMap,
    Query(q): Query<VerifyQuery>,
) -> Result<Response, AppError> {
    let token = q.token.unwrap_or_default();
    let pending = if token.trim().is_empty() {
        None
    } else {
        super::email::pending(&state, token.trim()).await?
    };
    let Some(pending) = pending else {
        return Ok(keep_link_private(message(&state, lang, MessageKind::LinkInvalid)));
    };
    let tr = lang.tr();
    let email = super::email::mask_email(&pending.email);
    let who = match pending.provider.as_deref() {
        Some(provider) => tr.t_args(
            "auth-verify-who-link",
            &[("email", email.into()), ("provider", provider_label(provider).into())],
        ),
        None => tr.t_args("auth-verify-who", &[("email", email.into())]),
    };
    let page = Verify {
        layout: Layout::new(
            &state,
            lang,
            "/account/sign-in",
            "account",
            tr.t("auth-verify-title"),
            tr.t("auth-sign-in-description"),
        )
        .private(),
        token,
        next: q.next.as_deref().and_then(safe_next).unwrap_or("").to_owned(),
        who,
        asked_here: pending.asked_by(read_cookie(&headers, state.auth.link_cookie())),
    };
    Ok(keep_link_private(render(&page)))
}

/// Headers of a page whose URL carries a sign-in token.
///
/// - `Referrer-Policy: same-origin`: the URL never leaves the site. Not `no-referrer`: on such
///   a page browsers send `Origin: null` with the form, and the cross-site guard would refuse
///   the sign-in itself.
/// - `Cache-Control: no-store, no-transform`: nothing keeps the page, and a proxy may not add
///   to it (Cloudflare then leaves its analytics script out, which would see the address).
pub fn keep_link_private(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store, no-transform"));
    response
}

#[derive(Debug, Clone, Copy)]
pub enum MessageKind {
    NotFound,
    TooManyRequests,
    LinkInvalid,
    OAuthState,
    OAuthCancelled,
    OAuthFailed,
    OAuthNoEmail,
    OAuthLinkedElsewhere,
    AccountDeleted,
}

impl MessageKind {
    fn parts(self) -> (StatusCode, &'static str, &'static str) {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "not-found-title", "not-found-text"),
            Self::TooManyRequests => {
                (StatusCode::TOO_MANY_REQUESTS, "auth-too-many-title", "auth-too-many-text")
            }
            Self::LinkInvalid => {
                (StatusCode::BAD_REQUEST, "auth-link-invalid-title", "auth-link-invalid-text")
            }
            Self::OAuthState => {
                (StatusCode::BAD_REQUEST, "auth-oauth-state-title", "auth-oauth-state-text")
            }
            Self::OAuthCancelled => {
                (StatusCode::OK, "auth-oauth-cancelled-title", "auth-oauth-cancelled-text")
            }
            Self::OAuthFailed => {
                (StatusCode::BAD_GATEWAY, "auth-oauth-failed-title", "auth-oauth-failed-text")
            }
            Self::OAuthNoEmail => {
                (StatusCode::BAD_REQUEST, "auth-oauth-no-email-title", "auth-oauth-no-email-text")
            }
            Self::OAuthLinkedElsewhere => {
                (StatusCode::CONFLICT, "auth-oauth-elsewhere-title", "auth-oauth-elsewhere-text")
            }
            Self::AccountDeleted => {
                (StatusCode::OK, "account-deleted-title", "account-deleted-text")
            }
        }
    }
}

#[derive(Template)]
#[template(path = "auth_message.html")]
struct Message {
    layout: Layout,
    title: String,
    text: String,
    action_href: String,
    action_label: String,
    /// The device's saved data is removed by the script of this page (after deleting an account).
    wipe: bool,
}

pub fn message(state: &AppState, lang: Lang, kind: MessageKind) -> Response {
    message_with(state, lang, kind, false)
}

pub fn message_with(state: &AppState, lang: Lang, kind: MessageKind, wipe: bool) -> Response {
    let tr = lang.tr();
    let (status, title_key, text_key) = kind.parts();
    let (title, text) = (tr.t(title_key), tr.t(text_key));
    let (action_href, action_label) = match kind {
        MessageKind::AccountDeleted | MessageKind::NotFound => {
            (format!("/{}", lang.code()), tr.t("auth-back-home"))
        }
        _ => (format!("/{}/account/sign-in", lang.code()), tr.t("auth-back-sign-in")),
    };
    let page = Message {
        layout: Layout::new(
            state,
            lang,
            "/account/sign-in",
            "account",
            title.clone(),
            text.clone(),
        )
        .private(),
        title,
        text,
        action_href,
        action_label,
        wipe,
    };
    (status, render(&page)).into_response()
}
