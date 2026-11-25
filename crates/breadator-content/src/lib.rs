//! Breadator's knowledge base.
//!
//! Content lives in the repository as text and is compiled into the server:
//!
//! - `content/articles/<slug>/<lang>.md`: TOML front matter between `+++` lines, then Markdown.
//!   Citations are written `[@key]` or `[@key1; @key2]`; a key is a bibliography key
//!   (`research/data/bibliography.json`) or a claim id (`research/data/claims.jsonl`), whose own
//!   source is then cited. Raw HTML in Markdown is shown as text, never interpreted.
//! - `content/recipes/<slug>.toml`: an engine formula plus translated title, notes and steps.
//!
//! [`Library::load`] validates everything and returns every problem at once: an unknown citation,
//! an article missing a language, front matter that differs between languages, an invalid recipe
//! formula. The server refuses to start on an invalid library and CI runs the same check.

mod markdown;
pub mod search;

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

pub use markdown::{Heading, render as render_markdown};

pub const LANGS: [&str; 2] = ["fr", "en"];
/// Calculator pages articles may link to (`/<lang>/calculators/<name>`).
pub const CALCULATORS: [&str; 6] =
    ["formula", "fermentation", "starter", "flour", "temperature", "yeast"];

/// A source file handed to the loader: path relative to the repository root, and its text.
pub struct SourceFile<'a> {
    pub path: &'a str,
    pub text: &'a str,
}

/// A YouTube video's public metadata (`research/data/videos.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub title: String,
    #[serde(default)]
    pub channel: Option<String>,
    /// ISO 8601 publication date.
    #[serde(default)]
    pub published: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{path}: {message}")]
pub struct ContentError {
    pub path: String,
    pub message: String,
}

/// One bibliography entry (`research/data/bibliography.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub key: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub year: Option<i32>,
    pub title: String,
    #[serde(default)]
    pub venue: Option<String>,
    #[serde(default)]
    pub doi: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    /// Open-access copy, when one exists.
    #[serde(default)]
    pub oa_url: Option<String>,
    /// Licence of material adapted from this source (e.g. `CC-BY-SA-4.0`).
    #[serde(default)]
    pub license: Option<String>,
}

impl Reference {
    /// Best link for a reader: open access first, then DOI, then the URL.
    pub fn link(&self) -> Option<String> {
        self.oa_url
            .clone()
            .or_else(|| self.doi.as_ref().map(|d| format!("https://doi.org/{d}")))
            .or_else(|| self.url.clone())
    }
}

/// The part of a research claim the content layer needs (`research/data/claims.jsonl`: every
/// source's claims merged, their citations rewritten to bibliography keys).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub topic: String,
    pub claim: String,
    pub source: ClaimSource,
    pub evidence: String,
    #[serde(default)]
    pub cites: Vec<String>,
    #[serde(default)]
    pub confidence: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimSource {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(default)]
    pub locator: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

impl Claim {
    /// Link to the claim's source, positioned at its timestamp for YouTube videos.
    pub fn link(&self) -> Option<String> {
        if let Some(id) = self.source.reference.strip_prefix("youtube:") {
            let t = self.source.locator.as_deref().and_then(timestamp_seconds);
            return Some(match t {
                Some(s) => format!("https://www.youtube.com/watch?v={id}&t={s}s"),
                None => format!("https://www.youtube.com/watch?v={id}"),
            });
        }
        self.source.url.clone()
    }
}

/// "12:40" or "1:02:03" → seconds.
fn timestamp_seconds(locator: &str) -> Option<u32> {
    let first = locator.split(['–', '-', ';', ',', ' ']).next()?;
    let mut total = 0u32;
    let mut parts = 0;
    for part in first.split(':') {
        total = total.checked_mul(60)?.checked_add(part.parse().ok()?)?;
        parts += 1;
    }
    (parts >= 2).then_some(total)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrontMatter {
    pub title: String,
    /// One sentence for the page description and the index.
    pub description: String,
    /// Taxonomy topic (research/CONVENTIONS.md), e.g. `starch` or `scalds.yudane`.
    pub topic: String,
    /// Position within the topic.
    #[serde(default)]
    pub order: i32,
    /// Last substantive update, YYYY-MM-DD.
    pub updated: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub slug: String,
    pub lang: &'static str,
    pub front: FrontMatter,
    /// Rendered body.
    pub html: String,
    /// Plain text of the body, for search.
    pub text: String,
    /// Citation keys in order of first appearance; `[n]` in the body is `citations[n-1]`.
    pub citations: Vec<String>,
    /// Site-internal link targets in the body.
    pub links: Vec<String>,
    /// The body's sections (`##` headings), for the page's table of contents.
    pub headings: Vec<Heading>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Translated {
    pub fr: String,
    pub en: String,
}

impl Translated {
    pub fn get(&self, lang: &str) -> &str {
        if lang == "fr" { &self.fr } else { &self.en }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(skip)]
    pub slug: String,
    pub title: Translated,
    pub description: Translated,
    /// `lean-hearth`, `pan`, `enriched`, `pizza`, `flatbread`, `rye`…
    pub bread_type: String,
    /// Citation keys (bibliography or claims) the recipe comes from.
    pub sources: Vec<String>,
    pub formula: breadator_engine::formula::Formula,
    #[serde(default)]
    pub steps: Vec<Translated>,
    #[serde(default)]
    pub notes: Option<Translated>,
}

#[derive(Debug, Default)]
pub struct Library {
    /// Keyed by (slug, lang).
    articles: BTreeMap<(String, &'static str), Article>,
    recipes: BTreeMap<String, Recipe>,
    references: HashMap<String, Reference>,
    claims: HashMap<String, Claim>,
    videos: HashMap<String, Video>,
}

impl Library {
    /// Loads and validates every content and research file given. Paths decide the role:
    /// `content/articles/<slug>/<lang>.md`, `content/recipes/<slug>.toml`,
    /// `research/data/bibliography.json`, `research/data/claims.jsonl`,
    /// `research/data/videos.json`; others are ignored.
    pub fn load<'a>(
        files: impl IntoIterator<Item = SourceFile<'a>>,
    ) -> Result<Self, Vec<ContentError>> {
        let mut lib = Library::default();
        let mut errors = Vec::new();
        let mut err = |path: &str, message: String| {
            errors.push(ContentError { path: path.to_owned(), message })
        };
        let mut article_sources = Vec::new();
        let mut recipe_sources = Vec::new();

        for file in files {
            let path = file.path.replace('\\', "/");
            let parts: Vec<&str> = path.split('/').collect();
            match parts.as_slice() {
                ["research", "data", "bibliography.json"] => {
                    match serde_json::from_str::<Vec<Reference>>(file.text) {
                        Ok(refs) => {
                            for r in refs {
                                if lib.references.insert(r.key.clone(), r.clone()).is_some() {
                                    err(&path, format!("duplicate bibliography key '{}'", r.key));
                                }
                            }
                        }
                        Err(e) => err(&path, e.to_string()),
                    }
                }
                ["research", "data", "videos.json"] => {
                    match serde_json::from_str::<HashMap<String, Video>>(file.text) {
                        Ok(v) => lib.videos = v,
                        Err(e) => err(&path, e.to_string()),
                    }
                }
                ["research", "data", "claims.jsonl"] => {
                    for (n, line) in
                        file.text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty())
                    {
                        match serde_json::from_str::<Claim>(line) {
                            Ok(c) => {
                                if lib.claims.insert(c.id.clone(), c.clone()).is_some() {
                                    err(
                                        &path,
                                        format!("line {}: duplicate claim id '{}'", n + 1, c.id),
                                    );
                                }
                            }
                            Err(e) => err(&path, format!("line {}: {e}", n + 1)),
                        }
                    }
                }
                ["content", "articles", slug, file_name] => match file_name.strip_suffix(".md") {
                    Some(lang) => match LANGS.iter().find(|l| **l == lang) {
                        Some(lang) => {
                            article_sources.push((path.clone(), slug.to_string(), *lang, file.text))
                        }
                        None => err(&path, format!("unknown language '{lang}'")),
                    },
                    None => err(&path, "articles are Markdown files named <lang>.md".into()),
                },
                ["content", "recipes", file_name] => match file_name.strip_suffix(".toml") {
                    Some(slug) => recipe_sources.push((path.clone(), slug.to_string(), file.text)),
                    None => err(&path, "recipes are TOML files".into()),
                },
                _ => {}
            }
        }

        for (path, slug, lang, text) in article_sources {
            match parse_article(&slug, lang, text) {
                Ok(a) => {
                    lib.articles.insert((slug, lang), a);
                }
                Err(message) => err(&path, message),
            }
        }
        for (path, slug, text) in recipe_sources {
            match toml::from_str::<Recipe>(text) {
                Ok(mut r) => {
                    r.slug = slug.clone();
                    if let Err(e) = breadator_engine::formula::solve(&r.formula) {
                        err(&path, format!("formula: {e}"));
                    }
                    lib.recipes.insert(slug, r);
                }
                Err(e) => err(&path, e.to_string()),
            }
        }
        lib.validate(&mut err);
        if errors.is_empty() { Ok(lib) } else { Err(errors) }
    }

    fn validate(&self, err: &mut impl FnMut(&str, String)) {
        let known = |key: &str| self.references.contains_key(key) || self.claims.contains_key(key);
        for ((slug, lang), a) in &self.articles {
            let path = format!("content/articles/{slug}/{lang}.md");
            for key in &a.citations {
                if !known(key) {
                    err(&path, format!("unknown citation '{key}'"));
                }
            }
            for link in &a.links {
                if let Some(problem) = self.check_link(lang, link) {
                    err(&path, format!("link '{link}': {problem}"));
                }
            }
            for other in LANGS.iter().filter(|l| *l != lang) {
                match self.articles.get(&(slug.clone(), *other)) {
                    None => err(&path, format!("missing the '{other}' version")),
                    Some(b)
                        if (b.front.topic.as_str(), b.front.order)
                            != (a.front.topic.as_str(), a.front.order) =>
                    {
                        err(&path, format!("topic/order differ from the '{other}' version"))
                    }
                    Some(b) if b.citations.len() != a.citations.len() => err(
                        &path,
                        format!(
                            "cites {} sources, the '{other}' version {}",
                            a.citations.len(),
                            b.citations.len()
                        ),
                    ),
                    _ => {}
                }
            }
        }
        for (slug, r) in &self.recipes {
            let path = format!("content/recipes/{slug}.toml");
            if r.sources.is_empty() {
                err(&path, "a recipe needs at least one source".into());
            }
            for key in &r.sources {
                if !known(key) {
                    err(&path, format!("unknown source '{key}'"));
                }
            }
        }
        for c in self.claims.values() {
            for key in &c.cites {
                if !self.references.contains_key(key) && !key.starts_with("doi:") {
                    err(&format!("claim {}", c.id), format!("cites unknown reference '{key}'"));
                }
            }
        }
    }

    /// Problem with a site-internal link of an article in `lang`, if any.
    fn check_link(&self, lang: &str, link: &str) -> Option<String> {
        let path = link.split(['#', '?']).next().unwrap_or(link);
        let parts: Vec<&str> = path.trim_end_matches('/').split('/').skip(1).collect();
        if parts.first() != Some(&lang) {
            return Some(format!("must start with /{lang}/"));
        }
        match parts[1..] {
            ["learn", slug] => {
                let exists = self.articles.keys().any(|(s, l)| s == slug && *l == lang);
                (!exists).then(|| format!("no article '{slug}'"))
            }
            ["recipes", slug] => {
                (!self.recipes.contains_key(slug)).then(|| format!("no recipe '{slug}'"))
            }
            ["calculators", c] if CALCULATORS.contains(&c) => None,
            [] | ["learn"] | ["recipes"] | ["sources"] | ["search"] => None,
            _ => Some("unknown page".into()),
        }
    }

    pub fn article(&self, slug: &str, lang: &str) -> Option<&Article> {
        let lang = LANGS.iter().find(|l| **l == lang)?;
        self.articles.get(&(slug.to_owned(), *lang))
    }

    /// Articles of one language, grouped by topic in (topic, order, title) order.
    pub fn articles(&self, lang: &str) -> Vec<&Article> {
        let mut v: Vec<&Article> = self.articles.values().filter(|a| a.lang == lang).collect();
        v.sort_by(|a, b| {
            (a.front.topic.as_str(), a.front.order, a.front.title.as_str()).cmp(&(
                b.front.topic.as_str(),
                b.front.order,
                b.front.title.as_str(),
            ))
        });
        v
    }

    pub fn recipe(&self, slug: &str) -> Option<&Recipe> {
        self.recipes.get(slug)
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.values()
    }

    pub fn reference(&self, key: &str) -> Option<&Reference> {
        self.references.get(key)
    }

    pub fn claim(&self, id: &str) -> Option<&Claim> {
        self.claims.get(id)
    }

    /// Title, channel and date of a YouTube video cited by a claim (`youtube:<id>`).
    pub fn video(&self, id: &str) -> Option<&Video> {
        self.videos.get(id)
    }

    /// Every bibliography entry, sorted by first author then year.
    pub fn bibliography(&self) -> Vec<&Reference> {
        let mut v: Vec<&Reference> = self.references.values().collect();
        v.sort_by(|a, b| {
            (a.authors.first(), a.year, &a.title).cmp(&(b.authors.first(), b.year, &b.title))
        });
        v
    }

    pub fn claims(&self) -> impl Iterator<Item = &Claim> {
        self.claims.values()
    }
}

fn parse_article(slug: &str, lang: &'static str, text: &str) -> Result<Article, String> {
    if !slug.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-') {
        return Err("slugs use lowercase letters, digits and dashes".into());
    }
    let text = text.strip_prefix('\u{feff}').unwrap_or(text).replace("\r\n", "\n");
    let rest = text.strip_prefix("+++\n").ok_or("front matter must start with a '+++' line")?;
    let (front, body) =
        rest.split_once("\n+++\n").ok_or("front matter must end with a '+++' line")?;
    let front: FrontMatter = toml::from_str(front).map_err(|e| format!("front matter: {e}"))?;
    let rendered = markdown::render(body);
    Ok(Article {
        slug: slug.to_owned(),
        lang,
        front,
        html: rendered.html,
        text: rendered.text,
        citations: rendered.citations,
        links: rendered.links,
        headings: rendered.headings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BIB: &str = r#"[{"key": "zeng2023", "authors": ["Zeng, F."], "year": 2023,
        "title": "Effect of fermentation methods on properties of dough and whole wheat bread",
        "venue": "J. Sci. Food Agric.", "doi": "10.1002/jsfa.12565"}]"#;
    const CLAIMS: &str = r#"{"id": "novita-0001", "topic": "scalds.tangzhong", "claim": "c", "source": {"type": "video", "ref": "youtube:ocBV6-O5SL4", "locator": "12:40"}, "evidence": "measured", "cites": ["zeng2023"]}"#;

    fn article(title: &str, body: &str) -> String {
        format!(
            "+++\ntitle = \"{title}\"\ndescription = \"d\"\ntopic = \"starch\"\norder = 1\nupdated = \"2026-11-24\"\n+++\n{body}"
        )
    }

    fn load(files: &[(&str, &str)]) -> Result<Library, Vec<ContentError>> {
        Library::load(files.iter().map(|(path, text)| SourceFile { path, text }))
    }

    #[test]
    fn valid_library_loads_and_resolves_citations() {
        let fr = article(
            "Amidon",
            "La rétrogradation [@zeng2023]. Voir aussi [@novita-0001; @zeng2023].",
        );
        let en =
            article("Starch", "Retrogradation [@zeng2023]. See also [@novita-0001; @zeng2023].");
        let lib = load(&[
            ("research/data/bibliography.json", BIB),
            ("research/data/claims.jsonl", CLAIMS),
            ("content/articles/starch/fr.md", &fr),
            ("content/articles/starch/en.md", &en),
        ])
        .unwrap();
        let a = lib.article("starch", "en").unwrap();
        assert_eq!(a.citations, vec!["zeng2023", "novita-0001"]);
        assert!(a.html.contains(r##"<a href="#cite-1""##), "{}", a.html);
        assert!(a.html.contains(r##"<a href="#cite-2""##), "{}", a.html);
        assert_eq!(
            lib.claim("novita-0001").unwrap().link().unwrap(),
            "https://www.youtube.com/watch?v=ocBV6-O5SL4&t=760s"
        );
        assert_eq!(
            lib.reference("zeng2023").unwrap().link().unwrap(),
            "https://doi.org/10.1002/jsfa.12565"
        );
    }

    #[test]
    fn every_problem_is_reported() {
        let fr = article(
            "A",
            "Texte [@inconnu]. [Voir](/fr/learn/absent), [ici](/en/calculators/formula), \
             [calcul](/fr/calculators/formula).",
        );
        let errors = load(&[
            ("research/data/bibliography.json", BIB),
            ("content/articles/starch/fr.md", &fr),
            ("content/articles/Bad_Slug/en.md", &article("B", "x")),
            ("content/recipes/r.toml", "title = 1"),
        ])
        .unwrap_err();
        let text: Vec<String> = errors.iter().map(ToString::to_string).collect();
        let has = |needle: &str| text.iter().any(|e| e.contains(needle));
        assert!(has("unknown citation 'inconnu'"), "{text:?}");
        assert!(has("missing the 'en' version"), "{text:?}");
        assert!(has("slugs use lowercase"), "{text:?}");
        assert!(has("content/recipes/r.toml"), "{text:?}");
        assert!(has("link '/fr/learn/absent': no article 'absent'"), "{text:?}");
        assert!(has("link '/en/calculators/formula': must start with /fr/"), "{text:?}");
        assert!(!has("'/fr/calculators/formula'"), "{text:?}");
    }

    #[test]
    fn recipes_are_parsed_and_their_formula_checked() {
        let recipe = r#"
bread_type = "lean-hearth"
sources = ["zeng2023"]
title = { fr = "Pain", en = "Bread" }
description = { fr = "d", en = "d" }
steps = [{ fr = "Pétrir", en = "Mix" }]
[formula]
flours = [{ id = "t65", pct = 100 }]
ingredients = [{ id = "water", kind = "water", pct = 70 }, { id = "salt", kind = "salt", pct = 2 }]
target = { by = "dough", grams = 1000 }
"#;
        let lib = load(&[
            ("research/data/bibliography.json", BIB),
            ("content/recipes/pain.toml", recipe),
        ])
        .unwrap();
        assert_eq!(lib.recipe("pain").unwrap().title.get("fr"), "Pain");
        let bad = recipe.replace("pct = 100", "pct = 90");
        let errors =
            load(&[("research/data/bibliography.json", BIB), ("content/recipes/pain.toml", &bad)])
                .unwrap_err();
        assert!(errors[0].message.starts_with("formula:"), "{errors:?}");
    }

    #[test]
    fn timestamps_parse() {
        assert_eq!(timestamp_seconds("12:40"), Some(760));
        assert_eq!(timestamp_seconds("1:02:03–1:05:00"), Some(3723));
        assert_eq!(timestamp_seconds("description"), None);
    }
}
