//! Full-text search over articles, recipes and research claims (SQLite FTS5, in memory).
//!
//! Built once at start-up from the [`Library`]. Used by the search page and by the chatbot's
//! `search_knowledge` tool, which needs passages with their citation keys.

use std::sync::Mutex;

use rusqlite::{Connection, params};
use serde::Serialize;

use crate::Library;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Hit {
    /// `article`, `recipe` or `claim`.
    pub kind: String,
    /// Article or recipe slug, or claim id.
    pub id: String,
    pub lang: String,
    pub title: String,
    /// Excerpt around the match; matched terms are wrapped in `[` `]`.
    pub snippet: String,
    /// Citation keys supporting this passage (the article's citations, the recipe's sources, or
    /// the claim id itself).
    pub citations: Vec<String>,
}

pub struct Index {
    db: Mutex<Connection>,
}

#[derive(Debug, thiserror::Error)]
#[error("search index: {0}")]
pub struct SearchError(#[from] rusqlite::Error);

impl Index {
    pub fn build(lib: &Library) -> Result<Self, SearchError> {
        let db = Connection::open_in_memory()?;
        // unicode61 with diacritics removed: "levure" finds "levûre" typos and vice versa.
        db.execute_batch(
            "CREATE VIRTUAL TABLE docs USING fts5(kind UNINDEXED, id UNINDEXED, lang UNINDEXED, \
             title, body, citations UNINDEXED, tokenize = 'unicode61 remove_diacritics 2');",
        )?;
        {
            let mut insert = db.prepare("INSERT INTO docs VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
            for lang in crate::LANGS {
                for a in lib.articles(lang) {
                    let body = format!("{}\n{}", a.front.description, a.text);
                    insert.execute(params![
                        "article",
                        a.slug,
                        lang,
                        a.front.title,
                        body,
                        a.citations.join(" ")
                    ])?;
                }
                for r in lib.recipes() {
                    let steps: Vec<&str> = r.steps.iter().map(|s| s.get(lang)).collect();
                    let body = format!("{}\n{}", r.description.get(lang), steps.join("\n"));
                    insert.execute(params![
                        "recipe",
                        r.slug,
                        lang,
                        r.title.get(lang),
                        body,
                        r.sources.join(" ")
                    ])?;
                }
            }
            // Claims are mostly English; indexed once, language "any".
            for c in lib.claims() {
                insert.execute(params!["claim", c.id, "any", c.topic, c.claim, c.id])?;
            }
        }
        Ok(Self { db: Mutex::new(db) })
    }

    /// Up to `limit` hits for a free-text query in `lang` (claims match every language).
    pub fn search(&self, query: &str, lang: &str, limit: usize) -> Result<Vec<Hit>, SearchError> {
        let Some(fts) = fts_query(query) else { return Ok(Vec::new()) };
        let db = self.db.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = db.prepare_cached(
            "SELECT kind, id, lang, title, snippet(docs, 4, '[', ']', '…', 24), citations FROM docs \
             WHERE docs MATCH ?1 AND (lang = ?2 OR lang = 'any') ORDER BY bm25(docs, 0, 0, 0, 5.0, 1.0) LIMIT ?3",
        )?;
        let rows = stmt.query_map(params![fts, lang, limit as i64], |r| {
            let citations: String = r.get(5)?;
            Ok(Hit {
                kind: r.get(0)?,
                id: r.get(1)?,
                lang: r.get(2)?,
                title: r.get(3)?,
                snippet: r.get(4)?,
                citations: citations.split_whitespace().map(str::to_owned).collect(),
            })
        })?;
        Ok(rows.collect::<Result<_, _>>()?)
    }
}

/// User text → FTS5 query: words quoted (no operator injection), prefix-matched, ANDed.
fn fts_query(query: &str) -> Option<String> {
    let terms: Vec<String> = query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() >= 2)
        .take(12)
        .map(|w| format!("\"{}\"*", w.to_lowercase()))
        .collect();
    (!terms.is_empty()).then(|| terms.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceFile;

    #[test]
    fn finds_articles_by_language_and_claims_everywhere() {
        let bib = r#"[{"key": "k", "title": "t"}]"#;
        let claims = r#"{"id": "c1", "topic": "starch", "claim": "Yudane gelatinizes most of the starch", "source": {"type": "video", "ref": "youtube:x"}, "evidence": "measured"}"#;
        let fr = "+++\ntitle = \"L'amidon\"\ndescription = \"Gélatinisation\"\ntopic = \"starch\"\nupdated = \"2026-11-24\"\n+++\nLe yudane gélatinise l'amidon [@k].";
        let en = "+++\ntitle = \"Starch\"\ndescription = \"Gelatinization\"\ntopic = \"starch\"\nupdated = \"2026-11-24\"\n+++\nYudane gelatinizes starch [@k].";
        let files = [
            ("research/data/bibliography.json", bib),
            ("research/data/claims.jsonl", claims),
            ("content/articles/starch/fr.md", fr),
            ("content/articles/starch/en.md", en),
        ];
        let lib =
            Library::load(files.iter().map(|(path, text)| SourceFile { path, text })).unwrap();
        let index = Index::build(&lib).unwrap();
        let hits = index.search("gelatinise amidon", "fr", 5).unwrap();
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert_eq!(hits[0].id, "starch");
        assert_eq!(hits[0].citations, vec!["k"]);
        let hits = index.search("yudane", "en", 5).unwrap();
        assert_eq!(hits.len(), 2, "{hits:?}");
        assert!(index.search("\" OR *", "en", 5).unwrap().is_empty());
    }
}
