//! Formula calculator: baker's math with levain, poolish and scald.
//!
//! Without JavaScript the page is a GET form covering the common case (target, two flours, water,
//! salt, yeast, levain, poolish, scald); the server builds the engine formula from it. Any formula,
//! including ones the simple form cannot express (from the JavaScript editor or a recipe), travels
//! as `?s=` (see `state_codec`) and is rendered as is.

use askama::Template;
use axum::extract::{Query, State};
use axum::response::Response;
use breadator_engine::formula::{Formula, Solution, solve};
use breadator_engine::quick::QuickForm;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::{describe_error, describe_warning, entry_name};
use crate::app::{AppState, LangPath, Layout, render};
use crate::i18n::Tr;
use crate::state_codec::{self, number};

#[derive(Debug, Default, Deserialize)]
pub struct Params {
    s: Option<String>,
    by: Option<String>,
    grams: Option<String>,
    count: Option<String>,
    loss: Option<String>,
    flour1: Option<String>,
    flour2: Option<String>,
    flour2_pct: Option<String>,
    water: Option<String>,
    salt: Option<String>,
    yeast_kind: Option<String>,
    yeast: Option<String>,
    levain: Option<String>,
    levain_hyd: Option<String>,
    poolish: Option<String>,
    poolish_yeast: Option<String>,
    scald_method: Option<String>,
    scald: Option<String>,
}

/// Parses an enum from its serde code (`"instant_yeast"`, `"yudane"`…); unknown → `None`.
fn parse_code<T: DeserializeOwned>(v: Option<&str>) -> Option<T> {
    serde_json::from_value(serde_json::Value::String(v?.to_owned())).ok()
}

/// Serde code of an enum value.
fn code<T: Serialize>(v: &T) -> String {
    serde_json::to_value(v).ok().and_then(|v| v.as_str().map(str::to_owned)).unwrap_or_default()
}

/// The simple form's fields, defaults filled in for missing or unreadable values.
fn quick_form(p: &Params, tr: &Tr) -> QuickForm {
    let d = QuickForm::default();
    let n = |v: &Option<String>, default: f64| number(v.as_deref()).unwrap_or(default);
    let name = |v: &Option<String>| {
        v.as_deref().map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned)
    };
    QuickForm {
        by: parse_code(p.by.as_deref()).unwrap_or(d.by),
        grams: n(&p.grams, d.grams),
        count: n(&p.count, d.count),
        loss: n(&p.loss, d.loss),
        flour1: name(&p.flour1).unwrap_or_else(|| tr.t("default-flour1")),
        flour2: name(&p.flour2).unwrap_or_default(),
        flour2_pct: n(&p.flour2_pct, d.flour2_pct),
        water: n(&p.water, d.water),
        salt: n(&p.salt, d.salt),
        yeast_kind: parse_code(p.yeast_kind.as_deref()),
        yeast: n(&p.yeast, d.yeast),
        levain: n(&p.levain, d.levain),
        levain_hyd: n(&p.levain_hyd, d.levain_hyd),
        poolish: n(&p.poolish, d.poolish),
        poolish_yeast: n(&p.poolish_yeast, d.poolish_yeast),
        scald_method: parse_code(p.scald_method.as_deref()),
        scald: n(&p.scald, d.scald),
    }
}

pub struct Row {
    pub name: String,
    pub grams: String,
    pub pct: String,
}

pub struct StageView {
    pub title: String,
    pub rows: Vec<Row>,
    pub mass: String,
    pub hydration: String,
    pub notes: Vec<String>,
}

pub struct Metric {
    pub label: String,
    pub value: String,
}

pub struct Results {
    pub totals: Vec<Row>,
    pub stages: Vec<StageView>,
    pub final_mix: Vec<Row>,
    pub metrics: Vec<Metric>,
    pub warnings: Vec<String>,
}

pub(crate) fn results(tr: &Tr, s: &Solution) -> Results {
    let row = |id: &str, grams: f64, pct: f64| Row {
        name: entry_name(tr, id),
        grams: tr.g(grams),
        pct: tr.pct(pct),
    };
    let totals = s.totals.iter().map(|l| row(&l.id, l.grams, l.pct)).collect();
    let final_mix = s.final_mix.iter().map(|l| row(&l.id, l.grams, l.pct)).collect();
    let stages = s
        .stages
        .iter()
        .map(|st| {
            let mut notes = Vec::new();
            if let Some(seed) = &st.seed {
                notes.push(tr.t_args(
                    "formula-seed-build",
                    &[
                        ("seed", tr.g(seed.seed_g).into()),
                        ("flour", tr.g(seed.fresh_flour_g).into()),
                        ("water", tr.g(seed.fresh_water_g).into()),
                    ],
                ));
            }
            if st.evaporation_water_g > 0.0 {
                notes.push(tr.t_args(
                    "formula-evaporation",
                    &[
                        ("extra", tr.g(st.evaporation_water_g).into()),
                        ("mass", tr.g(st.mass_g).into()),
                    ],
                ));
            }
            StageView {
                title: entry_name(tr, &st.id),
                rows: st.lines.iter().map(|l| row(&l.id, l.grams, l.pct)).collect(),
                mass: tr.g(st.mass_g),
                hydration: tr.pct(st.hydration_pct),
                notes,
            }
        })
        .collect();
    let m = &s.metrics;
    let metric = |key: &str, value: String| Metric { label: tr.t(key), value };
    let mut metrics = vec![
        metric("metric-total-flour", format!("{} g", tr.g(m.total_flour_g))),
        metric("metric-dough", format!("{} g", tr.g(m.dough_g))),
        metric("metric-hydration-water", format!("{} %", tr.pct(m.hydration_water_pct))),
        metric("metric-hydration-total", format!("{} %", tr.pct(m.hydration_total_pct))),
        metric("metric-salt", format!("{} %", tr.pct(m.salt_pct))),
    ];
    if m.prefermented_flour_pct > 0.0 {
        metrics
            .push(metric("metric-prefermented", format!("{} %", tr.pct(m.prefermented_flour_pct))));
    }
    for lv in &m.levains {
        if let Some(i) = lv.inoculation_final_mix_pct {
            metrics.push(metric("metric-inoculation-final", format!("{} %", tr.pct(i))));
        }
        metrics.push(metric(
            "metric-inoculation-total",
            format!("{} %", tr.pct(lv.inoculation_total_flour_pct)),
        ));
    }
    if m.scalded_flour_pct > 0.0 {
        metrics.push(metric("metric-scalded", format!("{} %", tr.pct(m.scalded_flour_pct))));
    }
    if m.sugar_pct > 0.0 {
        metrics.push(metric("metric-sugar", format!("{} %", tr.pct(m.sugar_pct))));
    }
    if m.fat_pct > 0.0 {
        metrics.push(metric("metric-fat", format!("{} %", tr.pct(m.fat_pct))));
    }
    metrics.push(metric("metric-dough-yield", tr.num(m.dough_yield, 0)));
    Results {
        totals,
        stages,
        final_mix,
        metrics,
        warnings: s.warnings.iter().map(|w| describe_warning(tr, w)).collect(),
    }
}

#[derive(Template)]
#[template(path = "formula.html")]
struct Page {
    layout: Layout,
    form: QuickForm,
    /// The formula came from `?s=` and may not fit the simple form.
    advanced: bool,
    results: Option<Results>,
    error: Option<String>,
    /// Formula as JSON for the editor island (`</` escaped for the script element).
    state_json: String,
    share: String,
}

impl Page {
    fn sel(&self, field: &str, value: &str) -> &'static str {
        let none = || "none".to_owned();
        let current = match field {
            "by" => code(&self.form.by),
            "yeast_kind" => self.form.yeast_kind.as_ref().map_or_else(none, code),
            "scald_method" => self.form.scald_method.as_ref().map_or_else(none, code),
            _ => return "",
        };
        if current == value { "selected" } else { "" }
    }

    fn v(&self, x: &f64) -> String {
        // Form values use a dot: <input type=number> expects it whatever the language.
        let s = format!("{x}");
        s.strip_suffix(".0").map(str::to_owned).unwrap_or(s)
    }
}

pub async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(params): Query<Params>,
) -> Response {
    let tr = lang.tr();
    let form = quick_form(&params, &tr);
    let from_state = params.s.as_deref().and_then(state_codec::decode::<Formula>);
    let advanced = from_state.is_some();
    let formula = match from_state {
        Some(f) => Ok(f),
        None => form.to_formula(),
    };
    let (results, error, state_json, share) = match &formula {
        Ok(f) => {
            // `<` escaped as \u003c: nothing in the JSON can close or comment the script element.
            let json = serde_json::to_string(f).unwrap_or_default().replace('<', "\\u003c");
            let share = format!(
                "{}/{}/calculators/formula?s={}",
                state.public_url,
                lang.code(),
                state_codec::encode(f)
            );
            match solve(f) {
                Ok(s) => (Some(results(&tr, &s)), None, json, share),
                Err(e) => (None, Some(describe_error(&tr, &e)), json, share),
            }
        }
        Err(e) => (None, Some(describe_error(&tr, e)), "null".to_owned(), String::new()),
    };
    let layout = Layout::new(
        &state,
        lang,
        "/calculators/formula",
        "formula",
        tr.t("formula-title"),
        tr.t("formula-description"),
    );
    render(&Page { layout, form, advanced, results, error, state_json, share })
}
