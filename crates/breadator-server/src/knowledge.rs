//! The knowledge base compiled into the binary: articles and recipes from `content/`, the merged
//! research files, and the full-text index built from them at start-up.

use breadator_content::search::Index;
use breadator_content::{ContentError, Library, SourceFile};
use include_dir::{Dir, include_dir};

static CONTENT: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../../content");
const BIBLIOGRAPHY: &str = include_str!("../../../research/data/bibliography.json");
const CLAIMS: &str = include_str!("../../../research/data/claims.jsonl");
const VIDEOS: &str = include_str!("../../../research/data/videos.json");

pub struct Knowledge {
    pub lib: Library,
    pub index: Index,
}

fn walk(dir: &'static Dir<'static>, out: &mut Vec<(String, &'static str)>) {
    for f in dir.files() {
        if let Some(text) = f.contents_utf8() {
            out.push((format!("content/{}", f.path().to_string_lossy().replace('\\', "/")), text));
        }
    }
    for d in dir.dirs() {
        walk(d, out);
    }
}

impl Knowledge {
    /// Loads and validates everything; the error lists every problem (the server refuses to start).
    pub fn load() -> Result<Self, Vec<ContentError>> {
        let mut files = Vec::new();
        walk(&CONTENT, &mut files);
        files.push(("research/data/bibliography.json".into(), BIBLIOGRAPHY));
        files.push(("research/data/claims.jsonl".into(), CLAIMS));
        files.push(("research/data/videos.json".into(), VIDEOS));
        Self::from_files(&files)
    }

    fn from_files(files: &[(String, &str)]) -> Result<Self, Vec<ContentError>> {
        let lib = Library::load(files.iter().map(|(path, text)| SourceFile { path, text }))?;
        let index = Index::build(&lib).map_err(|e| {
            vec![ContentError { path: "search index".into(), message: e.to_string() }]
        })?;
        Ok(Self { lib, index })
    }

    /// A small, fixed knowledge base for tests, so that tests of other features do not depend on
    /// the repository's content (which writers edit): one article in both languages citing a
    /// paper and a video claim, and one recipe.
    #[cfg(test)]
    pub fn fixture() -> Self {
        const BIB: &str = r#"[{"key": "zeng2023", "authors": ["Zeng, F."], "year": 2023,
            "title": "Effect of fermentation methods on properties of dough and whole wheat bread",
            "venue": "J. Sci. Food Agric.", "doi": "10.1002/jsfa.12565"}]"#;
        const CLAIMS: &str = r#"{"id": "fixture-0001", "topic": "scalds.yudane", "claim": "Yudane is flour mixed with about its weight of boiling water and rested before mixing.", "source": {"type": "video", "ref": "youtube:ocBV6-O5SL4", "locator": "12:40", "author": "Novita Listyani"}, "evidence": "creator-rule", "cites": ["zeng2023"]}"#;
        const VIDEOS: &str =
            r#"{"ocBV6-O5SL4": {"title": "Tangzhong video", "channel": "Novita Listyani"}}"#;
        const RECIPE: &str = r#"
bread_type = "pan"
sources = ["fixture-0001"]
title = { fr = "Pain de mie", en = "Milk bread" }
description = { fr = "Pain de mie au yudane.", en = "Yudane milk bread." }
[formula]
flours = [{ id = "bread_flour", pct = 100 }]
ingredients = [{ id = "water", kind = "water", pct = 70 }, { id = "salt", kind = "salt", pct = 2 }]
target = { by = "dough", grams = 900 }
"#;
        let article = |title: &str, body: &str| {
            format!(
                "+++\ntitle = \"{title}\"\ndescription = \"d\"\ntopic = \"scalds\"\norder = 1\nupdated = \"2026-11-24\"\n+++\n{body}"
            )
        };
        let fr = article(
            "Le yudane",
            "Le yudane ébouillante une partie de la farine [@fixture-0001; @zeng2023].",
        );
        let en = article(
            "Yudane",
            "Yudane scalds part of the flour with boiling water [@fixture-0001; @zeng2023].",
        );
        let files = [
            ("research/data/bibliography.json".to_owned(), BIB),
            ("research/data/claims.jsonl".to_owned(), CLAIMS),
            ("research/data/videos.json".to_owned(), VIDEOS),
            ("content/recipes/milk-bread.toml".to_owned(), RECIPE),
            ("content/articles/yudane/fr.md".to_owned(), fr.as_str()),
            ("content/articles/yudane/en.md".to_owned(), en.as_str()),
        ];
        Self::from_files(&files).unwrap_or_else(|e| panic!("fixture knowledge: {e:?}"))
    }
}

/// How a citation key reads in a bibliography list.
pub struct Citation {
    pub text: String,
    pub href: Option<String>,
    /// `text` without the place in the work: "Creator — video title".
    pub work: String,
    /// The place in the work a claim was taken from ("12:40", "description"), if any.
    pub at: Option<String>,
}

/// "Authors (year). Title. Venue." for references; "Creator — video title, 12:40" for claims.
pub fn cite(lib: &Library, key: &str) -> Citation {
    if let Some(r) = lib.reference(key) {
        let authors = match r.authors.as_slice() {
            [] => String::new(),
            [one] => format!("{one} "),
            [first, ..] => format!("{first} et al. "),
        };
        let year = r.year.map(|y| format!("({y}). ")).unwrap_or_default();
        let venue = r.venue.as_deref().map(|v| format!(" {v}.")).unwrap_or_default();
        let text = format!("{authors}{year}{}.{venue}", r.title.trim_end_matches('.'));
        return Citation { work: text.clone(), text, href: r.link(), at: None };
    }
    if let Some(c) = lib.claim(key) {
        let author = c.source.author.clone().unwrap_or_default();
        let what = match c.source.reference.strip_prefix("youtube:") {
            Some(id) => lib.video(id).map_or_else(|| format!("YouTube {id}"), |v| v.title.clone()),
            None => c.source.reference.clone(),
        };
        // Locators may end with the source file and lines the claim was read from
        // (`…; glossary/glossary.tex:L394-L397`): useful to the corpus, noise to a reader.
        let locator: Vec<&str> = c
            .source
            .locator
            .as_deref()
            .unwrap_or("")
            .split("; ")
            .filter(|part| !part.is_empty() && !part.contains(".tex") && !part.contains(".md:"))
            .collect();
        let lead = if author.is_empty() { String::new() } else { format!("{author} — ") };
        let work = format!("{lead}{what}");
        let at = (!locator.is_empty()).then(|| locator.join("; "));
        let text = at.as_ref().map_or_else(|| work.clone(), |at| format!("{work}, {at}"));
        return Citation { text, href: c.link(), work, at };
    }
    Citation { text: key.to_owned(), href: None, work: key.to_owned(), at: None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_knowledge_is_valid() {
        if let Err(errors) = Knowledge::load() {
            let list: Vec<String> = errors.iter().map(ToString::to_string).collect();
            panic!("{}", list.join("\n"));
        }
    }

    #[test]
    fn fixture_is_searchable_and_cites_read_well() {
        let k = Knowledge::fixture();
        let hits = k.index.search("yudane", "fr", 5).unwrap();
        assert!(hits.iter().any(|h| h.kind == "article" && h.id == "yudane"), "{hits:?}");
        let paper = cite(&k.lib, "zeng2023");
        assert!(
            paper.text.starts_with("Zeng, F. (2023). Effect of fermentation"),
            "{}",
            paper.text
        );
        assert_eq!(paper.href.as_deref(), Some("https://doi.org/10.1002/jsfa.12565"));
        let video = cite(&k.lib, "fixture-0001");
        assert_eq!(video.text, "Novita Listyani — Tangzhong video, 12:40");
        assert_eq!(
            video.href.as_deref(),
            Some("https://www.youtube.com/watch?v=ocBV6-O5SL4&t=760s")
        );
    }
}
