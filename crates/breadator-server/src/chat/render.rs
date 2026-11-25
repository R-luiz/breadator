//! Answers as the reader sees them: the `[n]` markers mapped to the sources the searches
//! returned, the model's Markdown rendered without links, images or raw HTML, and the list of
//! sources under it (cited like an article's, with [`crate::knowledge::cite`]).

use std::fmt::Write as _;

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};

use super::tools::{Source, ToolRecord};
use crate::i18n::Tr;
use crate::knowledge::{Knowledge, cite};

/// The labels an answer marks unsourced statements with, by language.
pub const UNSOURCED_FR: &str = "(non sourcé)";
pub const UNSOURCED_EN: &str = "(unsourced)";

enum Piece<'a> {
    Text(&'a str),
    /// `[1]`, `[1, 3]`, `[2; 4]`: the numbers it cites.
    Marker(Vec<usize>),
}

/// The numbers of a citation marker starting at `s` (which starts with `[`) and its length.
fn marker(s: &str) -> Option<(Vec<usize>, usize)> {
    let close = s.get(1..)?.find(']')? + 1;
    let inner = &s[1..close];
    if inner.is_empty() || inner.len() > 40 {
        return None;
    }
    let numbers = inner
        .split([',', ';'])
        .map(|part| {
            let part = part.trim();
            (!part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
                .then(|| part.parse::<usize>().ok())
                .flatten()
        })
        .collect::<Option<Vec<usize>>>()?;
    Some((numbers, close + 1))
}

fn pieces(text: &str) -> Vec<Piece<'_>> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut at = 0;
    while let Some(open) = text[at..].find('[') {
        let here = at + open;
        match marker(&text[here..]) {
            Some((numbers, len)) => {
                if here > start {
                    out.push(Piece::Text(&text[start..here]));
                }
                // `[1][2]` is one marker citing both.
                match out.last_mut() {
                    Some(Piece::Marker(previous)) if here == start => previous.extend(numbers),
                    _ => out.push(Piece::Marker(numbers)),
                }
                start = here + len;
                at = start;
            }
            None => at = here + 1,
        }
    }
    if start < text.len() {
        out.push(Piece::Text(&text[start..]));
    }
    out
}

/// The unsourced labels models write (`[non sourcé]`, `(Not sourced)`…) as the two the page
/// styles: `(non sourcé)` and `(unsourced)`.
fn normalize_labels(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find(['[', '(']) {
        out.push_str(&rest[..open]);
        let tail = &rest[open..];
        let close = if tail.starts_with('[') { ']' } else { ')' };
        let label = tail.find(close).filter(|end| *end <= 24).and_then(|end| {
            let inner = tail[1..end].trim().to_lowercase();
            let label = match inner.as_str() {
                "non sourcé" | "non-sourcé" | "non sourcée" | "non sourcés" | "non sourcées"
                | "sans source" => UNSOURCED_FR,
                "unsourced" | "not sourced" | "non sourced" | "no source" => UNSOURCED_EN,
                _ => return None,
            };
            Some((label, end + 1))
        });
        match label {
            Some((label, len)) => {
                out.push_str(label);
                rest = &tail[len..];
            }
            None => {
                out.push_str(&tail[..1]);
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The answer as stored: markers renumbered 1, 2… in the order the answer first cites them,
/// numbers no search returned dropped (a marker left citing nothing becomes `unsourced`), and
/// the cited sources under their new numbers.
pub fn finalize(text: &str, sources: &[Source], unsourced: &str) -> (String, Vec<Source>) {
    let text = normalize_labels(text);
    let mut cited: Vec<Source> = Vec::new();
    // (number in the searches, number in the answer)
    let mut renumbered: Vec<(usize, usize)> = Vec::new();
    let mut out = String::with_capacity(text.len());
    for piece in pieces(&text) {
        match piece {
            Piece::Text(t) => out.push_str(t),
            Piece::Marker(numbers) => {
                let mut kept: Vec<usize> = Vec::new();
                for n in numbers {
                    let new = match renumbered.iter().find(|(old, _)| *old == n) {
                        Some((_, new)) => *new,
                        None => {
                            let Some(source) = sources.iter().find(|s| s.n == n) else { continue };
                            let new = cited.len() + 1;
                            cited.push(Source { n: new, ..source.clone() });
                            renumbered.push((n, new));
                            new
                        }
                    };
                    if !kept.contains(&new) {
                        kept.push(new);
                    }
                }
                if kept.is_empty() {
                    out.push_str(unsourced);
                } else {
                    let list: Vec<String> = kept.iter().map(ToString::to_string).collect();
                    let _ = write!(out, "[{}]", list.join(", "));
                }
            }
        }
    }
    (out, cited)
}

/// A stored answer without its markers, as history for the model: its numbers belonged to
/// the searches of that answer and would be mistaken for the new ones.
pub fn strip_markers(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for piece in pieces(text) {
        if let Piece::Text(t) = piece {
            if out.ends_with(' ') && t.starts_with([' ', '.', ',', ';', ':', ')']) {
                out.pop();
            }
            out.push_str(t);
        }
    }
    out
}

pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// Text of the answer with its markers as links to the source list (`#m<message>-s<n>`) and
/// its unsourced labels marked.
fn decorate<'a>(text: &str, message_id: i64, cited: usize, events: &mut Vec<Event<'a>>) {
    for piece in pieces(text) {
        match piece {
            Piece::Marker(numbers) if numbers.iter().all(|n| (1..=cited).contains(n)) => {
                let links: Vec<String> = numbers
                    .iter()
                    .map(|n| format!("<a href=\"#m{message_id}-s{n}\">{n}</a>"))
                    .collect();
                events.push(Event::InlineHtml(CowStr::from(format!(
                    "<sup class=\"cite\">[{}]</sup>",
                    links.join(", ")
                ))));
            }
            Piece::Marker(numbers) => {
                let list: Vec<String> = numbers.iter().map(ToString::to_string).collect();
                events.push(Event::Text(CowStr::from(format!("[{}]", list.join(", ")))));
            }
            Piece::Text(mut t) => {
                while let Some((at, label)) = [UNSOURCED_FR, UNSOURCED_EN]
                    .iter()
                    .filter_map(|label| t.find(label).map(|at| (at, *label)))
                    .min()
                {
                    if at > 0 {
                        events.push(Event::Text(CowStr::from(t[..at].to_owned())));
                    }
                    events.push(Event::InlineHtml(CowStr::from(format!(
                        "<span class=\"unsourced\">{label}</span>"
                    ))));
                    t = &t[at + label.len()..];
                }
                if !t.is_empty() {
                    events.push(Event::Text(CowStr::from(t.to_owned())));
                }
            }
        }
    }
}

/// The answer's Markdown as HTML: paragraphs, emphasis, lists, tables and code; headings become
/// bold paragraphs; links and images keep only their text; raw HTML is shown as text.
pub fn answer_html(text: &str, message_id: i64, cited: usize) -> String {
    let parser = Parser::new_ext(text, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH);
    let mut events: Vec<Event> = Vec::new();
    // Text arrives in pieces (`[`, `1]`): joined before markers are looked for.
    let mut pending = String::new();
    let mut in_code = false;
    for event in parser {
        if let Event::Text(t) | Event::Html(t) | Event::InlineHtml(t) = &event {
            pending.push_str(t);
            continue;
        }
        if !pending.is_empty() {
            if in_code {
                events.push(Event::Text(CowStr::from(std::mem::take(&mut pending))));
            } else {
                decorate(&std::mem::take(&mut pending), message_id, cited, &mut events);
            }
        }
        match event {
            Event::Start(Tag::Link { .. } | Tag::Image { .. })
            | Event::End(TagEnd::Link | TagEnd::Image) => {}
            Event::Start(Tag::HtmlBlock) => events.push(Event::Start(Tag::Paragraph)),
            Event::End(TagEnd::HtmlBlock) => events.push(Event::End(TagEnd::Paragraph)),
            Event::Start(Tag::Heading { .. }) => {
                events.push(Event::Start(Tag::Paragraph));
                events.push(Event::Start(Tag::Strong));
            }
            Event::End(TagEnd::Heading(_)) => {
                events.push(Event::End(TagEnd::Strong));
                events.push(Event::End(TagEnd::Paragraph));
            }
            Event::Start(Tag::CodeBlock(_)) => {
                in_code = true;
                events.push(event);
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code = false;
                events.push(event);
            }
            other => events.push(other),
        }
    }
    if !pending.is_empty() {
        decorate(&pending, message_id, cited, &mut events);
    }
    let mut out = String::with_capacity(text.len() * 3 / 2);
    html::push_html(&mut out, events.into_iter());
    out
}

/// The numbered sources of an answer: each reference or video statement as an article lists it,
/// then the page the passage was found on.
pub fn sources_html(knowledge: &Knowledge, tr: &Tr, message_id: i64, cited: &[Source]) -> String {
    let lib = &knowledge.lib;
    let mut out = String::new();
    for source in cited {
        let refs: Vec<String> = source
            .keys
            .iter()
            .map(|key| {
                let c = cite(lib, key);
                match c.href {
                    Some(href) => format!(
                        "<a href=\"{}\" rel=\"noopener noreferrer\">{}</a>",
                        escape(&href),
                        escape(&c.text)
                    ),
                    None => escape(&c.text),
                }
            })
            .collect();
        let page = match source.kind.as_str() {
            "article" => {
                let title = lib
                    .article(&source.id, tr.lang())
                    .map_or(source.title.as_str(), |a| a.front.title.as_str());
                Some((format!("/{}/learn/{}", tr.lang(), source.id), title.to_owned()))
            }
            "recipe" => {
                let title = lib
                    .recipe(&source.id)
                    .map_or_else(|| source.title.clone(), |r| r.title.get(tr.lang()).to_owned());
                Some((format!("/{}/recipes/{}", tr.lang(), source.id), title))
            }
            _ => None,
        };
        let _ = write!(out, "<li id=\"m{message_id}-s{}\" value=\"{}\">", source.n, source.n);
        out.push_str(&refs.join("; "));
        if let Some((href, title)) = page {
            if !refs.is_empty() {
                out.push_str(" — ");
            }
            let _ = write!(out, "<a href=\"{}\">{}</a>", escape(&href), escape(&title));
        }
        out.push_str("</li>");
    }
    if out.is_empty() {
        return out;
    }
    format!(
        "<ol class=\"chat-sources\" aria-label=\"{}\">{out}</ol>",
        escape(&tr.t("chat-sources"))
    )
}

/// The calculators and searches an answer used, as a note under it.
fn tools_html(tr: &Tr, tools: &[ToolRecord]) -> String {
    let mut names: Vec<String> = Vec::new();
    for record in tools.iter().filter(|r| r.ok) {
        if let Some(name) = tr.try_t(&format!("chat-tool-{}", record.name))
            && !names.contains(&name)
        {
            names.push(name);
        }
    }
    if names.is_empty() {
        return String::new();
    }
    format!(
        "<p class=\"note chat-tools\">{} {}</p>",
        escape(&tr.t("chat-tools-used")),
        escape(&names.join(", "))
    )
}

/// A stored answer as the page shows it.
pub fn message_html(
    knowledge: &Knowledge,
    tr: &Tr,
    message_id: i64,
    content: &str,
    cited: &[Source],
    tools: &[ToolRecord],
) -> String {
    format!(
        "<div class=\"chat-text\">{}</div>{}{}",
        answer_html(content, message_id, cited.len()),
        tools_html(tr, tools),
        sources_html(knowledge, tr, message_id, cited)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    fn source(n: usize, keys: &[&str], kind: &str, id: &str) -> Source {
        Source {
            n,
            keys: keys.iter().map(|k| (*k).to_owned()).collect(),
            kind: kind.to_owned(),
            id: id.to_owned(),
            title: "Yudane".to_owned(),
        }
    }

    #[test]
    fn markers_are_renumbered_in_citation_order_and_invented_ones_dropped() {
        let sources = [
            source(1, &["zeng2023"], "article", "yudane"),
            source(2, &["fixture-0001"], "claim", "fixture-0001"),
            source(3, &[], "recipe", "milk-bread"),
        ];
        let text = "Scald the flour [2]. It keeps the crumb soft [2, 1]; bake it hot [9]. \
                    Rest it overnight [Not sourced]. See the recipe [3][7]. Not a marker: [a] [].";
        let (stored, cited) = finalize(text, &sources, UNSOURCED_EN);
        assert_eq!(
            stored,
            "Scald the flour [1]. It keeps the crumb soft [1, 2]; bake it hot (unsourced). \
             Rest it overnight (unsourced). See the recipe [3]. Not a marker: [a] []."
        );
        let keys: Vec<(usize, Vec<String>)> = cited.iter().map(|s| (s.n, s.keys.clone())).collect();
        assert_eq!(
            keys,
            vec![
                (1, vec!["fixture-0001".to_owned()]),
                (2, vec!["zeng2023".to_owned()]),
                (3, Vec::new())
            ]
        );
        assert_eq!(cited[2].kind, "recipe");
        assert_eq!(
            finalize("Sans source [non sourcé].", &[], UNSOURCED_FR).0,
            "Sans source (non sourcé)."
        );
        assert_eq!(
            strip_markers("Le sel freine [1, 2]. La levure [3] (non sourcé)."),
            "Le sel freine. La levure (non sourcé)."
        );
    }

    #[test]
    fn answers_render_without_links_images_or_html() {
        let html = answer_html(
            "## Pourquoi\n\nLe **sel** freine [1] (non sourcé) <script>alert(1)</script> \
             [lien](https://evil.example) ![x](https://evil.example/i.png)\n\n\
             <div onclick=\"x\">bloc</div>\n\n| a | b |\n|---|---|\n| 1 | [2] |\n\n`[1]` [7]",
            42,
            2,
        );
        assert!(html.contains("<p><strong>Pourquoi</strong></p>"), "{html}");
        assert!(
            html.contains(
                "<strong>sel</strong> freine <sup class=\"cite\">[<a href=\"#m42-s1\">1</a>]</sup>"
            ),
            "{html}"
        );
        assert!(html.contains("<span class=\"unsourced\">(non sourcé)</span>"), "{html}");
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"), "{html}");
        assert!(!html.contains("href=\"https://evil") && !html.contains("<img"), "{html}");
        assert!(html.contains("lien") && html.contains("&lt;div onclick"), "{html}");
        assert!(
            html.contains("<td><sup class=\"cite\">[<a href=\"#m42-s2\">2</a>]</sup></td>"),
            "{html}"
        );
        // Code stays code; a number beyond the list stays text.
        assert!(html.contains("<code>[1]</code> [7]"), "{html}");
    }

    #[test]
    fn sources_are_cited_like_an_article() {
        let knowledge = Knowledge::fixture();
        let tr = Lang::Fr.tr();
        let cited = [
            source(1, &["fixture-0001", "zeng2023"], "article", "yudane"),
            source(2, &[], "recipe", "milk-bread"),
        ];
        let html = sources_html(&knowledge, &tr, 7, &cited);
        assert!(html.starts_with("<ol class=\"chat-sources\""), "{html}");
        assert!(html.contains("<li id=\"m7-s1\" value=\"1\"><a href=\"https://www.youtube.com/watch?v=ocBV6-O5SL4&amp;t=760s\" rel=\"noopener noreferrer\">Novita Listyani — Tangzhong video, 12:40</a>; <a href=\"https://doi.org/10.1002/jsfa.12565\""), "{html}");
        assert!(html.contains(" — <a href=\"/fr/learn/yudane\">Le yudane</a></li>"), "{html}");
        assert!(html.contains("<li id=\"m7-s2\" value=\"2\"><a href=\"/fr/recipes/milk-bread\">Pain de mie</a></li>"), "{html}");
        assert_eq!(sources_html(&knowledge, &tr, 7, &[]), "");
    }
}
