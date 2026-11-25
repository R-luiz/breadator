//! Languages, translations (Fluent catalogs in `locales/`) and number formatting.

use std::collections::HashMap;
use std::sync::LazyLock;

use axum::http::HeaderMap;
use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{Loader, static_loader};
use unic_langid::{LanguageIdentifier, langid};

use crate::auth::{Units, current_units};

const GRAMS_PER_OUNCE: f64 = 28.349_523_125;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
        // No Unicode isolation marks around placeables: they would end up inside numbers.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

static FR: LanguageIdentifier = langid!("fr");
static EN: LanguageIdentifier = langid!("en");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Fr,
    En,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::Fr, Lang::En];

    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "fr" => Some(Self::Fr),
            "en" => Some(Self::En),
            _ => None,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Fr => "fr",
            Self::En => "en",
        }
    }

    fn id(self) -> &'static LanguageIdentifier {
        match self {
            Self::Fr => &FR,
            Self::En => &EN,
        }
    }

    /// Language for `/`: explicit cookie choice first, then the browser's preference order.
    pub fn negotiate(headers: &HeaderMap) -> Self {
        let cookie = headers
            .get_all(axum::http::header::COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .flat_map(|v| v.split(';'))
            .find_map(|kv| kv.trim().strip_prefix("lang=").and_then(Self::parse));
        if let Some(lang) = cookie {
            return lang;
        }
        let accept = headers
            .get(axum::http::header::ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        let mut best: Option<(f32, Self)> = None;
        for part in accept.split(',') {
            let mut it = part.trim().split(';');
            let tag = it.next().unwrap_or("").trim().to_ascii_lowercase();
            let q = it
                .find_map(|p| p.trim().strip_prefix("q=").and_then(|q| q.parse::<f32>().ok()))
                .unwrap_or(1.0);
            let primary = tag.split('-').next().unwrap_or("");
            if let Some(lang) = Self::parse(primary)
                && best.is_none_or(|(bq, _)| q > bq)
            {
                best = Some((q, lang));
            }
        }
        best.map_or(Self::En, |(_, l)| l)
    }

    pub fn tr(self) -> Tr {
        Tr(self)
    }

    /// The whole catalog (every `.ftl` file of the language) as a JSON object `{key: message}`
    /// for the browser islands, which fill `{ $name }` placeholders themselves. Served as an
    /// asset (`assets.rs`).
    pub fn catalog_json(self) -> &'static str {
        static FR_JSON: LazyLock<String> = LazyLock::new(|| catalog_to_json("fr"));
        static EN_JSON: LazyLock<String> = LazyLock::new(|| catalog_to_json("en"));
        match self {
            Self::Fr => &FR_JSON,
            Self::En => &EN_JSON,
        }
    }
}

static LOCALE_FILES: include_dir::Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/locales");

/// `key = message` lines of `locales/<lang>/*.ftl` to JSON (catalog messages are single-line by
/// convention).
fn catalog_to_json(lang: &str) -> String {
    let mut map = std::collections::BTreeMap::new();
    for file in LOCALE_FILES.get_dir(lang).into_iter().flat_map(|d| d.files()) {
        let Some(source) = file.contents_utf8() else { continue };
        for (key, message) in source.lines().filter_map(|l| l.split_once(" = ")) {
            if !key.starts_with(['#', ' ']) {
                map.insert(key, message);
            }
        }
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

/// `v` with `decimals` places, rounded half away from zero on the number as it is written
/// (`0.975` → `0.98`, `22.5` → `23`). That is how the browser's `Intl.NumberFormat` rounds,
/// so a page and its island show the same figure; `{:.2}` rounds the binary value instead
/// (`0.97`) and ties to even (`22`). A value that rounds to zero is `0`, never `-0`.
fn round_half_up(v: f64, decimals: usize) -> String {
    if !v.is_finite() {
        return v.to_string();
    }
    // The shortest decimal that reads back as `v`; Rust writes it without an exponent.
    let written = v.abs().to_string();
    let (whole, fraction) = written.split_once('.').unwrap_or((&written, ""));
    let mut digits: Vec<u8> = whole.bytes().chain(fraction.bytes()).map(|b| b - b'0').collect();
    let mut whole_len = whole.len();
    let keep = whole_len + decimals;
    let up = digits.get(keep).is_some_and(|d| *d >= 5);
    digits.resize(keep, 0);
    if up {
        let mut i = keep;
        loop {
            if i == 0 {
                digits.insert(0, 1);
                whole_len += 1;
                break;
            }
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    let mut out = String::with_capacity(digits.len() + 2);
    if v < 0.0 && digits.iter().any(|d| *d != 0) {
        out.push('-');
    }
    for (i, d) in digits.iter().enumerate() {
        if i == whole_len {
            out.push('.');
        }
        out.push(char::from(b'0' + d));
    }
    out
}

/// Translation and formatting helpers handed to templates.
#[derive(Debug, Clone, Copy)]
pub struct Tr(pub Lang);

impl Tr {
    pub fn lang(&self) -> &'static str {
        self.0.code()
    }

    pub fn t(&self, key: &str) -> String {
        LOCALES.lookup(self.0.id(), key)
    }

    /// Translation of `key`, or `None` when no catalog defines it.
    pub fn try_t(&self, key: &str) -> Option<String> {
        LOCALES.try_lookup(self.0.id(), key)
    }

    pub fn t_args(&self, key: &str, args: &[(&'static str, FluentValue<'static>)]) -> String {
        let map: HashMap<std::borrow::Cow<'static, str>, FluentValue<'static>> =
            args.iter().map(|(k, v)| (std::borrow::Cow::Borrowed(*k), v.clone())).collect();
        LOCALES.lookup_with_args(self.0.id(), key, &map)
    }

    /// Number with `decimals` places and the language's decimal separator.
    pub fn num(&self, v: f64, decimals: usize) -> String {
        let s = round_half_up(v, decimals);
        match self.0 {
            Lang::Fr => s.replace('.', ","),
            Lang::En => s,
        }
    }

    /// Grams as a baker weighs them: 0.01 g under 1 g, 0.1 g under 20 g, whole grams above.
    /// The caller writes the unit after it. With US units the ounces come first:
    /// `2 lb 0.5 oz · 920`, which reads `2 lb 0.5 oz · 920 g` once the unit is there.
    pub fn g(&self, grams: f64) -> String {
        let metric = self.grams(grams);
        match current_units() {
            Units::Us if grams >= 0.0 => format!("{} · {metric}", self.ounces(grams)),
            _ => metric,
        }
    }

    /// A range of weights, the unit written after it by the caller like [`Tr::g`]: `25 – 42`,
    /// or with US units `0.88 oz – 1.48 oz · 25 – 42`.
    pub fn g_range(&self, low: f64, high: f64) -> String {
        let metric = format!("{} – {}", self.grams(low), self.grams(high));
        match current_units() {
            Units::Us if low >= 0.0 => {
                format!("{} – {} · {metric}", self.ounces(low), self.ounces(high))
            }
            _ => metric,
        }
    }

    fn grams(&self, grams: f64) -> String {
        let a = grams.abs();
        let decimals = if a < 1.0 {
            2
        } else if a < 20.0 {
            1
        } else {
            0
        };
        self.num(grams, decimals)
    }

    /// `0.018 oz`, `3.53 oz`, `14.1 oz`, `2 lb 0.5 oz`: pounds from 16 oz.
    fn ounces(&self, grams: f64) -> String {
        let oz = grams / GRAMS_PER_OUNCE;
        if oz < 16.0 {
            let decimals = if oz < 0.1 {
                3
            } else if oz < 10.0 {
                2
            } else {
                1
            };
            return format!("{} oz", self.num(oz, decimals));
        }
        let mut pounds = (oz / 16.0).floor();
        let mut rest = ((oz - 16.0 * pounds) * 10.0).round() / 10.0;
        if rest >= 16.0 {
            pounds += 1.0;
            rest = 0.0;
        }
        if rest == 0.0 {
            format!("{} lb", self.num(pounds, 0))
        } else {
            format!("{} lb {} oz", self.num(pounds, 0), self.num(rest, 1))
        }
    }

    /// A temperature in °C with one decimal; the caller writes `°C` after it. With US units the
    /// whole degrees Fahrenheit come first: `76 °F · 24.5`.
    pub fn temp(&self, celsius: f64) -> String {
        let metric = self.num(celsius, 1);
        match current_units() {
            Units::Us => format!("{} °F · {metric}", self.num(celsius * 9.0 / 5.0 + 32.0, 0)),
            Units::Metric => metric,
        }
    }

    /// Baker's percentage: one decimal, two under 1 %.
    pub fn pct(&self, pct: f64) -> String {
        self.num(pct, if pct.abs() < 1.0 { 2 } else { 1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiation_prefers_cookie_then_quality() {
        let mut h = HeaderMap::new();
        h.insert("accept-language", "en-GB,en;q=0.9,fr;q=0.8".parse().unwrap());
        assert_eq!(Lang::negotiate(&h), Lang::En);
        h.insert("accept-language", "de-DE,fr-FR;q=0.9,en;q=0.5".parse().unwrap());
        assert_eq!(Lang::negotiate(&h), Lang::Fr);
        h.insert("cookie", "a=b; lang=en".parse().unwrap());
        assert_eq!(Lang::negotiate(&h), Lang::En);
        assert_eq!(Lang::negotiate(&HeaderMap::new()), Lang::En);
    }

    #[test]
    fn numbers_follow_the_language() {
        assert_eq!(Lang::Fr.tr().g(1234.4), "1234");
        assert_eq!(Lang::Fr.tr().g(3.25), "3,3");
        assert_eq!(Lang::En.tr().g(0.456), "0.46");
        assert_eq!(Lang::En.tr().num(-0.001, 1), "0.0");
    }

    /// The island formats with `Intl.NumberFormat`, which rounds halves up on the number as
    /// written: the page must show the same figure (the fermentation page's default hydration
    /// factor, 0.975, read ×0,97 on the page and ×0,98 once the island had recomputed).
    #[test]
    fn numbers_round_like_the_browser() {
        let en = Lang::En.tr();
        assert_eq!(en.num(0.975, 2), "0.98");
        assert_eq!(en.num(22.5, 0), "23");
        assert_eq!(en.num(1.005, 2), "1.01");
        assert_eq!(en.num(0.125, 2), "0.13");
        assert_eq!(en.num(-2.5, 0), "-3");
        assert_eq!(en.num(999.96, 1), "1000.0");
        assert_eq!(en.num(9.5, 0), "10");
        assert_eq!(en.num(0.4, 0), "0");
        assert_eq!(en.num(-0.4, 0), "0");
        assert_eq!(en.num(7.0, 2), "7.00");
        assert_eq!(en.num(1234.0, 0), "1234");
        assert_eq!(en.num(0.000_4, 3), "0.000");
        assert_eq!(en.num(1.0e21, 0), "1000000000000000000000");
        assert!(!Lang::Fr.tr().t("site-name").is_empty());
    }

    #[test]
    fn us_units_put_ounces_and_fahrenheit_before_the_metric_value() {
        use crate::auth::with_units;
        let (en, fr) = (Lang::En.tr(), Lang::Fr.tr());
        with_units(Units::Us, || {
            // The caller appends the unit: "… · 920 g", "… · 24.5 °C".
            assert_eq!(en.g(920.0), "2 lb 0.5 oz · 920");
            assert_eq!(en.g(907.184_74), "2 lb · 907");
            assert_eq!(en.g(100.0), "3.53 oz · 100");
            assert_eq!(en.g(400.0), "14.1 oz · 400");
            assert_eq!(en.g(0.5), "0.018 oz · 0.50");
            assert_eq!(fr.g(3.5), "0,12 oz · 3,5");
            // 453.5 g is 15.997 oz: not "0 lb 16.0 oz", and 31.99 oz is 2 lb, not "1 lb 16.0 oz".
            assert_eq!(en.g(453.5), "16.0 oz · 454");
            assert_eq!(en.g(907.0), "2 lb · 907");
            assert_eq!(en.temp(24.5), "76 °F · 24.5");
            assert_eq!(fr.temp(-2.0), "28 °F · -2,0");
            // A range keeps one unit pair: the caller's " g" closes the metric half.
            assert_eq!(en.g_range(25.0, 42.0), "0.88 oz – 1.48 oz · 25 – 42");
            assert_eq!(en.g_range(648.0, 812.0), "1 lb 6.9 oz – 1 lb 12.6 oz · 648 – 812");
        });
        // Outside a request, and for metric devices: grams and °C alone.
        assert_eq!(en.g(920.0), "920");
        assert_eq!(with_units(Units::Metric, || en.temp(24.5)), "24.5");
    }

    #[test]
    fn catalogs_parse_and_define_the_same_keys() {
        use fluent_templates::fluent_bundle::FluentResource;
        for dir in LOCALE_FILES.dirs() {
            for file in dir.files() {
                let src = file.contents_utf8().expect("catalogs are UTF-8");
                if let Err((_, errors)) = FluentResource::try_new(src.to_owned()) {
                    panic!("{} does not parse: {errors:?}", file.path().display());
                }
            }
        }
        // The islands read the catalogs through `catalog_json`: same keys in both languages.
        let keys = |lang: Lang| -> Vec<String> {
            let json: serde_json::Value = serde_json::from_str(lang.catalog_json()).unwrap();
            json.as_object().unwrap().keys().cloned().collect()
        };
        assert_eq!(keys(Lang::En), keys(Lang::Fr));
        let msg = Lang::Fr.tr().t_args("err-duplicate", &[("id", "eau".into())]);
        assert_eq!(msg, "Deux éléments s'appellent « eau » : donnez-leur des noms différents.");
    }
}
