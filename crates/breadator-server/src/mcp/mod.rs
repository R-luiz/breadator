//! The MCP endpoint (`POST /mcp`): the site's tools and library for AI assistants (Claude and
//! any other Model Context Protocol client). See `docs/ARCHITECTURE.md` (MCP).
//!
//! - **Open and read-only.** Everything served here is what the site gives without an account:
//!   the calculators, the knowledge base, the recipes. The endpoint takes no cookie and sits
//!   outside the session middleware, so a signed-in browser lends it nothing.
//! - **Stateless.** No session: every request stands alone, a deployment in the middle of a
//!   conversation loses nothing. The SDK serves the 2026-07-28 revision (per-request metadata,
//!   `server/discover`) and the earlier ones, whose `initialize` handshake hosted clients still
//!   open with, answered without a session id.
//! - **Tools** are [`crate::tools`], all of them; a tool whose text exists in two languages
//!   takes a `lang` argument here. **Resources** are the articles (Markdown) and the recipes
//!   (JSON), for a person to attach to a conversation.
//! - **Guards**: the `Host` must be the site's and an `Origin`, when a browser sends one, the
//!   site's own (the transport's rules against DNS rebinding); bodies are capped; requests are
//!   counted per client address ([`limit`]).

mod limit;
#[cfg(test)]
mod tests;

use std::sync::{Arc, LazyLock};
use std::time::Instant;

use axum::Router;
use axum::http::header;
use axum::http::request::Parts;
use rmcp::model::{
    CacheScope, CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock,
    DiscoverResult, Icon, Implementation, ListResourceTemplatesResult, ListResourcesResult,
    ListToolsResult, PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResponse,
    ReadResourceResult, Resource, ResourceContents, ResourceTemplate, ServerCapabilities,
    ServerConfig, Tool, ToolAnnotations,
};
use rmcp::service::RequestContext;
use rmcp::transport::streamable_http_server::session::never::NeverSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{ErrorData, RoleServer, ServerHandler};
use serde_json::{Map, Value, json};

use self::limit::Limiter;
use crate::app::AppState;
use crate::auth::guard::origin_of;
use crate::i18n::Lang;
use crate::tools::{self, Context};

/// The endpoint's path: what an assistant is given is `PUBLIC_URL` followed by it.
pub const PATH: &str = "/mcp";
/// No request comes near it: the largest is a full formula.
const MAX_BODY_BYTES: usize = 64 * 1024;
/// How long a client may keep the lists and the texts without asking again: they change with
/// a deployment only.
const TTL_MS: u64 = 5 * 60 * 1000;
/// Requests a client address may send in a minute. An assistant sends a few per tool call
/// (hosted ones ask what the server offers before each), and the users of a hosted assistant
/// share its servers' addresses.
const REQUESTS_PER_MINUTE: u32 = 300;

const ARTICLES: &str = "breadator://articles/";
const RECIPES: &str = "breadator://recipes/";

/// The tools as the protocol lists them: a title, read-only, and `lang` where a text is returned.
static TOOLS: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    tools::TOOLS
        .iter()
        .map(|tool| {
            let mut schema = tool.parameters.as_object().cloned().unwrap_or_default();
            if tool.localized
                && let Some(Value::Object(properties)) = schema.get_mut("properties")
            {
                properties.insert(
                    "lang".to_owned(),
                    json!({
                        "type": "string",
                        "enum": ["en", "fr"],
                        "description": "Language of the text returned (default en).",
                    }),
                );
            }
            // Nothing here changes anything, and nothing reaches beyond the site's own data.
            let annotations =
                ToolAnnotations::with_title(tool.title).read_only(true).open_world(false);
            Tool::new(tool.name, tool.description, Arc::new(schema))
                .with_title(tool.title)
                .with_annotations(annotations)
        })
        .collect()
});

/// The `lang` argument of a tool that returns text, taken out of `args`: the tool itself is
/// told the language, not given the argument.
fn take_lang(args: &mut Map<String, Value>) -> Result<Lang, String> {
    let unknown =
        |given: &dyn std::fmt::Display| format!("lang: {given} is not available; use en or fr");
    match args.remove("lang") {
        None | Some(Value::Null) => Ok(Lang::En),
        // `fr-FR` is French.
        Some(Value::String(code)) => code
            .split(['-', '_'])
            .next()
            .and_then(|primary| Lang::parse(&primary.trim().to_ascii_lowercase()))
            .ok_or_else(|| unknown(&format_args!("'{code}'"))),
        Some(other) => Err(unknown(&other)),
    }
}

/// `breadator://articles/fr/yudane` → (`breadator://articles/`, French, `yudane`).
fn parse_uri(uri: &str) -> Option<(&'static str, Lang, &str)> {
    let (kind, rest) = [ARTICLES, RECIPES]
        .into_iter()
        .find_map(|kind| uri.strip_prefix(kind).map(|rest| (kind, rest)))?;
    let (lang, slug) = rest.split_once('/')?;
    Some((kind, Lang::parse(lang)?, slug))
}

#[derive(Clone)]
struct Server {
    state: AppState,
}

impl Server {
    fn context(&self, lang: Lang) -> Context<'_> {
        Context { knowledge: &self.state.knowledge, lang, site: Some(&self.state.public_url) }
    }
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        let site = &self.state.public_url;
        let mut server = Implementation::new("breadator", env!("CARGO_PKG_VERSION"))
            .with_title("Breadator")
            .with_description(
                "Bread-making calculators and a knowledge base in which every statement cites its source.",
            )
            .with_website_url(site.clone());
        if let Some(icon) = self.state.assets.url("icon.svg") {
            server = server.with_icons(vec![
                Icon::new(format!("{site}{icon}"))
                    .with_mime_type("image/svg+xml")
                    .with_sizes(vec!["any".to_owned()]),
            ]);
        }
        let instructions = format!(
            "Breadator ({site}) is a bread-making app: calculators run by one engine, and a knowledge base in which every statement cites its source (papers, books, bakers' documented tests).\n\
             - Numbers come from the calculators: ingredient weights and baker's percentages (compute_formula, solve_formula), fermentation and starter times (predict_fermentation, predict_starter, plan_existing_preferment), flour blends and hydration (blend_flour), water temperature (water_temperature), yeast conversions (convert_yeast). Pass the values the baker gave: the engine applies its defaults to the rest, and `inputs`, where a result has it, shows what was assumed.\n\
             - Units are metric: grams, °C, hours, and baker's percentages (% of all the flour of the bread).\n\
             - Times count from mixing or from feeding, fridge time included, and are estimates: `mid` is the forecast, `low` to `high` the range 80 % of bakes fall in. `notes` and `warnings` are codes to pass on as cautions; `extrapolation` means conditions outside those the model was fitted on.\n\
             - Facts come from the knowledge base: search_knowledge returns passages whose statements end with a source number such as [2], and `sources` in the same result gives the citation and link behind each number. `evidence` on a passage says what it rests on: `measured` or `literature` (a controlled test, a paper), `creator-rule` or `anecdote` (a baker's practice, a single observation), `extrapolation` (reasoned beyond the data). list_articles and read_article give whole articles, list_recipes and get_recipe the recipes, list_sources the bibliography.\n\
             - Texts exist in English and in French (`lang`, default en). A search covers the articles and recipes of that language, and statements taken from sources, which are in English: with `lang: fr`, search with French and with English keywords."
        );
        ServerConfig::new(ServerCapabilities::builder().enable_resources().enable_tools().build())
            .with_server_info(server)
            .with_instructions(instructions)
    }

    async fn discover(
        &self,
        _context: RequestContext<RoleServer>,
    ) -> Result<DiscoverResult, ErrorData> {
        let versions = ServerHandler::supported_protocol_versions(self).into_owned();
        let mut result = DiscoverResult::from_server_info(versions, self.get_info());
        result.ttl_ms = TTL_MS;
        result.cache_scope = CacheScope::Public;
        Ok(result)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(TOOLS.clone())
            .with_ttl_ms(TTL_MS)
            .with_cache_scope(CacheScope::Public))
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        TOOLS.iter().find(|tool| tool.name == name).cloned()
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let Some(tool) = tools::get(&request.name) else {
            return Err(ErrorData::invalid_params(format!("Unknown tool: {}", request.name), None));
        };
        let started = Instant::now();
        let mut args = request.arguments.unwrap_or_default();
        let lang = if tool.localized { take_lang(&mut args) } else { Ok(Lang::En) };
        let ctx = self.context(*lang.as_ref().unwrap_or(&Lang::En));
        let mut sources = Vec::new();
        let result = lang.and_then(|_| tools::run(&ctx, tool.name, &args, &mut sources));
        // Which tool, how it went and who speaks which revision; never the arguments: they
        // are a baker's own business.
        let header = |name: &str| {
            context
                .extensions
                .get::<Parts>()
                .and_then(|parts| parts.headers.get(name))
                .and_then(|value| value.to_str().ok())
                .map(|value| value.chars().take(80).collect::<String>())
                .unwrap_or_default()
        };
        tracing::info!(
            tool = tool.name,
            ok = result.is_ok(),
            micros = started.elapsed().as_micros() as u64,
            protocol = %header("mcp-protocol-version"),
            agent = %header(header::USER_AGENT.as_str()),
            "mcp tool call"
        );
        let result = match result {
            // A text (an article) goes out as text, anything else as data and as its JSON text.
            Ok(Value::String(text)) => CallToolResult::success(vec![ContentBlock::text(text)]),
            Ok(mut value) => {
                if !sources.is_empty() {
                    value["sources"] = tools::source_list(&ctx, &sources);
                }
                CallToolResult::structured(value)
            }
            // For the model to read and correct, not a failure of the protocol.
            Err(message) => CallToolResult::error(vec![ContentBlock::text(message)]),
        };
        Ok(result.into())
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        let lib = &self.state.knowledge.lib;
        let mut resources = Vec::new();
        for lang in Lang::ALL {
            let code = lang.code();
            resources.extend(lib.articles(code).into_iter().map(|article| {
                Resource::new(format!("{ARTICLES}{code}/{}", article.slug), &article.front.title)
                    .with_description(&article.front.description)
                    .with_mime_type("text/markdown")
            }));
            resources.extend(lib.recipes().map(|recipe| {
                Resource::new(format!("{RECIPES}{code}/{}", recipe.slug), recipe.title.get(code))
                    .with_description(recipe.description.get(code))
                    .with_mime_type("application/json")
            }));
        }
        Ok(ListResourcesResult::with_all_items(resources)
            .with_ttl_ms(TTL_MS)
            .with_cache_scope(CacheScope::Public))
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
        let templates = vec![
            ResourceTemplate::new(format!("{ARTICLES}{{lang}}/{{slug}}"), "Article")
                .with_description("An article of the knowledge base with its sources, in en or fr.")
                .with_mime_type("text/markdown"),
            ResourceTemplate::new(format!("{RECIPES}{{lang}}/{{slug}}"), "Recipe")
                .with_description("A recipe with its formula, weights and steps, in en or fr.")
                .with_mime_type("application/json"),
        ];
        Ok(ListResourceTemplatesResult::with_all_items(templates)
            .with_ttl_ms(TTL_MS)
            .with_cache_scope(CacheScope::Public))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, ErrorData> {
        let found = parse_uri(&request.uri).and_then(|(kind, lang, slug)| {
            let ctx = self.context(lang);
            if kind == ARTICLES {
                tools::article_markdown(&ctx, slug).map(|text| (text, "text/markdown"))
            } else {
                let recipe = tools::recipe(&ctx, slug, None).ok()?;
                Some((serde_json::to_string_pretty(&recipe).ok()?, "application/json"))
            }
        });
        let Some((text, mime)) = found else {
            return Err(ErrorData::resource_not_found(
                "Resource not found",
                Some(json!({ "uri": request.uri })),
            ));
        };
        let contents = ResourceContents::text(text, request.uri).with_mime_type(mime);
        Ok(ReadResourceResult::new(vec![contents])
            .with_ttl_ms(TTL_MS)
            .with_cache_scope(CacheScope::Public)
            .into())
    }
}

/// The site's host without its port, and its origin with one (`https://host:443`): what the
/// transport compares the `Host` and `Origin` headers with.
fn site_authority(public_url: &str) -> (String, String) {
    let origin = origin_of(public_url);
    let (scheme, authority) = origin.split_once("://").unwrap_or(("https", &origin));
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) if !port.is_empty() && port.bytes().all(|b| b.is_ascii_digit()) => {
            (host, port)
        }
        _ => (authority, if scheme == "http" { "80" } else { "443" }),
    };
    (host.to_owned(), format!("{scheme}://{host}:{port}"))
}

pub fn routes(state: AppState) -> Router {
    router(state, REQUESTS_PER_MINUTE)
}

fn router(state: AppState, per_minute: u32) -> Router {
    let (host, origin) = site_authority(&state.public_url);
    let config = StreamableHttpServerConfig::default()
        // No session for the handshake-based revisions either, and plain JSON replies.
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_allowed_hosts([host])
        .with_allowed_origins([origin])
        .with_max_request_body_bytes(MAX_BODY_BYTES);
    let service = StreamableHttpService::new(
        move || Ok(Server { state: state.clone() }),
        Arc::new(NeverSessionManager::default()),
        config,
    );
    Router::new().route_service(PATH, service).layer(axum::middleware::from_fn_with_state(
        Arc::new(Limiter::new(per_minute)),
        limit::layer,
    ))
}
