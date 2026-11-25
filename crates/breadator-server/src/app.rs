//! Router, shared state, middleware and the routes that are not pages.

use std::sync::Arc;
use std::time::Duration;

use askama::Template;
use axum::Router;
use axum::extract::{DefaultBodyLimit, FromRequestParts, Path, State};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, StatusCode, Uri, header};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::Config;
use crate::assets::Assets;
use crate::auth::{self, AuthState, Badge};
use crate::chat::{self, ChatState};
use crate::db::Db;
use crate::i18n::{Lang, Tr};
use crate::knowledge::Knowledge;
use crate::pages;
use crate::{account, mcp, sync};

pub struct Inner {
    pub assets: Assets,
    pub public_url: String,
    pub db: Db,
    pub auth: AuthState,
    pub knowledge: Knowledge,
    pub chat: ChatState,
}

#[derive(Clone)]
pub struct AppState(pub Arc<Inner>);

impl AppState {
    /// Opens the database (applying pending migrations), builds the account machinery and loads
    /// the knowledge base (refusing to start on invalid content).
    pub fn new(config: Config) -> Result<Self, String> {
        let knowledge = Knowledge::load().map_err(|errors| {
            let list: Vec<String> = errors.iter().map(ToString::to_string).collect();
            format!("invalid content:\n{}", list.join("\n"))
        })?;
        Self::with_knowledge(config, knowledge)
    }

    /// The application over an already loaded knowledge base (tests bring a fixed one).
    pub fn with_knowledge(config: Config, knowledge: Knowledge) -> Result<Self, String> {
        let db = Db::open(&config.database_path).map_err(|e| {
            format!("cannot open the database {}: {e}", config.database_path.display())
        })?;
        let auth = AuthState::new(&config)?;
        let chat = ChatState::new(&config)?;
        Ok(Self(Arc::new(Inner {
            assets: Assets::load(),
            public_url: config.public_url,
            db,
            auth,
            knowledge,
            chat,
        })))
    }
}

impl std::ops::Deref for AppState {
    type Target = Inner;
    fn deref(&self) -> &Inner {
        &self.0
    }
}

/// Content-Security-Policy: no inline script; WASM compilation allowed; Cloudflare Web Analytics.
const CSP: &str = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' https://static.cloudflareinsights.com; \
connect-src 'self' https://cloudflareinsights.com; img-src 'self' data:; style-src 'self'; font-src 'self'; \
manifest-src 'self'; worker-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'; object-src 'none'";

pub fn router(state: AppState) -> Router {
    let headers = [
        (header::CONTENT_SECURITY_POLICY, CSP),
        (header::STRICT_TRANSPORT_SECURITY, "max-age=31536000"),
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::REFERRER_POLICY, "strict-origin-when-cross-origin"),
        (
            header::HeaderName::from_static("permissions-policy"),
            "camera=(), microphone=(), geolocation=()",
        ),
    ];
    let mut router = Router::new()
        .route("/", get(root))
        .route("/healthz", get(|| async { "ok" }))
        .route("/robots.txt", get(robots))
        .route("/sitemap.xml", get(sitemap))
        .route("/sw.js", get(service_worker))
        .route("/manifest.webmanifest", get(manifest))
        .route("/static/{*path}", get(static_file))
        .route("/api/v1/formula", post(pages::api::formula))
        .route("/api/v1/water-temperature", post(pages::api::water_temperature))
        .route("/api/v1/yeast", post(pages::api::yeast))
        .route("/api/v1/fermentation", post(pages::api::fermentation))
        .route("/api/v1/starter", post(pages::api::starter))
        .route("/api/v1/flour", post(pages::api::flour))
        .route("/{lang}", get(pages::home::page))
        .route("/{lang}/", get(trailing_slash))
        .route("/{lang}/calculators", get(pages::home::calculators))
        .route("/{lang}/units/{units}", get(pages::home::units))
        .route("/{lang}/calculators/formula", get(pages::formula::page))
        .route("/{lang}/calculators/temperature", get(pages::temperature::page))
        .route("/{lang}/calculators/yeast", get(pages::yeast::page))
        .route("/{lang}/calculators/fermentation", get(pages::fermentation::page))
        .route(
            "/{lang}/calculators/preferment",
            get(pages::preferment::page).post(pages::preferment::calculate),
        )
        .route("/{lang}/calculators/starter", get(pages::starter::page))
        .route("/{lang}/calculators/flour", get(pages::flour::page))
        .route("/{lang}/learn", get(pages::learn::index))
        .route("/{lang}/learn/{slug}", get(pages::learn::article))
        .route("/{lang}/sources", get(pages::learn::sources))
        .route("/{lang}/search", get(pages::learn::search))
        .route("/{lang}/recipes", get(pages::recipes::index))
        .route("/{lang}/recipes/{slug}", get(pages::recipes::recipe))
        .route("/{lang}/assistants", get(pages::assistants::page))
        .merge(auth::routes())
        .merge(account::routes())
        .merge(sync::routes())
        .merge(chat::routes())
        .fallback(not_found)
        .with_state(state.clone())
        // Closest to the routes, so even its refusals get the security headers below.
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth::guard::guard))
        // Merged after the guard, so outside it: the MCP endpoint takes no cookie (no session
        // to look up, nothing a third-party page could ride on) and has its own `Origin` rule.
        .merge(mcp::routes(state));
    for (name, value) in headers {
        router = router
            .layer(SetResponseHeaderLayer::if_not_present(name, HeaderValue::from_static(value)));
    }
    // No form or JSON request of the site comes near 64 KiB; the sync API sets its own limit.
    router
        .layer(DefaultBodyLimit::max(64 * 1024))
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(20)))
        // The path only, at every level: query strings carry sign-in tokens and OAuth codes.
        .layer(TraceLayer::new_for_http().make_span_with(|req: &axum::extract::Request| {
            tracing::debug_span!("request", method = %req.method(), path = %req.uri().path())
        }))
}

/// `/{lang}` segment of a page route; anything but a known language is a 404 page.
pub struct LangPath(pub Lang);

impl<S: Send + Sync> FromRequestParts<S> for LangPath {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(params) = Path::<Vec<(String, String)>>::from_request_parts(parts, state)
            .await
            .map_err(|_| not_found_page(Lang::En))?;
        params
            .iter()
            .find(|(k, _)| k == "lang")
            .and_then(|(_, v)| Lang::parse(v))
            .map(LangPath)
            .ok_or_else(|| not_found_page(Lang::En))
    }
}

/// Fields every page layout needs.
pub struct Layout {
    pub tr: Tr,
    pub title: String,
    pub description: String,
    /// Path after the language segment, e.g. `/calculators/formula` (empty for home).
    pub path: String,
    pub public_url: String,
    pub css: String,
    pub icon: String,
    pub script: Option<String>,
    /// The islands' translation catalog for this language (a cached asset).
    pub i18n: String,
    pub wasm: Option<String>,
    /// Navigation section to highlight.
    pub section: &'static str,
    /// The signed-in user, for the header; `None` when signed out.
    pub user: Option<Badge>,
    pub noindex: bool,
    /// Results also show ounces and °F (see `auth::current_units`).
    pub units_us: bool,
}

pub struct Alternate {
    pub lang: &'static str,
    pub href: String,
    pub label: String,
}

impl Layout {
    pub fn new(
        state: &Inner,
        lang: Lang,
        path: &str,
        section: &'static str,
        title: String,
        description: String,
    ) -> Self {
        Self {
            tr: lang.tr(),
            title,
            description,
            path: path.to_owned(),
            public_url: state.public_url.clone(),
            css: state.assets.url("app.css").unwrap_or("").to_owned(),
            icon: state.assets.url("icon.svg").unwrap_or("").to_owned(),
            script: state.assets.url("dist/app.js").map(str::to_owned),
            i18n: state.assets.url(&format!("i18n/{}.json", lang.code())).unwrap_or("").to_owned(),
            wasm: state.assets.url("dist/breadator_wasm_bg.wasm").map(str::to_owned),
            section,
            user: auth::current_badge(),
            noindex: false,
            units_us: auth::current_units() == auth::Units::Us,
        }
    }

    /// Marks the page as personal: not indexed (account, sign-in, saved formulas).
    pub fn private(mut self) -> Self {
        self.noindex = true;
        self
    }

    pub fn canonical(&self) -> String {
        format!("{}/{}{}", self.public_url, self.tr.lang(), self.path)
    }

    pub fn href(&self, path: &str) -> String {
        format!("/{}{}", self.tr.lang(), path)
    }

    pub fn alternates(&self) -> Vec<Alternate> {
        Lang::ALL
            .iter()
            .map(|l| Alternate {
                lang: l.code(),
                href: format!("{}/{}{}", self.public_url, l.code(), self.path),
                label: l.tr().t("lang-name"),
            })
            .collect()
    }

    pub fn is(&self, section: &str) -> bool {
        self.section == section
    }

    /// A calculator, their index or the saved formulas: the header shows the calculators as tabs.
    pub fn in_calculators(&self) -> bool {
        matches!(
            self.section,
            "calculators"
                | "formula"
                | "fermentation"
                | "preferment"
                | "starter"
                | "flour"
                | "temperature"
                | "yeast"
                | "my"
        )
    }
}

pub fn render(template: &impl Template) -> Response {
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "template rendering failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
        }
    }
}

async fn root(headers: HeaderMap) -> Redirect {
    Redirect::to(&format!("/{}", Lang::negotiate(&headers).code()))
}

async fn trailing_slash(LangPath(lang): LangPath) -> Redirect {
    Redirect::permanent(&format!("/{}", lang.code()))
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFound {
    tr: Tr,
}

pub(crate) fn not_found_page(lang: Lang) -> Response {
    (StatusCode::NOT_FOUND, render(&NotFound { tr: lang.tr() })).into_response()
}

async fn not_found(uri: Uri) -> Response {
    let lang = uri.path().split('/').nth(1).and_then(Lang::parse).unwrap_or(Lang::En);
    not_found_page(lang)
}

async fn static_file(State(state): State<AppState>, uri: Uri) -> Response {
    match state.assets.get(uri.path()) {
        Some(asset) => (
            [
                (header::CONTENT_TYPE, asset.mime),
                (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
            ],
            asset.bytes,
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

async fn robots(State(state): State<AppState>) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!(
            "User-agent: *\nAllow: /\nDisallow: /api/\n\nSitemap: {}/sitemap.xml\n",
            state.public_url
        ),
    )
}

/// Pages listed in the sitemap, by path after the language segment.
pub const PUBLIC_PAGES: &[&str] = &[
    "",
    "/calculators",
    "/calculators/formula",
    "/calculators/fermentation",
    "/calculators/preferment",
    "/calculators/starter",
    "/calculators/flour",
    "/calculators/temperature",
    "/calculators/yeast",
    "/learn",
    "/recipes",
    "/sources",
    "/assistants",
];

async fn sitemap(State(state): State<AppState>) -> impl IntoResponse {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
    );
    let lib = &state.knowledge.lib;
    let mut paths: Vec<String> = PUBLIC_PAGES.iter().map(|p| (*p).to_owned()).collect();
    // Articles exist in every language (validated); recipes are translated in place.
    paths.extend(lib.articles("en").iter().map(|a| format!("/learn/{}", a.slug)));
    paths.extend(lib.recipes().map(|r| format!("/recipes/{}", r.slug)));
    for path in &paths {
        for lang in Lang::ALL {
            xml.push_str(&format!("<url><loc>{}/{}{}</loc>", state.public_url, lang.code(), path));
            for alt in Lang::ALL {
                xml.push_str(&format!(
                    "<xhtml:link rel=\"alternate\" hreflang=\"{0}\" href=\"{1}/{0}{2}\"/>",
                    alt.code(),
                    state.public_url,
                    path
                ));
            }
            xml.push_str("</url>\n");
        }
    }
    xml.push_str("</urlset>\n");
    ([(header::CONTENT_TYPE, "application/xml; charset=utf-8")], xml)
}

async fn manifest(State(state): State<AppState>) -> impl IntoResponse {
    let icon = state.assets.url("icon.svg").unwrap_or("");
    let body = serde_json::json!({
        "name": "Breadator",
        "short_name": "Breadator",
        "description": "Bread-making calculators and science / Calculateurs et science du pain",
        "start_url": "/",
        "scope": "/",
        "display": "standalone",
        "background_color": "#faf6ef",
        "theme_color": "#8a5a2b",
        "icons": [{"src": icon, "sizes": "any", "type": "image/svg+xml", "purpose": "any maskable"}]
    });
    ([(header::CONTENT_TYPE, "application/manifest+json")], body.to_string())
}

/// Service worker: precaches every hashed asset, network-first for pages with an offline fallback
/// to the last cached copy. Its cache name follows the asset version, so a deploy replaces it.
async fn service_worker(State(state): State<AppState>) -> impl IntoResponse {
    let mut urls: Vec<&str> = state.assets.urls().collect();
    urls.sort_unstable();
    let body = format!(
        "const VERSION = {version:?};\nconst PRECACHE = {urls};\n{code}",
        version = state.assets.version,
        urls = serde_json::to_string(&urls).unwrap_or_else(|_| "[]".into()),
        code = include_str!("sw.js"),
    );
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        body,
    )
}
