//! The endpoint through the real router: both protocol eras, the tools, the resources and the
//! guards, as an assistant's HTTP client sees them.

use axum::http::{Method, StatusCode, header};
use serde_json::{Value, json};

use super::*;
use crate::testkit::{Client, HTTPS_URL, Reply, TestApp};

const HOST: &str = "breadator.test";
/// The stateless revision, and the newest of those that open with a handshake.
const MODERN: &str = "2026-07-28";
const LEGACY: &str = "2025-06-18";

/// A client as an assistant's backend is: no cookie, no `Origin`.
fn assistant(app: &TestApp) -> Client {
    let mut client = app.client().without_origin();
    client.extra = vec![
        ("host", HOST.to_owned()),
        ("accept", "application/json, text/event-stream".to_owned()),
        ("user-agent", "Claude-User".to_owned()),
    ];
    client
}

fn with(mut client: Client, headers: &[(&'static str, &str)]) -> Client {
    for &(name, value) in headers {
        client.extra.retain(|(n, _)| *n != name);
        client.extra.push((name, value.to_owned()));
    }
    client
}

/// A request of the stateless revision: its version, the client and its capabilities travel in
/// `_meta`, and the method and the name are repeated in headers.
async fn modern(app: &TestApp, method: &'static str, mut params: Value) -> Reply {
    params["_meta"] = json!({
        "io.modelcontextprotocol/protocolVersion": MODERN,
        "io.modelcontextprotocol/clientInfo": { "name": "test", "version": "1.0.0" },
        "io.modelcontextprotocol/clientCapabilities": {},
    });
    let name: Option<String> =
        params.get("name").or(params.get("uri")).and_then(Value::as_str).map(str::to_owned);
    let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
    let mut headers = vec![("mcp-protocol-version", MODERN), ("mcp-method", method)];
    if let Some(name) = &name {
        headers.push(("mcp-name", name));
    }
    with(assistant(app), &headers).json(Method::POST, PATH, &body).await
}

/// A request as the handshake-based revisions send it once initialized: nothing but the
/// version header.
async fn legacy(app: &TestApp, method: &str, params: Value) -> Reply {
    let body = json!({ "jsonrpc": "2.0", "id": 7, "method": method, "params": params });
    with(assistant(app), &[("mcp-protocol-version", LEGACY)]).json(Method::POST, PATH, &body).await
}

async fn call(app: &TestApp, name: &str, arguments: Value) -> Value {
    let reply = modern(app, "tools/call", json!({ "name": name, "arguments": arguments })).await;
    assert_eq!(reply.status, StatusCode::OK, "{name}: {}", reply.body);
    reply.json()["result"].clone()
}

#[tokio::test]
async fn a_client_of_the_handshake_era_is_served_without_a_session() {
    let app = TestApp::new();
    // What a hosted assistant sends first when a connector is added.
    let hello = json!({
        "jsonrpc": "2.0", "id": 0, "method": "initialize",
        "params": {
            "protocolVersion": LEGACY,
            "capabilities": {},
            "clientInfo": { "name": "claude-ai", "version": "0.1.0" },
        },
    });
    let reply = assistant(&app).json(Method::POST, PATH, &hello).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert!(reply.headers.get("mcp-session-id").is_none(), "{:?}", reply.headers);
    assert_eq!(reply.headers[header::CONTENT_TYPE], "application/json");
    let result = &reply.json()["result"];
    assert_eq!(result["protocolVersion"], LEGACY);
    assert_eq!(result["serverInfo"]["name"], "breadator");
    assert_eq!(result["serverInfo"]["title"], "Breadator");
    assert_eq!(result["serverInfo"]["websiteUrl"], HTTPS_URL);
    assert!(result["capabilities"]["tools"].is_object(), "{result}");
    assert!(result["capabilities"]["resources"].is_object(), "{result}");
    assert!(result["instructions"].as_str().unwrap().contains("search_knowledge"), "{result}");

    // The notification that ends the handshake is accepted, with nothing to say.
    let done = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
    let reply = with(assistant(&app), &[("mcp-protocol-version", LEGACY)])
        .json(Method::POST, PATH, &done)
        .await;
    assert_eq!(reply.status, StatusCode::ACCEPTED, "{}", reply.body);

    // Each later request stands alone: no session id, and the machine may have restarted.
    let reply = legacy(&app, "tools/list", json!({})).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    let listed = reply.json();
    let listed = listed["result"]["tools"].as_array().unwrap();
    assert_eq!(listed.len(), tools::TOOLS.len());

    let reply = legacy(
        &app,
        "tools/call",
        json!({ "name": "convert_yeast", "arguments": { "grams": 21, "from": "fresh_yeast", "to": "instant_yeast" } }),
    )
    .await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    let result = &reply.json()["result"];
    assert_eq!(result["structuredContent"], json!({"grams": 6.93, "low_g": 5.25, "high_g": 8.82}));
    assert_eq!(result["isError"], false);
}

#[tokio::test]
async fn a_client_of_the_stateless_era_discovers_and_lists() {
    let app = TestApp::new();
    let reply = modern(&app, "server/discover", json!({})).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    let result = &reply.json()["result"];
    let versions = result["supportedVersions"].as_array().unwrap();
    assert!(versions.contains(&json!(MODERN)) && versions.contains(&json!(LEGACY)), "{result}");
    assert_eq!(result["_meta"]["io.modelcontextprotocol/serverInfo"]["name"], "breadator");
    assert!(result["capabilities"]["tools"].is_object(), "{result}");
    assert!(result["instructions"].as_str().unwrap().contains(HTTPS_URL), "{result}");
    // Anyone may keep it for a while: it says nothing of the caller.
    assert_eq!((&result["ttlMs"], &result["cacheScope"]), (&json!(300_000), &json!("public")));

    let reply = modern(&app, "tools/list", json!({})).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    let result = &reply.json()["result"];
    assert_eq!((&result["ttlMs"], &result["cacheScope"]), (&json!(300_000), &json!("public")));
    let listed = result["tools"].as_array().unwrap();
    let names: Vec<&str> = listed.iter().map(|tool| tool["name"].as_str().unwrap()).collect();
    let registered: Vec<&str> = tools::TOOLS.iter().map(|tool| tool.name).collect();
    assert_eq!(names, registered);
    for tool in listed {
        // What a host shows a person, and what tells it that nothing is changed anywhere.
        assert!(tool["title"].as_str().is_some_and(|title| !title.is_empty()), "{tool}");
        assert_eq!(tool["annotations"]["title"], tool["title"], "{tool}");
        assert_eq!(tool["annotations"]["readOnlyHint"], true, "{tool}");
        assert_eq!(tool["annotations"]["openWorldHint"], false, "{tool}");
        assert_eq!(tool["inputSchema"]["type"], "object", "{tool}");
        // A text in two languages is asked for in one of them.
        let lang = &tool["inputSchema"]["properties"]["lang"];
        let localized = tools::get(tool["name"].as_str().unwrap()).unwrap().localized;
        assert_eq!(lang["enum"] == json!(["en", "fr"]), localized, "{tool}");
    }

    // A revision that does not exist is refused with those that do.
    let body = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tools/list",
        "params": { "_meta": {
            "io.modelcontextprotocol/protocolVersion": "2099-01-01",
            "io.modelcontextprotocol/clientCapabilities": {},
        } },
    });
    let headers = [("mcp-protocol-version", "2099-01-01"), ("mcp-method", "tools/list")];
    let reply = with(assistant(&app), &headers).json(Method::POST, PATH, &body).await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST, "{}", reply.body);
    let error = &reply.json()["error"];
    assert_eq!(error["code"], -32022, "{error}");
    assert!(error["data"]["supported"].as_array().unwrap().contains(&json!(MODERN)), "{error}");
}

#[tokio::test]
async fn calculators_answer_with_data_and_wrong_arguments_with_a_message() {
    let app = TestApp::new();
    let result = call(
        &app,
        "compute_formula",
        json!({ "by": "baked", "grams": 800, "count": 2, "water": 72, "levain": 20 }),
    )
    .await;
    assert_eq!(result["isError"], false, "{result}");
    let data = &result["structuredContent"];
    assert_eq!(data["metrics"]["dough_g"], 1860);
    assert_eq!(data["stages"][0]["id"], "levain");
    // The same data as text, for a client that reads only that.
    let text: Value = serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(&text, data);

    let result =
        call(&app, "compute_formula", json!({ "by": "dough", "grams": 900, "hydration": 70 }))
            .await;
    assert_eq!(result["isError"], true, "{result}");
    assert!(result.get("structuredContent").is_none(), "{result}");
    let message = result["content"][0]["text"].as_str().unwrap();
    assert!(message.contains("unknown argument 'hydration'; the arguments are: "), "{message}");
    assert!(message.contains("water"), "{message}");

    // A tool that does not exist is the protocol's error, not a tool's.
    let reply = modern(&app, "tools/call", json!({ "name": "bake_bread", "arguments": {} })).await;
    let error = &reply.json()["error"];
    assert_eq!(error["code"], -32602, "{}", reply.body);
    assert_eq!(error["message"], "Unknown tool: bake_bread");
}

#[tokio::test]
async fn the_knowledge_base_answers_in_the_language_asked_with_its_sources() {
    let app = TestApp::new();
    let result =
        call(&app, "search_knowledge", json!({ "queries": ["yudane"], "lang": "fr-FR" })).await;
    assert_eq!(result["isError"], false, "{result}");
    let data = &result["structuredContent"];
    let passage = data["passages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passage| passage["title"] == "Le yudane")
        .unwrap_or_else(|| panic!("{data}"));
    let text = passage["text"].as_str().unwrap();
    assert!(text.starts_with("Le yudane ébouillante une partie de la farine ["), "{text}");
    // The passage links to the page it was read on, in the language asked.
    assert_eq!(passage["page"], format!("{HTTPS_URL}/fr/learn/yudane"));
    // The number after the statement is in the list, with the works cited.
    let n: u64 = text.rsplit_once('[').unwrap().1.trim_end_matches(']').parse().unwrap();
    let source = data["sources"].as_array().unwrap().iter().find(|s| s["n"] == n).unwrap();
    assert_eq!(
        source["citations"][0],
        json!({
            "text": "Novita Listyani — Tangzhong video, 12:40",
            "url": "https://www.youtube.com/watch?v=ocBV6-O5SL4&t=760s",
        })
    );
    assert_eq!(source["citations"][1]["url"], "https://doi.org/10.1002/jsfa.12565");

    // An article is a text: it goes out as one, English unless asked otherwise.
    let result = call(&app, "read_article", json!({ "slug": "yudane" })).await;
    assert_eq!(result["isError"], false, "{result}");
    assert!(result.get("structuredContent").is_none(), "{result}");
    let article = result["content"][0]["text"].as_str().unwrap();
    assert!(article.starts_with("# Yudane\n"), "{article}");
    assert!(
        article.contains("boiling water [1][2]") && article.contains("\n## Sources\n"),
        "{article}"
    );

    let result = call(&app, "get_recipe", json!({ "slug": "milk-bread", "lang": "fr" })).await;
    assert_eq!(result["structuredContent"]["title"], "Pain de mie", "{result}");

    for lang in [json!("de"), json!(3)] {
        let result = call(&app, "list_articles", json!({ "lang": lang })).await;
        assert_eq!(result["isError"], true, "{result}");
        let message = result["content"][0]["text"].as_str().unwrap();
        assert!(message.contains("use en or fr"), "{message}");
    }
    // A tool whose result has no language does not take one.
    let result = call(
        &app,
        "convert_yeast",
        json!({ "grams": 1, "from": "fresh_yeast", "to": "instant_yeast", "lang": "fr" }),
    )
    .await;
    assert_eq!(result["isError"], true, "{result}");
}

#[tokio::test]
async fn articles_and_recipes_are_resources_in_both_languages() {
    let app = TestApp::new();
    let reply = modern(&app, "resources/list", json!({})).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    let result = &reply.json()["result"];
    assert_eq!(result["cacheScope"], "public");
    let listed: Vec<(&str, &str, &str)> = result["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["uri"].as_str().unwrap(),
                r["name"].as_str().unwrap(),
                r["mimeType"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        listed,
        [
            ("breadator://articles/fr/yudane", "Le yudane", "text/markdown"),
            ("breadator://recipes/fr/milk-bread", "Pain de mie", "application/json"),
            ("breadator://articles/en/yudane", "Yudane", "text/markdown"),
            ("breadator://recipes/en/milk-bread", "Milk bread", "application/json"),
        ]
    );

    let reply = modern(&app, "resources/templates/list", json!({})).await;
    let templates = reply.json()["result"]["resourceTemplates"].clone();
    assert_eq!(templates[0]["uriTemplate"], "breadator://articles/{lang}/{slug}", "{}", reply.body);
    assert_eq!(templates[1]["uriTemplate"], "breadator://recipes/{lang}/{slug}");

    let reply =
        modern(&app, "resources/read", json!({ "uri": "breadator://articles/fr/yudane" })).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    let contents = &reply.json()["result"]["contents"][0];
    assert_eq!(contents["uri"], "breadator://articles/fr/yudane");
    assert_eq!(contents["mimeType"], "text/markdown");
    assert!(contents["text"].as_str().unwrap().starts_with("# Le yudane\n"), "{contents}");

    let reply =
        modern(&app, "resources/read", json!({ "uri": "breadator://recipes/en/milk-bread" })).await;
    let contents = &reply.json()["result"]["contents"][0];
    let recipe: Value = serde_json::from_str(contents["text"].as_str().unwrap()).unwrap();
    assert_eq!(recipe["weights"]["metrics"]["dough_g"], 900, "{recipe}");

    for uri in [
        "breadator://articles/en/croissants",
        "breadator://articles/de/yudane",
        "breadator://recipes/en",
        "https://example.org/",
    ] {
        let reply = modern(&app, "resources/read", json!({ "uri": uri })).await;
        let error = &reply.json()["error"];
        assert_eq!(error["code"], -32602, "{uri}: {}", reply.body);
        assert_eq!(error["data"]["uri"], uri);
    }
}

#[tokio::test]
async fn the_endpoint_is_guarded() {
    let app = TestApp::new();
    let list = json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list" });
    let version = [("mcp-protocol-version", LEGACY)];

    // Nothing to read with GET (no stream is offered) and no session to end.
    for method in [Method::GET, Method::DELETE] {
        let reply = assistant(&app).request(method, PATH, None, Vec::new()).await;
        assert_eq!(reply.status, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(reply.headers[header::ALLOW], "POST");
    }
    // A person who opens the address in a browser is sent to the page that explains it, in
    // the browser's language.
    let mut browser = app.client();
    browser.extra = vec![
        ("accept", "text/html,application/xhtml+xml,*/*;q=0.8".to_owned()),
        ("accept-language", "fr-FR,fr;q=0.9,en;q=0.5".to_owned()),
    ];
    let reply = browser.get(PATH).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert_eq!(reply.location(), "/fr/assistants");
    assert_eq!(browser.get(&reply.location()).await.status, StatusCode::OK);

    // Another site's page, in a browser, is turned away; the site's own is not, and a
    // signed-in browser is served like anyone: the session is not even looked at.
    let foreign = with(assistant(&app), &version);
    let mut foreign = with(foreign, &[("origin", "https://evil.example")]);
    let reply = foreign.json(Method::POST, PATH, &list).await;
    assert_eq!(reply.status, StatusCode::FORBIDDEN, "{}", reply.body);
    let (signed_in, _) = app.signed_in("baker@example.com").await;
    let mut own = with(signed_in, &version);
    own.extra.push(("host", HOST.to_owned()));
    own.extra.push(("accept", "application/json, text/event-stream".to_owned()));
    let reply = own.json(Method::POST, PATH, &list).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert!(reply.set_cookies().is_empty() && !reply.headers.contains_key(header::CACHE_CONTROL));

    // Reached under another name (DNS rebinding), the endpoint answers nothing.
    let mut elsewhere = with(with(assistant(&app), &version), &[("host", "attacker.example")]);
    let reply = elsewhere.json(Method::POST, PATH, &list).await;
    assert_eq!(reply.status, StatusCode::FORBIDDEN, "{}", reply.body);

    // The site's security headers are on its replies too.
    let reply = with(assistant(&app), &version).json(Method::POST, PATH, &list).await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert_eq!(reply.headers[header::X_CONTENT_TYPE_OPTIONS], "nosniff");

    // A body larger than any call is refused before it is read whole.
    let padding = "x".repeat(MAX_BODY_BYTES);
    let large =
        json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "pad": padding } });
    let reply = with(assistant(&app), &version).json(Method::POST, PATH, &large).await;
    assert_eq!(reply.status, StatusCode::PAYLOAD_TOO_LARGE, "{}", reply.body);

    // What is not JSON-RPC is told so.
    let reply = with(assistant(&app), &version)
        .request(Method::POST, PATH, Some("application/json"), b"{\"jsonrpc\":".to_vec())
        .await;
    assert!(reply.status.is_client_error(), "{}", reply.body);
}

#[tokio::test]
async fn a_client_address_is_held_to_its_requests_per_minute() {
    let app = TestApp::new();
    // The same endpoint, allowing three requests a minute.
    let endpoint = router(app.state.clone(), 3);
    let list = json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list" });
    let mut statuses = Vec::new();
    let mut last = None;
    for _ in 0..4 {
        let request = axum::http::Request::builder()
            .method(Method::POST)
            .uri(PATH)
            .header(header::HOST, HOST)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .header("mcp-protocol-version", LEGACY)
            .body(axum::body::Body::from(list.to_string()))
            .unwrap();
        let response = tower::ServiceExt::oneshot(endpoint.clone(), request).await.unwrap();
        statuses.push(response.status());
        last = Some(response);
    }
    // The minutes are counted from the endpoint's start: these four are in its first.
    assert_eq!(statuses[..3], [StatusCode::OK; 3]);
    let refused = last.unwrap();
    assert_eq!(refused.status(), StatusCode::TOO_MANY_REQUESTS);
    let wait: u64 = refused.headers()[header::RETRY_AFTER].to_str().unwrap().parse().unwrap();
    assert!((1..=60).contains(&wait), "{wait}");
    let body = http_body_util::BodyExt::collect(refused.into_body()).await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["error"]["code"], -32000, "{body}");
}

#[test]
fn the_site_is_the_only_host_and_origin_accepted() {
    for (public_url, host, origin) in [
        (
            "https://breadator.robinluiz.fr",
            "breadator.robinluiz.fr",
            "https://breadator.robinluiz.fr:443",
        ),
        ("http://localhost:8080", "localhost", "http://localhost:8080"),
        ("http://Example.org/", "example.org", "http://example.org:80"),
        ("http://[::1]:8095", "[::1]", "http://[::1]:8095"),
    ] {
        assert_eq!(
            site_authority(public_url),
            (host.to_owned(), origin.to_owned()),
            "{public_url}"
        );
    }
}
