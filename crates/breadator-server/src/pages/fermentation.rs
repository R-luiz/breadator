//! Fermentation calculator: time to an endpoint for a dose, or the dose for a time, across a
//! room / fridge / room schedule, with an 80 % range and the evidence behind each factor.

use askama::Template;
use axum::extract::{Query, State};
use axum::response::Response;
use breadator_engine::leaven::{
    DoughFactors, Endpoint, EndpointPhase, Evidence, FermentNote, FermentResult, Leavening,
    QuickFerment, Solve,
};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use super::describe_error;
use crate::app::{AppState, LangPath, Layout, render};
use crate::i18n::Tr;
use crate::state_codec::number;

#[derive(Debug, Default, Deserialize)]
pub struct Params {
    solve: Option<String>,
    leavening: Option<String>,
    dose: Option<String>,
    starter_hyd: Option<String>,
    starter_state: Option<String>,
    target_h: Option<String>,
    endpoint: Option<String>,
    dough_c: Option<String>,
    room_c: Option<String>,
    dough_g: Option<String>,
    container: Option<String>,
    warm_h: Option<String>,
    fridge_h: Option<String>,
    fridge_c: Option<String>,
    salt: Option<String>,
    sugar: Option<String>,
    fat: Option<String>,
    hydration: Option<String>,
    whole_grain: Option<String>,
    spice: Option<String>,
    cocoa: Option<String>,
    scalded: Option<String>,
    milk: Option<String>,
    eggs: Option<String>,
}

fn code<T: DeserializeOwned>(v: Option<&str>) -> Option<T> {
    serde_json::from_value(serde_json::Value::String(v?.to_owned())).ok()
}

/// The form's fields → engine input; missing or unreadable values take the engine defaults.
fn quick_ferment(p: &Params) -> QuickFerment {
    let d = QuickFerment::default();
    let dd = DoughFactors::default();
    let n = |v: &Option<String>, default: f64| number(v.as_deref()).unwrap_or(default);
    let leavening: Leavening = code(p.leavening.as_deref()).unwrap_or(d.leavening);
    // A yeast form defaults to a yeast dose and endpoint, not the starter's.
    let (dose_default, endpoint_default) = if leavening == Leavening::Sourdough {
        (d.dose_pct, d.endpoint)
    } else {
        (0.5, Endpoint::Doubled)
    };
    QuickFerment {
        solve: code(p.solve.as_deref()).unwrap_or(d.solve),
        leavening,
        dose_pct: n(&p.dose, dose_default),
        starter_hydration_pct: n(&p.starter_hyd, d.starter_hydration_pct),
        starter_state: code(p.starter_state.as_deref()).unwrap_or(d.starter_state),
        target_h: n(&p.target_h, d.target_h),
        endpoint: code(p.endpoint.as_deref()).unwrap_or(endpoint_default),
        dough_c: n(&p.dough_c, d.dough_c),
        room_c: n(&p.room_c, d.room_c),
        dough_g: n(&p.dough_g, d.dough_g),
        container: code(p.container.as_deref()).unwrap_or(d.container),
        warm_h: n(&p.warm_h, d.warm_h),
        fridge_h: n(&p.fridge_h, d.fridge_h),
        fridge_c: n(&p.fridge_c, d.fridge_c),
        dough: DoughFactors {
            salt_pct: n(&p.salt, dd.salt_pct),
            sugar_pct: n(&p.sugar, dd.sugar_pct),
            fat_pct: n(&p.fat, dd.fat_pct),
            hydration_pct: n(&p.hydration, dd.hydration_pct),
            whole_grain_pct: n(&p.whole_grain, dd.whole_grain_pct),
            spice_pct: n(&p.spice, dd.spice_pct),
            cocoa_pct: n(&p.cocoa, dd.cocoa_pct),
            scalded_pct: n(&p.scalded, dd.scalded_pct),
            milk: p.milk.is_some(),
            eggs: p.eggs.is_some(),
        },
        vigour: None,
        anchor_c: None,
    }
}

/// "3 h 25", "45 min": rounded to 5 minutes.
pub fn duration(tr: &Tr, hours: f64) -> String {
    let minutes = (hours * 60.0 / 5.0).round() as i64 * 5;
    if minutes < 60 {
        tr.t_args("duration-min", &[("m", minutes.max(5).to_string().into())])
    } else {
        tr.t_args(
            "duration-hm",
            &[
                ("h", (minutes / 60).to_string().into()),
                ("m", format!("{:02}", minutes % 60).into()),
            ],
        )
    }
}

pub struct Row {
    pub label: String,
    pub value: String,
    pub detail: String,
}

pub struct ResultView {
    pub headline: String,
    pub range: String,
    pub where_: Option<String>,
    pub schedule: Vec<Row>,
    pub modifiers: Vec<Row>,
    pub notes: Vec<String>,
}

fn evidence(tr: &Tr, e: Evidence) -> String {
    tr.t(match e {
        Evidence::Measured => "evidence-measured",
        Evidence::Literature => "evidence-literature",
        Evidence::CreatorRule => "evidence-creator-rule",
        Evidence::Extrapolation => "evidence-extrapolation",
    })
}

fn view(tr: &Tr, q: &QuickFerment, r: &FermentResult) -> ResultView {
    let headline = match q.solve {
        Solve::Time => {
            tr.t_args("ferment-time-result", &[("t", duration(tr, r.hours.mid_h).into())])
        }
        Solve::Dose => {
            let key = if q.leavening == Leavening::Sourdough {
                "ferment-dose-starter"
            } else {
                "ferment-dose-yeast"
            };
            tr.t_args(key, &[("pct", tr.pct(r.dose_pct).into())])
        }
    };
    let range = tr.t_args(
        "ferment-range",
        &[
            ("low", duration(tr, r.hours.low_h).into()),
            ("high", duration(tr, r.hours.high_h).into()),
        ],
    );
    let fixed = q.warm_h + q.fridge_h;
    let where_ = (q.fridge_h > 0.0).then(|| match r.endpoint_phase {
        EndpointPhase::BeforeFridge => tr.t("ferment-where-before"),
        EndpointPhase::Fridge => tr.t_args(
            "ferment-where-fridge",
            &[("t", duration(tr, r.hours.mid_h - q.warm_h).into())],
        ),
        EndpointPhase::Room => {
            tr.t_args("ferment-where-after", &[("t", duration(tr, r.hours.mid_h - fixed).into())])
        }
    });
    let mut schedule = Vec::new();
    let phases = &r.schedule.phases;
    let mut labels = Vec::new();
    if q.fridge_h > 0.0 {
        if q.warm_h > 0.0 {
            labels.push(("ferment-phase-warm", q.warm_h));
        }
        labels.push(("ferment-phase-fridge", q.fridge_h));
    }
    for ((key, hours), p) in labels.iter().zip(phases) {
        schedule.push(Row {
            label: tr.t(key),
            value: duration(tr, *hours),
            detail: tr.t_args(
                "ferment-phase-detail",
                &[
                    ("p", tr.num(p.progress.min(9.99) * 100.0, 0).into()),
                    ("t", tr.num(p.end_temp_c, 1).into()),
                ],
            ),
        });
    }
    let modifiers = r
        .modifiers
        .iter()
        .map(|m| Row {
            label: tr.t(&format!("modifier-{}", m.code)),
            value: format!("×{}", tr.num(m.factor, 2)),
            detail: format!(
                "{} – {} · {}",
                tr.num(m.low, 2),
                tr.num(m.high, 2),
                evidence(tr, m.evidence)
            ),
        })
        .collect();
    let notes: Vec<String> = r
        .notes
        .iter()
        .map(|n| match n {
            FermentNote::Extrapolation => tr.t("ferment-note-extrapolation"),
            FermentNote::UseOsmotolerant { sugar_pct } => {
                tr.t_args("ferment-note-osmotolerant", &[("sugar", tr.pct(*sugar_pct).into())])
            }
            FermentNote::VeryShort => tr.t("ferment-note-very-short"),
            FermentNote::EndpointBeforeFridge => tr.t("ferment-note-before-fridge"),
            FermentNote::NoDoseFits { too_slow: true } => tr.t("ferment-note-no-dose-slow"),
            FermentNote::NoDoseFits { too_slow: false } => tr.t("ferment-note-no-dose-fast"),
            FermentNote::RichDoughCold => tr.t("ferment-note-rich-cold"),
        })
        .collect();
    ResultView { headline, range, where_, schedule, modifiers, notes }
}

#[derive(Template)]
#[template(path = "fermentation.html")]
struct Page {
    layout: Layout,
    form: QuickFerment,
    result: Option<ResultView>,
    error: Option<String>,
}

impl Page {
    fn v(&self, x: &f64) -> String {
        let s = format!("{x}");
        s.strip_suffix(".0").map(str::to_owned).unwrap_or(s)
    }

    /// `selected` / `checked` when the form's enum field has this serde code.
    fn is(&self, field: &str, value: &str) -> bool {
        let code = |v: serde_json::Result<serde_json::Value>| {
            v.ok().and_then(|v| v.as_str().map(str::to_owned))
        };
        let current = match field {
            "solve" => code(serde_json::to_value(self.form.solve)),
            "leavening" => code(serde_json::to_value(self.form.leavening)),
            "starter_state" => code(serde_json::to_value(self.form.starter_state)),
            "endpoint" => code(serde_json::to_value(self.form.endpoint)),
            "container" => code(serde_json::to_value(self.form.container)),
            _ => None,
        };
        current.as_deref() == Some(value)
    }

    fn sel(&self, field: &str, value: &str) -> &'static str {
        if self.is(field, value) { "selected" } else { "" }
    }

    fn chk(&self, field: &str, value: &str) -> &'static str {
        if self.is(field, value) { "checked" } else { "" }
    }
}

pub async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(params): Query<Params>,
) -> Response {
    let tr = lang.tr();
    let form = quick_ferment(&params);
    let (result, error) = match form.solve() {
        Ok(r) => (Some(view(&tr, &form, &r)), None),
        Err(e) => (None, Some(describe_error(&tr, &e))),
    };
    let layout = Layout::new(
        &state,
        lang,
        "/calculators/fermentation",
        "fermentation",
        tr.t("ferment-title"),
        tr.t("ferment-description"),
    );
    render(&Page { layout, form, result, error })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn durations_round_to_five_minutes() {
        let tr = Lang::Fr.tr();
        assert_eq!(duration(&tr, 3.42), "3 h 25");
        assert_eq!(duration(&tr, 0.3), "20 min");
        assert_eq!(
            Lang::En.tr().t_args("duration-hm", &[("h", "1".into()), ("m", "05".into())]),
            "1 h 05"
        );
    }

    #[test]
    fn yeast_form_gets_yeast_defaults() {
        let p = Params { leavening: Some("instant_yeast".into()), ..Default::default() };
        let q = quick_ferment(&p);
        assert_eq!(q.endpoint, Endpoint::Doubled);
        assert!(q.solve().is_ok());
    }
}
