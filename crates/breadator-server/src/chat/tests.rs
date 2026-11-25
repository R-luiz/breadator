//! The chat over the real router, with OpenRouter replaced by a wiremock server that plays a
//! scripted conversation (no real model call) and the fixed test knowledge base.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use axum::http::{Method, StatusCode};
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

use super::limits::{
    self, ADDRESS_ANSWERS_PER_DAY, Asker, DAY_BUDGET_USD, MONTH_BUDGET_USD, USER_DAY_BUDGET_USD,
};
use super::store;
use super::tools::{self, Source};
use crate::db;
use crate::i18n::Lang;
use crate::knowledge::Knowledge;
use crate::testkit::{HTTPS_URL, TestApp, away_from_window_end, received_json};

const COMPLETIONS: &str = "/chat/completions";

/// Answers the n-th call with the n-th response; any call beyond the script is a test failure
/// (the server would have made more calls than it may).
struct Script {
    responses: Vec<ResponseTemplate>,
    next: AtomicUsize,
}

impl Respond for Script {
    fn respond(&self, _: &Request) -> ResponseTemplate {
        let i = self.next.fetch_add(1, Ordering::SeqCst);
        self.responses
            .get(i)
            .cloned()
            .unwrap_or_else(|| ResponseTemplate::new(418).set_body_string("unscripted call"))
    }
}

fn sse(chunks: &[Value]) -> ResponseTemplate {
    let mut body = String::from(": OPENROUTER PROCESSING\n\n");
    for chunk in chunks {
        body.push_str(&format!("data: {chunk}\n\n"));
    }
    body.push_str("data: [DONE]\n\n");
    ResponseTemplate::new(200)
        .insert_header("content-type", "text/event-stream")
        .set_body_string(body)
}

fn usage(tokens_in: i64, tokens_out: i64, cost: f64) -> Value {
    json!({ "choices": [], "usage": { "prompt_tokens": tokens_in, "completion_tokens": tokens_out, "cost": cost } })
}

/// A call answered with tool calls `(name, arguments)`.
fn tool_calls(calls: &[(&str, Value)], cost: f64) -> ResponseTemplate {
    let calls: Vec<Value> = calls
        .iter()
        .enumerate()
        .map(|(i, (name, arguments))| {
            json!({ "index": i, "id": format!("call_{name}"), "type": "function",
                    "function": { "name": name, "arguments": arguments.to_string() } })
        })
        .collect();
    sse(&[
        json!({ "model": "z-ai/glm-5.3-flash", "provider": "Relace",
                "choices": [{ "delta": { "role": "assistant", "content": "", "tool_calls": calls } }] }),
        json!({ "choices": [{ "delta": {}, "finish_reason": "tool_calls" }] }),
        usage(3000, 40, cost),
    ])
}

/// A call answered with text, in two pieces.
fn text(answer: &str, cost: f64) -> ResponseTemplate {
    let cut = answer.char_indices().nth(answer.chars().count() / 2).map_or(0, |(i, _)| i);
    sse(&[
        json!({ "model": "z-ai/glm-5.3-flash", "provider": "Relace",
                "choices": [{ "delta": { "role": "assistant", "content": &answer[..cut] } }] }),
        json!({ "choices": [{ "delta": { "content": &answer[cut..] }, "finish_reason": "stop" }] }),
        usage(3200, 90, cost),
    ])
}

async fn model(responses: Vec<ResponseTemplate>) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(COMPLETIONS))
        .respond_with(Script { responses, next: AtomicUsize::new(0) })
        .mount(&server)
        .await;
    server
}

fn app(server: &MockServer) -> TestApp {
    let uri = server.uri();
    TestApp::with(HTTPS_URL, move |c| {
        c.openrouter_api_key = Some("sk-test".to_owned());
        c.openrouter_url = uri;
    })
}

/// `(event, data)` of a server-sent event stream.
fn events(body: &str) -> Vec<(String, Value)> {
    body.split("\n\n")
        .filter_map(|block| {
            let name = block.lines().find_map(|l| l.strip_prefix("event: "))?;
            let data = block.lines().find_map(|l| l.strip_prefix("data: "))?;
            Some((name.to_owned(), serde_json::from_str(data).unwrap()))
        })
        .collect()
}

async fn real(app: &TestApp, sql: &str) -> f64 {
    let sql = sql.to_owned();
    app.state.db.read(move |c| c.query_row(&sql, [], |r| r.get(0))).await.unwrap()
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-12, "{a} != {b}");
}

/// The `chat_usage` key of an account's address.
fn account(email: &str) -> String {
    format!("a:{}", crate::auth::sha256_hex(email))
}

/// What the test client is seen as without a `Fly-Client-IP` header.
const NO_ADDRESS: &str = "0.0.0.0";

/// The sources a search of the fixture returns, numbered as the model sees them.
fn searched(query: &str) -> Vec<Source> {
    let knowledge = Knowledge::fixture();
    let ctx = tools::Context { knowledge: &knowledge, lang: Lang::En, site: None };
    let mut sources = Vec::new();
    let arguments = json!({ "queries": [query] }).to_string();
    tools::execute(&ctx, "search_knowledge", &arguments, &mut sources);
    sources
}

#[tokio::test]
async fn tool_rounds_feed_engine_results_back_and_answers_cite_the_passages() {
    let server = model(vec![
        tool_calls(&[("search_knowledge", json!({ "queries": ["yudane"] }))], 0.0001),
        tool_calls(&[("compute_formula", json!({ "by": "dough", "grams": 900, "water": 70 }))], 0.0002),
        text("Yudane scalds part of the flour [2]. It is a video tip [1, 2]. Weigh 523 g of flour. Bake at 230 °C [9].", 0.0003),
    ])
    .await;
    let app = app(&server);
    let (mut me, my_id) = app.signed_in("me@example.com").await;

    let reply = me
        .json(
            Method::POST,
            "/api/v1/chat",
            &json!({ "question": "How do I make a yudane loaf of 900 g?", "lang": "en" }),
        )
        .await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert_eq!(reply.headers["content-type"], "text/event-stream");
    assert_eq!(reply.headers["cache-control"], "no-store");
    let events = events(&reply.body);
    let names: Vec<&str> = events.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names.first(), Some(&"start"));
    assert_eq!(names.last(), Some(&"done"), "{names:?}");
    let tools: Vec<&Value> =
        events.iter().filter(|(n, _)| n == "tool").map(|(_, d)| &d["kind"]).collect();
    assert_eq!(tools, [&json!("search"), &json!("compute")]);
    assert_eq!(names.iter().filter(|n| **n == "reset").count(), 2);
    let streamed: String =
        events.iter().filter(|(n, _)| n == "text").map(|(_, d)| d["t"].as_str().unwrap()).collect();
    assert!(streamed.starts_with("Yudane scalds part of the flour [2]."), "{streamed}");
    let conversation = events[0].1["conversation"].as_str().unwrap().to_owned();

    // Three calls: the search result goes back to the model, then the engine's weights, and the
    // last call may not ask for tools.
    let calls = received_json(&server, COMPLETIONS).await;
    assert_eq!(calls.len(), 3);
    assert_eq!(calls[0]["tool_choice"], "auto");
    assert_eq!(calls[0]["session_id"], conversation.as_str());
    assert!(calls[0]["messages"][0]["content"].as_str().unwrap().contains("search_knowledge"));
    assert_eq!(
        calls[0]["messages"][1],
        json!({ "role": "user", "content": "How do I make a yudane loaf of 900 g?" })
    );
    let second = calls[1]["messages"].as_array().unwrap();
    assert_eq!(second[2]["tool_calls"][0]["function"]["name"], "search_knowledge");
    assert_eq!(second[3]["role"], "tool");
    assert_eq!(second[3]["tool_call_id"], "call_search_knowledge");
    let passages: Value = serde_json::from_str(second[3]["content"].as_str().unwrap()).unwrap();
    assert!(passages["passages"].as_array().is_some_and(|p| !p.is_empty()), "{passages}");
    assert_eq!(calls[2]["tool_choice"], "none");
    let third = calls[2]["messages"].as_array().unwrap();
    let weights: Value = serde_json::from_str(third[5]["content"].as_str().unwrap()).unwrap();
    // 900 g of dough at 100 + 70 + 2 (default salt): the engine's 523 g of flour.
    assert_eq!(
        weights["totals"][0],
        json!({ "id": "flour", "grams": 523, "pct": 100 }),
        "{weights}"
    );

    // Stored: markers renumbered in citation order, the invented [9] marked unsourced.
    let (id, content, citations, tool_calls, model, tokens_in, tokens_out, cost): (i64, String, String, String, String, i64, i64, f64) = app
        .state
        .db
        .read(|c| {
            c.query_row(
                "SELECT id, content, citations, tool_calls, model, tokens_in, tokens_out, cost_usd FROM chat_messages WHERE role = 'assistant'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?, r.get(7)?)),
            )
        })
        .await
        .unwrap();
    assert_eq!(
        content,
        "Yudane scalds part of the flour [1]. It is a video tip [2, 1]. Weigh 523 g of flour. Bake at 230 °C (unsourced)."
    );
    let cited: Vec<Source> = serde_json::from_str(&citations).unwrap();
    let found = searched("yudane");
    let keys = |n: usize| found.iter().find(|s| s.n == n).unwrap().keys.clone();
    assert_eq!(cited.len(), 2);
    assert_eq!((cited[0].n, cited[0].keys.clone()), (1, keys(2)));
    assert_eq!((cited[1].n, cited[1].keys.clone()), (2, keys(1)));
    let tool_calls: Value = serde_json::from_str(&tool_calls).unwrap();
    assert_eq!(tool_calls[1]["name"], "compute_formula");
    assert_eq!(tool_calls[1]["ok"], true);
    assert_eq!(model, "z-ai/glm-5.3-flash");
    assert_eq!((tokens_in, tokens_out), (9200, 170));
    close(cost, 0.0006);

    // The rendered answer links its markers to its source list, as an article does.
    let done = &events.last().unwrap().1;
    let html = done["html"].as_str().unwrap();
    assert!(
        html.contains(&format!("<sup class=\"cite\">[<a href=\"#m{id}-s1\">1</a>]</sup>")),
        "{html}"
    );
    assert!(html.contains(&format!("<li id=\"m{id}-s1\" value=\"1\">")), "{html}");
    assert!(html.contains("<span class=\"unsourced\">(unsourced)</span>"), "{html}");
    assert!(html.contains("Novita Listyani — Tangzhong video, 12:40"), "{html}");
    assert!(html.contains("formula calculator"), "{html}");
    assert_eq!(done["left"], limits::USER_ANSWERS_PER_DAY - 1);

    // Cost accounting: every call's `usage.cost` in the site's ledgers (month and day) and in
    // the account's day; the answer counts once against its allowance.
    let day = limits::utc_day(db::now());
    close(
        real(&app, &format!("SELECT cost_usd FROM chat_spend WHERE month = '{}'", &day[..7])).await,
        0.0006,
    );
    close(
        real(&app, &format!("SELECT cost_usd FROM chat_spend WHERE month = '{day}'")).await,
        0.0006,
    );
    let usage = format!(
        "FROM chat_usage WHERE subject = '{}' AND day = '{day}'",
        account("me@example.com")
    );
    assert_eq!(app.count(&format!("SELECT requests {usage}")).await, 1);
    assert_eq!(app.count(&format!("SELECT tokens_in {usage}")).await, 9200);
    close(real(&app, &format!("SELECT cost_usd {usage}")).await, 0.0006);
    // The client's network is counted the same way.
    let network = format!(
        "FROM chat_usage WHERE subject = '{}' AND day = '{day}'",
        limits::address_subject(NO_ADDRESS)
    );
    assert_eq!(app.count(&format!("SELECT requests {network}")).await, 1);
    close(real(&app, &format!("SELECT cost_usd {network}")).await, 0.0006);
    // Nothing names the account.
    assert_eq!(
        app.count(&format!("SELECT count(*) FROM chat_usage WHERE subject LIKE '%{my_id}%' OR subject LIKE '%example%'")).await,
        0
    );
}

#[tokio::test]
async fn without_javascript_the_form_answers_on_the_conversation_page() {
    let server = model(vec![
        tool_calls(&[("search_knowledge", json!({ "queries": ["yudane"] }))], 0.0001),
        text("Le yudane ébouillante la farine [1].", 0.0001),
        text("Oui (non sourcé).", 0.0001),
    ])
    .await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;

    let reply = me.form("/fr/chat", &[("question", "  C'est quoi le yudane ?  ")]).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    let page_path = reply.location();
    assert!(page_path.starts_with("/fr/chat/"), "{page_path}");
    // The page waits for the answer being written, then shows it with its sources.
    let page = me.get(&page_path).await;
    assert_eq!(page.status, StatusCode::OK);
    assert_eq!(page.headers["cache-control"], "no-store");
    assert!(page.body.contains("<p>C&#39;est quoi le yudane ?</p>"), "{}", page.body);
    assert!(
        page.body.contains("Le yudane ébouillante la farine <sup class=\"cite\">"),
        "{}",
        page.body
    );
    assert!(page.body.contains("<a href=\"/fr/learn/yudane\">Le yudane</a>"), "{}", page.body);
    assert!(page.body.contains("recherche dans la base de connaissances"));
    assert!(page.body.contains("Réponses restantes aujourd&#39;hui : 9"), "{}", page.body);
    let calls = received_json(&server, COMPLETIONS).await;
    assert!(calls[0]["messages"][0]["content"].as_str().unwrap().starts_with("Tu es l'assistant"));

    // A follow-up in the same conversation sends the earlier turn, without its markers.
    let id = page_path.rsplit('/').next().unwrap();
    let reply =
        me.form("/fr/chat", &[("question", "Et le tangzhong ?"), ("conversation", id)]).await;
    assert_eq!(reply.location(), page_path);
    let page = me.get(&page_path).await;
    assert!(page.body.contains("<span class=\"unsourced\">(non sourcé)</span>"), "{}", page.body);
    let calls = received_json(&server, COMPLETIONS).await;
    let messages = calls[2]["messages"].as_array().unwrap();
    assert_eq!(messages[1], json!({ "role": "user", "content": "C'est quoi le yudane ?" }));
    assert_eq!(
        messages[2],
        json!({ "role": "assistant", "content": "Le yudane ébouillante la farine." })
    );
    assert_eq!(messages[3], json!({ "role": "user", "content": "Et le tangzhong ?" }));
    assert!(
        page.body.contains("<a href=\"/fr/chat/")
            && page.body.contains("aria-current=\"page\">C&#39;est quoi le yudane ?</a>")
    );

    // Deleting it removes its messages; it is gone for good.
    let reply = me.form(&format!("{page_path}/delete"), &[]).await;
    assert_eq!(reply.location(), "/fr/chat?msg=deleted");
    assert_eq!(app.count("SELECT count(*) FROM chat_messages").await, 0);
    assert!(me.get("/fr/chat?msg=deleted").await.body.contains("Conversation supprimée."));
    assert_eq!(me.get(&page_path).await.status, StatusCode::NOT_FOUND);
    assert_eq!(me.form(&format!("{page_path}/delete"), &[]).await.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn budgets_and_allowances_refuse_before_any_model_call() {
    away_from_window_end(86_400).await;
    let server = model(Vec::new()).await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    let (mine, network) = (account("me@example.com"), limits::address_subject(NO_ADDRESS));
    let day = limits::utc_day(db::now());
    let month = day[..7].to_owned();
    let set = |sql: String| {
        let state = app.state.clone();
        async move { state.db.write(move |c| c.execute_batch(&sql)).await.unwrap() }
    };
    let ask = json!({ "question": "Why is my crumb dense?", "lang": "en" });

    let cases = [
        (
            format!("INSERT INTO chat_spend (month, cost_usd) VALUES ('{day}', {DAY_BUDGET_USD})"),
            "budget-day",
        ),
        (
            format!(
                "DELETE FROM chat_spend; INSERT INTO chat_spend (month, cost_usd) VALUES ('{month}', {MONTH_BUDGET_USD})"
            ),
            "budget-month",
        ),
        (
            format!(
                "DELETE FROM chat_spend; INSERT INTO chat_usage (subject, day, requests) VALUES ('{mine}', '{day}', 10)"
            ),
            "user-day",
        ),
        (
            // 100 answers this month, none today.
            format!(
                "DELETE FROM chat_usage; INSERT INTO chat_usage (subject, day, requests) VALUES ('{mine}', '{month}-01', 60), ('{mine}', '{month}-02', 40)"
            ),
            "user-month",
        ),
        (
            // One answer today, but it used the account's share of the day's budget.
            format!(
                "DELETE FROM chat_usage; INSERT INTO chat_usage (subject, day, requests, cost_usd) VALUES ('{mine}', '{day}', 1, {USER_DAY_BUDGET_USD})"
            ),
            "user-budget",
        ),
        (
            // Other accounts behind the same network had its answers for the day.
            format!(
                "DELETE FROM chat_usage; INSERT INTO chat_usage (subject, day, requests) VALUES ('{network}', '{day}', {ADDRESS_ANSWERS_PER_DAY})"
            ),
            "address-day",
        ),
    ];
    for (sql, code) in cases {
        set(sql).await;
        let reply = me.json(Method::POST, "/api/v1/chat", &ask).await;
        assert_eq!(reply.status, StatusCode::TOO_MANY_REQUESTS, "{code}: {}", reply.body);
        assert_eq!(reply.json()["error"]["code"], code);
        assert!(!reply.json()["error"]["message"].as_str().unwrap().is_empty());
        let form = me.form("/en/chat", &[("question", "Why is my crumb dense?")]).await;
        assert_eq!(form.status, StatusCode::TOO_MANY_REQUESTS, "{code}");
        assert!(
            form.body.contains("class=\"error\"") && form.body.contains("Why is my crumb dense?")
        );
        // The page says so before anyone types.
        assert!(me.get("/en/chat").await.body.contains("class=\"notice\""), "{code}");
    }
    assert!(server.received_requests().await.unwrap().is_empty(), "no model call");
    assert_eq!(app.count("SELECT count(*) FROM chat_messages").await, 0, "nothing stored");

    // Under the cut-offs the same question goes through.
    set("DELETE FROM chat_usage; DELETE FROM chat_spend; INSERT INTO chat_spend (month, cost_usd) VALUES ('2000-01', 5)".to_owned()).await;
    let reply = me.json(Method::POST, "/api/v1/chat", &ask).await;
    assert_eq!(reply.status, StatusCode::OK);
}

#[tokio::test]
async fn signed_out_and_unconfigured_never_reach_the_model() {
    let server = model(Vec::new()).await;
    let app = app(&server);
    let mut visitor = app.client();
    let page = visitor.get("/en/chat").await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(page.body.contains("/en/account/sign-in?next=%2Fen%2Fchat"), "{}", page.body);
    assert!(!page.body.contains("<textarea"));
    assert!(page.body.contains("name=\"robots\" content=\"noindex\""));
    let reply = visitor.json(Method::POST, "/api/v1/chat", &json!({ "question": "Hi?" })).await;
    assert_eq!(reply.status, StatusCode::UNAUTHORIZED);
    let reply = visitor.form("/en/chat", &[("question", "Hi?")]).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert!(reply.location().starts_with("/en/account/sign-in?next="), "{}", reply.location());
    assert_eq!(visitor.get("/en/chat/some-id").await.status, StatusCode::SEE_OTHER);
    assert!(server.received_requests().await.unwrap().is_empty());

    // Without OPENROUTER_API_KEY the page explains it and questions are refused.
    let app = TestApp::new();
    let (mut me, _) = app.signed_in("me@example.com").await;
    let page = me.get("/en/chat").await;
    assert!(page.body.contains("The assistant is unavailable at the moment."), "{}", page.body);
    assert!(!page.body.contains("<textarea"));
    let reply = me.json(Method::POST, "/api/v1/chat", &json!({ "question": "Hi?" })).await;
    assert_eq!(reply.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(reply.json()["error"]["code"], "unavailable");
}

#[tokio::test]
async fn questions_are_checked_and_conversations_are_private() {
    let server = model(vec![text("Bread [unsourced].", 0.0001)]).await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    let (mut you, _) = app.signed_in("you@example.com").await;

    let reply = me.json(Method::POST, "/api/v1/chat", &json!({ "question": "   " })).await;
    assert_eq!(
        (reply.status, reply.json()["error"]["code"].clone()),
        (StatusCode::UNPROCESSABLE_ENTITY, json!("empty"))
    );
    let long = "a".repeat(limits::QUESTION_MAX_CHARS + 1);
    let reply =
        me.json(Method::POST, "/api/v1/chat", &json!({ "question": long, "lang": "fr" })).await;
    assert_eq!(reply.json()["error"]["code"], "too-long");
    assert_eq!(reply.json()["error"]["message"], "Une question compte au plus 1000 caractères.");

    let reply =
        me.json(Method::POST, "/api/v1/chat", &json!({ "question": "What is bread?" })).await;
    let id = events(&reply.body)[0].1["conversation"].as_str().unwrap().to_owned();
    // Someone else's conversation does not exist for you.
    assert_eq!(you.get(&format!("/en/chat/{id}")).await.status, StatusCode::NOT_FOUND);
    let reply = you
        .json(Method::POST, "/api/v1/chat", &json!({ "question": "And?", "conversation": id }))
        .await;
    assert_eq!(reply.status, StatusCode::NOT_FOUND);
    assert_eq!(you.form(&format!("/en/chat/{id}/delete"), &[]).await.status, StatusCode::NOT_FOUND);
    assert!(!you.get("/en/chat").await.body.contains("What is bread?"));
    let page = me.get(&format!("/en/chat/{id}")).await;
    assert!(
        page.body.contains("What is bread?")
            && page.body.contains("<span class=\"unsourced\">(unsourced)</span>")
    );
    assert_eq!(received_json(&server, COMPLETIONS).await.len(), 1);
}

#[tokio::test]
async fn a_failed_answer_costs_no_allowance_and_offers_the_question_again() {
    let server = model(vec![
        ResponseTemplate::new(502)
            .set_body_json(json!({ "error": { "code": 502, "message": "no provider" } })),
    ])
    .await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    let reply = me
        .json(Method::POST, "/api/v1/chat", &json!({ "question": "Why so sour?", "lang": "en" }))
        .await;
    let events = events(&reply.body);
    assert_eq!(
        events.last().unwrap(),
        &(
            "failed".to_owned(),
            json!({ "message": "The assistant could not answer. Try again in a moment." })
        )
    );
    let id = events[0].1["conversation"].as_str().unwrap();
    let page = me.get(&format!("/en/chat/{id}")).await;
    assert!(page.body.contains("data-unanswered"));
    assert!(page.body.contains(">Why so sour?</textarea>"), "{}", page.body);
    // Refused before any work: its booking was taken back, nothing billed, nothing counted.
    assert_eq!(app.count("SELECT coalesce(sum(requests), 0) FROM chat_usage").await, 0);
    close(real(&app, "SELECT coalesce(sum(cost_usd), 0.0) FROM chat_usage").await, 0.0);
    close(real(&app, "SELECT coalesce(sum(cost_usd), 0.0) FROM chat_spend").await, 0.0);
}

#[tokio::test]
async fn a_deleted_and_recreated_account_has_no_fresh_allowance() {
    away_from_window_end(86_400).await;
    let server = model(Vec::new()).await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    // Today's answers of the account, booked as answers book them (from another network, so
    // only the account's allowance is at stake).
    let asker = Asker {
        account: account("me@example.com"),
        address: limits::address_subject("198.51.100.1"),
    };
    let day = limits::utc_day(db::now());
    app.state
        .db
        .write(move |c| {
            for _ in 0..limits::USER_ANSWERS_PER_DAY {
                store::reserve(c, &asker, &day, true, 0.0001)?;
            }
            Ok(())
        })
        .await
        .unwrap();
    let deleted = me.form("/en/account/delete", &[("confirm", "delete")]).await;
    assert_eq!(deleted.location(), "/en/account/deleted");

    // The same address signs up again (in another case): the day's answers are still used.
    let (mut again, _) = app.signed_in("Me@Example.com").await;
    let reply = again.json(Method::POST, "/api/v1/chat", &json!({ "question": "Why?" })).await;
    assert_eq!(reply.status, StatusCode::TOO_MANY_REQUESTS, "{}", reply.body);
    assert_eq!(reply.json()["error"]["code"], "user-day");
    assert!(again.get("/en/chat").await.body.contains("Answers left today: 0"));
    assert!(server.received_requests().await.unwrap().is_empty(), "no model call");
}

#[tokio::test]
async fn one_network_has_its_daily_answers_whatever_the_accounts() {
    away_from_window_end(86_400).await;
    let server = model(vec![text("Yes.", 0.0001)]).await;
    let app = app(&server);
    let day = limits::utc_day(db::now());
    let network = limits::address_subject("2001:db8:5:1::7");
    let sql = format!(
        "INSERT INTO chat_usage (subject, day, requests) VALUES ('{network}', '{day}', {ADDRESS_ANSWERS_PER_DAY})"
    );
    app.state.db.write(move |c| c.execute_batch(&sql)).await.unwrap();

    // Another account in another /64 of the same /48: refused, with its own message.
    let (mut you, _) = app.signed_in("you@example.com").await;
    you.extra.push(("fly-client-ip", "2001:db8:5:2::9".to_owned()));
    let reply = you.json(Method::POST, "/api/v1/chat", &json!({ "question": "Why?" })).await;
    assert_eq!(reply.status, StatusCode::TOO_MANY_REQUESTS, "{}", reply.body);
    assert_eq!(reply.json()["error"]["code"], "address-day");
    assert_eq!(
        reply.json()["error"]["message"],
        "Your network has already had 15 answers today. Come back tomorrow (UTC)."
    );
    assert!(server.received_requests().await.unwrap().is_empty(), "no model call");
    // Another network is not concerned.
    let (mut other, _) = app.signed_in("other@example.com").await;
    other.extra.push(("fly-client-ip", "2001:db8:6::1".to_owned()));
    let reply = other.json(Method::POST, "/api/v1/chat", &json!({ "question": "Why?" })).await;
    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(events(&reply.body).last().unwrap().0, "done");
}

#[tokio::test]
async fn calls_that_never_returned_keep_their_ceiling_booked() {
    away_from_window_end(86_400).await;
    let server = model(Vec::new()).await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    let day = limits::utc_day(db::now());
    // Calls booked at their ceiling whose answers died with the process: never settled.
    let booked = {
        let day = day.clone();
        app.state
            .db
            .write(move |c| {
                for n in 0.. {
                    let asker = Asker { account: format!("a:{n}"), address: format!("c:{n}") };
                    if !store::reserve(c, &asker, &day, true, 0.005)? {
                        return Ok(n);
                    }
                }
                unreachable!()
            })
            .await
            .unwrap()
    };
    assert_eq!(booked, 7, "the day's budget is reached through reservations alone");
    close(
        real(&app, &format!("SELECT cost_usd FROM chat_spend WHERE month = '{day}'")).await,
        0.035,
    );
    let reply = me.json(Method::POST, "/api/v1/chat", &json!({ "question": "Why?" })).await;
    assert_eq!(reply.status, StatusCode::TOO_MANY_REQUESTS, "{}", reply.body);
    assert_eq!(reply.json()["error"]["code"], "budget-day");
    assert!(server.received_requests().await.unwrap().is_empty(), "no further model call");
}

#[tokio::test]
async fn a_call_is_booked_at_its_ceiling_until_its_cost_is_known() {
    let slow = text("Slowly.", 0.0001).set_delay(Duration::from_millis(600));
    let server = model(vec![slow]).await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    let day = limits::utc_day(db::now());
    let spent =
        format!("SELECT coalesce(sum(cost_usd), 0.0) FROM chat_spend WHERE month = '{day}'");
    let answers =
        format!("SELECT requests FROM chat_usage WHERE subject = '{}'", account("me@example.com"));
    let ask = json!({ "question": "Why?" });
    let (reply, (during, counted)) =
        tokio::join!(me.json(Method::POST, "/api/v1/chat", &ask), async {
            tokio::time::sleep(Duration::from_millis(300)).await;
            (real(&app, &spent).await, app.count(&answers).await)
        });
    // While the call runs (a deploy or a crash now would leave it so): its ceiling is booked and
    // the answer counted.
    assert!(during > 0.0001, "{during}");
    assert_eq!(counted, 1);
    // Once it returned, its real cost replaced the ceiling.
    assert_eq!(events(&reply.body).last().unwrap().0, "done");
    close(real(&app, &spent).await, 0.0001);
    assert_eq!(app.count(&answers).await, 1);
}

#[tokio::test]
async fn usage_of_earlier_months_is_not_kept() {
    let app = TestApp::new();
    let day = limits::utc_day(db::now());
    let sql = format!(
        "INSERT INTO chat_usage (subject, day, requests) VALUES ('a:x', '2000-01-31', 3), ('a:x', '{day}', 1)"
    );
    app.state.db.write(move |c| c.execute_batch(&sql)).await.unwrap();
    app.state.db.purge_expired().await.unwrap();
    assert_eq!(app.count("SELECT count(*) FROM chat_usage").await, 1);
    assert_eq!(app.count(&format!("SELECT requests FROM chat_usage WHERE day = '{day}'")).await, 1);
}

#[tokio::test]
async fn one_answer_at_a_time_per_account() {
    let slow = text("Slowly.", 0.0001).set_delay(Duration::from_millis(400));
    let server = model(vec![slow]).await;
    let app = app(&server);
    let (mut me, _) = app.signed_in("me@example.com").await;
    let mut again = me.clone_session();
    let ask = json!({ "question": "Why?" });
    let (first, second) = tokio::join!(me.json(Method::POST, "/api/v1/chat", &ask), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        again.json(Method::POST, "/api/v1/chat", &ask).await
    });
    assert_eq!(first.status, StatusCode::OK);
    assert_eq!(events(&first.body).last().unwrap().0, "done");
    assert_eq!(second.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(second.json()["error"]["code"], "user-busy");
}
