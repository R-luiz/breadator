//! Passages of an article for the model: its paragraphs cut where the article cites, so that
//! each statement keeps exactly the sources the article gives for it.
//!
//! The library keeps an article as rendered HTML, where `[@key]` became
//! `<sup class="cite">[<a href="#cite-3">3</a>]</sup>` (numbers index the article's citation
//! list). That markup is regular (it comes from the Markdown renderer), so a small scanner is
//! enough to get paragraphs, headings and the keys cited after each statement.

/// A paragraph, list, table or quote of an article, under its section heading.
#[derive(Debug, PartialEq)]
pub struct Block {
    pub heading: String,
    pub segments: Vec<Segment>,
}

/// A stretch of text and the keys cited right after it (none for an uncited tail).
#[derive(Debug, PartialEq)]
pub struct Segment {
    pub text: String,
    pub keys: Vec<String>,
}

const CITE_OPEN: &str = r#"<sup class="cite">"#;

/// Lowercase without diacritics, as the search index compares words.
pub fn fold(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'â' | 'ä' | 'á' | 'ã' => out.push('a'),
            'é' | 'è' | 'ê' | 'ë' => out.push('e'),
            'î' | 'ï' | 'í' => out.push('i'),
            'ô' | 'ö' | 'ó' => out.push('o'),
            'ù' | 'û' | 'ü' | 'ú' => out.push('u'),
            'ç' => out.push('c'),
            'ñ' => out.push('n'),
            'œ' => out.push_str("oe"),
            'æ' => out.push_str("ae"),
            'ß' => out.push_str("ss"),
            c => out.push(c),
        }
    }
    out
}

fn unescape(text: &str, out: &mut String) {
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let entity = [
            ("&amp;", '&'),
            ("&lt;", '<'),
            ("&gt;", '>'),
            ("&quot;", '"'),
            ("&#39;", '\''),
            ("&#x27;", '\''),
            ("&apos;", '\''),
        ]
        .into_iter()
        .find(|(name, _)| rest.starts_with(name));
        match entity {
            Some((name, c)) => {
                out.push(c);
                rest = &rest[name.len()..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
}

/// Keys of the citation numbers linked inside a `<sup class="cite">`.
fn cite_keys(sup: &str, citations: &[String]) -> Vec<String> {
    sup.split("#cite-")
        .skip(1)
        .filter_map(|after| {
            let digits = after.find(|c: char| !c.is_ascii_digit()).unwrap_or(after.len());
            let n: usize = after[..digits].parse().ok()?;
            citations.get(n.checked_sub(1)?).cloned()
        })
        .collect()
}

/// Spaces collapsed, lines (list items, table rows) kept.
fn tidy(text: &str) -> String {
    let lines: Vec<String> = text
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect();
    lines.join("\n")
}

fn segments(inner: &str, citations: &[String]) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut rest = inner;
    while let Some(lt) = rest.find('<') {
        unescape(&rest[..lt], &mut text);
        let tag = &rest[lt..];
        if let Some(sup) = tag.strip_prefix(CITE_OPEN) {
            let end = sup.find("</sup>").unwrap_or(sup.len());
            let keys = cite_keys(&sup[..end], citations);
            if !keys.is_empty() {
                out.push(Segment { text: std::mem::take(&mut text), keys });
            }
            rest = sup.get(end + "</sup>".len()..).unwrap_or("");
            continue;
        }
        let close = tag.find('>').map_or(tag.len(), |i| i + 1);
        // `</thead>` is not `</th>`: the whole name, without attributes or a self-closing slash.
        let name = tag[1..close]
            .split(|c: char| c == '>' || c.is_whitespace())
            .next()
            .unwrap_or("")
            .trim_end_matches('/');
        if matches!(name, "/li" | "/tr" | "/p" | "br") {
            text.push('\n');
        } else if matches!(name, "/td" | "/th") {
            text.push_str(" | ");
        }
        rest = &tag[close..];
    }
    unescape(rest, &mut text);
    // A tail of punctuation or table bars says nothing.
    if text.chars().any(char::is_alphanumeric) {
        out.push(Segment { text, keys: Vec::new() });
    }
    for segment in &mut out {
        segment.text = tidy(&segment.text);
    }
    out
}

/// End of the element opened by the tag `name` whose content starts at `html`: (content length,
/// length including the closing tag). Nested elements of the same name are skipped over.
fn element_end(html: &str, name: &str) -> (usize, usize) {
    let (open, close) = (format!("<{name}"), format!("</{name}>"));
    let mut depth = 1;
    let mut at = 0;
    while let Some(lt) = html[at..].find('<') {
        let here = at + lt;
        let tail = &html[here..];
        if tail.starts_with(&close) {
            depth -= 1;
            if depth == 0 {
                return (here, here + close.len());
            }
        } else if tail.strip_prefix(&open).is_some_and(|after| after.starts_with(['>', ' '])) {
            depth += 1;
        }
        at = here + 1;
    }
    (html.len(), html.len())
}

/// The blocks of an article's HTML; `citations` is the article's citation list.
pub fn blocks(html: &str, citations: &[String]) -> Vec<Block> {
    let mut out = Vec::new();
    let mut heading = String::new();
    let mut rest = html;
    while let Some(lt) = rest.find('<') {
        let tag = &rest[lt + 1..];
        let name_len = tag.find(|c: char| !c.is_ascii_alphanumeric()).unwrap_or(tag.len());
        let name = &tag[..name_len];
        let Some(open_end) = tag.find('>') else { break };
        let content = &tag[open_end + 1..];
        let is_heading = name.len() == 2 && name.starts_with('h') && name != "hr";
        if !is_heading && !["p", "ul", "ol", "table", "blockquote", "pre"].contains(&name) {
            rest = content;
            continue;
        }
        let (inner, total) = element_end(content, name);
        let parts = segments(&content[..inner], citations);
        if is_heading {
            heading = parts.into_iter().map(|s| s.text).collect::<Vec<_>>().join(" ");
        } else if !parts.is_empty() {
            out.push(Block { heading: heading.clone(), segments: parts });
        }
        rest = &content[total..];
    }
    out
}

impl Block {
    /// How many of `terms` (folded) start a word of the block or of its heading.
    fn score(&self, terms: &[String]) -> usize {
        let mut text = fold(&self.heading);
        for segment in &self.segments {
            text.push(' ');
            text.push_str(&fold(&segment.text));
        }
        terms
            .iter()
            .filter(|term| {
                text.match_indices(term.as_str()).any(|(at, _)| {
                    !text[..at].chars().next_back().is_some_and(char::is_alphanumeric)
                })
            })
            .count()
    }
}

/// The `max` blocks that match the most of `terms`, in the article's order; the first block when
/// none matches (the search matched the title or the description).
pub fn best<'a>(blocks: &'a [Block], terms: &[String], max: usize) -> Vec<&'a Block> {
    let mut scored: Vec<(usize, usize)> = blocks
        .iter()
        .enumerate()
        .map(|(i, block)| (block.score(terms), i))
        .filter(|(score, _)| *score > 0)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored.truncate(max);
    let mut chosen: Vec<usize> = scored.into_iter().map(|(_, i)| i).collect();
    if chosen.is_empty() && !blocks.is_empty() {
        chosen.push(0);
    }
    chosen.sort_unstable();
    chosen.into_iter().map(|i| &blocks[i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_html_becomes_cited_segments() {
        let rendered = breadator_content::render_markdown(
            "Intro without source.\n\n## Autolyse & repos\n\nCalvel l'a décrite en 1974 [@calvel2001]. \
             Les gains de *goût* sont faibles [@novita-0105; @calvel2001]. Fin sans source.\n\n\
             - Rabats doux [@breadcode-0828]\n- Pâte \"ferme\" <b>\n\n| a | b |\n|---|---|\n| 1 | 2 [@yang2019] |\n",
        );
        let blocks = blocks(&rendered.html, &rendered.citations);
        let seg = |text: &str, keys: &[&str]| Segment {
            text: text.to_owned(),
            keys: keys.iter().map(|k| (*k).to_owned()).collect(),
        };
        assert_eq!(blocks.len(), 4, "{blocks:#?}");
        assert_eq!(
            blocks[0],
            Block { heading: String::new(), segments: vec![seg("Intro without source.", &[])] }
        );
        assert_eq!(blocks[1].heading, "Autolyse & repos");
        assert_eq!(
            blocks[1].segments,
            vec![
                seg("Calvel l'a décrite en 1974", &["calvel2001"]),
                seg(". Les gains de goût sont faibles", &["novita-0105", "calvel2001"]),
                seg(". Fin sans source.", &[]),
            ]
        );
        assert_eq!(
            blocks[2].segments,
            vec![seg("Rabats doux", &["breadcode-0828"]), seg("Pâte \"ferme\" <b>", &[])]
        );
        assert_eq!(blocks[3].segments[0], seg("a | b |\n1 | 2", &["yang2019"]));
    }

    #[test]
    fn best_blocks_match_whole_word_starts_without_accents() {
        let rendered = breadator_content::render_markdown(
            "Le pétrissage hydrate la farine.\n\n## Rabats\n\nUn rabat égalise les bulles.\n\n\
             ## Température\n\nLa pâte chauffe au pétrissage ; la température monte.\n",
        );
        let blocks = blocks(&rendered.html, &rendered.citations);
        let terms = |words: &[&str]| words.iter().map(|w| fold(w)).collect::<Vec<_>>();
        let texts = |chosen: Vec<&Block>| -> Vec<String> {
            chosen.iter().map(|b| b.segments[0].text.clone()).collect()
        };
        assert_eq!(
            texts(best(&blocks, &terms(&["Petrissage", "température"]), 1)),
            vec!["La pâte chauffe au pétrissage ; la température monte."]
        );
        // Headings count; "bat" is inside "rabat", not the start of a word.
        assert_eq!(
            texts(best(&blocks, &terms(&["rabats"]), 2)),
            vec!["Un rabat égalise les bulles."]
        );
        assert_eq!(
            texts(best(&blocks, &terms(&["bat"]), 2)),
            vec!["Le pétrissage hydrate la farine."]
        );
    }
}
