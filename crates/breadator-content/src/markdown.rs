//! Markdown → HTML with numbered citations.
//!
//! `[@a]` and `[@a; @b]` become superscript links `[1]`, `[1, 2]` to `#cite-<n>` entries that the
//! page lists under the article, numbered in order of first appearance. Raw HTML in the source
//! is escaped and shown as text: content can never inject markup or script. Headings get an
//! `id` from their text, so that a section can be linked to.

use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};

/// A section heading of the body (`##`), for the page's table of contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub id: String,
    pub text: String,
}

pub struct Rendered {
    pub html: String,
    pub text: String,
    pub citations: Vec<String>,
    /// Site-internal link targets (`/en/learn/…`), checked by the library.
    pub links: Vec<String>,
    /// The `##` headings in order.
    pub headings: Vec<Heading>,
}

/// "Levain : une croissance…" → `levain-une-croissance`: lowercase ASCII letters and digits,
/// the usual Latin diacritics folded, everything else a single hyphen.
fn slug(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        let folded = match c {
            'à' | 'â' | 'ä' | 'á' | 'ã' => "a",
            'é' | 'è' | 'ê' | 'ë' => "e",
            'î' | 'ï' | 'í' => "i",
            'ô' | 'ö' | 'ó' => "o",
            'ù' | 'û' | 'ü' | 'ú' => "u",
            'ç' => "c",
            'ñ' => "n",
            'œ' => "oe",
            'æ' => "ae",
            'ß' => "ss",
            c if c.is_ascii_alphanumeric() => {
                out.push(c);
                continue;
            }
            _ => "-",
        };
        if folded != "-" || !(out.is_empty() || out.ends_with('-')) {
            out.push_str(folded);
        }
    }
    let trimmed = out.trim_end_matches('-');
    if trimmed.is_empty() { "section".to_owned() } else { trimmed.to_owned() }
}

/// Splits `text` into plain parts and citation groups `[@k1; @k2]`.
fn split_citations(text: &str) -> Vec<Result<&str, Vec<&str>>> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[@") {
        let Some(len) = rest[start..].find(']') else { break };
        let inner = &rest[start + 1..start + len];
        let keys: Option<Vec<&str>> =
            inner.split(';').map(|k| k.trim().strip_prefix('@').filter(|k| valid_key(k))).collect();
        match keys {
            Some(keys) if !keys.is_empty() => {
                if start > 0 {
                    out.push(Ok(&rest[..start]));
                }
                out.push(Err(keys));
                rest = &rest[start + len + 1..];
            }
            _ => {
                out.push(Ok(&rest[..start + 2]));
                rest = &rest[start + 2..];
            }
        }
    }
    if !rest.is_empty() {
        out.push(Ok(rest));
    }
    out
}

fn valid_key(k: &str) -> bool {
    !k.is_empty()
        && k.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b':' | b'.'))
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn render(source: &str) -> Rendered {
    let mut citations: Vec<String> = Vec::new();
    let mut links: Vec<String> = Vec::new();
    let mut headings: Vec<Heading> = Vec::new();
    // Ids taken so far; those of the article page itself are reserved.
    let mut ids: Vec<String> =
        ["main", "toc-heading", "sources-heading"].map(str::to_owned).to_vec();
    // The heading being read: index of its start event, and where its text starts in `text`.
    let mut open_heading: Option<(usize, usize)> = None;
    let mut text = String::new();
    let mut events: Vec<Event> = Vec::new();
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_STRIKETHROUGH;
    // Text events can be split (e.g. around brackets); merge consecutive ones before scanning.
    let mut pending = String::new();
    let flush = |pending: &mut String,
                 events: &mut Vec<Event>,
                 citations: &mut Vec<String>,
                 text: &mut String| {
        if pending.is_empty() {
            return;
        }
        for part in split_citations(pending) {
            match part {
                Ok(t) => {
                    text.push_str(t);
                    events.push(Event::Text(CowStr::from(t.to_owned())));
                }
                Err(keys) => {
                    let numbers: Vec<String> = keys
                        .iter()
                        .map(|k| {
                            let n = match citations.iter().position(|c| c == k) {
                                Some(i) => i + 1,
                                None => {
                                    citations.push((*k).to_owned());
                                    citations.len()
                                }
                            };
                            format!(r##"<a href="#cite-{n}">{n}</a>"##)
                        })
                        .collect();
                    events.push(Event::InlineHtml(CowStr::from(format!(
                        r#"<sup class="cite">[{}]</sup>"#,
                        numbers.join(", ")
                    ))));
                }
            }
        }
        pending.clear();
    };
    for event in Parser::new_ext(source, options) {
        match event {
            Event::Text(t) => pending.push_str(&t),
            other => {
                flush(&mut pending, &mut events, &mut citations, &mut text);
                match other {
                    // Raw HTML is shown, never interpreted.
                    Event::Html(h) | Event::InlineHtml(h) => events.push(Event::Text(h)),
                    Event::Code(c) => {
                        text.push_str(&c);
                        events.push(Event::Code(c));
                    }
                    Event::SoftBreak | Event::HardBreak => {
                        text.push(' ');
                        events.push(other);
                    }
                    // A table wider than a phone scrolls inside its own box, not the page.
                    Event::Start(Tag::Table(_)) => {
                        events.push(Event::Html(CowStr::from(r#"<div class="table-scroll">"#)));
                        events.push(other);
                    }
                    Event::End(TagEnd::Table) => {
                        events.push(other);
                        events.push(Event::Html(CowStr::from("</div>")));
                    }
                    Event::Start(Tag::Heading { .. }) => {
                        open_heading = Some((events.len(), text.len()));
                        events.push(other);
                    }
                    Event::End(TagEnd::Heading(_)) => {
                        if let Some((at, from)) = open_heading.take() {
                            let title = text[from..].trim().to_owned();
                            // Two sections with the same title: the second id gets a number.
                            // `cite-<n>` belongs to the page's source list.
                            let base = match slug(&title) {
                                s if s.starts_with("cite-") => format!("section-{s}"),
                                s => s,
                            };
                            let mut id = base.clone();
                            let mut n = 2;
                            while ids.contains(&id) {
                                id = format!("{base}-{n}");
                                n += 1;
                            }
                            ids.push(id.clone());
                            if let Event::Start(Tag::Heading { level, id: slot, .. }) =
                                &mut events[at]
                            {
                                *slot = Some(CowStr::from(id.clone()));
                                if *level == HeadingLevel::H2 {
                                    headings.push(Heading { id, text: title });
                                }
                            }
                        }
                        text.push('\n');
                        events.push(other);
                    }
                    Event::End(TagEnd::Paragraph | TagEnd::Item) => {
                        text.push('\n');
                        events.push(other);
                    }
                    // Links to other sites open with no referrer and no opener.
                    Event::Start(Tag::Link { dest_url, title, .. })
                        if dest_url.starts_with("http://") || dest_url.starts_with("https://") =>
                    {
                        events.push(Event::InlineHtml(CowStr::from(format!(
                            r#"<a href="{}" title="{}" rel="noopener noreferrer">"#,
                            escape(&dest_url),
                            escape(&title)
                        ))));
                    }
                    Event::Start(Tag::Link { ref dest_url, .. }) => {
                        if dest_url.starts_with('/') {
                            links.push(dest_url.to_string());
                        }
                        events.push(other);
                    }
                    other => events.push(other),
                }
            }
        }
    }
    flush(&mut pending, &mut events, &mut citations, &mut text);
    let mut out = String::with_capacity(source.len() * 3 / 2);
    html::push_html(&mut out, events.into_iter());
    Rendered { html: out, text, citations, links, headings }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn citations_are_numbered_by_first_appearance() {
        let r =
            render("A [@b]. B [@a; @b]. Not a citation: [@ x] and [link](https://example.org).");
        assert_eq!(r.citations, vec!["b", "a"]);
        assert!(
            r.html.contains(r##"<sup class="cite">[<a href="#cite-1">1</a>]</sup>"##),
            "{}",
            r.html
        );
        assert!(
            r.html.contains(r##"[<a href="#cite-2">2</a>, <a href="#cite-1">1</a>]"##),
            "{}",
            r.html
        );
        assert!(r.html.contains("[@ x]"), "{}", r.html);
        assert!(r.html.contains(r#"rel="noopener noreferrer""#), "{}", r.html);
    }

    #[test]
    fn raw_html_is_escaped() {
        let r = render("<script>alert(1)</script>\n\nText <b>bold</b>");
        assert!(!r.html.contains("<script>"), "{}", r.html);
        assert!(!r.html.contains("<b>"), "{}", r.html);
        assert!(r.html.contains("&lt;script&gt;"), "{}", r.html);
    }

    #[test]
    fn headings_get_unique_ids_and_h2_are_listed() {
        let r = render(
            "## Levain : une croissance à partir de la farine\n\nA.\n\n### Détail\n\nB.\n\n## Levain : une croissance à partir de la farine\n\nC.\n\n## 65 °C ou 75–80 °C ?\n",
        );
        assert!(
            r.html.contains(r#"<h2 id="levain-une-croissance-a-partir-de-la-farine">"#),
            "{}",
            r.html
        );
        assert!(r.html.contains(r#"<h3 id="detail">"#), "{}", r.html);
        assert!(
            r.html.contains(r#"<h2 id="levain-une-croissance-a-partir-de-la-farine-2">"#),
            "{}",
            r.html
        );
        let ids: Vec<&str> = r.headings.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "levain-une-croissance-a-partir-de-la-farine",
                "levain-une-croissance-a-partir-de-la-farine-2",
                "65-c-ou-75-80-c"
            ]
        );
        assert_eq!(r.headings[2].text, "65 °C ou 75–80 °C ?");
    }
}
