//! Private form submission for planning from an already-made sourdough preferment.

use std::borrow::Cow;
use std::collections::HashMap;

use askama::Template;
use axum::extract::{Form, State};
use axum::http::{HeaderValue, header};
use axum::response::Response;
use breadator_engine::ferment::Band;
use breadator_engine::leaven::{Endpoint, StarterState};
use breadator_engine::preferment::{
    MAX_OBSERVATIONS, Mix, Observation, PrefermentPlan, QuickPreferment, TargetPlan,
};

use super::describe_error;
use super::fermentation::duration;
use super::formula::Row;
use crate::app::{AppState, LangPath, Layout, render};
use crate::i18n::{Lang, Tr};
use crate::state_codec::number;

const PATH: &str = "/calculators/preferment";
const ENDPOINTS: [(Endpoint, &str, &str); 4] = [
    (Endpoint::Rise25, "rise25", "+25 %"),
    (Endpoint::Rise50, "rise50", "+50 %"),
    (Endpoint::Rise75, "rise75", "+75 %"),
    (Endpoint::Doubled, "doubled", "+100 %"),
];
type Fields = HashMap<String, String>;

fn obs_name(index: usize, field: &str) -> String {
    format!("obs{index}_{field}")
}

fn endpoint(value: Option<&str>) -> Result<Endpoint, &'static str> {
    ENDPOINTS
        .iter()
        .find(|(_, code, _)| *code == value.unwrap_or("rise50"))
        .map(|(endpoint, _, _)| *endpoint)
        .ok_or("preferment-endpoint")
}

fn parse(fields: &Fields, defaults: &QuickPreferment) -> Result<QuickPreferment, &'static str> {
    let raw = |key: &str| fields.get(key).map(String::as_str);
    let n = |key, label, default| match raw(key) {
        None => Ok(default),
        value => number(value).ok_or(label),
    };
    let seed_state = match raw("seed_state").unwrap_or("ripe") {
        "ripe" => StarterState::Ripe,
        "from_fridge" => StarterState::FromFridge,
        _ => return Err("preferment-seed-state"),
    };
    let manual_vigour = raw("manual_vigour")
        .filter(|text| !text.trim().is_empty())
        .map(|text| number(Some(text)).ok_or("preferment-vigour"))
        .transpose()?;
    let mut observations = Vec::new();
    for index in 1..=MAX_OBSERVATIONS {
        let value = |name| raw(&obs_name(index, name));
        let kind = value("kind").unwrap_or("");
        if kind.is_empty() {
            continue;
        }
        let obs = |name, label| number(value(name)).ok_or(label);
        let observed_h = obs("hours", "preferment-observed-hours")?;
        let temp_c = obs("temp", "preferment-observed-temperature")?;
        let starter_hyd = obs("starter_hyd", "preferment-observed-starter-hydration")?;
        observations.push(match kind {
            "peak" => Observation::Peak {
                observed_h,
                temp_c,
                seed_hydration_pct: starter_hyd,
                flour_parts: obs("flour", "starter-flour-parts")?,
                water_parts: obs("water", "starter-water-parts")?,
            },
            "bulk" => Observation::Bulk {
                observed_h,
                temp_c,
                starter_hydration_pct: starter_hyd,
                dose_pct: obs("dose", "preferment-observed-dose")?,
                hydration_pct: obs("hydration", "ferment-hydration")?,
                salt_pct: obs("salt", "formula-salt")?,
                whole_grain_pct: obs("whole_grain", "ferment-whole-grain")?,
                endpoint: endpoint(value("endpoint"))?,
            },
            _ => return Err("preferment-observed-kind"),
        });
    }
    Ok(QuickPreferment {
        seed_g: n("seed_g", "preferment-seed-g", defaults.seed_g)?,
        seed_hydration_pct: n(
            "seed_hydration_pct",
            "starter-seed-hyd",
            defaults.seed_hydration_pct,
        )?,
        seed_state,
        feed_flour_g: n("feed_flour_g", "preferment-flour-g", defaults.feed_flour_g)?,
        feed_water_g: n("feed_water_g", "preferment-water-g", defaults.feed_water_g)?,
        feed_c: n("feed_c", "preferment-feed-temp", defaults.feed_c)?,
        age_h: n("age_h", "preferment-age", defaults.age_h)?,
        baked_g: n("baked_g", "preferment-baked-g", defaults.baked_g)?,
        loss_pct: n("loss_pct", "formula-loss", defaults.loss_pct)?,
        hydration_pct: n("hydration_pct", "ferment-hydration", defaults.hydration_pct)?,
        salt_pct: n("salt_pct", "formula-salt", defaults.salt_pct)?,
        whole_grain_pct: n("whole_grain_pct", "ferment-whole-grain", defaults.whole_grain_pct)?,
        bulk_c: n("bulk_c", "preferment-bulk-temp", defaults.bulk_c)?,
        bulk_h: n("bulk_h", "preferment-bulk-hours", defaults.bulk_h)?,
        endpoint: endpoint(raw("endpoint"))?,
        extra_hydration_pct: n(
            "extra_hydration_pct",
            "preferment-extra-hyd",
            defaults.extra_hydration_pct,
        )?,
        manual_vigour,
        observations,
    })
}

struct MixView {
    rows: Vec<Row>,
    yield_text: String,
}

fn mix_view(tr: &Tr, mix: &Mix) -> MixView {
    let row = |label, grams| Row {
        name: tr.t(label),
        grams: tr.g(grams),
        pct: tr.pct(100.0 * grams / mix.total_flour_g),
    };
    let mut rows = Vec::with_capacity(4 + usize::from(mix.additional_levain_g > 1e-6));
    rows.push(row("preferment-mix-preferment", mix.preferment_g));
    if mix.additional_levain_g > 1e-6 {
        rows.push(row("preferment-mix-extra", mix.additional_levain_g));
    }
    rows.extend([
        row("entry-flour", mix.flour_g),
        row("entry-water", mix.water_g),
        row("entry-salt", mix.salt_g),
    ]);
    MixView {
        rows,
        yield_text: tr.t_args(
            "preferment-yield",
            &[("dough", tr.g(mix.dough_g).into()), ("baked", tr.g(mix.baked_g).into())],
        ),
    }
}

struct Primary {
    mix: MixView,
    time: String,
    band: String,
    leftover: Option<String>,
    no_extra: bool,
    water_limit: bool,
    extrapolated: bool,
}

struct AlternativeView {
    same_weight: Option<MixView>,
    same_weight_pct: String,
    same_weight_timing: Option<(String, String)>,
    same_time: Option<MixView>,
}

struct ResultView {
    available: String,
    age: String,
    peak: String,
    peak_band: String,
    peak_extrapolated: bool,
    calibration: String,
    primary: Option<Primary>,
    issue: Option<String>,
    alternatives: Option<AlternativeView>,
}

fn band(tr: &Tr, hours: Band) -> String {
    tr.t_args(
        "preferment-band",
        &[("low", duration(tr, hours.low_h).into()), ("high", duration(tr, hours.high_h).into())],
    )
}

fn view(tr: &Tr, input: &QuickPreferment, plan: PrefermentPlan) -> ResultView {
    let (primary, issue) = match plan.target {
        TargetPlan::Mix { mix, hours, extrapolated } => {
            let leftover_g = plan.available_g - mix.preferment_g;
            let leftover = (leftover_g > 1e-6)
                .then(|| tr.t_args("preferment-leftover", &[("g", tr.g(leftover_g).into())]));
            let no_extra = mix.additional_levain_g <= 1e-6;
            let water_limit = leftover.is_some() && !no_extra;
            (
                Some(Primary {
                    mix: mix_view(tr, &mix),
                    time: tr
                        .t_args("ferment-time-result", &[("t", duration(tr, hours.mid_h).into())]),
                    band: band(tr, hours),
                    leftover,
                    no_extra,
                    water_limit,
                    extrapolated,
                }),
                None,
            )
        }
        TargetPlan::WaterConflict => (None, Some(tr.t("preferment-water-conflict"))),
        TargetPlan::NoDose { too_slow } => (
            None,
            Some(tr.t(if too_slow {
                "preferment-time-too-short"
            } else {
                "preferment-time-too-long"
            })),
        ),
    };
    let calibration = match plan.vigour {
        None => tr.t("preferment-reference"),
        Some(v) if v.n == 0 => {
            tr.t_args("preferment-manual", &[("v", tr.num(v.factor(), 2).into())])
        }
        Some(v) => tr.t_args(
            "preferment-calibrated",
            &[("v", tr.num(v.factor(), 2).into()), ("n", f64::from(v.n).into())],
        ),
    };
    ResultView {
        available: tr.t_args(
            "preferment-available",
            &[
                ("g", tr.g(plan.available_g).into()),
                ("hyd", tr.pct(plan.preferment_hydration_pct).into()),
            ],
        ),
        age: tr.t_args("preferment-age-result", &[("age", duration(tr, input.age_h).into())]),
        peak: tr.t_args("starter-peak-result", &[("t", duration(tr, plan.peak.mid_h).into())]),
        peak_band: band(tr, plan.peak),
        peak_extrapolated: plan.peak_extrapolated,
        calibration,
        primary,
        issue,
        alternatives: plan.alternatives.map(|a| AlternativeView {
            same_weight: a.same_weight.as_ref().map(|m| mix_view(tr, m)),
            same_weight_pct: tr
                .t_args("preferment-all-pff", &[("pct", tr.pct(a.same_weight_pct).into())]),
            same_weight_timing: a.same_weight_hours.map(|hours| {
                let mid = duration(tr, hours.mid_h);
                (tr.t_args("ferment-time-result", &[("t", mid.into())]), band(tr, hours))
            }),
            same_time: a.same_time.as_ref().map(|m| mix_view(tr, m)),
        }),
    }
}

#[derive(Template)]
#[template(path = "preferment.html")]
struct Page {
    layout: Layout,
    defaults: QuickPreferment,
    values: Fields,
    result: Option<ResultView>,
    error: Option<String>,
    submitted: bool,
}

impl Page {
    fn raw(&self, key: &str) -> &str {
        self.values.get(key).map_or("", String::as_str)
    }

    fn selected(&self, key: &str, code: &str, default: &str) -> bool {
        self.values.get(key).map_or(default, String::as_str) == code
    }

    fn value(&self, key: &str, default: &f64) -> Cow<'_, str> {
        self.values
            .get(key)
            .map_or_else(|| Cow::Owned(default.to_string()), |v| Cow::Borrowed(v.as_str()))
    }

    fn endpoints(&self) -> &'static [(Endpoint, &'static str, &'static str)] {
        &ENDPOINTS
    }

    fn slots(&self) -> std::ops::RangeInclusive<usize> {
        1..=MAX_OBSERVATIONS
    }

    fn obs_name(&self, index: &usize, field: &str) -> String {
        obs_name(*index, field)
    }

    fn obs_label(&self, index: &usize) -> String {
        self.layout.tr.t_args("preferment-observation", &[("n", (*index as f64).into())])
    }

    fn has_calibration_input(&self) -> bool {
        !self.raw("manual_vigour").is_empty()
            || self.slots().any(|i| !self.raw(&obs_name(i, "kind")).is_empty())
    }
}

fn show(state: &AppState, lang: Lang, values: Fields, submitted: bool) -> Response {
    let tr = lang.tr();
    let defaults = QuickPreferment::default();
    let mut layout = Layout::new(
        state,
        lang,
        PATH,
        "preferment",
        tr.t("preferment-title"),
        tr.t("preferment-description"),
    );
    let (result, error) = if submitted {
        layout = layout.private();
        match parse(&values, &defaults) {
            Ok(input) => match input.solve() {
                Ok(plan) => (Some(view(&tr, &input, plan)), None),
                Err(e) => (None, Some(describe_error(&tr, &e))),
            },
            Err(field) => (
                None,
                Some(tr.t_args("preferment-invalid-field", &[("field", tr.t(field).into())])),
            ),
        }
    } else {
        (None, None)
    };
    let mut response = render(&Page { layout, defaults, values, result, error, submitted });
    if submitted {
        response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    }
    response
}

pub async fn page(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    show(&state, lang, Fields::new(), false)
}

pub async fn calculate(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Form(values): Form<Fields>,
) -> Response {
    show(&state, lang, values, true)
}
