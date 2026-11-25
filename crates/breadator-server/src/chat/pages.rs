//! The chat pages and the streamed answer.
//!
//! Without JavaScript a question is a form POST: the answer is written in its own task and the
//! browser is sent to the conversation, whose page waits for the answer before it is sent
//! (Post/Redirect/Get, so a reload never asks twice). With JavaScript (`web/src/chat.ts`) the
//! question goes to `POST /api/v1/chat`, which streams the answer as server-sent events: text
//! as it is written, a line per tool, then the stored answer rendered with its sources.

use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use askama::Template;
use axum::body::{Body, Bytes};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{Form, Json};
use futures_core::Stream;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
use tokio::sync::watch;

use super::answer::{self, Event, Job};
use super::limits::{self, Asker, Refusal, Standing};
use super::store::{self, Conversation, Message};
use super::tools::{Kind, Source, ToolRecord};
use super::{Admission, render};
use crate::app::{AppState, LangPath, Layout, not_found_page, render as render_page};
use crate::auth::{AppError, ClientIp, CurrentUser, MaybeUser, email::urlencode};
use crate::db::{self, DbError};
use crate::i18n::{Lang, Tr};

/// How long the page of a conversation waits for an answer being written (three calls at most,
/// each with its own timeout).
const WAIT: Duration = Duration::from_secs(150);
/// Comment lines keeping a quiet event stream open through proxies.
const PING_EVERY: Duration = Duration::from_secs(15);
const TITLE_CHARS: usize = 80;

pub struct Banner {
    pub class: &'static str,
    pub text: String,
}

pub struct ConversationLink {
    pub href: String,
    pub title: String,
    pub current: bool,
}

pub struct MessageView {
    pub user: bool,
    /// The question (escaped by the template).
    pub text: String,
    /// The answer, rendered by [`render::message_html`].
    pub html: String,
}

pub struct CurrentView {
    pub id: String,
    pub title: String,
    pub delete_href: String,
    pub messages: Vec<MessageView>,
    /// The last question got no answer (the model failed): it is offered again in the form.
    pub unanswered: bool,
}

#[derive(Template)]
#[template(path = "chat.html")]
struct ChatPage {
    layout: Layout,
    available: bool,
    signed_in: bool,
    sign_in_href: String,
    conversations: Vec<ConversationLink>,
    current: Option<CurrentView>,
    banner: Option<Banner>,
    /// Why the form cannot be used right now (budget or allowance reached).
    blocked: Option<String>,
    draft: String,
    left: String,
    max_chars: usize,
}

fn layout(state: &AppState, lang: Lang, path: &str) -> Layout {
    let tr = lang.tr();
    Layout::new(state, lang, path, "chat", tr.t("chat-title"), tr.t("chat-description")).private()
}

/// Conversations, answers and allowances are personal: never stored by a cache.
fn no_store(mut response: Response) -> Response {
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn respond(page: &ChatPage, status: StatusCode) -> Response {
    let mut response = render_page(page);
    if response.status() == StatusCode::OK {
        *response.status_mut() = status;
    }
    no_store(response)
}

pub fn refusal_text(tr: &Tr, refusal: Refusal) -> String {
    let key = format!("chat-refusal-{}", refusal.code());
    match refusal {
        Refusal::TooLong => {
            tr.t_args(&key, &[("max", limits::QUESTION_MAX_CHARS.to_string().into())])
        }
        Refusal::UserDay => {
            tr.t_args(&key, &[("n", limits::USER_ANSWERS_PER_DAY.to_string().into())])
        }
        Refusal::UserMonth => {
            tr.t_args(&key, &[("n", limits::USER_ANSWERS_PER_MONTH.to_string().into())])
        }
        Refusal::AddressDay => {
            tr.t_args(&key, &[("n", limits::ADDRESS_ANSWERS_PER_DAY.to_string().into())])
        }
        _ => tr.t(&key),
    }
}

struct Loaded {
    conversations: Vec<Conversation>,
    current: Option<(Conversation, Vec<Message>)>,
    standing: Standing,
}

/// The account's conversations and standing, with conversation `current`; `None` when that
/// conversation is not the account's.
async fn load(
    state: &AppState,
    user_id: &str,
    asker: &Asker,
    current: Option<&str>,
) -> Result<Option<Loaded>, DbError> {
    let (user_id, asker) = (user_id.to_owned(), asker.clone());
    let current = current.map(str::to_owned);
    let day = limits::utc_day(db::now());
    state
        .db
        .read(move |c| {
            let current = match current {
                None => None,
                Some(id) => match store::conversation(c, &user_id, &id)? {
                    None => return Ok(None),
                    Some(conversation) => {
                        let messages = store::messages(c, &conversation.id)?;
                        Some((conversation, messages))
                    }
                },
            };
            Ok(Some(Loaded {
                conversations: store::conversations(c, &user_id)?,
                current,
                standing: store::standing(c, &asker, &day)?,
            }))
        })
        .await
}

fn parsed<T: serde::de::DeserializeOwned + Default>(json: Option<&str>) -> T {
    json.and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default()
}

fn page(
    state: &AppState,
    layout: Layout,
    loaded: Loaded,
    banner: Option<Banner>,
    draft: String,
) -> ChatPage {
    let tr = layout.tr;
    let lang = tr.lang();
    let current_id = loaded.current.as_ref().map(|(c, _)| c.id.clone());
    let conversations = loaded
        .conversations
        .into_iter()
        .map(|c| ConversationLink {
            href: format!("/{lang}/chat/{}", c.id),
            current: current_id.as_deref() == Some(c.id.as_str()),
            title: c.title,
        })
        .collect();
    let mut draft = draft;
    let current = loaded.current.map(|(conversation, messages)| {
        let unanswered = messages.last().is_some_and(|m| m.role == "user");
        if unanswered && draft.is_empty() {
            draft = messages.last().map(|m| m.content.clone()).unwrap_or_default();
        }
        let messages = messages
            .into_iter()
            .map(|m| {
                if m.role == "user" {
                    return MessageView { user: true, text: m.content, html: String::new() };
                }
                let cited: Vec<Source> = parsed(m.citations.as_deref());
                let tools: Vec<ToolRecord> = parsed(m.tool_calls.as_deref());
                let html =
                    render::message_html(&state.knowledge, &tr, m.id, &m.content, &cited, &tools);
                MessageView { user: false, text: String::new(), html }
            })
            .collect();
        CurrentView {
            delete_href: format!("/{lang}/chat/{}/delete", conversation.id),
            id: conversation.id,
            title: conversation.title,
            messages,
            unanswered,
        }
    });
    let available = state.chat.key.is_some();
    ChatPage {
        layout,
        available,
        signed_in: true,
        sign_in_href: String::new(),
        conversations,
        current,
        banner,
        blocked: loaded.standing.refusal().filter(|_| available).map(|r| refusal_text(&tr, r)),
        draft,
        left: tr.t_args("chat-left", &[("n", loaded.standing.left_today().to_string().into())]),
        max_chars: limits::QUESTION_MAX_CHARS,
    }
}

fn signed_out(state: &AppState, lang: Lang) -> ChatPage {
    let here = format!("/{}/chat", lang.code());
    ChatPage {
        layout: layout(state, lang, "/chat"),
        available: state.chat.key.is_some(),
        signed_in: false,
        sign_in_href: format!("/{}/account/sign-in?next={}", lang.code(), urlencode(&here)),
        conversations: Vec::new(),
        current: None,
        banner: None,
        blocked: None,
        draft: String::new(),
        left: String::new(),
        max_chars: limits::QUESTION_MAX_CHARS,
    }
}

#[derive(Deserialize)]
pub struct IndexQuery {
    msg: Option<String>,
}

/// `/{lang}/chat`: the explanation and sign-in link, or the account's conversations and a new
/// one.
pub async fn index(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    MaybeUser(user): MaybeUser,
    ClientIp(client): ClientIp,
    Query(q): Query<IndexQuery>,
) -> Result<Response, AppError> {
    let Some(current) = user else {
        return Ok(respond(&signed_out(&state, lang), StatusCode::OK));
    };
    let layout = layout(&state, lang, "/chat");
    let asker = Asker::new(&current.user, &client);
    let Some(loaded) = load(&state, &current.user.id, &asker, None).await? else {
        return Ok(not_found_page(lang));
    };
    let banner = (q.msg.as_deref() == Some("deleted"))
        .then(|| Banner { class: "notice", text: lang.tr().t("chat-deleted") });
    Ok(respond(&page(&state, layout, loaded, banner, String::new()), StatusCode::OK))
}

/// A body sent once `future` has produced it: the page of an answer being written.
struct Deferred(Option<Pin<Box<dyn Future<Output = String> + Send>>>);

impl Stream for Deferred {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let Some(future) = this.0.as_mut() else { return Poll::Ready(None) };
        match future.as_mut().poll(cx) {
            Poll::Ready(html) => {
                this.0 = None;
                Poll::Ready(Some(Ok(Bytes::from(html))))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// The page of a conversation once its answer is finished.
fn waiting_page(
    state: AppState,
    layout: Layout,
    (user_id, asker): (String, Asker),
    id: String,
    mut done: watch::Receiver<()>,
) -> Response {
    let future = async move {
        // Closed when the answer is stored or has failed.
        let _ = tokio::time::timeout(WAIT, done.changed()).await;
        let loaded = match load(&state, &user_id, &asker, Some(&id)).await {
            Ok(Some(loaded)) => loaded,
            // Deleted meanwhile: the list of conversations.
            Ok(None) => match load(&state, &user_id, &asker, None).await {
                Ok(Some(loaded)) => loaded,
                Ok(None) => return String::new(),
                Err(e) => {
                    tracing::error!(error = %e, "database error");
                    return "internal error".to_owned();
                }
            },
            Err(e) => {
                tracing::error!(error = %e, "database error");
                return "internal error".to_owned();
            }
        };
        page(&state, layout, loaded, None, String::new()).render().unwrap_or_else(|e| {
            tracing::error!(error = %e, "template rendering failed");
            "internal error".to_owned()
        })
    };
    let body = Body::from_stream(Deferred(Some(Box::pin(future))));
    no_store(([(header::CONTENT_TYPE, "text/html; charset=utf-8")], body).into_response())
}

/// `/{lang}/chat/{id}`: a conversation, sent once its answer (if one is being written) is there.
pub async fn conversation(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, id)): Path<(String, String)>,
    current: CurrentUser,
    ClientIp(client): ClientIp,
) -> Result<Response, AppError> {
    let asker = Asker::new(&current.user, &client);
    let user_id = current.user.id;
    let layout = layout(&state, lang, &format!("/chat/{id}"));
    let waiting = state.chat.limiter().waiting(&user_id, &id);
    if let Some(done) = waiting {
        return Ok(waiting_page(state, layout, (user_id, asker), id, done));
    }
    let Some(loaded) = load(&state, &user_id, &asker, Some(&id)).await? else {
        return Ok(not_found_page(lang));
    };
    Ok(respond(&page(&state, layout, loaded, None, String::new()), StatusCode::OK))
}

/// The first line of the question, shortened.
fn title(question: &str) -> String {
    let line = question.lines().next().unwrap_or("").trim();
    if line.chars().count() <= TITLE_CHARS {
        return line.to_owned();
    }
    let cut: String = line.chars().take(TITLE_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

/// Checks a question against every limit, then stores it and reserves the account's answer.
async fn accept(
    state: &AppState,
    user_id: &str,
    asker: &Asker,
    lang: Lang,
    question: &str,
    conversation: Option<&str>,
) -> Result<(Job, Admission), Refusal> {
    if state.chat.key.is_none() {
        return Err(Refusal::Unavailable);
    }
    let question = question.trim().to_owned();
    if question.is_empty() {
        return Err(Refusal::Empty);
    }
    if question.chars().count() > limits::QUESTION_MAX_CHARS {
        return Err(Refusal::TooLong);
    }
    let failed = |e: DbError| {
        tracing::error!(error = %e, "database error");
        Refusal::Failed
    };
    let day = limits::utc_day(db::now());
    let (user, existing) = (user_id.to_owned(), conversation.map(str::to_owned));
    let who = asker.clone();
    let (found, standing, history) = state
        .db
        .read(move |c| {
            let found = match &existing {
                Some(id) => store::conversation(c, &user, id)?.is_some(),
                None => true,
            };
            let standing = store::standing(c, &who, &day)?;
            let history = match &existing {
                Some(id) if found => store::turns(c, id, limits::HISTORY_TURNS)?,
                _ => Vec::new(),
            };
            Ok((found, standing, history))
        })
        .await
        .map_err(failed)?;
    if !found {
        return Err(Refusal::NotFound);
    }
    if let Some(refusal) = standing.refusal() {
        return Err(refusal);
    }
    let conversation_id =
        conversation.map_or_else(|| uuid::Uuid::now_v7().to_string(), str::to_owned);
    let admission = Admission::new(state, user_id, &conversation_id)?;
    let new_title = conversation.is_none().then(|| title(&question));
    let (user, id, text) = (user_id.to_owned(), conversation_id.clone(), question.clone());
    state
        .db
        .write(move |c| {
            store::insert_question(c, &user, &id, new_title.as_deref(), &text, db::now())
        })
        .await
        .map_err(failed)?;
    let job = Job { asker: asker.clone(), conversation_id, lang, question, history };
    Ok((job, admission))
}

#[derive(Deserialize)]
pub struct AskForm {
    #[serde(default)]
    question: String,
    conversation: Option<String>,
}

/// `POST /{lang}/chat`: a question without JavaScript.
pub async fn ask_form(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    current: CurrentUser,
    ClientIp(client): ClientIp,
    Form(form): Form<AskForm>,
) -> Result<Response, AppError> {
    let asker = Asker::new(&current.user, &client);
    let user_id = current.user.id;
    let conversation = form.conversation.filter(|c| !c.is_empty());
    match accept(&state, &user_id, &asker, lang, &form.question, conversation.as_deref()).await {
        Ok((job, admission)) => {
            let to = format!("/{}/chat/{}", lang.code(), job.conversation_id);
            tokio::spawn(answer::run(state, job, admission, None));
            Ok(Redirect::to(&to).into_response())
        }
        Err(Refusal::NotFound) => Ok(not_found_page(lang)),
        Err(refusal) => {
            let path = conversation
                .as_deref()
                .map_or_else(|| "/chat".to_owned(), |c| format!("/chat/{c}"));
            let layout = layout(&state, lang, &path);
            let Some(loaded) = load(&state, &user_id, &asker, conversation.as_deref()).await?
            else {
                return Ok(not_found_page(lang));
            };
            let banner = Banner { class: "error", text: refusal_text(&lang.tr(), refusal) };
            let page = page(&state, layout, loaded, Some(banner), form.question);
            Ok(respond(&page, refusal.status()))
        }
    }
}

/// `POST /{lang}/chat/{id}/delete`
pub async fn delete(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, id)): Path<(String, String)>,
    current: CurrentUser,
) -> Result<Response, AppError> {
    let user_id = current.user.id;
    let deleted = state.db.write(move |c| store::delete_conversation(c, &user_id, &id)).await?;
    Ok(if deleted {
        Redirect::to(&format!("/{}/chat?msg=deleted", lang.code())).into_response()
    } else {
        not_found_page(lang)
    })
}

#[derive(Deserialize)]
pub struct AskJson {
    #[serde(default)]
    question: String,
    conversation: Option<String>,
    /// Language of the page (system prompt, labels); default: the account's.
    lang: Option<String>,
}

fn sse_event(name: &str, data: &serde_json::Value) -> Bytes {
    Bytes::from(format!("event: {name}\ndata: {data}\n\n"))
}

fn encode(event: Event) -> Bytes {
    match event {
        Event::Text(text) => sse_event("text", &json!({ "t": text })),
        Event::Tool(kind) => {
            let kind = match kind {
                Kind::Search => "search",
                Kind::Compute => "compute",
            };
            sse_event("tool", &json!({ "kind": kind }))
        }
        Event::Reset => sse_event("reset", &json!({})),
        Event::Done { html, left } => sse_event("done", &json!({ "html": html, "left": left })),
        Event::Failed(message) => sse_event("failed", &json!({ "message": message })),
    }
}

/// The answer's events as `text/event-stream`, with a comment line when it is quiet.
struct EventStream {
    first: Option<Bytes>,
    events: UnboundedReceiver<Event>,
    ping: tokio::time::Interval,
}

impl Stream for EventStream {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if let Some(first) = this.first.take() {
            return Poll::Ready(Some(Ok(first)));
        }
        match this.events.poll_recv(cx) {
            Poll::Ready(Some(event)) => Poll::Ready(Some(Ok(encode(event)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => match this.ping.poll_tick(cx) {
                Poll::Ready(_) => Poll::Ready(Some(Ok(Bytes::from_static(b": ping\n\n")))),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

/// `POST /api/v1/chat`: a question from `web/src/chat.ts`, answered as server-sent events
/// (`start` with the conversation and, when it is new, its title; then `text`, `tool`,
/// `reset`… and `done` or `failed`); a refusal is a JSON error.
pub async fn ask_api(
    State(state): State<AppState>,
    current: CurrentUser,
    ClientIp(client): ClientIp,
    Json(body): Json<AskJson>,
) -> Response {
    let lang = body.lang.as_deref().and_then(Lang::parse).unwrap_or(current.user.locale);
    let conversation = body.conversation.filter(|c| !c.is_empty());
    let asker = Asker::new(&current.user, &client);
    let user_id = &current.user.id;
    match accept(&state, user_id, &asker, lang, &body.question, conversation.as_deref()).await {
        Err(refusal) => {
            let message = refusal_text(&lang.tr(), refusal);
            let error = json!({ "error": { "code": refusal.code(), "message": message } });
            no_store((refusal.status(), Json(error)).into_response())
        }
        Ok((job, admission)) => {
            let (tx, events) = unbounded_channel();
            let new_title = conversation.is_none().then(|| title(&job.question));
            let start = json!({ "conversation": job.conversation_id, "title": new_title });
            let first = sse_event("start", &start);
            tokio::spawn(answer::run(state, job, admission, Some(tx)));
            let start = tokio::time::Instant::now() + PING_EVERY;
            let stream = EventStream {
                first: Some(first),
                events,
                ping: tokio::time::interval_at(start, PING_EVERY),
            };
            no_store(
                (
                    [
                        (header::CONTENT_TYPE, "text/event-stream"),
                        (header::HeaderName::from_static("x-accel-buffering"), "no"),
                    ],
                    Body::from_stream(stream),
                )
                    .into_response(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_are_the_first_line_shortened() {
        assert_eq!(title("  Pourquoi ma pâte colle ?\nDétails…"), "Pourquoi ma pâte colle ?");
        let long = "é".repeat(200);
        let t = title(&long);
        assert_eq!(t.chars().count(), TITLE_CHARS);
        assert!(t.ends_with('…'));
    }
}
