//! Account pages: profile, settings, signed-in devices, sign-in methods, data export and
//! deletion, plus the page listing the formulas saved on this device.
//!
//! Every action is a plain form POST that redirects back to the page (Post/Redirect/Get), so the
//! account works without JavaScript; only passkeys need the script.

use askama::Template;
use axum::Form;
use axum::Json;
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use serde_json::json;

use crate::app::{AppState, LangPath, Layout, render};
use crate::auth::guard::clear_cookie;
use crate::auth::pages::{MessageKind, ProviderButton};
use crate::auth::{ApiError, AppError, CurrentUser, Units, session};
use crate::db;
use crate::i18n::{Lang, Tr};

mod format;
#[cfg(test)]
mod tests;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{lang}/account", get(page))
        .route("/{lang}/account/profile", post(profile))
        .route("/{lang}/account/settings", post(settings))
        .route("/{lang}/account/sign-out", post(sign_out))
        .route("/{lang}/account/sign-out-all", post(sign_out_all))
        .route("/{lang}/account/sessions/{id}/revoke", post(revoke_session))
        .route("/{lang}/account/passkeys/{id}/remove", post(remove_passkey))
        .route("/{lang}/account/identities/{id}/remove", post(remove_identity))
        .route("/{lang}/account/export", get(export))
        .route("/{lang}/account/delete", post(delete))
        .route("/{lang}/account/deleted", get(deleted))
        .route("/{lang}/my/formulas", get(my_formulas))
        .route("/api/v1/me", get(me))
}

/// Success and error banners the page can show after a redirect: codes, never free text.
const MESSAGES: &[&str] = &["saved", "linked", "revoked", "passkey-removed", "identity-removed"];
const ERRORS: &[&str] = &["last-method", "confirm"];

struct Banner {
    class: &'static str,
    text: String,
}

pub struct DeviceView {
    pub id: i64,
    pub label: String,
    pub created: String,
    pub last_seen: String,
    pub current: bool,
}

pub struct IdentityView {
    pub id: i64,
    pub label: String,
    pub email: String,
    pub linked: String,
}

pub struct PasskeyView {
    pub id: i64,
    pub name: String,
    pub created: String,
    pub last_used: String,
}

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage {
    layout: Layout,
    banner: Option<Banner>,
    display_name: String,
    email: String,
    locale: &'static str,
    units: &'static str,
    devices: Vec<DeviceView>,
    identities: Vec<IdentityView>,
    passkeys: Vec<PasskeyView>,
    link_providers: Vec<ProviderButton>,
    passkeys_enabled: bool,
    /// More than one way to sign in: removing one is allowed.
    can_remove: bool,
}

impl AccountPage {
    fn is_locale(&self, code: &str) -> bool {
        self.locale == code
    }

    fn is_units(&self, code: &str) -> bool {
        self.units == code
    }
}

#[derive(Deserialize)]
pub struct PageQuery {
    msg: Option<String>,
    err: Option<String>,
}

fn banner(tr: &Tr, q: &PageQuery) -> Option<Banner> {
    if let Some(code) = q.err.as_deref().filter(|c| ERRORS.contains(c)) {
        return Some(Banner { class: "error", text: tr.t(&format!("account-err-{code}")) });
    }
    q.msg
        .as_deref()
        .filter(|c| MESSAGES.contains(c))
        .map(|code| Banner { class: "notice", text: tr.t(&format!("account-msg-{code}")) })
}

fn provider_label(provider: &str) -> &'static str {
    match provider {
        "google" => "Google",
        "github" => "GitHub",
        _ => "?",
    }
}

async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    current: CurrentUser,
    Query(q): Query<PageQuery>,
) -> Result<Response, AppError> {
    let tr = lang.tr();
    let now = db::now();
    let user_id = current.user.id.clone();
    let (devices, identities, passkeys) = state
        .db
        .read(move |c| {
            Ok((
                session::devices(c, &user_id, now)?,
                session::identities(c, &user_id)?,
                session::passkeys(c, &user_id)?,
            ))
        })
        .await?;
    let user = &current.user;
    let link_providers = ["google", "github"]
        .into_iter()
        .filter(|p| {
            state.auth.provider(p).is_some() && !identities.iter().any(|i| i.provider == *p)
        })
        .map(|p| ProviderButton {
            label: tr.t_args("auth-with-provider", &[("provider", provider_label(p).into())]),
            href: format!("/auth/{p}/start?lang={}&link=1", lang.code()),
        })
        .collect();
    let methods = i64::from(user.email.is_some())
        + i64::try_from(identities.len() + passkeys.len()).unwrap_or(i64::MAX);
    let page = AccountPage {
        layout: Layout::new(
            &state,
            lang,
            "/account",
            "account",
            tr.t("account-title"),
            tr.t("account-description"),
        )
        .private(),
        banner: banner(&tr, &q),
        display_name: user.display_name.clone(),
        email: user.email.clone().unwrap_or_default(),
        locale: user.locale.code(),
        units: user.units.as_str(),
        devices: devices
            .into_iter()
            .map(|d| DeviceView {
                id: d.id,
                label: format::device_label(&tr, &d.user_agent),
                created: format::datetime(d.created_at),
                last_seen: format::datetime(d.last_seen_at),
                current: d.id == current.session_id,
            })
            .collect(),
        identities: identities
            .into_iter()
            .map(|i| IdentityView {
                id: i.id,
                label: provider_label(&i.provider).to_owned(),
                email: i.email.unwrap_or_default(),
                linked: format::date(i.created_at),
            })
            .collect(),
        passkeys: passkeys
            .into_iter()
            .map(|p| PasskeyView {
                id: p.id,
                name: p.name,
                created: format::date(p.created_at),
                last_used: p.last_used_at.map(format::date).unwrap_or_default(),
            })
            .collect(),
        link_providers,
        passkeys_enabled: state.auth.passkeys.is_some(),
        can_remove: methods > 1,
    };
    Ok(render(&page))
}

fn back(lang: Lang, query: &str) -> Response {
    Redirect::to(&format!("/{}/account?{query}", lang.code())).into_response()
}

#[derive(Deserialize)]
pub struct ProfileForm {
    display_name: String,
}

async fn profile(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    current: CurrentUser,
    Form(form): Form<ProfileForm>,
) -> Result<Response, AppError> {
    let name = session::clean_name(&form.display_name);
    let id = current.user.id;
    state
        .db
        .write(move |c| {
            c.execute(
                "UPDATE users SET display_name = ?2 WHERE id = ?1",
                rusqlite::params![id, name],
            )
        })
        .await?;
    Ok(back(lang, "msg=saved"))
}

#[derive(Deserialize)]
pub struct SettingsForm {
    locale: String,
    units: String,
}

async fn settings(
    State(state): State<AppState>,
    current: CurrentUser,
    Form(form): Form<SettingsForm>,
) -> Result<Response, AppError> {
    let locale = Lang::parse(&form.locale).unwrap_or(current.user.locale);
    let units = Units::parse(&form.units).unwrap_or(current.user.units);
    let id = current.user.id;
    state
        .db
        .write(move |c| {
            c.execute(
                "UPDATE users SET locale = ?2, units = ?3 WHERE id = ?1",
                rusqlite::params![id, locale.code(), units.as_str()],
            )
        })
        .await?;
    // The chosen language also answers `/` from now on (see `Lang::negotiate`).
    let mut response = back(locale, "msg=saved");
    if let Ok(cookie) = header::HeaderValue::from_str(&format!(
        "lang={}; Max-Age=31536000; Path=/; SameSite=Lax{}",
        locale.code(),
        if state.auth.secure_cookies { "; Secure" } else { "" }
    )) {
        response.headers_mut().append(header::SET_COOKIE, cookie);
    }
    // And this device shows the chosen units at once, whatever it showed before.
    response.headers_mut().append(
        header::SET_COOKIE,
        crate::pages::home::units_cookie(units, state.auth.secure_cookies),
    );
    Ok(response)
}

#[derive(Deserialize)]
pub struct WipeForm {
    wipe: Option<String>,
}

fn signed_out(state: &AppState, lang: Lang, wipe: bool) -> Response {
    let target = format!("/{}/account/sign-in{}", lang.code(), if wipe { "?wiped=1" } else { "" });
    let mut response = Redirect::to(&target).into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        clear_cookie(state.auth.session_cookie(), state.auth.secure_cookies),
    );
    response
}

async fn sign_out(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    current: CurrentUser,
    Form(form): Form<WipeForm>,
) -> Result<Response, AppError> {
    let (user_id, session_id) = (current.user.id, current.session_id);
    state.db.write(move |c| session::revoke(c, &user_id, session_id)).await?;
    Ok(signed_out(&state, lang, form.wipe.is_some()))
}

async fn sign_out_all(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    current: CurrentUser,
    Form(form): Form<WipeForm>,
) -> Result<Response, AppError> {
    let user_id = current.user.id;
    state.db.write(move |c| session::revoke_all(c, &user_id)).await?;
    Ok(signed_out(&state, lang, form.wipe.is_some()))
}

async fn revoke_session(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, id)): Path<(String, i64)>,
    current: CurrentUser,
) -> Result<Response, AppError> {
    let user_id = current.user.id;
    state.db.write(move |c| session::revoke(c, &user_id, id)).await?;
    if id == current.session_id {
        return Ok(signed_out(&state, lang, false));
    }
    Ok(back(lang, "msg=revoked"))
}

/// Removes a sign-in method unless it is the last one. Count and delete happen in one
/// transaction so two concurrent removals cannot both pass.
async fn remove_method(
    state: &AppState,
    user_id: String,
    table: &'static str,
    id: i64,
) -> Result<bool, AppError> {
    Ok(state
        .db
        .write(move |c| {
            let tx = c.transaction()?;
            if session::method_count(&tx, &user_id)? <= 1 {
                return Ok(false);
            }
            tx.execute(
                &format!("DELETE FROM {table} WHERE id = ?1 AND user_id = ?2"),
                rusqlite::params![id, user_id],
            )?;
            tx.commit()?;
            Ok(true)
        })
        .await?)
}

async fn remove_passkey(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, id)): Path<(String, i64)>,
    current: CurrentUser,
) -> Result<Response, AppError> {
    Ok(if remove_method(&state, current.user.id, "passkeys", id).await? {
        back(lang, "msg=passkey-removed")
    } else {
        back(lang, "err=last-method")
    })
}

async fn remove_identity(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, id)): Path<(String, i64)>,
    current: CurrentUser,
) -> Result<Response, AppError> {
    Ok(if remove_method(&state, current.user.id, "identities", id).await? {
        back(lang, "msg=identity-removed")
    } else {
        back(lang, "err=last-method")
    })
}

/// "Download my data": everything stored about the account, as one JSON document. Records and
/// chat messages are written straight from their rows (the sync quota bounds their size), never
/// gathered into a tree first.
async fn export(State(state): State<AppState>, current: CurrentUser) -> Result<Response, AppError> {
    let user = current.user;
    let user_id = user.id.clone();
    let usage_key = crate::chat::account_subject(&user);
    let now = db::now();
    let account = json!({
        "id": user.id,
        "email": user.email,
        "display_name": user.display_name,
        "locale": user.locale.code(),
        "units": user.units.as_str(),
        "created_at": user.created_at,
    });
    let text = state
        .db
        .read(move |c| {
            let identities: Vec<_> = session::identities(c, &user_id)?
                .into_iter()
                .map(|i| json!({"provider": i.provider, "email": i.email, "linked_at": i.created_at}))
                .collect();
            let passkeys: Vec<_> = session::passkeys(c, &user_id)?
                .into_iter()
                .map(|p| json!({"name": p.name, "created_at": p.created_at, "last_used_at": p.last_used_at}))
                .collect();
            // Signed-in devices: what the account page lists, never a token.
            let devices: Vec<_> = session::devices(c, &user_id, now)?
                .into_iter()
                .map(|d| json!({"user_agent": d.user_agent, "signed_in_at": d.created_at, "last_seen_at": d.last_seen_at}))
                .collect();
            let mut out = format!(
                "{{\n\"exported_at\": {now},\n\"account\": {account},\n\"sign_in_methods\": {},\n\"devices\": {},",
                json!({ "providers": identities, "passkeys": passkeys }),
                json!(devices),
            );
            crate::sync::export_records(c, &user_id, &mut out)?;
            out.push_str(",\n\"chat\": {\"conversations\": ");
            crate::sync::export_chat(c, &user_id, &mut out)?;
            out.push_str(&format!(
                ",\n\"usage\": {}}}\n}}\n",
                crate::chat::export_usage(c, &usage_key)?
            ));
            Ok(out)
        })
        .await?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"breadator-data.json\""),
            (header::CACHE_CONTROL, "no-store"),
        ],
        text,
    )
        .into_response())
}

#[derive(Deserialize)]
pub struct DeleteForm {
    confirm: String,
    wipe: Option<String>,
}

/// Words accepted to confirm the deletion, in either language.
fn confirms_deletion(typed: &str) -> bool {
    let typed = typed.trim().to_lowercase();
    ["delete", "supprimer"].contains(&typed.as_str())
}

async fn delete(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    current: CurrentUser,
    Form(form): Form<DeleteForm>,
) -> Result<Response, AppError> {
    if !confirms_deletion(&form.confirm) {
        return Ok(back(lang, "err=confirm"));
    }
    let (user_id, email) = (current.user.id, current.user.email);
    state
        .db
        .write(move |c| {
            let tx = c.transaction()?;
            // Magic links of the address name it in clear: they go with the account.
            if let Some(email) = &email {
                tx.execute("DELETE FROM email_tokens WHERE email = ?1 COLLATE NOCASE", [email])?;
            }
            // Foreign keys cascade: sessions, sign-in methods, records, chat. The chat
            // allowances (`chat_usage`, keyed on a hash of the address) stay until the month ends.
            tx.execute("DELETE FROM users WHERE id = ?1", [&user_id])?;
            tx.commit()
        })
        .await?;
    let mut response = Redirect::to(&format!(
        "/{}/account/deleted{}",
        lang.code(),
        if form.wipe.is_some() { "?wipe=1" } else { "" }
    ))
    .into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        clear_cookie(state.auth.session_cookie(), state.auth.secure_cookies),
    );
    Ok(response)
}

#[derive(Deserialize)]
pub struct DeletedQuery {
    wipe: Option<String>,
}

async fn deleted(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(q): Query<DeletedQuery>,
) -> Response {
    crate::auth::pages::message_with(&state, lang, MessageKind::AccountDeleted, q.wipe.is_some())
}

/// `GET /api/v1/me`: who is signed in (the front end checks it before syncing).
async fn me(current: CurrentUser) -> Result<Response, ApiError> {
    let user = current.user;
    Ok(Json(json!({
        "id": user.id,
        "email": user.email,
        "display_name": user.display_name,
        "locale": user.locale.code(),
        "units": user.units.as_str(),
    }))
    .into_response())
}

#[derive(Template)]
#[template(path = "my_formulas.html")]
struct MyFormulas {
    layout: Layout,
}

/// The saved formulas live in this browser (IndexedDB) and, when signed in, sync with the
/// account: the list is drawn by the script; the page itself only explains.
async fn my_formulas(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    render(&MyFormulas {
        layout: Layout::new(
            &state,
            lang,
            "/my/formulas",
            "my",
            tr.t("my-title"),
            tr.t("my-description"),
        )
        .private(),
    })
}
