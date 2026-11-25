//! Flour calculator: protein of a blend, a starting hydration with its range, the water of
//! additions and of a scald, vital gluten for a protein target.

use askama::Template;
use axum::extract::{Query, State};
use axum::response::Response;
use breadator_engine::flour::{
    AdditionDose, FlourNote, FlourResult, FlourShare, Group, QuickFlour, Reference, ScaldKind,
    types,
};
use serde::Deserialize;

use super::describe_error;
use crate::app::{AppState, LangPath, Layout, render};
use crate::i18n::{Lang, Tr};
use crate::state_codec::number;

/// Rows of the blend the form offers.
const ROWS: usize = 3;

/// Additions offered by the form: its field name and the engine's kind (`flour-add-<field>` is
/// the label). Mirrored in `web/src/flour.ts`.
pub const ADDITIONS: [(&str, &str); 8] = [
    ("gluten", "vital_wheat_gluten"),
    ("bran", "wheat_bran"),
    ("oat_bran", "oat_bran"),
    ("psyllium", "psyllium_husk"),
    ("chia", "chia_ground"),
    ("flax", "flaxseed_whole_soaked"),
    ("seeds", "seeds_sunflower_sesame_pumpkin"),
    ("grains", "cracked_grain_oat_flakes_buckwheat_groats_quinoa"),
];

const SCALDS: [(&str, ScaldKind); 3] =
    [("yudane", ScaldKind::Yudane), ("hot", ScaldKind::Hot), ("cooked", ScaldKind::Cooked)];

#[derive(Debug, Default, Deserialize)]
pub struct Params {
    f1: Option<String>,
    f2: Option<String>,
    f3: Option<String>,
    p1: Option<String>,
    p2: Option<String>,
    p3: Option<String>,
    prot1: Option<String>,
    prot2: Option<String>,
    prot3: Option<String>,
    flour_g: Option<String>,
    gluten: Option<String>,
    bran: Option<String>,
    oat_bran: Option<String>,
    psyllium: Option<String>,
    chia: Option<String>,
    flax: Option<String>,
    seeds: Option<String>,
    grains: Option<String>,
    scald_pct: Option<String>,
    scald: Option<String>,
    target: Option<String>,
    gluten_protein: Option<String>,
    /// A recipe that already works: its flour (`ref`) and its hydration (`ref_h`).
    #[serde(rename = "ref")]
    reference: Option<String>,
    ref_h: Option<String>,
}

/// What the form shows: the values as typed, so that an empty row stays empty.
pub struct Row {
    pub n: usize,
    pub id: String,
    pub pct: String,
    pub protein: String,
}

pub struct Field {
    pub name: &'static str,
    pub label: String,
    pub value: String,
}

/// The flour a new visitor starts from: the everyday bread flour of the language's main country.
fn default_flour(lang: Lang) -> &'static str {
    match lang {
        Lang::Fr => "fr_t65",
        Lang::En => "us_bread",
    }
}

fn text(v: &Option<String>) -> String {
    v.as_deref().map(str::trim).unwrap_or("").to_owned()
}

fn quick(p: &Params, lang: Lang) -> (QuickFlour, Vec<Row>, Vec<Field>) {
    let d = QuickFlour::default();
    let ids = [&p.f1, &p.f2, &p.f3];
    let shares = [&p.p1, &p.p2, &p.p3];
    let proteins = [&p.prot1, &p.prot2, &p.prot3];
    let untouched = ids.iter().all(|v| v.is_none());
    let mut rows = Vec::with_capacity(ROWS);
    let mut flours = Vec::new();
    for i in 0..ROWS {
        let mut id = text(ids[i]);
        let mut pct = text(shares[i]);
        if untouched && i == 0 {
            id = default_flour(lang).to_owned();
            pct = "100".to_owned();
        }
        if !id.is_empty() {
            flours.push(FlourShare {
                id: id.clone(),
                pct: number(Some(&pct)).unwrap_or(0.0),
                protein_pct: number(proteins[i].as_deref()),
            });
        }
        rows.push(Row { n: i + 1, id, pct, protein: text(proteins[i]) });
    }
    let doses =
        [&p.gluten, &p.bran, &p.oat_bran, &p.psyllium, &p.chia, &p.flax, &p.seeds, &p.grains];
    let tr = lang.tr();
    let mut fields = Vec::with_capacity(ADDITIONS.len());
    let mut additions = Vec::new();
    for ((name, kind), value) in ADDITIONS.iter().zip(doses) {
        if let Some(pct) = number(value.as_deref()).filter(|v| *v != 0.0) {
            additions.push(AdditionDose { kind: (*kind).to_owned(), pct });
        }
        fields.push(Field { name, label: tr.t(&format!("flour-add-{name}")), value: text(value) });
    }
    // Both the flour and the hydration of the working recipe are needed to start from it.
    let reference = number(p.ref_h.as_deref()).filter(|_| !text(&p.reference).is_empty()).map(
        |hydration_pct| Reference {
            flours: vec![FlourShare { id: text(&p.reference), pct: 100.0, protein_pct: None }],
            hydration_pct,
        },
    );
    let q = QuickFlour {
        flours,
        total_flour_g: number(p.flour_g.as_deref()).unwrap_or(d.total_flour_g),
        reference,
        additions,
        scald_pct: number(p.scald_pct.as_deref()).unwrap_or(0.0),
        scald: SCALDS
            .iter()
            .find(|(code, _)| Some(*code) == p.scald.as_deref())
            .map_or(d.scald, |(_, kind)| *kind),
        target_protein_pct: number(p.target.as_deref()),
        gluten_protein_pct: number(p.gluten_protein.as_deref()).unwrap_or(d.gluten_protein_pct),
    };
    (q, rows, fields)
}

/// Flour types by the group the form lists them under (`flour-group-<code>`).
fn group_code(id: &str, group: Group) -> &'static str {
    match group {
        Group::Rye => "rye",
        Group::Ancient | Group::Durum => "wheats",
        Group::Other => "grains",
        Group::Wheat => match id.split('_').next() {
            Some("fr") => "fr",
            Some("de") => "de",
            Some("it") => "it",
            _ => "us",
        },
    }
}

pub struct OptGroup {
    pub label: String,
    pub options: Vec<(&'static str, String)>,
}

fn flour_options(tr: &Tr) -> Vec<OptGroup> {
    ["fr", "de", "it", "us", "rye", "wheats", "grains"]
        .iter()
        .map(|code| OptGroup {
            label: tr.t(&format!("flour-group-{code}")),
            options: types()
                .iter()
                .filter(|t| group_code(t.id, t.group) == *code)
                .map(|t| (t.id, tr.t(&format!("flour-{}", t.id))))
                .collect(),
        })
        .collect()
}

pub struct ResultView {
    pub headline: String,
    pub water: Option<String>,
    pub parts: Vec<(String, String)>,
    pub metrics: Vec<(String, String)>,
    pub additions: Vec<(String, String, String)>,
    pub gluten: Option<String>,
    pub notes: Vec<String>,
}

/// "64–74" with the language's decimal separator.
fn range(tr: &Tr, low: f64, high: f64, decimals: usize) -> String {
    format!("{}–{}", tr.num(low, decimals), tr.num(high, decimals))
}

fn note(tr: &Tr, n: &FlourNote) -> String {
    let flour = |id: &str| tr.t(&format!("flour-{id}")).into();
    match n {
        FlourNote::RyeNeedsAcidity => tr.t("flour-note-rye"),
        FlourNote::AdditionsBeyondRange => tr.t("flour-note-additions"),
        FlourNote::PsylliumBeyondRange => tr.t("flour-note-psyllium"),
        FlourNote::ScaldBeyondRange => tr.t("flour-note-scald"),
        FlourNote::ProteinFarFromType { id } => {
            tr.t_args("flour-note-protein", &[("flour", flour(id))])
        }
        FlourNote::NoHydrationData { id } => {
            tr.t_args("flour-note-no-data", &[("flour", flour(id))])
        }
        FlourNote::NoAbsorptionData { id } => {
            tr.t_args("flour-note-no-absorption", &[("flour", flour(id))])
        }
        FlourNote::TargetBelowBlend => tr.t("flour-note-target"),
    }
}

fn view(tr: &Tr, q: &QuickFlour, r: &FlourResult) -> ResultView {
    let pct = |v: f64| format!("{} %", tr.num(v, 1));
    let headline = match r.hydration {
        Some(h) => tr.t_args(
            "flour-hydration-result",
            &[("mid", tr.num(h.mid, 0).into()), ("range", range(tr, h.low, h.high, 0).into())],
        ),
        None => tr.t("flour-hydration-unknown"),
    };
    let water = r.water_g.map(|w| {
        tr.t_args(
            "flour-water-result",
            &[
                ("mid", tr.g(w.mid).into()),
                ("range", tr.g_range(w.low, w.high).into()),
                ("flour", tr.g(q.total_flour_g).into()),
            ],
        )
    });
    let mut parts = Vec::new();
    let composed = r.additions_points > 0.0 || r.scald_points > 0.0;
    if r.hydration.is_some() && (composed || r.from_reference) {
        let base = if r.from_reference { "flour-part-reference" } else { "flour-part-flours" };
        parts.push((tr.t(base), pct(r.flours_points)));
        if r.additions_points > 0.0 {
            parts.push((tr.t("flour-part-additions"), format!("+ {}", pct(r.additions_points))));
        }
        if r.scald_points > 0.0 {
            parts.push((tr.t("flour-part-scald"), format!("+ {}", pct(r.scald_points))));
        }
    }
    let mut metrics = vec![(tr.t("flour-protein"), pct(r.protein_pct))];
    if let Some(p) = r.protein_with_gluten_pct {
        metrics.push((tr.t("flour-protein-with-gluten"), pct(p)));
    }
    metrics.push((tr.t("flour-fibre"), pct(r.fibre_pct)));
    if let Some(a) = r.absorption_pct {
        metrics.push((tr.t("flour-absorption"), pct(a)));
    }
    if r.rye_pct > 0.0 {
        metrics.push((tr.t("flour-rye-share"), pct(r.rye_pct)));
    }
    if r.network_pct < 100.0 {
        metrics.push((tr.t("flour-no-network"), pct(100.0 - r.network_pct)));
    }
    if let Some(h) = r.retard_h {
        metrics.push((
            tr.t("flour-retard"),
            tr.t_args("flour-retard-hours", &[("h", tr.num(h, 0).into())]),
        ));
    }
    let additions = r
        .additions
        .iter()
        .map(|l| {
            let name = ADDITIONS
                .iter()
                .find(|(_, kind)| *kind == l.kind)
                .map_or_else(|| l.kind.clone(), |(field, _)| tr.t(&format!("flour-add-{field}")));
            (name, tr.g(l.grams), tr.g(l.water_g))
        })
        .collect();
    let gluten = r.gluten.map(|g| {
        tr.t_args(
            "flour-gluten-result",
            &[
                ("grams", tr.g(g.grams).into()),
                ("pct", tr.num(g.pct, 1).into()),
                ("water", tr.g(g.extra_water_g).into()),
            ],
        )
    });
    ResultView {
        headline,
        water,
        parts,
        metrics,
        additions,
        gluten,
        notes: r.notes.iter().map(|n| note(tr, n)).collect(),
    }
}

#[derive(Template)]
#[template(path = "flour.html")]
struct Page {
    layout: Layout,
    rows: Vec<Row>,
    options: Vec<OptGroup>,
    additions: Vec<Field>,
    flour_g: String,
    scald_pct: String,
    scald: &'static str,
    target: String,
    gluten_protein: String,
    reference: String,
    ref_h: String,
    result: Option<ResultView>,
    error: Option<String>,
}

fn plain(x: f64) -> String {
    let s = format!("{x}");
    s.strip_suffix(".0").map(str::to_owned).unwrap_or(s)
}

pub async fn page(
    State(state): State<AppState>,
    LangPath(lang): LangPath,
    Query(params): Query<Params>,
) -> Response {
    let tr = lang.tr();
    let (q, rows, additions) = quick(&params, lang);
    let (result, error) = match q.solve() {
        Ok(r) => (Some(view(&tr, &q, &r)), None),
        Err(e) => (None, Some(describe_error(&tr, &e))),
    };
    let layout = Layout::new(
        &state,
        lang,
        "/calculators/flour",
        "flour",
        tr.t("flour-title"),
        tr.t("flour-description"),
    );
    render(&Page {
        layout,
        rows,
        options: flour_options(&tr),
        additions,
        flour_g: plain(q.total_flour_g),
        scald_pct: text(&params.scald_pct),
        scald: SCALDS.iter().find(|(_, kind)| *kind == q.scald).map_or("yudane", |(code, _)| code),
        target: text(&params.target),
        gluten_protein: plain(q.gluten_protein_pct),
        reference: text(&params.reference),
        ref_h: text(&params.ref_h),
        result,
        error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_flour_type_has_a_name_and_a_group_in_both_languages() {
        for lang in Lang::ALL {
            let tr = lang.tr();
            let listed: usize = flour_options(&tr).iter().map(|g| g.options.len()).sum();
            assert_eq!(listed, types().len());
            for t in types() {
                assert!(tr.try_t(&format!("flour-{}", t.id)).is_some(), "{} {}", lang.code(), t.id);
            }
            for (field, kind) in ADDITIONS {
                assert!(tr.try_t(&format!("flour-add-{field}")).is_some(), "{field}");
                assert!(
                    breadator_engine::flour::additions().iter().any(|a| a.kind == kind),
                    "{kind}"
                );
            }
        }
    }

    #[test]
    fn a_blend_typed_in_the_form_reads_back_as_a_result() {
        let p = Params {
            f1: Some("fr_t65".into()),
            p1: Some("80".into()),
            f2: Some("fr_rye_t130".into()),
            p2: Some("20".into()),
            f3: Some(String::new()),
            seeds: Some("10".into()),
            target: Some("13".into()),
            ..Default::default()
        };
        let (q, rows, _) = quick(&p, Lang::Fr);
        assert_eq!(q.flours.len(), 2);
        assert_eq!(rows[2].id, "");
        let tr = Lang::Fr.tr();
        let v = view(&tr, &q, &q.solve().unwrap());
        // 80 % T65 + 20 % rye T130 = 68.8, plus 5 points for 10 % of soaked seeds.
        assert!(v.headline.contains("74 % (66–82 %)"), "{}", v.headline);
        let weights: Vec<(&str, &str)> =
            v.additions.iter().map(|(_, grams, water)| (grams.as_str(), water.as_str())).collect();
        assert_eq!(weights, vec![("100", "50")]);
        assert!(v.gluten.is_some());
        assert_eq!(v.notes.len(), 1);
        // A new visitor starts from the language's everyday bread flour.
        assert_eq!(quick(&Params::default(), Lang::En).0.flours[0].id, "us_bread");
    }
}
