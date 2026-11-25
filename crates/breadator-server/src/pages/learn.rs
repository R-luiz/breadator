//! Knowledge base pages: topic index, articles with their sources, bibliography, search.

use askama::Template;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::app::{AppState, LangPath, Layout, not_found_page, render};
use crate::i18n::Tr;
use crate::knowledge::cite;

pub struct ArticleLink {
    pub href: String,
    pub title: String,
    pub description: String,
}

pub struct TopicGroup {
    pub title: String,
    pub articles: Vec<ArticleLink>,
}

/// Translated topic name (`topic-starch`…); the first segment of a dotted topic.
pub(crate) fn topic_title(tr: &Tr, topic: &str) -> String {
    let root = topic.split('.').next().unwrap_or(topic);
    tr.try_t(&format!("topic-{root}")).unwrap_or_else(|| root.replace('-', " "))
}

/// Topics in the order a baker meets them: what a dough is made of, the leavening, the process
/// from mixing to the oven, then what comes after. A topic missing here comes last.
const TOPIC_ORDER: [&str; 26] = [
    "bread-type",
    "flour",
    "gluten",
    "starch",
    "enzymes",
    "fibre",
    "hydration",
    "salt",
    "sugar",
    "fat",
    "dairy-egg",
    "yeast",
    "starter",
    "sourdough-microbiology",
    "preferment",
    "scalds",
    "mixing",
    "dough-temperature",
    "bulk",
    "shaping",
    "proof",
    "baking",
    "staling-storage",
    "flavour",
    "nutrition",
    "troubleshooting",
];

pub(crate) fn topic_rank(topic: &str) -> usize {
    let root = topic.split('.').next().unwrap_or(topic);
    TOPIC_ORDER.iter().position(|t| *t == root).unwrap_or(TOPIC_ORDER.len())
}

#[derive(Template)]
#[template(path = "learn.html")]
struct IndexPage {
    layout: Layout,
    groups: Vec<TopicGroup>,
}

pub async fn index(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    let mut groups: Vec<TopicGroup> = Vec::new();
    // The library lists articles by topic then by their `order`; the sort is stable.
    let mut articles = state.knowledge.lib.articles(lang.code());
    articles.sort_by_key(|a| topic_rank(&a.front.topic));
    for a in articles {
        let title = topic_title(&tr, &a.front.topic);
        let link = ArticleLink {
            href: format!("/{}/learn/{}", lang.code(), a.slug),
            title: a.front.title.clone(),
            description: a.front.description.clone(),
        };
        match groups.last_mut() {
            Some(g) if g.title == title => g.articles.push(link),
            _ => groups.push(TopicGroup { title, articles: vec![link] }),
        }
    }
    let layout = Layout::new(
        &state,
        lang,
        "/learn",
        "learn",
        tr.t("learn-title"),
        tr.t("learn-description"),
    );
    render(&IndexPage { layout, groups })
}

pub struct SourceView {
    pub n: usize,
    pub text: String,
    pub href: Option<String>,
}

#[derive(Template)]
#[template(path = "article.html")]
struct ArticlePage {
    layout: Layout,
    topic: String,
    updated: String,
    /// The article's sections, linked from the table of contents.
    headings: Vec<breadator_content::Heading>,
    html: String,
    sources: Vec<SourceView>,
}

pub async fn article(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Path((_, slug)): Path<(String, String)>,
) -> Response {
    let lib = &state.knowledge.lib;
    let Some(a) = lib.article(&slug, lang.code()) else {
        return not_found_page(lang);
    };
    let tr = lang.tr();
    let sources = a
        .citations
        .iter()
        .enumerate()
        .map(|(i, key)| {
            let c = cite(lib, key);
            SourceView { n: i + 1, text: c.text, href: c.href }
        })
        .collect();
    let layout = Layout::new(
        &state,
        lang,
        &format!("/learn/{slug}"),
        "learn",
        a.front.title.clone(),
        a.front.description.clone(),
    );
    render(&ArticlePage {
        layout,
        topic: topic_title(&tr, &a.front.topic),
        updated: a.front.updated.clone(),
        headings: a.headings.clone(),
        html: a.html.clone(),
        sources,
    })
}

pub struct ReferenceView {
    pub key: String,
    pub text: String,
    pub href: Option<String>,
}

#[derive(Template)]
#[template(path = "sources.html")]
struct SourcesPage {
    layout: Layout,
    references: Vec<ReferenceView>,
}

pub async fn sources(State(state): State<AppState>, LangPath(lang): LangPath) -> Response {
    let tr = lang.tr();
    let lib = &state.knowledge.lib;
    let references = lib
        .bibliography()
        .into_iter()
        .map(|r| {
            let c = cite(lib, &r.key);
            ReferenceView { key: r.key.clone(), text: c.text, href: c.href }
        })
        .collect();
    let layout = Layout::new(
        &state,
        lang,
        "/sources",
        "learn",
        tr.t("sources-title"),
        tr.t("sources-description"),
    );
    render(&SourcesPage { layout, references })
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchParams {
    q: Option<String>,
}

pub struct HitView {
    pub href: Option<String>,
    pub title: String,
    pub kind: String,
    /// Snippet split into (text, is_match) runs; matches come from the index's `[`…`]` markers.
    pub snippet: Vec<(String, bool)>,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchPage {
    layout: Layout,
    q: String,
    hits: Vec<HitView>,
}

fn snippet_runs(s: &str) -> Vec<(String, bool)> {
    let mut out = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find('[') {
        let Some(len) = rest[start..].find(']') else { break };
        if start > 0 {
            out.push((rest[..start].to_owned(), false));
        }
        out.push((rest[start + 1..start + len].to_owned(), true));
        rest = &rest[start + len + 1..];
    }
    if !rest.is_empty() {
        out.push((rest.to_owned(), false));
    }
    out
}

pub async fn search(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(p): Query<SearchParams>,
) -> Response {
    let tr = lang.tr();
    let q = p.q.unwrap_or_default().chars().take(200).collect::<String>();
    let hits = match state.knowledge.index.search(&q, lang.code(), 30) {
        Ok(hits) => hits,
        Err(e) => {
            tracing::error!(error = %e, "search failed");
            Vec::new()
        }
    };
    let lib = &state.knowledge.lib;
    let hits = hits
        .into_iter()
        .map(|h| {
            let (href, title) = match h.kind.as_str() {
                "article" => (Some(format!("/{}/learn/{}", lang.code(), h.id)), h.title),
                "recipe" => (Some(format!("/{}/recipes/{}", lang.code(), h.id)), h.title),
                _ => {
                    let c = cite(lib, &h.id);
                    (c.href, c.text)
                }
            };
            HitView {
                href,
                title,
                kind: tr.t(&format!("search-kind-{}", h.kind)),
                snippet: snippet_runs(&h.snippet),
            }
        })
        .collect();
    let layout = Layout::new(
        &state,
        lang,
        "/search",
        "learn",
        tr.t("search-title"),
        tr.t("search-description"),
    );
    let mut response = render(&SearchPage { layout, q, hits });
    // Result pages are not worth indexing.
    response.headers_mut().insert("x-robots-tag", axum::http::HeaderValue::from_static("noindex"));
    response.into_response()
}

#[cfg(test)]
mod tests {
    #[test]
    fn snippet_markers_become_runs() {
        let runs = super::snippet_runs("le [yudane] gélatinise [l'amidon]…");
        // "le ", "yudane", " gélatinise ", "l'amidon", "…"
        assert_eq!(runs.len(), 5);
        assert_eq!(runs[1], ("yudane".to_owned(), true));
    }
}
