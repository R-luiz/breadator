//! One answer: at most [`MAX_CALLS`] model calls, each booked at its ceiling cost before it is
//! made and corrected to its real cost as soon as that is known, the tools they ask for, and the
//! answer stored with its citations.
//!
//! It runs in its own task: a reader who closes the page does not stop it half-way (the calls
//! already made are paid for, so the answer is kept). A deploy or a crash does stop it: the call
//! then in flight stays booked at its ceiling, so the budget never forgets it.

use std::time::Duration;

use serde_json::{Value, json};
use tokio::sync::mpsc::UnboundedSender;

use super::limits::{self, Asker};
use super::openrouter::{self, Usage};
use super::store::{self, NewAnswer};
use super::tools::{self, Kind};
use super::{Admission, prompt, render};
use crate::app::AppState;
use crate::db;
use crate::i18n::Lang;

/// Model calls of one answer: up to two rounds of tools, then the answer.
pub const MAX_CALLS: usize = 3;
/// Tools run per round; further calls of the same round are answered with an error.
const TOOLS_PER_ROUND: usize = 4;
/// One call, streamed: the answer of a slow provider included.
const CALL_TIMEOUT: Duration = Duration::from_secs(45);

/// A question accepted for an answer.
pub struct Job {
    pub asker: Asker,
    pub conversation_id: String,
    pub lang: Lang,
    pub question: String,
    /// Earlier (question, answer) pairs, oldest first.
    pub history: Vec<(String, String)>,
}

/// What the page following the answer live receives.
#[derive(Debug)]
pub enum Event {
    /// A piece of answer text.
    Text(String),
    /// A tool started.
    Tool(Kind),
    /// The text so far was a preamble to tool calls: drop it.
    Reset,
    /// The answer as stored and rendered, and the answers left to the account today.
    Done {
        html: String,
        left: i64,
    },
    Failed(String),
}

fn messages(job: &Job) -> Vec<Value> {
    let mut messages = vec![json!({ "role": "system", "content": prompt::system(job.lang) })];
    for (question, answer) in &job.history {
        messages.push(json!({ "role": "user", "content": question }));
        messages.push(json!({ "role": "assistant", "content": render::strip_markers(answer) }));
    }
    messages.push(json!({ "role": "user", "content": job.question }));
    messages
}

/// Books a call at `ceiling_usd` before it is made; `false` when it must not be made (the site's
/// budget is reached, or the booking failed).
async fn reserve(
    state: &AppState,
    asker: &Asker,
    day: &str,
    first: bool,
    ceiling_usd: f64,
) -> bool {
    let (asker, day) = (asker.clone(), day.to_owned());
    let reserved =
        state.db.write(move |c| store::reserve(c, &asker, &day, first, ceiling_usd)).await;
    match reserved {
        Ok(true) => true,
        Ok(false) => {
            tracing::warn!("chat budget reached during an answer");
            false
        }
        Err(e) => {
            tracing::error!(error = %e, "cannot book a chat call");
            false
        }
    }
}

/// Replaces a call's booking by its cost; a failure is logged (the ceiling stays booked).
async fn settle(
    state: &AppState,
    asker: &Asker,
    day: &str,
    first: bool,
    ceiling_usd: f64,
    usage: Option<Usage>,
) {
    let (asker, day) = (asker.clone(), day.to_owned());
    let settled =
        state.db.write(move |c| store::settle(c, &asker, &day, first, ceiling_usd, usage)).await;
    if let Err(e) = settled {
        tracing::error!(error = %e, ceiling_usd, "cannot settle a chat call");
    }
}

/// Writes the answer to `job`, then lets the account ask again (`admission` is dropped).
pub async fn run(
    state: AppState,
    job: Job,
    admission: Admission,
    events: Option<UnboundedSender<Event>>,
) {
    let send = |event: Event| {
        if let Some(tx) = &events {
            // The page may be gone: the answer is stored all the same.
            let _ = tx.send(event);
        }
    };
    let tr = job.lang.tr();
    let key = state.chat.key.clone().unwrap_or_default();
    let mut messages = messages(&job);
    let mut sources = Vec::new();
    let mut records = Vec::new();
    let mut total = Usage::default();
    let (mut model, mut provider) = (String::new(), String::new());
    let mut calls = 0;
    let mut answer = None;

    for call in 0..MAX_CALLS {
        let allow_tools = call + 1 < MAX_CALLS;
        let body =
            openrouter::request(&messages, &tools::SCHEMAS, &job.conversation_id, allow_tools)
                .to_string()
                .into_bytes();
        // The first call is the one that counts against the allowances.
        let first = call == 0;
        let ceiling = openrouter::estimate(body.len()).cost_usd;
        let day = limits::utc_day(db::now());
        if !reserve(&state, &job.asker, &day, first, ceiling).await {
            break;
        }
        let mut on_text = |text: &str| send(Event::Text(text.to_owned()));
        let outcome = openrouter::call(
            &state.chat,
            &key,
            &state.public_url,
            body,
            CALL_TIMEOUT,
            &mut on_text,
        )
        .await;
        calls += 1;
        settle(&state, &job.asker, &day, first, ceiling, outcome.usage).await;
        if let Some(usage) = outcome.usage {
            total.tokens_in += usage.tokens_in;
            total.tokens_out += usage.tokens_out;
            total.cost_usd += usage.cost_usd;
        }
        let completion = match outcome.completion {
            Ok(completion) => completion,
            Err(e) => {
                tracing::warn!(error = %e, call, "chat model call failed");
                break;
            }
        };
        if !completion.model.is_empty() {
            model.clone_from(&completion.model);
            provider.clone_from(&completion.provider);
        }
        if completion.tool_calls.is_empty() || !allow_tools {
            answer = Some(completion.content);
            break;
        }

        send(Event::Reset);
        let mut tool_calls = completion.tool_calls;
        for (i, tool_call) in tool_calls.iter_mut().enumerate() {
            if tool_call.id.is_empty() {
                tool_call.id = format!("call_{call}_{i}");
            }
        }
        let echoed: Vec<Value> = tool_calls
            .iter()
            .map(|c| {
                // Arguments that are not JSON go back as `{}`: providers reject the request
                // otherwise. The tool's error tells the model what was wrong.
                let arguments = if serde_json::from_str::<Value>(&c.arguments).is_ok() {
                    c.arguments.clone()
                } else {
                    "{}".to_owned()
                };
                json!({
                    "id": c.id,
                    "type": "function",
                    "function": { "name": c.name, "arguments": arguments },
                })
            })
            .collect();
        let content =
            if completion.content.is_empty() { Value::Null } else { completion.content.into() };
        let mut assistant =
            json!({ "role": "assistant", "content": content, "tool_calls": echoed });
        if !completion.reasoning_details.is_empty() {
            assistant["reasoning_details"] = Value::Array(completion.reasoning_details);
        }
        messages.push(assistant);
        for (i, tool_call) in tool_calls.iter().enumerate() {
            let result = if i < TOOLS_PER_ROUND {
                send(Event::Tool(tools::kind(&tool_call.name)));
                let ctx =
                    tools::Context { knowledge: &state.knowledge, lang: job.lang, site: None };
                let (value, record) =
                    tools::execute(&ctx, &tool_call.name, &tool_call.arguments, &mut sources);
                records.push(record);
                value
            } else {
                json!({ "error": format!("too many tool calls at once: at most {TOOLS_PER_ROUND}") })
            };
            messages.push(json!({
                "role": "tool",
                "tool_call_id": tool_call.id,
                "content": result.to_string(),
            }));
        }
    }

    let text = answer.map(|t| t.trim().to_owned()).filter(|t| !t.is_empty());
    let mut cited_count = 0;
    let stored = match text {
        Some(text) => {
            let unsourced = match job.lang {
                Lang::Fr => render::UNSOURCED_FR,
                Lang::En => render::UNSOURCED_EN,
            };
            let (content, cited) = render::finalize(&text, &sources, unsourced);
            cited_count = cited.len();
            let new = NewAnswer {
                content: content.clone(),
                tool_calls: serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()),
                citations: serde_json::to_string(&cited).unwrap_or_else(|_| "[]".into()),
                model: model.clone(),
                tokens_in: total.tokens_in,
                tokens_out: total.tokens_out,
                cost_usd: total.cost_usd,
            };
            let conversation = job.conversation_id.clone();
            let asker = job.asker.clone();
            let day = limits::utc_day(db::now());
            let saved = state
                .db
                .write(move |c| {
                    let id = store::insert_answer(c, &conversation, &new, db::now())?;
                    let standing = store::standing(c, &asker, &day)?;
                    Ok(id.map(|id| (id, standing.left_today())))
                })
                .await;
            match saved {
                Ok(Some((id, left))) => {
                    let html =
                        render::message_html(&state.knowledge, &tr, id, &content, &cited, &records);
                    send(Event::Done { html, left });
                    true
                }
                // The conversation was deleted while its answer was being written.
                Ok(None) => false,
                Err(e) => {
                    tracing::error!(error = %e, "cannot store a chat answer");
                    false
                }
            }
        }
        None => false,
    };
    if !stored {
        send(Event::Failed(tr.t("chat-error-failed")));
    }
    tracing::info!(
        model = %model,
        provider = %provider,
        calls,
        tools = records.len(),
        sources = cited_count,
        tokens_in = total.tokens_in,
        tokens_out = total.tokens_out,
        cost_usd = total.cost_usd,
        answered = stored,
        "chat answer"
    );
    drop(admission);
}
