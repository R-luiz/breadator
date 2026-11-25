//! How to use Breadator from an AI assistant: the address of the MCP endpoint and, for each
//! client, the steps and what to paste. Its texts are in `locales/<lang>/assistants.ftl`.

use askama::Template;
use axum::extract::State;
use axum::response::Response;

use crate::app::{AppState, LangPath, Layout, render};
use crate::{mcp, tools};

/// The page's path after the language segment; `GET /mcp` from a browser leads here.
pub const PATH: &str = "/assistants";

#[derive(Template)]
#[template(path = "assistants.html")]
struct Page {
    layout: Layout,
    /// The endpoint's address on this deployment.
    url: String,
    address_note: String,
    /// Requests to paste, in the page's language.
    examples: Vec<String>,
}

pub async fn page(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    let address_note =
        tr.t_args("assistants-address-note", &[("tools", (tools::TOOLS.len() as f64).into())]);
    let examples = (1..).map_while(|n| tr.try_t(&format!("assistants-example-{n}"))).collect();
    let layout = Layout::new(
        &state,
        lang,
        PATH,
        "assistants",
        tr.t("assistants-title"),
        tr.t("assistants-description"),
    );
    let url = format!("{}{}", state.public_url, mcp::PATH);
    render(&Page { layout, url, address_note, examples })
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::testkit::{HTTPS_URL, TestApp};

    /// The text of the `n`-th block to copy, as the browser shows it (and copies it).
    fn block(body: &str, n: usize) -> String {
        let start = body.match_indices("<pre>").nth(n).map(|(at, _)| at + 5).unwrap();
        let end = start + body[start..].find("</pre>").unwrap();
        body[start..end]
            .replace("<code>", "")
            .replace("</code>", "")
            .replace("&quot;", "\"")
            .replace("&#34;", "\"")
            .replace("&#39;", "'")
            .replace("&amp;", "&")
    }

    #[tokio::test]
    async fn the_guide_gives_this_deployments_address_and_what_to_paste() {
        let app = TestApp::new();
        let url = format!("{HTTPS_URL}/mcp");
        for (path, example) in [
            ("/fr/assistants", "Avec Breadator : deux pains"),
            ("/en/assistants", "With Breadator: two"),
        ] {
            let page = app.client().get(path).await;
            assert_eq!(page.status, StatusCode::OK, "{path}");
            let body = &page.body;
            // The address, the two Claude Code commands and the Cursor file, ready to paste.
            assert_eq!(block(body, 0), url, "{path}");
            assert_eq!(block(body, 1), format!("claude mcp add --transport http breadator {url}"));
            assert_eq!(
                block(body, 2),
                format!("claude mcp add --transport http --scope user breadator {url}")
            );
            assert_eq!(block(body, 3), "claude mcp list");
            let cursor: serde_json::Value = serde_json::from_str(&block(body, 4))
                .unwrap_or_else(|e| panic!("{path}: the Cursor file is not JSON ({e})"));
            assert_eq!(cursor["mcpServers"]["breadator"]["url"], url.as_str());
            // Every request to paste, then the developers' check.
            let examples = body.matches(r#"class="copyable example""#).count();
            assert_eq!(examples, 6, "{path}");
            assert!(block(body, 5).starts_with(example), "{path}: {}", block(body, 5));
            assert_eq!(
                block(body, 11),
                format!(
                    "npx -y @modelcontextprotocol/inspector --cli {url} --transport http --method tools/list"
                )
            );
            // The copy buttons are added by the page's script.
            assert!(body.contains(r#"data-island="copy""#), "{path}");
            // How many tools the endpoint has, from the registry.
            assert!(body.contains(" 14 "), "{path}");
        }
        let sitemap = app.client().get("/sitemap.xml").await.body;
        assert!(sitemap.contains(&format!("<loc>{HTTPS_URL}/fr/assistants</loc>")), "{sitemap}");
    }
}
