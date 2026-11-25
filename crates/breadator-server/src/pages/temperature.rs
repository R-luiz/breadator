//! Water temperature calculator: heat balance (default) or the classic base-temperature rule.

use askama::Template;
use axum::extract::{Query, State};
use axum::response::Response;
use breadator_engine::temperature::{
    QuickMethod, QuickTemperature, WaterTempResult, water_temperature,
};
use serde::Deserialize;

use super::{describe_error, describe_warning};
use crate::app::{AppState, LangPath, Layout, render};
use crate::state_codec::number;

#[derive(Debug, Default, Deserialize)]
pub struct Params {
    method: Option<String>,
    desired: Option<String>,
    flour_c: Option<String>,
    room_c: Option<String>,
    flour_g: Option<String>,
    water_g: Option<String>,
    pref_g: Option<String>,
    pref_c: Option<String>,
    pref_hyd: Option<String>,
    rise: Option<String>,
    friction: Option<String>,
    tap_c: Option<String>,
}

fn quick_temperature(p: &Params) -> QuickTemperature {
    let d = QuickTemperature::default();
    let n = |v: &Option<String>, default: f64| number(v.as_deref()).unwrap_or(default);
    QuickTemperature {
        method: if p.method.as_deref() == Some("classic") {
            QuickMethod::Classic
        } else {
            QuickMethod::Balance
        },
        desired: n(&p.desired, d.desired),
        flour_c: n(&p.flour_c, d.flour_c),
        room_c: n(&p.room_c, d.room_c),
        flour_g: n(&p.flour_g, d.flour_g),
        water_g: n(&p.water_g, d.water_g),
        pref_g: n(&p.pref_g, d.pref_g),
        pref_c: n(&p.pref_c, d.pref_c),
        pref_hyd: n(&p.pref_hyd, d.pref_hyd),
        rise: n(&p.rise, d.rise),
        friction: n(&p.friction, d.friction),
        tap_c: number(p.tap_c.as_deref()),
    }
}

pub struct ResultView {
    pub water: String,
    pub ice: Option<String>,
    pub expected: String,
    pub warnings: Vec<String>,
}

#[derive(Template)]
#[template(path = "temperature.html")]
struct Page {
    layout: Layout,
    form: QuickTemperature,
    result: Option<ResultView>,
    error: Option<String>,
}

impl Page {
    fn v(&self, x: &f64) -> String {
        let s = format!("{x}");
        s.strip_suffix(".0").map(str::to_owned).unwrap_or(s)
    }

    fn tap(&self) -> String {
        self.form.tap_c.map(|t| self.v(&t)).unwrap_or_default()
    }

    fn classic(&self) -> bool {
        self.form.method == QuickMethod::Classic
    }
}

pub async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(params): Query<Params>,
) -> Response {
    let tr = lang.tr();
    let form = quick_temperature(&params);
    let (result, error) = match water_temperature(&form.to_input()) {
        Ok(WaterTempResult { water_c, ice_g, tap_water_g, expected_dough_c, warnings }) => (
            Some(ResultView {
                water: tr.temp(water_c),
                ice: ice_g.zip(tap_water_g).zip(form.tap_c).map(|((i, w), tap)| {
                    tr.t_args(
                        "temperature-ice-result",
                        &[
                            ("tap", tr.temp(tap).into()),
                            ("ice", tr.g(i).into()),
                            ("water", tr.g(w).into()),
                        ],
                    )
                }),
                expected: tr.temp(expected_dough_c),
                warnings: warnings.iter().map(|w| describe_warning(&tr, w)).collect(),
            }),
            None,
        ),
        Err(e) => (None, Some(describe_error(&tr, &e))),
    };
    let layout = Layout::new(
        &state,
        lang,
        "/calculators/temperature",
        "temperature",
        tr.t("temperature-title"),
        tr.t("temperature-description"),
    );
    render(&Page { layout, form, result, error })
}
