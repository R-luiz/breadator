//! Starter calculator: when a refreshed starter peaks, which feed peaks at a chosen time, and the
//! grams to weigh for a build.

use askama::Template;
use axum::extract::{Query, State};
use axum::response::Response;
use breadator_engine::leaven::{FermentNote, QuickStarter, Solve, StarterResult, StarterState};
use serde::Deserialize;

use super::describe_error;
use super::fermentation::duration;
use crate::app::{AppState, LangPath, Layout, render};
use crate::i18n::Tr;
use crate::state_codec::number;

#[derive(Debug, Default, Deserialize)]
pub struct Params {
    solve: Option<String>,
    flour: Option<String>,
    water: Option<String>,
    seed_hyd: Option<String>,
    hyd: Option<String>,
    temp_c: Option<String>,
    target_h: Option<String>,
    seed_state: Option<String>,
    need_g: Option<String>,
}

fn quick_starter(p: &Params) -> QuickStarter {
    let d = QuickStarter::default();
    let n = |v: &Option<String>, default: f64| number(v.as_deref()).unwrap_or(default);
    QuickStarter {
        solve: if p.solve.as_deref() == Some("dose") { Solve::Dose } else { Solve::Time },
        flour_parts: n(&p.flour, d.flour_parts),
        water_parts: n(&p.water, d.water_parts),
        seed_hydration_pct: n(&p.seed_hyd, d.seed_hydration_pct),
        hydration_pct: n(&p.hyd, d.hydration_pct),
        temp_c: n(&p.temp_c, d.temp_c),
        target_h: n(&p.target_h, d.target_h),
        seed_state: if p.seed_state.as_deref() == Some("from_fridge") {
            StarterState::FromFridge
        } else {
            StarterState::Ripe
        },
        need_g: n(&p.need_g, d.need_g),
        vigour: None,
    }
}

/// "1 : 5 : 5" with up to one decimal.
fn ratio(tr: &Tr, flour: f64, water: f64) -> String {
    let part = |x: f64| if (x - x.round()).abs() < 0.05 { tr.num(x, 0) } else { tr.num(x, 1) };
    format!("1 : {} : {}", part(flour), part(water))
}

pub struct ResultView {
    pub headline: String,
    pub range: String,
    pub details: Vec<(String, String)>,
    pub build: Option<(String, String, String)>,
    pub notes: Vec<String>,
}

fn view(tr: &Tr, q: &QuickStarter, r: &StarterResult) -> ResultView {
    let headline = match q.solve {
        Solve::Time => {
            tr.t_args("starter-peak-result", &[("t", duration(tr, r.hours.mid_h).into())])
        }
        Solve::Dose => tr.t_args(
            "starter-ratio-result",
            &[("ratio", ratio(tr, r.flour_parts, r.water_parts).into())],
        ),
    };
    let range = tr.t_args(
        "starter-range",
        &[
            ("low", duration(tr, r.hours.low_h).into()),
            ("high", duration(tr, r.hours.high_h).into()),
        ],
    );
    let details = vec![
        (tr.t("starter-feed"), ratio(tr, r.flour_parts, r.water_parts)),
        (tr.t("starter-hydration"), format!("{} %", tr.pct(r.hydration_pct))),
        (tr.t("starter-seed-share"), format!("{} %", tr.pct(r.iota * 100.0))),
    ];
    let build = r.build.map(|(s, f, w)| (tr.g(s), tr.g(f), tr.g(w)));
    let notes = r
        .notes
        .iter()
        .map(|n| match n {
            FermentNote::Extrapolation => tr.t("ferment-note-extrapolation"),
            FermentNote::NoDoseFits { too_slow: true } => tr.t("starter-note-too-fast"),
            FermentNote::NoDoseFits { too_slow: false } => tr.t("starter-note-too-slow"),
            _ => String::new(),
        })
        .filter(|s| !s.is_empty())
        .collect();
    ResultView { headline, range, details, build, notes }
}

#[derive(Template)]
#[template(path = "starter.html")]
struct Page {
    layout: Layout,
    form: QuickStarter,
    result: Option<ResultView>,
    error: Option<String>,
}

impl Page {
    fn v(&self, x: &f64) -> String {
        let s = format!("{x}");
        s.strip_suffix(".0").map(str::to_owned).unwrap_or(s)
    }

    fn dose(&self) -> bool {
        self.form.solve == Solve::Dose
    }

    fn cold_seed(&self) -> bool {
        self.form.seed_state == StarterState::FromFridge
    }
}

pub async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(params): Query<Params>,
) -> Response {
    let tr = lang.tr();
    let form = quick_starter(&params);
    let (result, error) = match form.solve() {
        Ok(r) => (Some(view(&tr, &form, &r)), None),
        Err(e) => (None, Some(describe_error(&tr, &e))),
    };
    let layout = Layout::new(
        &state,
        lang,
        "/calculators/starter",
        "starter",
        tr.t("starter-title"),
        tr.t("starter-description"),
    );
    render(&Page { layout, form, result, error })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn ratios_read_like_a_baker_writes_them() {
        let tr = Lang::Fr.tr();
        assert_eq!(ratio(&tr, 5.0, 5.0), "1 : 5 : 5");
        assert_eq!(ratio(&tr, 5.0, 2.75), "1 : 5 : 2,8");
        let q = quick_starter(&Params {
            solve: Some("dose".into()),
            target_h: Some("10".into()),
            ..Default::default()
        });
        assert_eq!(q.solve, Solve::Dose);
        assert!(q.solve().is_ok());
    }
}
