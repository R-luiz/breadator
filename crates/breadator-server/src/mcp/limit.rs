//! Requests per minute and per client address, counted in memory: the endpoint is open to
//! anyone, and an assistant caught in a loop must not starve the site that shares its machine.
//!
//! A fixed window: every address starts again at zero each minute. Restarting forgets the
//! counts, which is fine for a limit that only smooths bursts. The addresses are those of the
//! rate limits of accounts (`auth::guard::client_ip`); the users of a hosted assistant share
//! the few addresses of its servers, so the limit is far above what one conversation sends.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use axum::extract::{ConnectInfo, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::auth::guard::client_ip;

/// Addresses counted in one minute; beyond it new addresses wait for the next minute, so the
/// table cannot be grown without bound.
const MAX_CLIENTS: usize = 10_000;
const WINDOW_SECS: u64 = 60;

pub struct Limiter {
    per_minute: u32,
    started: Instant,
    window: Mutex<Window>,
}

#[derive(Default)]
struct Window {
    /// Which minute since the start the counts are for.
    minute: u64,
    requests: HashMap<String, u32>,
}

impl Limiter {
    pub fn new(per_minute: u32) -> Self {
        Self { per_minute, started: Instant::now(), window: Mutex::default() }
    }

    /// Counts a request of `client`, `now_secs` after the start.
    fn admit(&self, client: &str, now_secs: u64) -> Result<(), Refused> {
        // The counts stay consistent between statements: a panic cannot corrupt them.
        let mut window = self.window.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let minute = now_secs / WINDOW_SECS;
        if window.minute != minute {
            window.minute = minute;
            window.requests.clear();
        }
        let wait = WINDOW_SECS - now_secs % WINDOW_SECS;
        if !window.requests.contains_key(client) && window.requests.len() >= MAX_CLIENTS {
            return Err(Refused { wait, first: false });
        }
        let count = window.requests.entry(client.to_owned()).or_insert(0);
        *count = count.saturating_add(1);
        if *count > self.per_minute {
            Err(Refused { wait, first: *count == self.per_minute + 1 })
        } else {
            Ok(())
        }
    }
}

/// A request over the limit.
#[derive(Debug, PartialEq, Eq)]
struct Refused {
    /// Seconds until the next minute.
    wait: u64,
    /// The address's first refusal of the minute: the one that is logged, so that an address
    /// hammering the endpoint writes one line a minute, not one per request.
    first: bool,
}

/// Middleware: `429` with `Retry-After` once the caller's address is over the limit.
pub async fn layer(State(limiter): State<Arc<Limiter>>, request: Request, next: Next) -> Response {
    let peer = request.extensions().get::<ConnectInfo<SocketAddr>>().map(|c| c.0.ip());
    let client = client_ip(request.headers(), peer);
    match limiter.admit(&client, limiter.started.elapsed().as_secs()) {
        Ok(()) => next.run(request).await,
        Err(Refused { wait, first }) => {
            if first {
                tracing::warn!(client = %client, "mcp: too many requests");
            }
            let body = json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": {
                    "code": -32000,
                    "message": format!("Too many requests: try again in {wait} s"),
                },
            });
            let mut response = (StatusCode::TOO_MANY_REQUESTS, axum::Json(body)).into_response();
            response.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from(wait));
            response
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_address_has_its_own_count_and_starts_again_each_minute() {
        let limiter = Limiter::new(3);
        for second in [0, 10, 59] {
            assert_eq!(limiter.admit("203.0.113.7", second), Ok(()));
        }
        // The fourth of the minute waits for its end; another address is not held back.
        assert_eq!(limiter.admit("203.0.113.7", 59), Err(Refused { wait: 1, first: true }));
        assert_eq!(limiter.admit("203.0.113.7", 59), Err(Refused { wait: 1, first: false }));
        assert_eq!(limiter.admit("2001:db8::/48", 59), Ok(()));
        assert_eq!(limiter.admit("203.0.113.7", 60), Ok(()));
        assert_eq!(limiter.admit("203.0.113.7", 75), Ok(()));
        assert_eq!(limiter.admit("203.0.113.7", 80), Ok(()));
        assert_eq!(limiter.admit("203.0.113.7", 80), Err(Refused { wait: 40, first: true }));
    }

    #[test]
    fn the_table_of_addresses_is_bounded() {
        let limiter = Limiter::new(3);
        for n in 0..MAX_CLIENTS {
            assert_eq!(limiter.admit(&format!("client-{n}"), 5), Ok(()));
        }
        // One address too many waits; those already counted go on; the next minute is empty.
        assert_eq!(limiter.admit("one-more", 5), Err(Refused { wait: 55, first: false }));
        assert_eq!(limiter.admit("client-0", 6), Ok(()));
        assert_eq!(limiter.admit("one-more", 61), Ok(()));
    }
}
