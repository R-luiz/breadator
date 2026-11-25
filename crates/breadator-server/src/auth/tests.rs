//! Sign-in flows end to end through the router: magic link, OAuth, sessions, CSRF, passkey endpoints.

use axum::http::{Method, StatusCode};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

use crate::testkit::{
    Client, HTTPS_URL, Logs, Reply, TestApp, oauth_config, received_json, wait_for_requests,
};

// ---- magic link ------------------------------------------------------------------------------

async fn resend_app() -> (TestApp, MockServer) {
    let resend = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/emails"))
        .and(header("authorization", "Bearer re_test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": "mail-1" })))
        .mount(&resend)
        .await;
    let url = resend.uri();
    let app = TestApp::with(HTTPS_URL, move |c| {
        c.resend_api_key = Some("re_test".into());
        c.resend_api_url = url;
    });
    (app, resend)
}

/// The sign-in link of a mail sent through the Resend mock.
fn link_in(mail: &Value) -> String {
    let text = mail["text"].as_str().expect("text part");
    text.split_whitespace().find(|w| w.starts_with("https://")).expect("link").to_owned()
}

fn local(link: &str) -> String {
    link.strip_prefix(HTTPS_URL).expect("link on the public origin").to_owned()
}

fn token_of(link: &str) -> String {
    let rest = link.split("token=").nth(1).expect("token");
    rest.split('&').next().unwrap().to_owned()
}

fn from_ip(client: &mut Client, ip: &str) {
    client.extra.retain(|(n, _)| *n != "fly-client-ip");
    client.extra.push(("fly-client-ip", ip.to_owned()));
}

#[tokio::test]
async fn magic_link_signs_in_once_and_creates_the_user() {
    let (app, resend) = resend_app().await;
    let mut browser = app.client();
    let reply = browser.form("/en/account/sign-in", &[("email", "Jane@Example.com")]).await;
    assert_eq!(reply.status, StatusCode::OK);
    assert!(reply.body.contains("jane@example.com"));

    let mails = wait_for_requests(&resend, "/emails", 1).await;
    assert_eq!(mails.len(), 1);
    assert_eq!(mails[0]["from"], "Breadator <no-reply@robinluiz.fr>");
    assert_eq!(mails[0]["to"][0], "jane@example.com");
    assert_eq!(mails[0]["subject"], "Your Breadator sign-in link");
    let link = link_in(&mails[0]);
    assert!(link.starts_with("https://breadator.test/en/account/sign-in/verify?token="));
    assert!(mails[0]["html"].as_str().unwrap().contains(&link.replace('&', "&amp;")));
    let (page, token) = (local(&link), token_of(&link));

    // Opening the link consumes nothing (mail scanners open links), and shows the button.
    for _ in 0..2 {
        let open = browser.get(&page).await;
        assert_eq!(open.status, StatusCode::OK);
        assert!(open.body.contains(&format!("name=\"token\" value=\"{token}\"")));
        assert_eq!(app.count("SELECT count(*) FROM users").await, 0);
    }

    let verify =
        browser.form("/en/account/sign-in/verify", &[("token", &token), ("next", "")]).await;
    assert_eq!(verify.status, StatusCode::SEE_OTHER);
    assert_eq!(verify.location(), "/en/account");
    let cookie = verify.cookie_line("__Host-session").expect("session cookie");
    // HttpOnly, Secure, SameSite=Lax, Path=/, host-only (no Domain), 60 days.
    for flag in ["HttpOnly", "Secure", "SameSite=Lax", "Path=/", "Max-Age=5184000"] {
        assert!(cookie.contains(flag), "{flag} in {cookie}");
    }
    assert!(!cookie.contains("Domain"), "{cookie}");
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1);

    let account = browser.get("/en/account").await;
    assert_eq!(account.status, StatusCode::OK);
    assert!(account.body.contains("jane@example.com"));
    assert!(account.body.contains("data-signed-in"));

    // The link works once.
    let mut other = app.client();
    let again = other.form("/en/account/sign-in/verify", &[("token", &token)]).await;
    assert_eq!(again.status, StatusCode::BAD_REQUEST);
    assert!(again.cookie_line("__Host-session").is_none());
    assert_eq!(other.get(&page).await.status, StatusCode::BAD_REQUEST);

    // Signing in again later reuses the account.
    browser.form("/fr/account/sign-in", &[("email", "jane@example.com")]).await;
    let mails = wait_for_requests(&resend, "/emails", 2).await;
    assert_eq!(mails[1]["subject"], "Votre lien de connexion Breadator");
    let second = token_of(&link_in(&mails[1]));
    let verify = other.form("/fr/account/sign-in/verify", &[("token", &second)]).await;
    assert_eq!(verify.status, StatusCode::SEE_OTHER);
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1);
    assert_eq!(app.count("SELECT count(*) FROM sessions").await, 2);
}

#[tokio::test]
async fn magic_link_expires_after_fifteen_minutes() {
    let (app, resend) = resend_app().await;
    let mut browser = app.client();
    browser.form("/en/account/sign-in", &[("email", "late@example.com")]).await;
    let link = link_in(&wait_for_requests(&resend, "/emails", 1).await[0]);
    let token = token_of(&link);
    let issued: (i64, i64) = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT created_at, expires_at FROM email_tokens", [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
        })
        .await
        .unwrap();
    assert_eq!(issued.1 - issued.0, 15 * 60);

    app.state
        .db
        .write(|c| c.execute("UPDATE email_tokens SET expires_at = expires_at - 1000", []))
        .await
        .unwrap();
    assert_eq!(browser.get(&local(&link)).await.status, StatusCode::BAD_REQUEST);
    let verify = browser.form("/en/account/sign-in/verify", &[("token", &token)]).await;
    assert_eq!(verify.status, StatusCode::BAD_REQUEST);
    assert!(verify.cookie_line("__Host-session").is_none());
    assert_eq!(app.count("SELECT count(*) FROM users").await, 0);
}

#[tokio::test]
async fn magic_link_does_not_reveal_whether_an_address_is_registered() {
    let (app, resend) = resend_app().await;
    app.signed_in("known@example.com").await;
    let mut browser = app.client();
    let known = browser.form("/en/account/sign-in", &[("email", "known@example.com")]).await;
    let unknown = browser.form("/en/account/sign-in", &[("email", "nobody@example.com")]).await;
    assert_eq!(known.status, unknown.status);
    assert_eq!(known.body.replace("known@", "x@"), unknown.body.replace("nobody@", "x@"));
    assert_eq!(known.headers.get("content-type"), unknown.headers.get("content-type"));
    // Each gets one mark of the asking browser, alike but for its random value; no session.
    assert_eq!(
        cookie_shapes(&known),
        vec!["__Host-signin; Max-Age=900; Path=/; HttpOnly; SameSite=Lax; Secure"]
    );
    assert_eq!(cookie_shapes(&known), cookie_shapes(&unknown));
    // Both received a link; the only difference is on the mail server's side.
    assert_eq!(wait_for_requests(&resend, "/emails", 2).await.len(), 2);
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1);
}

/// The `Set-Cookie` lines of a reply with their values left out.
fn cookie_shapes(reply: &Reply) -> Vec<String> {
    reply
        .set_cookies()
        .iter()
        .map(|line| {
            let (pair, flags) = line.split_once(';').unwrap_or((line, ""));
            format!("{};{flags}", pair.split('=').next().unwrap_or(""))
        })
        .collect()
}

#[tokio::test]
async fn magic_link_is_rate_limited_per_address_and_per_client() {
    let (app, resend) = resend_app().await;
    let mut browser = app.client();
    // Per address: five an hour, whatever the casing or the client.
    for i in 0..5 {
        from_ip(&mut browser, &format!("203.0.113.{i}"));
        let email = if i % 2 == 0 { "busy@example.com" } else { "BUSY@example.com" };
        let reply = browser.form("/en/account/sign-in", &[("email", email)]).await;
        assert_eq!(reply.status, StatusCode::OK, "request {i}");
    }
    from_ip(&mut browser, "203.0.113.99");
    let limited = browser.form("/en/account/sign-in", &[("email", "busy@example.com")]).await;
    assert_eq!(limited.status, StatusCode::TOO_MANY_REQUESTS);
    assert!(limited.body.contains("Too many attempts"));
    assert_eq!(wait_for_requests(&resend, "/emails", 5).await.len(), 5);
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert_eq!(
        wait_for_requests(&resend, "/emails", 6).await.len(),
        5,
        "the limited request sends nothing"
    );

    // Per client address: twenty an hour across different addresses.
    let mut other = app.client();
    from_ip(&mut other, "198.51.100.7");
    for i in 0..20 {
        let reply =
            other.form("/en/account/sign-in", &[("email", &format!("user{i}@example.com"))]).await;
        assert_eq!(reply.status, StatusCode::OK, "request {i}");
    }
    let limited = other.form("/en/account/sign-in", &[("email", "one-more@example.com")]).await;
    assert_eq!(limited.status, StatusCode::TOO_MANY_REQUESTS);
    // Another client address is unaffected.
    from_ip(&mut other, "198.51.100.8");
    assert_eq!(
        other.form("/en/account/sign-in", &[("email", "fresh@example.com")]).await.status,
        StatusCode::OK
    );
}

#[tokio::test]
async fn malformed_addresses_and_unsafe_next_are_handled() {
    let (app, resend) = resend_app().await;
    let mut browser = app.client();
    let bad = browser.form("/en/account/sign-in", &[("email", "not an address")]).await;
    assert_eq!(bad.status, StatusCode::BAD_REQUEST);
    assert!(bad.body.contains("does not look like an email address"));
    assert!(crate::testkit::received_json(&resend, "/emails").await.is_empty());

    // `next` is followed only when it is a local page.
    browser
        .form("/en/account/sign-in", &[("email", "next@example.com"), ("next", "/en/my/formulas")])
        .await;
    let mails = wait_for_requests(&resend, "/emails", 1).await;
    let link = link_in(&mails[0]);
    assert!(link.contains("&next=%2Fen%2Fmy%2Fformulas"), "{link}");
    let verify = browser
        .form(
            "/en/account/sign-in/verify",
            &[("token", &token_of(&link)), ("next", "/en/my/formulas")],
        )
        .await;
    assert_eq!(verify.location(), "/en/my/formulas");

    for evil in ["//evil.example/x", "https://evil.example", "/\\evil.example", "/api/v1/me"] {
        let mut b = app.client();
        let before = crate::testkit::received_json(&resend, "/emails").await.len();
        b.form("/en/account/sign-in", &[("email", "evil@example.com"), ("next", evil)]).await;
        let mails = wait_for_requests(&resend, "/emails", before + 1).await;
        let token = token_of(&link_in(mails.last().unwrap()));
        let verify =
            b.form("/en/account/sign-in/verify", &[("token", &token), ("next", evil)]).await;
        assert_eq!(verify.location(), "/en/account", "next = {evil}");
    }
}

#[tokio::test]
async fn without_a_mail_key_nothing_is_sent_and_the_page_is_the_same() {
    let app = TestApp::new();
    let mut browser = app.client();
    let reply = browser.form("/en/account/sign-in", &[("email", "dev@example.com")]).await;
    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(app.count("SELECT count(*) FROM email_tokens").await, 1);
}

/// A deployment with no mail key and no provider: the page must not take an address and
/// promise a link that will never come.
#[tokio::test]
async fn without_any_way_to_sign_in_the_page_says_accounts_are_not_open() {
    let app = TestApp::new();
    let page = app.client().get("/en/account/sign-in").await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(page.body.contains("Accounts are not open yet"), "{}", page.body);
    assert!(!page.body.contains(r#"name="email""#) && !page.body.contains("data-passkey-login"));
    // With a mail key the form is there; with a provider alone, its button and no form.
    let (app, _resend) = resend_app().await;
    let page = app.client().get("/en/account/sign-in").await;
    assert!(page.body.contains(r#"name="email""#) && !page.body.contains("not open yet"));
    let (app, _google) = google_app().await;
    let page = app.client().get("/en/account/sign-in").await;
    assert!(page.body.contains("/auth/google/start") && !page.body.contains("not open yet"));
}

#[tokio::test]
async fn a_failing_mail_provider_does_not_change_the_answer() {
    let resend = MockServer::start().await;
    Mock::given(method("POST")).respond_with(ResponseTemplate::new(500)).mount(&resend).await;
    let url = resend.uri();
    let app = TestApp::with(HTTPS_URL, move |c| {
        c.resend_api_key = Some("re_test".into());
        c.resend_api_url = url;
    });
    let reply = app.client().form("/en/account/sign-in", &[("email", "x@example.com")]).await;
    assert_eq!(reply.status, StatusCode::OK);
}

#[tokio::test]
async fn a_magic_link_signs_in_by_itself_only_in_the_browser_that_asked_for_it() {
    let (app, resend) = resend_app().await;
    let mut asker = app.client();
    let reply = asker
        .form("/en/account/sign-in", &[("email", "jane@example.com"), ("next", "/en/my/formulas")])
        .await;
    let mark = reply.cookie_line("__Host-signin").expect("mark of the asking browser");
    for flag in ["HttpOnly", "Secure", "SameSite=Lax", "Path=/", "Max-Age=900"] {
        assert!(mark.contains(flag), "{flag} in {mark}");
    }
    assert!(!mark.contains("Domain"), "{mark}");
    let link = link_in(&wait_for_requests(&resend, "/emails", 1).await[0]);
    let (page, token) = (local(&link), token_of(&link));

    // Someone else's browser (the victim the asker sent the link to, or a mail scanner that runs
    // scripts): the page names the account, masked, warns, and waits for a press.
    let mut elsewhere = app.client();
    let open = elsewhere.get(&page).await;
    assert_eq!(open.status, StatusCode::OK);
    assert!(!open.body.contains("data-autosubmit"), "{}", open.body);
    assert!(open.body.contains("j•••@example.com"), "{}", open.body);
    assert!(!open.body.contains("jane@example.com"));
    assert!(open.body.contains("Press the button only if you asked for this link yourself"));
    // The address of the page stays on the site and out of caches and proxies. `same-origin`,
    // not `no-referrer`: with the latter a browser posts the form with `Origin: null`, which
    // the cross-site guard refuses, and nobody can sign in.
    assert_eq!(open.headers.get("referrer-policy").unwrap(), "same-origin");
    assert_eq!(open.headers.get("cache-control").unwrap(), "no-store, no-transform");
    // The mark of another request is no better.
    elsewhere.set_cookie("__Host-signin", &crate::auth::random_token());
    assert!(!elsewhere.get(&page).await.body.contains("data-autosubmit"));
    assert_eq!(app.count("SELECT count(*) FROM sessions").await, 0, "nothing used by looking");

    // The asking browser: the script presses the button at once, as before.
    let open = asker.get(&page).await;
    assert!(open.body.contains("data-autosubmit"), "{}", open.body);
    assert!(!open.body.contains("Press the button only if"));
    assert_eq!(open.headers.get("referrer-policy").unwrap(), "same-origin");
    let verify = asker
        .form("/en/account/sign-in/verify", &[("token", &token), ("next", "/en/my/formulas")])
        .await;
    assert_eq!(verify.status, StatusCode::SEE_OTHER);
    assert_eq!(verify.location(), "/en/my/formulas");
    assert!(verify.cookie_line("__Host-session").is_some());
    assert!(verify.cookie_line("__Host-signin").is_some_and(|c| c.contains("Max-Age=0")));
    assert_eq!(verify.headers.get("referrer-policy").unwrap(), "same-origin");
    // A used link is gone everywhere (and its page leaks nothing either).
    let gone = elsewhere.get(&page).await;
    assert_eq!(gone.status, StatusCode::BAD_REQUEST);
    assert_eq!(gone.headers.get("referrer-policy").unwrap(), "same-origin");
}

#[tokio::test]
async fn sign_in_mails_stop_at_the_daily_cap() {
    let logs = Logs::start();
    let (app, resend) = resend_app().await;
    // All but one of today's mails are already gone.
    let now = crate::db::now();
    app.state
        .db
        .write(move |c| {
            let day = now / 86_400;
            c.execute(
                "INSERT INTO rate_limits (key, window, count, expires_at) VALUES ('mail:day', ?1, ?2, ?3)",
                rusqlite::params![day, super::email::MAILS_PER_DAY - 1, (day + 1) * 86_400],
            )
        })
        .await
        .unwrap();
    let mut browser = app.client();
    let last = browser.form("/en/account/sign-in", &[("email", "eightieth@example.com")]).await;
    assert_eq!(last.status, StatusCode::OK);
    assert_eq!(wait_for_requests(&resend, "/emails", 1).await.len(), 1, "the 80th mail is sent");

    for i in 0..2 {
        let email = format!("late{i}@example.com");
        let capped = browser.form("/en/account/sign-in", &[("email", &email)]).await;
        // The very same answer: nothing tells the asker that the mail stayed home.
        assert_eq!(capped.status, last.status);
        assert_eq!(
            capped.body.replace(&format!("late{i}@"), "x@"),
            last.body.replace("eightieth@", "x@")
        );
        assert_eq!(cookie_shapes(&capped), cookie_shapes(&last));
    }
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert_eq!(received_json(&resend, "/emails").await.len(), 1, "the 81st mail is not sent");
    assert_eq!(app.count("SELECT count(*) FROM email_tokens").await, 1);
    // The operator hears about it, once an hour.
    let text = logs.text();
    assert_eq!(text.matches("daily cap of sign-in emails reached").count(), 1, "{text}");
    assert!(text.contains("ERROR"), "{text}");
}

#[tokio::test]
async fn sign_in_links_are_logged_only_on_a_loopback_public_url() {
    let logs = Logs::start();
    // A public deployment without a mail key: the link is never written down.
    let app = TestApp::new();
    let reply = app.client().form("/en/account/sign-in", &[("email", "prod@example.com")]).await;
    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(app.count("SELECT count(*) FROM email_tokens").await, 1);
    let text = logs.text();
    assert!(text.contains("mail is not configured"), "{text}");
    assert!(!text.contains("token=") && !text.contains("prod@example.com"), "{text}");

    // Local development: the developer opens the logged link.
    let local = TestApp::with("http://localhost:8080", |_| {});
    local.client().form("/en/account/sign-in", &[("email", "dev@example.com")]).await;
    let text = logs.text();
    assert!(text.contains("http://localhost:8080/en/account/sign-in/verify?token="), "{text}");
}

#[tokio::test]
async fn request_logs_carry_the_path_never_the_query() {
    let logs = Logs::start();
    let app = TestApp::new();
    app.client().get("/en/account/sign-in/verify?token=SECRET-TOKEN-42&next=%2Fen").await;
    app.client().get("/auth/google/callback?code=SECRET-CODE-42&state=x").await;
    let text = logs.text();
    assert!(text.contains("path=/en/account/sign-in/verify"), "{text}");
    assert!(!text.contains("SECRET-TOKEN-42") && !text.contains("SECRET-CODE-42"), "{text}");
}

#[test]
fn only_loopback_public_urls_count_as_local() {
    for url in
        ["http://localhost:8080", "http://LOCALHOST", "http://127.0.0.1:3000", "http://[::1]:8080"]
    {
        assert!(super::is_loopback(url), "{url}");
    }
    for url in [
        "https://breadator.robinluiz.fr",
        "https://localhost.evil.example",
        "http://10.0.0.1:8080",
        "not a URL",
    ] {
        assert!(!super::is_loopback(url), "{url}");
    }
}

// ---- sessions --------------------------------------------------------------------------------

#[tokio::test]
async fn session_cookie_is_plain_on_local_http() {
    let app = TestApp::with("http://localhost:8080", |_| {});
    let (client, _) = app.signed_in("dev@example.com").await;
    assert!(client.cookie("session").is_some());
    let mut client = client;
    assert_eq!(client.get("/en/account").await.status, StatusCode::OK);

    // A fresh sign-in over HTTP: no __Host- prefix, no Secure (browsers drop Secure cookies there).
    let mut browser = app.client();
    browser.form("/en/account/sign-in", &[("email", "dev@example.com")]).await;
    let token: String = app
        .state
        .db
        .write(|c| {
            // No mail key: the link is only logged, so mint a token as the handler would.
            let token = crate::auth::random_token();
            c.execute(
                "INSERT INTO email_tokens (token_hash, email, created_at, expires_at) VALUES (?1, 'dev@example.com', 0, 9999999999)",
                [crate::auth::sha256_hex(&token)],
            )?;
            Ok(token)
        })
        .await
        .unwrap();
    let verify = browser.form("/en/account/sign-in/verify", &[("token", &token)]).await;
    let cookie = verify.cookie_line("session").expect("session cookie");
    assert!(
        cookie.contains("HttpOnly") && cookie.contains("SameSite=Lax") && cookie.contains("Path=/")
    );
    assert!(!cookie.contains("Secure") && !cookie.starts_with("__Host-"), "{cookie}");
}

#[tokio::test]
async fn sign_out_everywhere_and_revocation() {
    let app = TestApp::new();
    let (mut laptop, user_id) = app.signed_in("me@example.com").await;
    // A second device of the same user.
    let token = app
        .state
        .db
        .write({
            let id = user_id.clone();
            move |c| crate::auth::session::create(c, &id, "Firefox", crate::db::now())
        })
        .await
        .unwrap();
    let mut phone = laptop.clone_session();
    phone.set_cookie("__Host-session", &token);

    assert_eq!(laptop.get("/en/account").await.status, StatusCode::OK);
    assert_eq!(phone.get("/en/account").await.status, StatusCode::OK);
    // The device list shows both, marking this one.
    let page = laptop.get("/en/account").await;
    assert!(page.body.contains("this device"));
    assert_eq!(
        page.body.matches("Sign out</button>").count(),
        1,
        "one revoke button, for the other device"
    );

    // Revoke the phone from the laptop.
    let phone_id: i64 = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT id FROM sessions WHERE user_agent = 'Firefox'", [], |r| r.get(0))
        })
        .await
        .unwrap();
    let revoke = laptop.form(&format!("/en/account/sessions/{phone_id}/revoke"), &[]).await;
    assert_eq!(revoke.status, StatusCode::SEE_OTHER);
    let after = phone.get("/en/account").await;
    assert_eq!(after.status, StatusCode::SEE_OTHER, "revoked session is signed out");
    assert!(after.location().starts_with("/en/account/sign-in"));
    // The dead cookie is cleared.
    assert!(after.cookie_line("__Host-session").is_some_and(|c| c.contains("Max-Age=0")));

    // Sign out of this device.
    let out = laptop.form("/en/account/sign-out", &[]).await;
    assert_eq!(out.status, StatusCode::SEE_OTHER);
    assert!(out.cookie_line("__Host-session").is_some_and(|c| c.contains("Max-Age=0")));
    assert_eq!(laptop.get("/en/account").await.status, StatusCode::SEE_OTHER);

    // Sign out everywhere kills every session of the user, and nobody else's.
    let (mut a, user_a) = app.signed_in("a@example.com").await;
    let (mut b, _) = app.signed_in("b@example.com").await;
    let second_token = app
        .state
        .db
        .write(move |c| crate::auth::session::create(c, &user_a, "x", crate::db::now()))
        .await
        .unwrap();
    let mut a2 = a.clone_session();
    a2.set_cookie("__Host-session", &second_token);
    assert_eq!(a2.get("/en/account").await.status, StatusCode::OK);
    let all = a.form("/en/account/sign-out-all", &[]).await;
    assert_eq!(all.status, StatusCode::SEE_OTHER);
    assert_eq!(a2.get("/en/account").await.status, StatusCode::SEE_OTHER);
    assert_eq!(b.get("/en/account").await.status, StatusCode::OK);
}

#[tokio::test]
async fn sessions_slide_and_expire() {
    let app = TestApp::new();
    let (mut client, user_id) = app.signed_in("slide@example.com").await;
    let now = crate::db::now();

    // Used within the day: nothing is written, no cookie re-issued.
    let fresh = client.get("/en/account").await;
    assert_eq!(fresh.status, StatusCode::OK);
    assert!(fresh.cookie_line("__Host-session").is_none());

    // Last used two days ago, 58 days left: the expiry slides back to 60 days and the cookie follows.
    app.state
        .db
        .write(move |c| {
            c.execute(
                "UPDATE sessions SET last_seen_at = ?1, expires_at = ?2",
                [now - 2 * 86_400, now + 58 * 86_400],
            )
        })
        .await
        .unwrap();
    let slid = client.get("/en/account").await;
    assert_eq!(slid.status, StatusCode::OK);
    let cookie = slid.cookie_line("__Host-session").expect("cookie re-issued");
    assert!(cookie.contains("Max-Age=5184000"));
    let expires: i64 = app
        .state
        .db
        .read(|c| c.query_row("SELECT expires_at FROM sessions", [], |r| r.get(0)))
        .await
        .unwrap();
    assert!(expires >= now + 60 * 86_400 - 5 && expires <= now + 60 * 86_400 + 5, "{expires}");

    // Expired: signed out, cookie dropped.
    app.state
        .db
        .write(move |c| c.execute("UPDATE sessions SET expires_at = ?1", [now - 1]))
        .await
        .unwrap();
    let expired = client.clone_session().get("/en/account").await;
    assert_eq!(expired.status, StatusCode::SEE_OTHER);
    let _ = user_id;
}

#[tokio::test]
async fn pages_show_sign_in_or_the_account_link() {
    let app = TestApp::new();
    let mut anonymous = app.client();
    let page = anonymous.get("/en/calculators/yeast").await;
    assert!(page.body.contains("/en/account/sign-in"));
    assert!(!page.body.contains("data-signed-in"));
    assert!(page.headers.get("cache-control").is_none(), "calculators stay cacheable for visitors");

    let (mut user, _) = app.signed_in("nav@example.com").await;
    let page = user.get("/en/calculators/yeast").await;
    assert!(page.body.contains("href=\"/en/account\""));
    assert!(page.body.contains(">Test</a>"));
    assert!(page.body.contains("data-signed-in"));
    assert_eq!(page.headers.get("cache-control").unwrap(), "private, no-cache");
    // Account pages are never stored, by browsers or the service worker.
    assert_eq!(user.get("/en/account").await.headers.get("cache-control").unwrap(), "no-store");
    assert_eq!(
        anonymous.get("/en/account/sign-in").await.headers.get("cache-control").unwrap(),
        "no-store"
    );
}

#[tokio::test]
async fn account_pages_require_a_session() {
    let app = TestApp::new();
    let mut anonymous = app.client();
    let reply = anonymous.get("/fr/account").await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert_eq!(reply.location(), "/fr/account/sign-in?next=%2Ffr%2Faccount");
    let api = anonymous.get("/api/v1/me").await;
    assert_eq!(api.status, StatusCode::UNAUTHORIZED);
    assert_eq!(api.json()["error"]["code"], "unauthorized");
}

// ---- CSRF ------------------------------------------------------------------------------------

#[tokio::test]
async fn state_changing_requests_must_come_from_this_site() {
    let (app, resend) = resend_app().await;
    let form = [("email", "csrf@example.com")];

    let mut no_origin = app.client().without_origin();
    assert_eq!(no_origin.form("/en/account/sign-in", &form).await.status, StatusCode::FORBIDDEN);

    let mut evil = app.client();
    evil.origin_header = Some("https://evil.example".into());
    assert_eq!(evil.form("/en/account/sign-in", &form).await.status, StatusCode::FORBIDDEN);
    evil.origin_header = Some("null".into());
    assert_eq!(evil.form("/en/account/sign-in", &form).await.status, StatusCode::FORBIDDEN);
    // A look-alike host is no better.
    evil.origin_header = Some("https://breadator.test.evil.example".into());
    assert_eq!(evil.form("/en/account/sign-in", &form).await.status, StatusCode::FORBIDDEN);
    assert_eq!(app.count("SELECT count(*) FROM email_tokens").await, 0);
    assert!(crate::testkit::received_json(&resend, "/emails").await.is_empty());

    // Referer stands in when there is no Origin.
    let mut referer = app.client().without_origin();
    referer.extra.push(("referer", format!("{HTTPS_URL}/en/account/sign-in")));
    assert_eq!(referer.form("/en/account/sign-in", &form).await.status, StatusCode::OK);
    referer.extra = vec![("referer", "https://evil.example/".into())];
    assert_eq!(referer.form("/en/account/sign-in", &form).await.status, StatusCode::FORBIDDEN);

    // Our own origin passes; GET never needs one.
    let mut own = app.client();
    assert_eq!(own.form("/en/account/sign-in", &form).await.status, StatusCode::OK);
    assert_eq!(app.client().without_origin().get("/en").await.status, StatusCode::OK);

    // A request carrying the session cookie is checked on the API too, forms included.
    let (mut user, _) = app.signed_in("victim@example.com").await;
    user.origin_header = None;
    let sync = user.json(Method::POST, "/api/v1/sync", &json!({})).await;
    assert_eq!(sync.status, StatusCode::FORBIDDEN);
    assert_eq!(sync.json()["error"]["code"], "cross_origin");
    assert_eq!(
        user.form("/en/account/delete", &[("confirm", "delete")]).await.status,
        StatusCode::FORBIDDEN
    );
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1);

    // The stateless engine API has no ambient credential to abuse: scripts and curl may call it.
    let mut script = app.client().without_origin();
    let reply = script
        .json(
            Method::POST,
            "/api/v1/yeast",
            &json!({ "grams": 10, "from": "fresh_yeast", "to": "instant_yeast" }),
        )
        .await;
    assert_eq!(reply.status, StatusCode::OK);
}

// ---- OAuth -----------------------------------------------------------------------------------

struct Started {
    state: String,
    challenge: String,
    nonce: String,
    location: reqwest::Url,
}

fn query_of(url: &reqwest::Url, key: &str) -> String {
    url.query_pairs().find(|(k, _)| k == key).map(|(_, v)| v.into_owned()).unwrap_or_default()
}

async fn start(client: &mut Client, provider: &str, extra: &str) -> Started {
    let reply = client.get(&format!("/auth/{provider}/start?lang=en{extra}")).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    let location = reqwest::Url::parse(&reply.location()).expect("provider URL");
    let cookie = reply.cookie_line("__Host-oauth").expect("state cookie");
    for flag in ["HttpOnly", "Secure", "SameSite=Lax", "Path=/", "Max-Age=600"] {
        assert!(cookie.contains(flag), "{flag} in {cookie}");
    }
    Started {
        state: query_of(&location, "state"),
        challenge: query_of(&location, "code_challenge"),
        nonce: query_of(&location, "nonce"),
        location,
    }
}

fn jwt(claims: &Value) -> String {
    let b = |s: &str| URL_SAFE_NO_PAD.encode(s.as_bytes());
    format!("{}.{}.{}", b(r#"{"alg":"none"}"#), b(&claims.to_string()), "unsigned")
}

fn form_of(req: &Request) -> std::collections::HashMap<String, String> {
    let url =
        reqwest::Url::parse(&format!("http://x/?{}", String::from_utf8_lossy(&req.body))).unwrap();
    url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect()
}

/// Token endpoint that insists on the right authorization code, client credentials and PKCE
/// verifier (as the provider would), then answers with `answer`.
async fn mount_token(server: &MockServer, challenge: &str, answer: Value) {
    let challenge = challenge.to_owned();
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(move |req: &Request| {
            let f = form_of(req);
            let verifier_ok = f
                .get("code_verifier")
                .is_some_and(|v| URL_SAFE_NO_PAD.encode(Sha256::digest(v.as_bytes())) == challenge);
            let ok = verifier_ok
                && f.get("code").map(String::as_str) == Some("the-code")
                && f.get("grant_type").map(String::as_str) == Some("authorization_code")
                && f.get("client_id").map(String::as_str) == Some("client-id")
                && f.get("client_secret").map(String::as_str) == Some("secret")
                && f.get("redirect_uri")
                    .is_some_and(|u| u.starts_with(HTTPS_URL) && u.ends_with("/callback"));
            if ok {
                ResponseTemplate::new(200).set_body_json(answer.clone())
            } else {
                ResponseTemplate::new(400).set_body_json(json!({ "error": "invalid_grant" }))
            }
        })
        .mount(server)
        .await;
}

fn google_claims(server: &MockServer, nonce: &str) -> Value {
    json!({
        "iss": server.uri(), "aud": "client-id", "exp": crate::db::now() + 300, "sub": "google-sub-1",
        "nonce": nonce, "email": "Pat@Example.com", "email_verified": true, "name": "Pat Baker",
    })
}

async fn google_app() -> (TestApp, MockServer) {
    let server = MockServer::start().await;
    let config = oauth_config(&server.uri(), "client-id");
    let app = TestApp::with(HTTPS_URL, move |c| c.google = Some(config));
    (app, server)
}

#[tokio::test]
async fn oauth_start_sends_state_pkce_and_nonce() {
    let (app, server) = google_app().await;
    let mut browser = app.client();
    let started = start(&mut browser, "google", "&next=%2Fen%2Fmy%2Fformulas").await;
    assert!(started.location.as_str().starts_with(&format!("{}/authorize?", server.uri())));
    assert_eq!(query_of(&started.location, "response_type"), "code");
    assert_eq!(query_of(&started.location, "client_id"), "client-id");
    assert_eq!(query_of(&started.location, "code_challenge_method"), "S256");
    assert_eq!(query_of(&started.location, "scope"), "openid email profile");
    assert_eq!(
        query_of(&started.location, "redirect_uri"),
        "https://breadator.test/auth/google/callback"
    );
    assert_eq!(started.state.len(), 43);
    assert_eq!(started.challenge.len(), 43);
    assert_eq!(started.nonce.len(), 43);
    // The browser holds the state in a cookie; the server holds it hashed with the verifier.
    assert_eq!(browser.cookie("__Host-oauth"), Some(started.state.as_str()));
    let stored: String = app
        .state
        .db
        .read(|c| c.query_row("SELECT state_hash FROM oauth_states", [], |r| r.get(0)))
        .await
        .unwrap();
    assert_eq!(stored, crate::auth::sha256_hex(&started.state));
    // A provider without credentials is not offered.
    assert_eq!(browser.get("/auth/github/start").await.status, StatusCode::NOT_FOUND);
    let page = browser.get("/en/account/sign-in").await;
    assert!(
        page.body.contains("Continue with Google") && !page.body.contains("Continue with GitHub")
    );
}

#[tokio::test]
async fn oauth_callback_rejects_a_state_that_is_not_ours() {
    let (app, server) = google_app().await;
    let mut browser = app.client();
    let started = start(&mut browser, "google", "").await;
    mount_token(
        &server,
        &started.challenge,
        json!({ "id_token": jwt(&google_claims(&server, &started.nonce)) }),
    )
    .await;

    // State in the URL differs from the cookie (a forged callback, login CSRF).
    let mut forged = browser.clone_session();
    let reply = forged.get("/auth/google/callback?code=the-code&state=AAAAAAAA").await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);
    assert!(reply.body.contains("Sign-in did not complete"));
    assert!(reply.cookie_line("__Host-session").is_none());

    // No cookie at all: the callback was not started in this browser.
    let mut stranger = app.client();
    let reply =
        stranger.get(&format!("/auth/google/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);

    // Cookie and URL agree but the server never issued it.
    let mut made_up = app.client();
    made_up.set_cookie("__Host-oauth", "made-up-state");
    let reply = made_up.get("/auth/google/callback?code=the-code&state=made-up-state").await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);

    // Expired.
    app.state.db.write(|c| c.execute("UPDATE oauth_states SET expires_at = 0", [])).await.unwrap();
    let reply =
        browser.get(&format!("/auth/google/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);
    assert_eq!(app.count("SELECT count(*) FROM users").await, 0);
    assert!(server.received_requests().await.unwrap().is_empty(), "no code was ever exchanged");
}

#[tokio::test]
async fn oauth_for_an_address_with_an_account_links_only_once_the_address_confirms() {
    let server = MockServer::start().await;
    let (app, resend) = {
        let resend = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": "mail-1" })))
            .mount(&resend)
            .await;
        let (config, url) = (oauth_config(&server.uri(), "client-id"), resend.uri());
        let app = TestApp::with(HTTPS_URL, move |c| {
            c.google = Some(config);
            c.resend_api_key = Some("re_test".into());
            c.resend_api_url = url;
        });
        (app, resend)
    };
    // An account that already exists for that (verified) address.
    let (_, existing) = app.signed_in("pat@example.com").await;

    let mut browser = app.client();
    let started = start(&mut browser, "google", "&next=%2Fen%2Fmy%2Fformulas").await;
    mount_token(
        &server,
        &started.challenge,
        json!({ "id_token": jwt(&google_claims(&server, &started.nonce)) }),
    )
    .await;
    let reply =
        browser.get(&format!("/auth/google/callback?code=the-code&state={}", started.state)).await;
    // Neither signed in nor linked: the address confirms first.
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert!(
        reply.body.contains("pat@example.com") && reply.body.contains("link your Google account")
    );
    assert!(reply.cookie_line("__Host-session").is_none());
    assert!(reply.cookie_line("__Host-signin").is_some());
    // The one-time cookie is spent.
    assert!(reply.cookie_line("__Host-oauth").is_some_and(|c| c.contains("Max-Age=0")));
    assert_eq!(app.count("SELECT count(*) FROM identities").await, 0);
    assert_eq!(app.count("SELECT count(*) FROM sessions").await, 1, "only the owner's own");

    let mails = wait_for_requests(&resend, "/emails", 1).await;
    assert_eq!(mails[0]["to"][0], "pat@example.com");
    assert_eq!(mails[0]["subject"], "Confirm signing in to Breadator with Google");
    assert!(mails[0]["text"].as_str().unwrap().contains("nothing is linked"));
    let link = link_in(&mails[0]);
    assert!(link.contains("&next=%2Fen%2Fmy%2Fformulas"), "{link}");
    let (page, token) = (local(&link), token_of(&link));

    // Opened anywhere else, the link says what it does and waits for a press.
    let open = app.client().get(&page).await;
    assert!(!open.body.contains("data-autosubmit"));
    assert!(
        open.body.contains("p•••@example.com") && open.body.contains("links your Google account")
    );
    // In the browser that came back from Google, it goes through by itself.
    assert!(browser.get(&page).await.body.contains("data-autosubmit"));

    let verify = browser
        .form("/en/account/sign-in/verify", &[("token", &token), ("next", "/en/my/formulas")])
        .await;
    assert_eq!(verify.status, StatusCode::SEE_OTHER);
    assert_eq!(verify.location(), "/en/my/formulas");
    assert!(
        verify
            .cookie_line("__Host-session")
            .is_some_and(|c| c.contains("HttpOnly") && c.contains("Secure"))
    );
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1, "linked, not duplicated");
    let owner: String = app
        .state
        .db
        .read(|c| c.query_row("SELECT user_id FROM identities WHERE provider = 'google' AND subject = 'google-sub-1'", [], |r| r.get(0)))
        .await
        .unwrap();
    assert_eq!(owner, existing);
    assert_eq!(browser.get("/en/account").await.status, StatusCode::OK);

    // The state is single use.
    let replay = browser
        .clone_session()
        .get(&format!("/auth/google/callback?code=the-code&state={}", started.state))
        .await;
    assert_eq!(replay.status, StatusCode::BAD_REQUEST);

    // Next time the identity alone signs in, creating nobody and mailing nobody.
    let mut again = app.client();
    let second = start(&mut again, "google", "").await;
    server.reset().await;
    mount_token(
        &server,
        &second.challenge,
        json!({ "id_token": jwt(&google_claims(&server, &second.nonce)) }),
    )
    .await;
    let reply =
        again.get(&format!("/auth/google/callback?code=the-code&state={}", second.state)).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert_eq!(reply.location(), "/en/account");
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1);
    assert_eq!(received_json(&resend, "/emails").await.len(), 1);
}

#[tokio::test]
async fn oauth_creates_an_account_for_a_new_verified_address() {
    let (app, server) = google_app().await;
    let mut browser = app.client();
    let started = start(&mut browser, "google", "").await;
    mount_token(
        &server,
        &started.challenge,
        json!({ "id_token": jwt(&google_claims(&server, &started.nonce)) }),
    )
    .await;
    let reply =
        browser.get(&format!("/auth/google/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    let (email, name): (String, String) = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT email, display_name FROM users", [], |r| Ok((r.get(0)?, r.get(1)?)))
        })
        .await
        .unwrap();
    assert_eq!((email.as_str(), name.as_str()), ("pat@example.com", "Pat Baker"));
}

#[tokio::test]
async fn oauth_google_refuses_what_it_cannot_trust() {
    for (label, tweak) in [
        ("unverified email", json!({ "email_verified": false })),
        ("email_verified as text false", json!({ "email_verified": "false" })),
        ("no email", json!({ "email": null })),
        ("wrong audience", json!({ "aud": "someone-else" })),
        ("wrong issuer", json!({ "iss": "https://evil.example" })),
        ("expired", json!({ "exp": 1 })),
        ("wrong nonce", json!({ "nonce": "not-the-nonce" })),
    ] {
        let (app, server) = google_app().await;
        let mut browser = app.client();
        let started = start(&mut browser, "google", "").await;
        let mut claims = google_claims(&server, &started.nonce);
        claims.as_object_mut().unwrap().extend(tweak.as_object().unwrap().clone());
        mount_token(&server, &started.challenge, json!({ "id_token": jwt(&claims) })).await;
        let reply = browser
            .get(&format!("/auth/google/callback?code=the-code&state={}", started.state))
            .await;
        assert!(
            reply.status.is_client_error() || reply.status == StatusCode::BAD_GATEWAY,
            "{label}: {}",
            reply.status
        );
        assert!(reply.cookie_line("__Host-session").is_none(), "{label}");
        assert_eq!(app.count("SELECT count(*) FROM users").await, 0, "{label}");
    }
}

#[tokio::test]
async fn oauth_pkce_mismatch_at_the_provider_ends_the_sign_in() {
    let (app, server) = google_app().await;
    let mut browser = app.client();
    let started = start(&mut browser, "google", "").await;
    // The provider remembers a different challenge than our verifier hashes to.
    mount_token(
        &server,
        "a-different-challenge",
        json!({ "id_token": jwt(&google_claims(&server, &started.nonce)) }),
    )
    .await;
    let reply =
        browser.get(&format!("/auth/google/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::BAD_GATEWAY);
    assert!(reply.cookie_line("__Host-session").is_none());
    assert_eq!(app.count("SELECT count(*) FROM users").await, 0);
}

async fn github_app(emails: Value) -> (TestApp, MockServer) {
    let server = MockServer::start().await;
    let config = oauth_config(&server.uri(), "client-id");
    let app = TestApp::with(HTTPS_URL, move |c| c.github = Some(config));
    Mock::given(method("GET"))
        .and(path("/user"))
        .and(header("authorization", "Bearer gho_token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "id": 4242, "login": "octocat", "name": null })),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/user/emails"))
        .and(header("authorization", "Bearer gho_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(emails))
        .mount(&server)
        .await;
    (app, server)
}

#[tokio::test]
async fn oauth_github_uses_the_primary_verified_email() {
    let (app, server) = github_app(json!([
        { "email": "spare@example.com", "primary": false, "verified": true },
        { "email": "Octo@Example.com", "primary": true, "verified": true },
    ]))
    .await;
    let mut browser = app.client();
    let started = start(&mut browser, "github", "").await;
    assert_eq!(query_of(&started.location, "scope"), "user:email");
    mount_token(
        &server,
        &started.challenge,
        json!({ "access_token": "gho_token", "token_type": "bearer" }),
    )
    .await;
    let reply =
        browser.get(&format!("/auth/github/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    let (email, name): (String, String) = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT email, display_name FROM users", [], |r| Ok((r.get(0)?, r.get(1)?)))
        })
        .await
        .unwrap();
    assert_eq!((email.as_str(), name.as_str()), ("octo@example.com", "octocat"));
    let subject: String = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT subject FROM identities WHERE provider = 'github'", [], |r| {
                r.get(0)
            })
        })
        .await
        .unwrap();
    assert_eq!(subject, "4242");
}

#[tokio::test]
async fn oauth_github_refuses_an_account_without_a_primary_verified_email() {
    for emails in [
        json!([{ "email": "a@example.com", "primary": true, "verified": false }]),
        json!([{ "email": "a@example.com", "primary": false, "verified": true }]),
        json!([]),
    ] {
        let (app, server) = github_app(emails).await;
        let mut browser = app.client();
        let started = start(&mut browser, "github", "").await;
        mount_token(&server, &started.challenge, json!({ "access_token": "gho_token" })).await;
        let reply = browser
            .get(&format!("/auth/github/callback?code=the-code&state={}", started.state))
            .await;
        assert_eq!(reply.status, StatusCode::BAD_REQUEST);
        assert!(reply.body.contains("No verified email address"));
        assert_eq!(app.count("SELECT count(*) FROM users").await, 0);
    }
}

#[tokio::test]
async fn oauth_linking_needs_the_same_signed_in_session() {
    let (app, server) =
        github_app(json!([{ "email": "other@example.com", "primary": true, "verified": true }]))
            .await;
    let (mut me, my_id) = app.signed_in("me@example.com").await;

    // Linking starts from a signed-in session and adds the identity to that account, whatever
    // address the provider account has.
    let started = start(&mut me, "github", "&link=1").await;
    mount_token(&server, &started.challenge, json!({ "access_token": "gho_token" })).await;
    let reply =
        me.get(&format!("/auth/github/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert_eq!(reply.location(), "/en/account?msg=linked");
    let owner: String = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT user_id FROM identities WHERE provider = 'github'", [], |r| {
                r.get(0)
            })
        })
        .await
        .unwrap();
    assert_eq!(owner, my_id);
    assert_eq!(app.count("SELECT count(*) FROM users").await, 1);

    // The same GitHub account cannot also be linked to someone else.
    let (mut you, _) = app.signed_in("you@example.com").await;
    let started = start(&mut you, "github", "&link=1").await;
    server.reset().await;
    github_mocks(&server).await;
    mount_token(&server, &started.challenge, json!({ "access_token": "gho_token" })).await;
    let reply =
        you.get(&format!("/auth/github/callback?code=the-code&state={}", started.state)).await;
    assert_eq!(reply.status, StatusCode::CONFLICT);

    // A link request resumed without the session that started it is refused.
    let started = start(&mut me, "github", "&link=1").await;
    let mut signed_out = app.client();
    signed_out.set_cookie("__Host-oauth", &started.state);
    server.reset().await;
    github_mocks(&server).await;
    mount_token(&server, &started.challenge, json!({ "access_token": "gho_token" })).await;
    let reply = signed_out
        .get(&format!("/auth/github/callback?code=the-code&state={}", started.state))
        .await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);
}

async fn github_mocks(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/user"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "id": 4242, "login": "octocat", "name": null })),
        )
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/user/emails"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!([{ "email": "other@example.com", "primary": true, "verified": true }]),
        ))
        .mount(server)
        .await;
}

// ---- passkeys --------------------------------------------------------------------------------

#[tokio::test]
async fn passkey_endpoints_guard_their_ceremonies() {
    let app = TestApp::new();
    let mut anonymous = app.client();

    // Registration needs a session.
    let reply = anonymous.json(Method::POST, "/api/v1/passkeys/register/start", &json!({})).await;
    assert_eq!(reply.status, StatusCode::UNAUTHORIZED);

    // Sign-in hands out a challenge for any discoverable credential.
    let start = anonymous.json(Method::POST, "/api/v1/passkeys/login/start", &json!({})).await;
    assert_eq!(start.status, StatusCode::OK);
    let body = start.json();
    assert!(body["ceremony"].as_str().is_some_and(|c| c.len() >= 40));
    assert!(body["options"]["publicKey"]["challenge"].as_str().is_some());
    assert_eq!(body["options"]["publicKey"]["rpId"], "breadator.test");
    assert_eq!(body["options"]["publicKey"]["userVerification"], "required");

    // An assertion for a ceremony nobody started, or garbage, is refused without a session.
    let bogus = json!({ "ceremony": "nope", "credential": {
        "id": "AAAA", "rawId": "AAAA", "type": "public-key",
        "response": { "authenticatorData": "AAAA", "clientDataJSON": "AAAA", "signature": "AAAA", "userHandle": "AAAA" },
        "extensions": {}
    }});
    let reply = anonymous.json(Method::POST, "/api/v1/passkeys/login/finish", &bogus).await;
    assert!(reply.status.is_client_error(), "{}", reply.status);
    assert!(reply.cookie_line("__Host-session").is_none());

    // A registration is bound to the user who started it.
    let (mut me, _) = app.signed_in("pk@example.com").await;
    let reg = me.json(Method::POST, "/api/v1/passkeys/register/start", &json!({})).await;
    assert_eq!(reg.status, StatusCode::OK);
    let options = reg.json();
    let selection = &options["options"]["publicKey"]["authenticatorSelection"];
    assert_eq!(
        selection["residentKey"], "required",
        "discoverable credentials, for username-less sign-in"
    );
    assert_eq!(selection["requireResidentKey"], true);
    assert_eq!(options["options"]["publicKey"]["user"]["name"], "pk@example.com");

    let (mut other, _) = app.signed_in("other@example.com").await;
    let finish = json!({ "ceremony": options["ceremony"], "name": "Laptop", "credential": {
        "id": "AAAA", "rawId": "AAAA", "type": "public-key",
        "response": { "attestationObject": "AAAA", "clientDataJSON": "AAAA" }, "extensions": {}
    }});
    let reply = other.json(Method::POST, "/api/v1/passkeys/register/finish", &finish).await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);
    assert_eq!(app.count("SELECT count(*) FROM passkeys").await, 0);
}

#[tokio::test]
async fn passkey_sign_in_starts_are_rate_limited_per_client() {
    let app = TestApp::new();
    let mut flood = app.client();
    from_ip(&mut flood, "203.0.113.50");
    for i in 0..30 {
        let reply = flood.json(Method::POST, "/api/v1/passkeys/login/start", &json!({})).await;
        assert_eq!(reply.status, StatusCode::OK, "start {i}");
    }
    let limited = flood.json(Method::POST, "/api/v1/passkeys/login/start", &json!({})).await;
    assert_eq!(limited.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(limited.json()["error"]["code"], "rate_limited");
    // Another client address is unaffected.
    let mut other = app.client();
    from_ip(&mut other, "198.51.100.50");
    let reply = other.json(Method::POST, "/api/v1/passkeys/login/start", &json!({})).await;
    assert_eq!(reply.status, StatusCode::OK);
}
