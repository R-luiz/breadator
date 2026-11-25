//! The library as tools: the articles, the recipes and the bibliography of the site's pages, in
//! a form a model reads.

use std::fmt::Write as _;

use breadator_content::Library;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, json};

use super::passages::{self, Gap};
use super::{Context, Source, calculator_url, solution, target, to_json};
use crate::knowledge::cite;
use crate::pages::learn::{topic_rank, topic_title};
use crate::pages::recipes;

/// References returned by one `list_sources` call.
const SOURCES_PAGE: usize = 30;
/// Words of a `list_sources` query that count: every one is looked for in every reference.
const QUERY_WORDS: usize = 8;

fn read<T: DeserializeOwned>(args: &Map<String, Value>) -> Result<T, String> {
    serde_json::from_value(Value::Object(args.clone())).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NoArgs {}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SlugArgs {
    slug: String,
}

/// A citation key as a reader sees it: the reference, or the work a claim was taken from, and
/// its link.
fn citation(lib: &Library, key: &str) -> Value {
    let c = cite(lib, key);
    match c.href {
        Some(url) => json!({ "text": c.text, "url": url }),
        None => json!({ "text": c.text }),
    }
}

/// Nulls and empty lists dropped: what a formula leaves unset says nothing.
fn prune(value: &mut Value) {
    match value {
        Value::Array(items) => items.iter_mut().for_each(prune),
        Value::Object(fields) => {
            fields.retain(|_, v| !v.is_null() && v.as_array().is_none_or(|a| !a.is_empty()));
            fields.values_mut().for_each(prune);
        }
        _ => {}
    }
}

/// What the `[n]` of a search's passages stand for: the citations behind a number, or, for a
/// passage that cites nothing itself, the article or recipe page it was read on. (A number
/// with citations may serve passages of several pages: each passage names its own.)
pub fn source_list(ctx: &Context, sources: &[Source]) -> Value {
    let lib = &ctx.knowledge.lib;
    let lang = ctx.lang.code();
    sources
        .iter()
        .map(|source| {
            if !source.keys.is_empty() {
                let citations: Vec<Value> =
                    source.keys.iter().map(|key| citation(lib, key)).collect();
                return json!({ "n": source.n, "citations": citations });
            }
            let (title, path) = match source.kind.as_str() {
                "recipe" => (
                    lib.recipe(&source.id).map_or(source.title.as_str(), |r| r.title.get(lang)),
                    format!("/recipes/{}", source.id),
                ),
                _ => (
                    lib.article(&source.id, lang)
                        .map_or(source.title.as_str(), |a| a.front.title.as_str()),
                    format!("/learn/{}", source.id),
                ),
            };
            json!({ "n": source.n, "page": { "title": title, "url": ctx.url(&path) } })
        })
        .collect()
}

pub fn list_articles(ctx: &Context, args: &Map<String, Value>) -> Result<Value, String> {
    let NoArgs {} = read(args)?;
    let tr = ctx.lang.tr();
    // The library lists articles by topic, then by their order within it; the sort is stable.
    let mut articles = ctx.knowledge.lib.articles(ctx.lang.code());
    articles.sort_by_key(|a| topic_rank(&a.front.topic));
    let articles: Vec<Value> = articles
        .iter()
        .map(|a| {
            json!({
                "slug": a.slug,
                "title": a.front.title,
                "description": a.front.description,
                "topic": topic_title(&tr, &a.front.topic),
                "url": ctx.url(&format!("/learn/{}", a.slug)),
            })
        })
        .collect();
    Ok(json!({ "articles": articles }))
}

/// The body of an article (its rendered HTML) as Markdown: sections, paragraphs, list items
/// and table rows on their own lines, each statement followed by the numbers of the sources it
/// cites (`[1][3]`: places in `citations`, as on the article's page), spaced as written.
fn body_markdown(html: &str, citations: &[String], out: &mut String) {
    let blocks = passages::blocks(html, citations);
    let mut heading = "";
    for block in &blocks {
        if block.heading != heading {
            heading = &block.heading;
            let _ = write!(out, "\n## {heading}\n");
        }
        out.push('\n');
        for segment in &block.segments {
            match segment.gap {
                Gap::None => {}
                Gap::Space => out.push(' '),
                Gap::Line => out.push('\n'),
            }
            out.push_str(&segment.text);
            if !segment.keys.is_empty() {
                out.push(' ');
            }
            for key in &segment.keys {
                if let Some(at) = citations.iter().position(|k| k == key) {
                    let _ = write!(out, "[{}]", at + 1);
                }
            }
        }
        out.push('\n');
    }
}

/// An article as text a model reads: Markdown in which each statement is followed by the
/// numbers of its sources, then the list of those sources.
pub fn article_markdown(ctx: &Context, slug: &str) -> Option<String> {
    let lib = &ctx.knowledge.lib;
    let article = lib.article(slug, ctx.lang.code())?;
    let mut out = format!(
        "# {}\n\n{}\n\n{} · {} · {}\n",
        article.front.title,
        article.front.description,
        topic_title(&ctx.lang.tr(), &article.front.topic),
        article.front.updated,
        ctx.url(&format!("/learn/{slug}")),
    );
    body_markdown(&article.html, &article.citations, &mut out);
    if !article.citations.is_empty() {
        out.push_str("\n## Sources\n\n");
        for (at, key) in article.citations.iter().enumerate() {
            let c = cite(lib, key);
            let _ = write!(out, "[{}] {}", at + 1, c.text);
            if let Some(url) = c.href {
                let _ = write!(out, " {url}");
            }
            out.push('\n');
        }
    }
    Some(out)
}

pub fn read_article(ctx: &Context, args: &Map<String, Value>) -> Result<Value, String> {
    let SlugArgs { slug } = read(args)?;
    article_markdown(ctx, slug.trim()).map(Value::String).ok_or_else(|| {
        format!(
            "no article '{slug}': list_articles gives the slugs, search_knowledge the article that covers a subject"
        )
    })
}

pub fn list_recipes(ctx: &Context, args: &Map<String, Value>) -> Result<Value, String> {
    let NoArgs {} = read(args)?;
    let tr = ctx.lang.tr();
    let lang = ctx.lang.code();
    // As the recipes page: by kind of bread, then by title.
    let mut found: Vec<(String, &str, Value)> = ctx
        .knowledge
        .lib
        .recipes()
        .map(|r| {
            let kind = recipes::bread_type(&tr, &r.bread_type);
            let title = r.title.get(lang);
            let entry = json!({
                "slug": r.slug,
                "title": title,
                "description": r.description.get(lang),
                "bread_type": kind,
                "url": ctx.url(&format!("/recipes/{}", r.slug)),
            });
            (kind, title, entry)
        })
        .collect();
    found.sort_by(|a, b| (a.0.as_str(), a.1).cmp(&(b.0.as_str(), b.1)));
    let recipes: Vec<Value> = found.into_iter().map(|(_, _, entry)| entry).collect();
    Ok(json!({ "recipes": recipes }))
}

/// A recipe with its weights, for the amount it is written for or for `amount`.
pub fn recipe(ctx: &Context, slug: &str, amount: Option<&Value>) -> Result<Value, String> {
    let lib = &ctx.knowledge.lib;
    let lang = ctx.lang.code();
    let recipe = lib
        .recipe(slug)
        .ok_or_else(|| format!("no recipe '{slug}': list_recipes gives the slugs"))?;
    let mut formula = recipe.formula.clone();
    if let Some(amount) = amount {
        formula.target = target(amount)?;
    }
    // One entry per work, with the places in it (video timestamps), as on the recipe's page.
    let sources: Vec<Value> = recipes::sources(lib, &recipe.sources)
        .into_iter()
        .map(|source| {
            let places: Vec<Value> = source
                .places
                .into_iter()
                .map(|(at, url)| json!({ "at": at, "url": url }))
                .collect();
            json!({ "text": source.text, "url": source.href, "places": places })
        })
        .collect();
    let steps: Vec<&str> = recipe.steps.iter().map(|step| step.get(lang)).collect();
    let mut out = json!({
        "title": recipe.title.get(lang),
        "description": recipe.description.get(lang),
        "bread_type": recipes::bread_type(&ctx.lang.tr(), &recipe.bread_type),
        "url": ctx.url(&format!("/recipes/{slug}")),
        "calculator_url": calculator_url(ctx, &formula),
        "formula": to_json(&formula)?,
        "steps": steps,
        "notes": recipe.notes.as_ref().map(|notes| notes.get(lang)),
        "sources": sources,
    });
    prune(&mut out);
    out["weights"] = solution(&formula)?;
    Ok(out)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RecipeArgs {
    slug: String,
    target: Option<Value>,
}

pub fn get_recipe(ctx: &Context, args: &Map<String, Value>) -> Result<Value, String> {
    let RecipeArgs { slug, target } = read(args)?;
    recipe(ctx, slug.trim(), target.as_ref())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourcesArgs {
    #[serde(default)]
    query: String,
    #[serde(default)]
    offset: usize,
}

pub fn list_sources(ctx: &Context, args: &Map<String, Value>) -> Result<Value, String> {
    let SourcesArgs { query, offset } = read(args)?;
    let lib = &ctx.knowledge.lib;
    let words: Vec<String> =
        query.split_whitespace().take(QUERY_WORDS).map(passages::fold).collect();
    let found: Vec<&str> = lib
        .bibliography()
        .into_iter()
        .filter(|reference| {
            let text = passages::fold(&cite(lib, &reference.key).text);
            words.iter().all(|word| text.contains(word.as_str()))
        })
        .map(|reference| reference.key.as_str())
        .collect();
    let sources: Vec<Value> =
        found.iter().skip(offset).take(SOURCES_PAGE).map(|key| citation(lib, key)).collect();
    let mut out = json!({ "total": found.len(), "sources": sources });
    if offset.saturating_add(SOURCES_PAGE) < found.len() {
        out["next_offset"] = (offset + SOURCES_PAGE).into();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_article_body_keeps_its_sections_lists_tables_and_citation_numbers() {
        let rendered = breadator_content::render_markdown(
            "Intro without source.\n\n## Autolyse & rest\n\nCalvel described it in 1974 [@calvel2001]. \
             The gains in *flavour* are small [@novita-0105; @calvel2001] (see below) ; unsourced end.\n\n\
             - Gentle folds [@breadcode-0828].\n- A \"firm\" dough\n\n## Numbers\n\n\
             | flour | water |\n|---|---|\n| 100 [@yang2019] | 70 |\n| 90 | 65 [@yang2019] |\n",
        );
        let mut out = String::new();
        body_markdown(&rendered.html, &rendered.citations, &mut out);
        assert_eq!(
            out,
            "\nIntro without source.\n\
             \n## Autolyse & rest\n\
             \nCalvel described it in 1974 [1]. The gains in flavour are small [2][1] (see below) ; unsourced end.\n\
             \nGentle folds [3].\nA \"firm\" dough\n\
             \n## Numbers\n\
             \nflour | water |\n100 [4] | 70 |\n90 | 65 [4] |\n"
        );
    }
}
