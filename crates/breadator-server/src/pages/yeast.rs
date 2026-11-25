//! Yeast conversion calculator.

use askama::Template;
use axum::extract::{Query, State};
use axum::response::Response;
use breadator_engine::ingredients::IngredientKind;
use breadator_engine::yeast::convert;
use serde::Deserialize;

use super::describe_error;
use crate::app::{AppState, LangPath, Layout, render};
use crate::state_codec::number;

#[derive(Debug, Default, Deserialize)]
pub struct Params {
    grams: Option<String>,
    from: Option<String>,
}

const KINDS: [(&str, IngredientKind); 4] = [
    ("fresh_yeast", IngredientKind::FreshYeast),
    ("active_dry_yeast", IngredientKind::ActiveDryYeast),
    ("instant_yeast", IngredientKind::InstantYeast),
    ("osmotolerant_yeast", IngredientKind::OsmotolerantYeast),
];

pub struct Row {
    pub name: String,
    pub grams: String,
    pub range: String,
}

#[derive(Template)]
#[template(path = "yeast.html")]
struct Page {
    layout: Layout,
    grams: String,
    from: String,
    kinds: Vec<(&'static str, String)>,
    rows: Vec<Row>,
    error: Option<String>,
}

impl Page {
    fn is_from(&self, code: &str) -> bool {
        self.from == code
    }
}

pub async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(params): Query<Params>,
) -> Response {
    let tr = lang.tr();
    let grams = number(params.grams.as_deref()).unwrap_or(10.0);
    let (from_code, from) = KINDS
        .iter()
        .copied()
        .find(|(code, _)| Some(*code) == params.from.as_deref())
        .unwrap_or(KINDS[0]);
    let mut rows = Vec::new();
    let mut error = None;
    for (code, to) in KINDS {
        if to == from {
            continue;
        }
        match convert(grams, from, to) {
            Ok(c) => rows.push(Row {
                name: tr.t(&format!("entry-{code}")),
                grams: tr.g(c.grams),
                range: tr.g_range(c.low_g, c.high_g),
            }),
            Err(e) => {
                error = Some(describe_error(&tr, &e));
                break;
            }
        }
    }
    let layout = Layout::new(
        &state,
        lang,
        "/calculators/yeast",
        "yeast",
        tr.t("yeast-title"),
        tr.t("yeast-description"),
    );
    let kinds = KINDS.iter().map(|(code, _)| (*code, tr.t(&format!("entry-{code}")))).collect();
    let s = format!("{grams}");
    render(&Page {
        layout,
        grams: s.strip_suffix(".0").map(str::to_owned).unwrap_or(s),
        from: from_code.to_owned(),
        kinds,
        rows,
        error,
    })
}
