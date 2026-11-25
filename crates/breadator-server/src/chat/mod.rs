//! The grounded chatbot: questions of signed-in bakers answered by a small model on OpenRouter
//! that must search the knowledge base before stating facts and call the engine for every
//! number. See `docs/ARCHITECTURE.md` (Chat) and `research/notes/chat-model/REPORT.md`.
//!
//! - [`pages`]: `/{lang}/chat` (works without JavaScript) and the streamed answer of
//!   `web/src/chat.ts` (`POST /api/v1/chat`, server-sent events).
//! - [`answer`]: the loop of model calls and tool runs behind one answer.
//! - [`tools`]: what the model can call, out of the site's tools ([`crate::tools`]);
//!   [`render`]: `[n]` markers to sources.
//! - [`limits`] and [`store`]: the owner's budget, the per-account allowances, the tables.
//! - [`openrouter`]: the request and its event stream; [`prompt`]: the system prompt.

mod answer;
mod limits;
mod openrouter;
mod pages;
mod prompt;
mod render;
mod store;
#[cfg(test)]
mod tests;
mod tools;

use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use axum::Router;
use axum::routing::{get, post};
use tokio::sync::watch;

use crate::Config;
use crate::app::AppState;
pub use limits::account_subject;
use limits::{Limiter, Refusal};
pub use store::export_usage;

pub struct ChatState {
    http: reqwest::Client,
    /// OpenRouter's API base (`OPENROUTER_URL`).
    api_url: String,
    /// `OPENROUTER_API_KEY`; `None`: the chat is unavailable.
    key: Option<String>,
    limiter: Mutex<Limiter>,
}

impl ChatState {
    pub fn new(config: &Config) -> Result<Self, String> {
        // Same process-wide TLS provider as the other clients (installing it twice is fine).
        let _ = rustls::crypto::ring::default_provider().install_default();
        // No overall timeout: each call sets its own, long enough for a streamed answer.
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!("breadator/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| format!("cannot build the HTTP client: {e}"))?;
        if config.openrouter_api_key.is_none() && config.public_url.starts_with("https://") {
            tracing::warn!("OPENROUTER_API_KEY is not set: the chat is unavailable");
        }
        Ok(Self {
            http,
            api_url: config.openrouter_url.clone(),
            key: config.openrouter_api_key.clone(),
            limiter: Mutex::new(Limiter::default()),
        })
    }

    fn limiter(&self) -> MutexGuard<'_, Limiter> {
        // The limiter's state stays consistent between statements: a panic cannot corrupt it.
        self.limiter.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

/// An answer being written for an account. Dropping it (answer stored, failed or abandoned)
/// lets the account ask again and wakes the page waiting for that answer.
pub struct Admission {
    state: AppState,
    user: String,
    _finished: watch::Sender<()>,
}

impl Admission {
    fn new(state: &AppState, user: &str, conversation: &str) -> Result<Self, Refusal> {
        let finished = state.chat.limiter().admit(user, conversation, Instant::now())?;
        Ok(Self { state: state.clone(), user: user.to_owned(), _finished: finished })
    }
}

impl Drop for Admission {
    fn drop(&mut self) {
        self.state.chat.limiter().release(&self.user);
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{lang}/chat", get(pages::index).post(pages::ask_form))
        .route("/{lang}/chat/{id}", get(pages::conversation))
        .route("/{lang}/chat/{id}/delete", post(pages::delete))
        .route("/api/v1/chat", post(pages::ask_api))
}
