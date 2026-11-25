use askama::Template;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{IntoResponse, Redirect, Response};

use crate::app::{AppState, LangPath, Layout, not_found_page, render};
use crate::auth::Units;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    layout: Layout,
}

pub async fn page(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    let layout =
        Layout::new(&state, lang, "", "home", tr.t("home-title"), tr.t("home-description"));
    render(&Home { layout })
}

#[derive(Template)]
#[template(path = "calculators.html")]
struct Calculators {
    layout: Layout,
}

/// The list of calculators (the header's "Calculators" link).
pub async fn calculators(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    let layout = Layout::new(
        &state,
        lang,
        "/calculators",
        "calculators",
        tr.t("calculators-title"),
        tr.t("calculators-description"),
    );
    render(&Calculators { layout })
}

/// The cookie that makes this device show results in `units` (a year, like the language's).
pub fn units_cookie(units: Units, secure: bool) -> HeaderValue {
    let secure = if secure { "; Secure" } else { "" };
    let cookie =
        format!("units={}; Max-Age=31536000; Path=/; SameSite=Lax{secure}", units.as_str());
    // The value is one of two fixed words: always a valid header.
    HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static(""))
}

/// `GET /{lang}/units/{metric|us}`: the footer's switch. Sets the cookie and goes back to the
/// page the link was on, with its query (a calculator keeps what was typed).
pub async fn units(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, units)): Path<(String, String)>,
    headers: HeaderMap,
) -> Response {
    let Some(units) = Units::parse(&units) else {
        return not_found_page(lang);
    };
    // Back to the referring page when it is one of ours; anything else goes home.
    let back = headers
        .get(header::REFERER)
        .and_then(|v| v.to_str().ok())
        .and_then(|url| url.strip_prefix(state.public_url.as_str()))
        .filter(|path| {
            path.starts_with('/')
                && !path.starts_with("//")
                && !path.contains("/units/")
                && path.len() <= 4000
                && !path.chars().any(|c| c.is_control() || c == '\\')
        })
        .map_or_else(|| format!("/{}", lang.code()), str::to_owned);
    let mut response = Redirect::to(&back).into_response();
    response
        .headers_mut()
        .append(header::SET_COOKIE, units_cookie(units, state.auth.secure_cookies));
    response
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::testkit::{HTTPS_URL, TestApp};

    const YEAST: &str = "/en/calculators/yeast?grams=100&from=fresh_yeast";

    #[tokio::test]
    async fn the_footer_switch_shows_ounces_and_fahrenheit_on_this_device() {
        let app = TestApp::new();
        let mut device = app.client();
        let metric = device.get(YEAST).await;
        assert!(metric.body.contains("33 g") && !metric.body.contains(" oz ·"), "{}", metric.body);
        assert!(!metric.body.contains("data-units"));

        // The switch sends back to the page it was on, with what was typed there.
        device.extra.push(("referer", format!("{HTTPS_URL}{YEAST}")));
        let switched = device.get("/en/units/us").await;
        assert_eq!(switched.status, StatusCode::SEE_OTHER);
        assert_eq!(switched.location(), YEAST);
        let us = device.get(YEAST).await;
        assert!(us.body.contains("1.16 oz · 33 g"), "{}", us.body);
        assert!(us.body.contains("0.88 oz – 1.48 oz · 25 – 42 g"), "{}", us.body);
        assert!(us.body.contains(r#"data-units="us""#), "the islands format the same way");
        let water = device.get("/en/calculators/temperature").await;
        assert!(water.body.contains(" °F · "), "{}", water.body);
        // Another device is untouched; a foreign referrer or an unknown unit leads nowhere odd.
        assert!(!app.client().get(YEAST).await.body.contains(" oz ·"));
        device.extra = vec![("referer", "https://evil.example/x".into())];
        assert_eq!(device.get("/en/units/metric").await.location(), "/en");
        assert!(!device.get(YEAST).await.body.contains(" oz ·"));
        assert_eq!(device.get("/en/units/stone").await.status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn an_account_set_to_us_units_shows_them_until_the_device_says_otherwise() {
        let app = TestApp::new();
        let (mut client, _) = app.signed_in("us@example.com").await;
        let saved = client.form("/en/account/settings", &[("locale", "en"), ("units", "us")]).await;
        assert_eq!(saved.status, StatusCode::SEE_OTHER);
        assert!(client.get(YEAST).await.body.contains("1.16 oz · 33 g"));
        // The same account on a device that never chose (no `units` cookie): the account's.
        let mut elsewhere = client.clone_session();
        elsewhere.set_cookie("units", "");
        assert!(elsewhere.get(YEAST).await.body.contains("1.16 oz · 33 g"));
        // A device that chose metric keeps metric.
        elsewhere.get("/en/units/metric").await;
        assert!(!elsewhere.get(YEAST).await.body.contains(" oz ·"));
    }
}
