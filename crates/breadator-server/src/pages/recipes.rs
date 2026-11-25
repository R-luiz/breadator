//! Recipes: index and recipe pages. A recipe is an engine formula plus translated steps; its page
//! shows the solved weights and opens the formula in the calculator to scale or change it.

use askama::Template;
use axum::extract::{Path, State};
use axum::response::Response;
use breadator_engine::formula::solve;

use super::formula::{Results, results};
use crate::app::{AppState, LangPath, Layout, not_found_page, render};
use crate::knowledge::cite;
use crate::state_codec;

pub struct RecipeLink {
    pub href: String,
    pub title: String,
    pub description: String,
    pub kind: String,
}

#[derive(Template)]
#[template(path = "recipes.html")]
struct IndexPage {
    layout: Layout,
    recipes: Vec<RecipeLink>,
}

pub async fn index(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    let code = lang.code();
    let mut recipes: Vec<RecipeLink> = state
        .knowledge
        .lib
        .recipes()
        .map(|r| RecipeLink {
            href: format!("/{code}/recipes/{}", r.slug),
            title: r.title.get(code).to_owned(),
            description: r.description.get(code).to_owned(),
            kind: tr
                .try_t(&format!("bread-type-{}", r.bread_type))
                .unwrap_or_else(|| r.bread_type.replace('-', " ")),
        })
        .collect();
    recipes.sort_by(|a, b| {
        (a.kind.as_str(), a.title.as_str()).cmp(&(b.kind.as_str(), b.title.as_str()))
    });
    let layout = Layout::new(
        &state,
        lang,
        "/recipes",
        "recipes",
        tr.t("recipes-title"),
        tr.t("recipes-description"),
    );
    render(&IndexPage { layout, recipes })
}

/// One work a recipe draws on, with the places in it (video timestamps) when several claims
/// come from the same work.
pub struct SourceView {
    pub text: String,
    pub href: Option<String>,
    pub places: Vec<(String, Option<String>)>,
}

/// The recipe's sources, one entry per work: nine claims from one video read as the video's
/// title followed by nine timestamps, not as nine lines with the same title.
fn sources(lib: &breadator_content::Library, keys: &[String]) -> Vec<SourceView> {
    let mut out: Vec<(String, Vec<crate::knowledge::Citation>)> = Vec::new();
    for key in keys {
        let c = cite(lib, key);
        match out.iter_mut().find(|(work, _)| *work == c.work) {
            Some((_, group)) => group.push(c),
            None => out.push((c.work.clone(), vec![c])),
        }
    }
    out.into_iter()
        .map(|(work, mut group)| {
            if group.len() == 1 {
                let c = group.remove(0);
                return SourceView { text: c.text, href: c.href, places: Vec::new() };
            }
            let first = group[0].href.clone();
            let places: Vec<_> =
                group.into_iter().filter_map(|c| c.at.map(|at| (at, c.href))).collect();
            let href = if places.is_empty() { first } else { None };
            SourceView { text: work, href, places }
        })
        .collect()
}

#[derive(Template)]
#[template(path = "recipe.html")]
struct RecipePage {
    layout: Layout,
    title: String,
    description: String,
    results: Option<Results>,
    calculator: String,
    steps: Vec<String>,
    notes: Option<String>,
    sources: Vec<SourceView>,
}

pub async fn recipe(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, slug)): Path<(String, String)>,
) -> Response {
    let lib = &state.knowledge.lib;
    let Some(r) = lib.recipe(&slug) else {
        return not_found_page(lang);
    };
    let tr = lang.tr();
    let code = lang.code();
    // Validated at start-up: every recipe formula solves.
    let results = solve(&r.formula).ok().map(|s| results(&tr, &s));
    let calculator = format!("/{code}/calculators/formula?s={}", state_codec::encode(&r.formula));
    let sources = sources(lib, &r.sources);
    let title = r.title.get(code).to_owned();
    let description = r.description.get(code).to_owned();
    let layout = Layout::new(
        &state,
        lang,
        &format!("/recipes/{slug}"),
        "recipes",
        title.clone(),
        description.clone(),
    );
    render(&RecipePage {
        layout,
        title,
        description,
        results,
        calculator,
        steps: r.steps.iter().map(|s| s.get(code).to_owned()).collect(),
        notes: r.notes.as_ref().map(|n| n.get(code).to_owned()),
        sources,
    })
}
