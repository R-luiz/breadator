//! `breadator`: the web server. `breadator healthcheck` probes a running server (Docker HEALTHCHECK).

mod account;
mod app;
mod assets;
mod auth;
mod chat;
mod db;
mod i18n;
mod knowledge;
mod pages;
mod state_codec;
mod sync;
#[cfg(test)]
mod testkit;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use tracing_subscriber::EnvFilter;

pub struct Config {
    pub port: u16,
    /// Canonical origin, used for canonical/hreflang links, the sitemap, OAuth redirect URIs, the
    /// WebAuthn relying party and the origin check of state-changing requests.
    pub public_url: String,
    /// SQLite file (`DATABASE_PATH`); its directory is created if missing.
    pub database_path: PathBuf,
    /// `RESEND_API_KEY`: without it, sign-in links are logged instead of emailed (development, CI).
    pub resend_api_key: Option<String>,
    /// `RESEND_API_URL`: for tests.
    pub resend_api_url: String,
    /// Present when `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` are set.
    pub google: Option<auth::oauth::OAuthConfig>,
    /// Present when `GITHUB_CLIENT_ID` and `GITHUB_CLIENT_SECRET` are set.
    pub github: Option<auth::oauth::OAuthConfig>,
    /// `OPENROUTER_API_KEY`: without it the chat page says the assistant is unavailable.
    pub openrouter_api_key: Option<String>,
    /// `OPENROUTER_URL`: API base of the chat model provider; overridden in tests.
    pub openrouter_url: String,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        let port = match std::env::var("PORT") {
            Ok(p) => p.parse().map_err(|_| format!("PORT must be a port number, got {p:?}"))?,
            Err(_) => 8080,
        };
        let public_url = std::env::var("PUBLIC_URL")
            .unwrap_or_else(|_| "https://breadator.robinluiz.fr".to_owned())
            .trim_end_matches('/')
            .to_owned();
        let database_path = std::env::var("DATABASE_PATH")
            .ok()
            .filter(|p| !p.trim().is_empty())
            .map_or_else(|| PathBuf::from("data/breadator.sqlite3"), PathBuf::from);
        let resend_api_key = std::env::var("RESEND_API_KEY")
            .ok()
            .map(|k| k.trim().to_owned())
            .filter(|k| !k.is_empty());
        let resend_api_url =
            std::env::var("RESEND_API_URL").unwrap_or_else(|_| "https://api.resend.com".to_owned());
        let openrouter_api_key = std::env::var("OPENROUTER_API_KEY")
            .ok()
            .map(|k| k.trim().to_owned())
            .filter(|k| !k.is_empty());
        let openrouter_url = std::env::var("OPENROUTER_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_owned())
            .trim_end_matches('/')
            .to_owned();
        Ok(Self {
            port,
            public_url,
            database_path,
            resend_api_key,
            resend_api_url,
            google: auth::oauth::OAuthConfig::google_from_env(),
            github: auth::oauth::OAuthConfig::github_from_env(),
            openrouter_api_key,
            openrouter_url,
        })
    }
}

fn main() -> ExitCode {
    let config = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("configuration error: {e}");
            return ExitCode::from(2);
        }
    };
    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        return healthcheck(config.port);
    }

    let filter = EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| EnvFilter::new("info"));
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        tracing_subscriber::fmt().json().with_env_filter(filter).init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }

    let runtime = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("cannot start the async runtime: {e}");
            return ExitCode::FAILURE;
        }
    };
    match runtime.block_on(serve(config)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            tracing::error!(error = %e, "server stopped");
            ExitCode::FAILURE
        }
    }
}

async fn serve(config: Config) -> std::io::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let state = app::AppState::new(config).map_err(std::io::Error::other)?;
    tokio::spawn(maintenance(state.clone()));
    let router = app::router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "listening");
    // The peer address is the fallback client address for rate limits (no proxy headers).
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
}

/// Housekeeping: expired sessions, magic links, OAuth requests, rate-limit windows, old tombstones.
async fn maintenance(state: app::AppState) {
    let mut tick = tokio::time::interval(Duration::from_secs(6 * 3600));
    loop {
        tick.tick().await;
        if let Err(e) = state.db.purge_expired().await {
            tracing::error!(error = %e, "database housekeeping failed");
        }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
    tracing::info!("shutting down");
}

/// Exit 0 when `GET /healthz` on the local server answers 200.
fn healthcheck(port: u16) -> ExitCode {
    let probe = || -> std::io::Result<bool> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(3))?;
        stream.set_read_timeout(Some(Duration::from_secs(3)))?;
        stream.write_all(b"GET /healthz HTTP/1.0\r\nHost: localhost\r\n\r\n")?;
        let mut head = [0u8; 32];
        let n = stream.read(&mut head)?;
        Ok(head[..n].starts_with(b"HTTP/1.1 200") || head[..n].starts_with(b"HTTP/1.0 200"))
    };
    match probe() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("healthcheck: {e}");
            ExitCode::FAILURE
        }
    }
}
