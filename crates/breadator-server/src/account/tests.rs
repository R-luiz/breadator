//! Account pages: settings, sign-in methods (never the last one), export and deletion.

use axum::http::{Method, StatusCode};
use serde_json::json;

use crate::auth::session;
use crate::i18n::Lang;
use crate::testkit::TestApp;

async fn add_passkey(app: &TestApp, user_id: &str, n: u32) -> i64 {
    let user_id = user_id.to_owned();
    app.state
        .db
        .write(move |c| {
            c.execute(
                "INSERT INTO passkeys (user_id, credential_id, credential, name, created_at)
                 VALUES (?1, ?2, '{}', ?3, 1700000000)",
                rusqlite::params![user_id, format!("cred-{user_id}-{n}"), format!("Key {n}")],
            )?;
            Ok(c.last_insert_rowid())
        })
        .await
        .unwrap()
}

async fn add_identity(app: &TestApp, user_id: &str, provider: &str, subject: &str) -> i64 {
    let (user_id, provider, subject) =
        (user_id.to_owned(), provider.to_owned(), subject.to_owned());
    app.state
        .db
        .write(move |c| {
            session::add_identity(c, &user_id, &provider, &subject, None, 1_700_000_000)?;
            Ok(c.last_insert_rowid())
        })
        .await
        .unwrap()
}

/// A user without an address (passkey-only) holding a session.
async fn passkey_only_user(app: &TestApp) -> (crate::testkit::Client, String) {
    let (id, token) = app
        .state
        .db
        .write(|c| {
            let now = crate::db::now();
            let id = session::create_user(c, None, "Key only", Lang::En, now)?;
            let token = session::create(c, &id, "TestAgent", now)?;
            Ok((id, token))
        })
        .await
        .unwrap();
    let mut client = app.client();
    client.set_cookie(app.state.auth.session_cookie(), &token);
    (client, id)
}

#[tokio::test]
async fn the_last_way_to_sign_in_can_never_be_removed() {
    let app = TestApp::new();
    let (mut client, user) = passkey_only_user(&app).await;
    let first = add_passkey(&app, &user, 1).await;
    let second = add_passkey(&app, &user, 2).await;

    let page = client.get("/en/account").await;
    assert!(page.body.contains("Key 1") && page.body.contains("Key 2"));
    assert!(page.body.contains(&format!("/en/account/passkeys/{first}/remove")));
    assert!(!page.body.contains("<p class=\"note\">This is your only way to sign in"));

    let removed = client.form(&format!("/en/account/passkeys/{first}/remove"), &[]).await;
    assert_eq!(removed.location(), "/en/account?msg=passkey-removed");
    assert_eq!(app.count("SELECT count(*) FROM passkeys").await, 1);

    // One left and no address: refused, and the page does not even offer the button.
    let refused = client.form(&format!("/en/account/passkeys/{second}/remove"), &[]).await;
    assert_eq!(refused.status, StatusCode::SEE_OTHER);
    assert_eq!(refused.location(), "/en/account?err=last-method");
    assert_eq!(app.count("SELECT count(*) FROM passkeys").await, 1);
    let page = client.get("/en/account?err=last-method").await;
    assert!(page.body.contains(
        "<p class=\"error\" role=\"status\">You cannot remove your last way to sign in."
    ));
    assert!(page.body.contains("<p class=\"note\">This is your only way to sign in"));
    assert!(!page.body.contains(&format!("/en/account/passkeys/{second}/remove")));

    // With a second method the first may go; the survivor is then protected in turn.
    let github = add_identity(&app, &user, "github", "99").await;
    let page = client.get("/en/account").await;
    assert!(page.body.contains(&format!("/en/account/identities/{github}/remove")));
    let removed = client.form(&format!("/en/account/passkeys/{second}/remove"), &[]).await;
    assert_eq!(removed.location(), "/en/account?msg=passkey-removed");
    let refused = client.form(&format!("/en/account/identities/{github}/remove"), &[]).await;
    assert_eq!(refused.location(), "/en/account?err=last-method");
    assert_eq!(app.count("SELECT count(*) FROM identities").await, 1);
}

#[tokio::test]
async fn an_email_address_counts_as_a_method() {
    let app = TestApp::new();
    let (mut client, user) = app.signed_in("both@example.com").await;
    let key = add_passkey(&app, &user, 1).await;
    let github = add_identity(&app, &user, "github", "7").await;
    // Address + passkey + GitHub: remove the last two, the link by email remains.
    assert_eq!(
        client.form(&format!("/en/account/passkeys/{key}/remove"), &[]).await.location(),
        "/en/account?msg=passkey-removed"
    );
    assert_eq!(
        client.form(&format!("/en/account/identities/{github}/remove"), &[]).await.location(),
        "/en/account?msg=identity-removed"
    );
    assert_eq!(
        app.count("SELECT count(*) FROM passkeys").await
            + app.count("SELECT count(*) FROM identities").await,
        0
    );

    // Someone else's method cannot be removed through your account.
    let (_, other) = app.signed_in("other@example.com").await;
    let theirs = add_passkey(&app, &other, 5).await;
    let reply = client.form(&format!("/en/account/passkeys/{theirs}/remove"), &[]).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert_eq!(app.count("SELECT count(*) FROM passkeys").await, 1);
}

#[tokio::test]
async fn profile_and_settings_are_saved() {
    let app = TestApp::new();
    let (mut client, _) = app.signed_in("me@example.com").await;
    let saved =
        client.form("/en/account/profile", &[("display_name", "  Baker \u{7} Bob  ")]).await;
    assert_eq!(saved.location(), "/en/account?msg=saved");
    let page = client.get("/en/account").await;
    assert!(
        page.body.contains("value=\"Baker  Bob\"") || page.body.contains("value=\"Baker Bob\""),
        "{}",
        page.body
    );
    assert!(page.body.contains(">Baker  Bob</a>") || page.body.contains(">Baker Bob</a>"));

    let long = "x".repeat(200);
    client.form("/en/account/profile", &[("display_name", &long)]).await;
    let name: String = app
        .state
        .db
        .read(|c| c.query_row("SELECT display_name FROM users", [], |r| r.get(0)))
        .await
        .unwrap();
    assert_eq!(name.chars().count(), 60);

    let saved = client.form("/en/account/settings", &[("locale", "fr"), ("units", "us")]).await;
    assert_eq!(saved.location(), "/fr/account?msg=saved");
    assert!(saved.cookie_line("lang").is_some_and(|c| c.starts_with("lang=fr")));
    let row: (String, String) = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT locale, units FROM users", [], |r| Ok((r.get(0)?, r.get(1)?)))
        })
        .await
        .unwrap();
    assert_eq!(row, ("fr".to_owned(), "us".to_owned()));
    let page = client.get("/fr/account?msg=saved").await;
    assert!(page.body.contains("Enregistré."));
    assert!(page.body.contains("<option value=\"us\" selected>"));

    // Unknown values leave the setting alone.
    client.form("/fr/account/settings", &[("locale", "xx"), ("units", "furlongs")]).await;
    let row: (String, String) = app
        .state
        .db
        .read(|c| {
            c.query_row("SELECT locale, units FROM users", [], |r| Ok((r.get(0)?, r.get(1)?)))
        })
        .await
        .unwrap();
    assert_eq!(row, ("fr".to_owned(), "us".to_owned()));
}

/// Rows in every table that belongs to a user (`email`: its address).
async fn rows_of(app: &TestApp, user_id: &str, email: &str) -> Vec<(&'static str, i64)> {
    let mut out = Vec::new();
    for table in [
        "identities",
        "passkeys",
        "sessions",
        "formulas",
        "starters",
        "bakes",
        "calibrations",
        "chat_conversations",
        "oauth_states",
    ] {
        let column = if table == "oauth_states" { "link_user_id" } else { "user_id" };
        let n =
            app.count(&format!("SELECT count(*) FROM {table} WHERE {column} = '{user_id}'")).await;
        out.push((table, n));
    }
    let messages = app
        .count(&format!(
            "SELECT count(*) FROM chat_messages m JOIN chat_conversations c ON c.id = m.conversation_id WHERE c.user_id = '{user_id}'"
        ))
        .await;
    out.push(("chat_messages", messages));
    let tokens = app
        .count(&format!("SELECT count(*) FROM email_tokens WHERE lower(email) = '{email}'"))
        .await;
    out.push(("email_tokens", tokens));
    out
}

/// The `chat_usage` key of an address.
fn usage_key(email: &str) -> String {
    format!("a:{}", crate::auth::sha256_hex(email))
}

async fn seed(app: &TestApp, client: &mut crate::testkit::Client, user_id: &str, tag: &str) {
    add_identity(app, user_id, "google", &format!("sub-{tag}")).await;
    add_passkey(app, user_id, 1).await;
    let uid = user_id.to_owned();
    let tag = tag.to_owned();
    app.state
        .db
        .write(move |c| {
            c.execute(
                "INSERT INTO oauth_states (state_hash, provider, code_verifier, nonce, lang, link_user_id, expires_at)
                 VALUES (?1, 'google', 'v', 'n', 'en', ?2, 9999999999)",
                rusqlite::params![format!("state-{tag}"), uid],
            )?;
            c.execute(
                "INSERT INTO chat_conversations (id, user_id, title, created_at, updated_at) VALUES (?1, ?2, 'Hello', 1, 1)",
                rusqlite::params![format!("conv-{tag}"), uid],
            )?;
            for i in 0..3 {
                c.execute(
                    "INSERT INTO chat_messages (conversation_id, role, content, created_at) VALUES (?1, 'user', 'hi', ?2)",
                    rusqlite::params![format!("conv-{tag}"), i],
                )?;
            }
            c.execute(
                "INSERT INTO chat_usage (subject, day, requests) VALUES (?1, '2026-11-24', 3)",
                [usage_key(&format!("{tag}@example.com"))],
            )?;
            // A magic link of the address, in another case than the account's.
            c.execute(
                "INSERT INTO email_tokens (token_hash, email, created_at, expires_at) VALUES (?1, ?2, 1, 9999999999)",
                rusqlite::params![format!("link-{tag}"), format!("{}@Example.com", tag.to_uppercase())],
            )?;
            Ok(())
        })
        .await
        .unwrap();
    let now = crate::db::now_ms();
    let body = json!({
        "formulas": [{ "id": uuid::Uuid::now_v7().to_string(), "name": "Loaf", "formula": { "stages": [] }, "updated_at": now }],
        "starters": [{ "id": uuid::Uuid::now_v7().to_string(), "name": "Rye", "flour": "rye", "hydration": 100, "updated_at": now }],
        "bakes": [{ "id": uuid::Uuid::now_v7().to_string(), "plan": {}, "actual": {}, "outcome": "good", "updated_at": now }],
        "calibrations": [{ "id": "instant_yeast", "mean": 0.1, "variance": 0.2, "n": 3, "updated_at": now }],
    });
    let reply = client.json(Method::POST, "/api/v1/sync", &body).await;
    assert_eq!(reply.json()["accepted"], 4, "{}", reply.body);
}

#[tokio::test]
async fn deleting_the_account_erases_everything_of_the_user_and_nothing_else() {
    let app = TestApp::new();
    let (mut me, my_id) = app.signed_in("me@example.com").await;
    let (mut you, your_id) = app.signed_in("you@example.com").await;
    seed(&app, &mut me, &my_id, "me").await;
    seed(&app, &mut you, &your_id, "you").await;
    let mine_before = rows_of(&app, &my_id, "me@example.com").await;
    let yours_before = rows_of(&app, &your_id, "you@example.com").await;
    assert!(mine_before.iter().all(|(_, n)| *n > 0), "every table is seeded: {mine_before:?}");

    // The confirmation must be typed.
    let refused = me.form("/en/account/delete", &[("confirm", "nope")]).await;
    assert_eq!(refused.location(), "/en/account?err=confirm");
    assert_eq!(rows_of(&app, &my_id, "me@example.com").await, mine_before);

    let deleted = me.form("/en/account/delete", &[("confirm", " Delete "), ("wipe", "1")]).await;
    assert_eq!(deleted.status, StatusCode::SEE_OTHER);
    assert_eq!(deleted.location(), "/en/account/deleted?wipe=1");
    assert!(deleted.cookie_line("__Host-session").is_some_and(|c| c.contains("Max-Age=0")));

    assert_eq!(app.count(&format!("SELECT count(*) FROM users WHERE id = '{my_id}'")).await, 0);
    let mine_after = rows_of(&app, &my_id, "me@example.com").await;
    assert!(mine_after.iter().all(|(_, n)| *n == 0), "{mine_after:?}");
    assert_eq!(
        rows_of(&app, &your_id, "you@example.com").await,
        yours_before,
        "other users are untouched"
    );
    assert_eq!(app.count(&format!("SELECT count(*) FROM users WHERE id = '{your_id}'")).await, 1);
    // The chat allowances outlive the account, under a hash of the address only.
    let kept = usage_key("me@example.com");
    assert_eq!(
        app.count(&format!("SELECT requests FROM chat_usage WHERE subject = '{kept}'")).await,
        3
    );
    assert_eq!(
        app.count(&format!(
            "SELECT count(*) FROM chat_usage WHERE subject LIKE '%{my_id}%' OR subject LIKE '%example%'"
        ))
        .await,
        0
    );
    // The session died with the account.
    let after = me.clone_session().get("/en/account").await;
    assert_eq!(after.status, StatusCode::SEE_OTHER);
    // The page after deletion is public and tells the script to clear this device too.
    let page = app.client().get("/en/account/deleted?wipe=1").await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(page.body.contains("Account deleted") && page.body.contains("data-wipe-local"));
    // French word works too.
    let (mut fr, fr_id) = app.signed_in("fr@example.com").await;
    assert_eq!(
        fr.form("/fr/account/delete", &[("confirm", "SUPPRIMER")]).await.location(),
        "/fr/account/deleted"
    );
    assert_eq!(app.count(&format!("SELECT count(*) FROM users WHERE id = '{fr_id}'")).await, 0);
}

#[tokio::test]
async fn export_returns_everything_stored_about_the_user() {
    let app = TestApp::new();
    let (mut me, my_id) = app.signed_in("me@example.com").await;
    let (mut you, your_id) = app.signed_in("you@example.com").await;
    seed(&app, &mut me, &my_id, "me").await;
    seed(&app, &mut you, &your_id, "you").await;

    let reply = me.get("/en/account/export").await;
    assert_eq!(reply.status, StatusCode::OK);
    assert!(
        reply.headers.get("content-disposition").unwrap().to_str().unwrap().contains("attachment")
    );
    assert_eq!(reply.headers.get("cache-control").unwrap(), "no-store");
    let doc = reply.json();
    assert_eq!(doc["account"]["email"], "me@example.com");
    assert_eq!(doc["account"]["id"], my_id);
    assert_eq!(doc["formulas"][0]["name"], "Loaf");
    assert_eq!(doc["starters"][0]["flour"], "rye");
    assert_eq!(doc["bakes"][0]["outcome"], "good");
    assert_eq!(doc["calibrations"][0]["id"], "instant_yeast");
    assert_eq!(doc["chat"]["conversations"][0]["messages"].as_array().unwrap().len(), 3);
    assert_eq!(
        doc["chat"]["usage"],
        json!([{ "day": "2026-11-24", "answers": 3, "tokens_in": 0, "tokens_out": 0, "cost_usd": 0.0 }])
    );
    assert_eq!(doc["sign_in_methods"]["providers"][0]["provider"], "google");
    // Signed-in devices, without any token.
    assert_eq!(doc["devices"].as_array().unwrap().len(), 1);
    assert_eq!(doc["devices"][0]["user_agent"], "TestAgent/1.0");
    let mut keys: Vec<&String> = doc["devices"][0].as_object().unwrap().keys().collect();
    keys.sort();
    assert_eq!(keys, ["last_seen_at", "signed_in_at", "user_agent"]);
    assert!(!reply.body.contains("sub-you") && !reply.body.contains("you@example.com"));
    // Credentials and tokens are not part of it.
    assert!(!reply.body.contains("token_hash") && !reply.body.contains("credential\""));
    let session_hashes: Vec<String> = app
        .state
        .db
        .read(|c| {
            let mut stmt = c.prepare("SELECT token_hash FROM sessions")?;
            stmt.query_map([], |r| r.get(0))?.collect()
        })
        .await
        .unwrap();
    assert!(session_hashes.iter().all(|h| !reply.body.contains(h.as_str())));
    assert!(!reply.body.contains("link-me"), "nor magic links");
}

#[tokio::test]
async fn ending_a_session_clears_the_offline_copies_of_pages() {
    let app = TestApp::new();
    let sw = app.client().get("/sw.js").await.body;
    // The service worker forgets its copies of pages (they show the account's name) on the
    // requests that end a session...
    let forget = sw.lines().find(|l| l.starts_with("const FORGET_PAGES")).expect("FORGET_PAGES");
    for route in ["sign-out|", "sign-out-all|", "delete|", "sessions\\/[^/]+\\/revoke"] {
        assert!(forget.contains(route), "{route} in {forget}");
    }
    assert!(sw.contains("FORGET_PAGES.test(url.pathname)"));
    assert!(sw.contains("event.waitUntil(caches.delete(PAGES_CACHE))"));
    // ...which are these routes, answered by the server.
    let (mut me, _) = app.signed_in("me@example.com").await;
    let session = app.count("SELECT id FROM sessions").await;
    let revoked = me.form(&format!("/en/account/sessions/{session}/revoke"), &[]).await;
    assert!(revoked.location().starts_with("/en/account/sign-in"), "{}", revoked.location());
    for (i, path) in ["/en/account/sign-out", "/en/account/sign-out-all", "/en/account/delete"]
        .into_iter()
        .enumerate()
    {
        let (mut you, _) = app.signed_in(&format!("user{i}@example.com")).await;
        let reply = you.form(path, &[("confirm", "delete")]).await;
        assert_eq!(reply.status, StatusCode::SEE_OTHER, "{path}");
    }
}

#[tokio::test]
async fn me_and_saved_formulas_pages() {
    let app = TestApp::new();
    let (mut me, my_id) = app.signed_in("me@example.com").await;
    let reply = me.get("/api/v1/me").await;
    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.json()["id"], my_id);
    assert_eq!(reply.json()["units"], "metric");

    // The saved-formulas page is public: its list is drawn on the device.
    let mut anonymous = app.client();
    let page = anonymous.get("/fr/my/formulas").await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(
        page.body.contains("data-island=\"my-formulas\"")
            && page.body.contains("nécessite JavaScript")
    );
    assert!(page.body.contains("name=\"robots\" content=\"noindex\""));
    assert!(page.body.contains("/fr/account/sign-in?next=/fr/my/formulas"));
    let page = me.get("/en/my/formulas").await;
    assert!(page.body.contains("Signed in: your saved formulas"));
}
