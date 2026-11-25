//! Flour blends: protein, a starting hydration with its range, the water that additions and a
//! scald call for, vital gluten for a protein target, and how long a cold proof the blend
//! tolerates.
//!
//! The numbers come from `research/data/model-flour.json` (copied into `flour/data.rs` by
//! `research/tools/export_flour_table.py`), where each one carries its sources and an evidence
//! label. Standards fix the ash of a flour type and little else, so about half of the per-type
//! values are estimates: the result is a starting point with an honest range, not a
//! measurement of the baker's own bag.
//!
//! Method (the model's `hydration_method`):
//! 1. without a reference recipe (mode A), each flour starts from its type's usual bread
//!    hydration, shifted by one point per point of protein when the bag's protein is known
//!    (Tipples 1978; within three points of the type's value); the blend is the share-weighted
//!    mean, its uncertainty the weighted half-width of the types' ranges, plus 2.5 points for a
//!    wheat–rye blend with 10–40 % rye;
//! 2. with a recipe that already works (mode B), its hydration is kept and only the difference
//!    in farinograph absorption between its flours and the new blend is added: the recipe
//!    already holds the process and the consistency the baker likes, so the range is much
//!    narrower (the flours common to both cancel);
//! 3. each addition adds `k` grams of water per gram, a scald `k` points per percent of flour
//!    scalded; their uncertainties combine in quadrature with the blend's.

use serde::{Deserialize, Serialize};

use crate::{EngineError, check_range};

#[rustfmt::skip]
mod data;

/// Shares, additions and scalds a blend can list (also bounds the work an input can ask for).
const MAX_FLOURS: usize = 6;
const MAX_ADDITIONS: usize = 16;
/// Absorption points per point of protein, at constant starch damage.
const PROTEIN_SLOPE: f64 = 1.0;
/// The slope holds within this many protein points of the type's usual value.
const PROTEIN_SHIFT_MAX: f64 = 3.0;
/// Extra uncertainty of a wheat–rye blend, in hydration points, and the rye shares it applies to.
const RYE_BLEND_EXTRA: f64 = 2.5;
const RYE_BLEND_SHARES: (f64, f64) = (10.0, 40.0);
/// Rye share from which sourdough or an acidifier improves the crumb.
const RYE_ACID_SHARE: f64 = 20.0;
/// Tested ranges of the coefficients, % of flour.
const ADDITIONS_TESTED_MAX: f64 = 15.0;
const PSYLLIUM_TESTED_MAX: f64 = 5.0;
const SCALD_TESTED_MAX: f64 = 40.0;
const VITAL_GLUTEN: &str = "vital_wheat_gluten";
const PSYLLIUM: &str = "psyllium_husk";

/// A value with the range it is expected to fall in.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Band {
    pub low: f64,
    pub mid: f64,
    pub high: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Group {
    Wheat,
    Rye,
    Ancient,
    Durum,
    /// Grains without a gluten network of their own: buckwheat, oat, barley, maize, rice…
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Amylase {
    Normal,
    High,
    Low,
    VeryLow,
    Unknown,
}

/// A flour type of the table (`fr_t65`, `de_1050`, `us_bread`, `einkorn`…).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlourType {
    pub id: &'static str,
    pub group: Group,
    /// Protein, % as is.
    pub protein: Band,
    pub fibre_pct: f64,
    /// Farinograph water absorption, %; `None` when no value exists for the grain.
    pub absorption: Option<Band>,
    /// Usual hydration of a bread made with this flour, %.
    pub hydration: Option<Band>,
    /// Whether the flour forms a gluten network on its own.
    pub network: bool,
    /// Longest cold proof the type usually tolerates, hours (an estimate from gluten strength).
    pub retard_h: Option<f64>,
    pub amylase: Amylase,
}

/// Water an addition takes at equal dough consistency, g per g.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdditionWater {
    pub kind: &'static str,
    pub k: f64,
    pub k_low: f64,
    pub k_high: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaldKind {
    /// Flour mixed with about its weight of boiling water (ends near 55 °C).
    #[default]
    Yudane,
    /// A scald held at 60–70 °C.
    Hot,
    /// A paste cooked to 75 °C or more (tangzhong, empois).
    Cooked,
}

/// Hydration points a scald adds per percent of the flour scalded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaldWater {
    pub kind: ScaldKind,
    pub k: f64,
    pub k_low: f64,
    pub k_high: f64,
}

pub fn types() -> &'static [FlourType] {
    data::TYPES
}

pub fn flour_type(id: &str) -> Option<&'static FlourType> {
    data::TYPES.iter().find(|t| t.id == id)
}

pub fn additions() -> &'static [AdditionWater] {
    data::ADDITIONS
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlourShare {
    /// A flour type id of [`types`].
    pub id: String,
    /// Share of the blend; shares are normalised, so parts or percentages both work.
    pub pct: f64,
    /// Protein printed on the bag, % as is; the type's usual value when absent.
    #[serde(default)]
    pub protein_pct: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionDose {
    /// An addition kind of [`additions`].
    pub kind: String,
    /// Baker's percentage (flour = 100).
    pub pct: f64,
}

/// A recipe that already works: its flours and the hydration it is made at.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub flours: Vec<FlourShare>,
    pub hydration_pct: f64,
}

/// The flour calculator's question.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickFlour {
    pub flours: Vec<FlourShare>,
    pub total_flour_g: f64,
    /// When given, the hydration is carried over from this recipe instead of starting from
    /// the flour types' usual values; additions and scald are counted on top of it.
    pub reference: Option<Reference>,
    pub additions: Vec<AdditionDose>,
    /// Flour scalded, % of the total flour.
    pub scald_pct: f64,
    pub scald: ScaldKind,
    /// Protein wanted for the blend, % as is: how much vital gluten gets there.
    pub target_protein_pct: Option<f64>,
    /// Protein of the vital gluten, % as is (the label's value).
    pub gluten_protein_pct: f64,
}

impl Default for QuickFlour {
    fn default() -> Self {
        Self {
            flours: vec![FlourShare { id: "fr_t65".into(), pct: 100.0, protein_pct: None }],
            total_flour_g: 1000.0,
            reference: None,
            additions: Vec::new(),
            scald_pct: 0.0,
            scald: ScaldKind::default(),
            target_protein_pct: None,
            gluten_protein_pct: data::GLUTEN_PROTEIN_PCT,
        }
    }
}

/// Water one addition brings to the dough.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AdditionLine {
    pub kind: String,
    pub grams: f64,
    pub water_g: f64,
}

/// Vital gluten that brings the blend to the protein wanted.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct GlutenDose {
    pub pct: f64,
    pub grams: f64,
    /// Water the gluten itself takes: to add on top of the water of `hydration`.
    pub extra_water_g: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum FlourNote {
    /// From about 20 % rye, sourdough or an acidifier improves the crumb.
    RyeNeedsAcidity,
    /// Seeds, bran and grains above the tested range of the coefficients.
    AdditionsBeyondRange,
    PsylliumBeyondRange,
    ScaldBeyondRange,
    /// The bag's protein is more than three points from the type's: the shift is capped.
    ProteinFarFromType {
        id: String,
    },
    /// No usual bread hydration is known for this flour: no hydration is given.
    NoHydrationData {
        id: String,
    },
    /// No absorption value is known for this flour: a recipe's hydration cannot be carried
    /// over to or from it.
    NoAbsorptionData {
        id: String,
    },
    /// The blend already has the protein wanted.
    TargetBelowBlend,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FlourResult {
    /// Protein of the flours, % as is.
    pub protein_pct: f64,
    /// Protein once the vital gluten of the additions is counted, when there is some.
    pub protein_with_gluten_pct: Option<f64>,
    pub fibre_pct: f64,
    /// Farinograph absorption of the blend, %; `None` when a flour has no value.
    pub absorption_pct: Option<f64>,
    /// Starting hydration, % of the flour, with its range; `None` when a flour has no data.
    pub hydration: Option<Band>,
    /// The same in grams of water for `total_flour_g`.
    pub water_g: Option<Band>,
    /// Whether `flours_points` comes from a reference recipe rather than the flour types.
    pub from_reference: bool,
    /// Parts of `hydration.mid`: the flours, the additions, the scald (points).
    pub flours_points: f64,
    pub additions_points: f64,
    pub scald_points: f64,
    pub additions: Vec<AdditionLine>,
    pub gluten: Option<GlutenDose>,
    /// Share of rye in the flour, %.
    pub rye_pct: f64,
    /// Share of the flour that forms a gluten network, %.
    pub network_pct: f64,
    /// Longest cold proof the blend is expected to tolerate, hours (estimate).
    pub retard_h: Option<f64>,
    pub notes: Vec<FlourNote>,
}

fn invalid(field: impl Into<String>, reason: impl Into<String>) -> EngineError {
    EngineError::Invalid { field: field.into(), reason: reason.into() }
}

type Blend<'a> = Vec<(&'a FlourShare, &'static FlourType)>;

/// The flours of a blend with their types, and the sum of their shares.
fn resolve<'a>(flours: &'a [FlourShare], field: &str) -> Result<(Blend<'a>, f64), EngineError> {
    if flours.is_empty() {
        return Err(EngineError::NoFlour);
    }
    if flours.len() > MAX_FLOURS {
        return Err(invalid(field, format!("more than {MAX_FLOURS} entries")));
    }
    let mut blend = Vec::with_capacity(flours.len());
    let mut total = 0.0;
    for (i, share) in flours.iter().enumerate() {
        check_range(&format!("{field}[{i}].pct"), share.pct, 0.0, 1.0e6)?;
        if let Some(p) = share.protein_pct {
            check_range(&format!("{field}[{i}].protein_pct"), p, 1.0, 30.0)?;
        }
        let ty = flour_type(&share.id)
            .ok_or_else(|| invalid(format!("{field}[{i}].id"), "unknown flour type"))?;
        total += share.pct;
        blend.push((share, ty));
    }
    if total <= 0.0 {
        return Err(EngineError::NoFlour);
    }
    Ok((blend, total))
}

/// The bag's protein of one flour of a blend, the absorption shift it implies (capped), and
/// whether the cap applied.
fn protein_shift(share: &FlourShare, ty: &FlourType) -> (f64, f64, bool) {
    let bag = share.protein_pct.unwrap_or(ty.protein.mid);
    let (shift, cap) = (PROTEIN_SLOPE * (bag - ty.protein.mid), PROTEIN_SLOPE * PROTEIN_SHIFT_MAX);
    (bag, shift.clamp(-cap, cap), shift.abs() > cap)
}

/// A flour type's place in a blend: its mass fraction, and its farinograph absorption (with the
/// bag's protein shift) and the half-width of the type's absorption range, when known.
struct Part {
    id: &'static str,
    fraction: f64,
    absorption: Option<(f64, f64)>,
}

fn parts(blend: &Blend<'_>, total: f64) -> Vec<Part> {
    blend
        .iter()
        .filter(|(share, _)| share.pct > 0.0)
        .map(|(share, ty)| Part {
            id: ty.id,
            fraction: share.pct / total,
            absorption: ty
                .absorption
                .map(|a| (a.mid + protein_shift(share, ty).1, (a.high - a.low) / 2.0)),
        })
        .collect()
}

/// Farinograph absorption of a blend; `None` when one of its flours has no value.
fn blend_absorption(parts: &[Part]) -> Option<f64> {
    parts.iter().try_fold(0.0, |sum, p| p.absorption.map(|(a, _)| sum + p.fraction * a))
}

impl QuickFlour {
    pub fn solve(&self) -> Result<FlourResult, EngineError> {
        if self.additions.len() > MAX_ADDITIONS {
            return Err(invalid("additions", format!("more than {MAX_ADDITIONS} entries")));
        }
        check_range("total_flour_g", self.total_flour_g, 1.0, 1.0e6)?;
        check_range("scald_pct", self.scald_pct, 0.0, 100.0)?;
        check_range("gluten_protein_pct", self.gluten_protein_pct, 40.0, 95.0)?;
        let (blend, total) = resolve(&self.flours, "flours")?;

        let mut notes = Vec::new();
        let (mut protein, mut fibre, mut rye, mut network) = (0.0, 0.0, 0.0, 0.0);
        let (mut usual, mut usual_spread) = (Some(0.0), 0.0);
        let (mut retard, mut retard_weight) = (0.0, 0.0);
        let mut no_usual = Vec::new();
        for (share, ty) in &blend {
            let w = share.pct / total;
            if w == 0.0 {
                continue;
            }
            let (bag, shift, capped) = protein_shift(share, ty);
            if capped {
                notes.push(FlourNote::ProteinFarFromType { id: ty.id.to_owned() });
            }
            protein += w * bag;
            fibre += w * ty.fibre_pct;
            if ty.group == Group::Rye {
                rye += w * 100.0;
            }
            if ty.network {
                network += w * 100.0;
                if let Some(h) = ty.retard_h {
                    retard += w * h;
                    retard_weight += w;
                }
            }
            match ty.hydration {
                Some(h) => {
                    usual = usual.map(|sum| sum + w * (h.mid + shift));
                    usual_spread += w * (h.high - h.low) / 2.0;
                }
                None => {
                    usual = None;
                    no_usual.push(ty.id);
                }
            }
        }
        let new_parts = parts(&blend, total);
        let absorption = blend_absorption(&new_parts);
        if rye >= RYE_ACID_SHARE {
            notes.push(FlourNote::RyeNeedsAcidity);
        }

        let (hydration, spread) = match &self.reference {
            None => {
                notes.extend(
                    no_usual.iter().map(|id| FlourNote::NoHydrationData { id: (*id).to_owned() }),
                );
                let rye_blend =
                    network > 0.0 && rye >= RYE_BLEND_SHARES.0 && rye <= RYE_BLEND_SHARES.1;
                (usual, usual_spread + if rye_blend { RYE_BLEND_EXTRA } else { 0.0 })
            }
            Some(reference) => {
                check_range("reference.hydration_pct", reference.hydration_pct, 30.0, 150.0)?;
                let (ref_blend, ref_total) = resolve(&reference.flours, "reference.flours")?;
                let ref_parts = parts(&ref_blend, ref_total);
                // u² = Σ (dᵢ·uᵢ)²: dᵢ the change in the share of flour type i, uᵢ the
                // half-width of its absorption range. A type common to both blends counts only
                // for the change in its share.
                let mut variance = 0.0;
                let mut seen: Vec<&str> = Vec::new();
                for part in new_parts.iter().chain(&ref_parts) {
                    if seen.contains(&part.id) {
                        continue;
                    }
                    seen.push(part.id);
                    let share = |of: &[Part]| -> f64 {
                        of.iter().filter(|p| p.id == part.id).map(|p| p.fraction).sum()
                    };
                    match part.absorption {
                        Some((_, half)) => {
                            variance += ((share(&new_parts) - share(&ref_parts)) * half).powi(2);
                        }
                        None => notes.push(FlourNote::NoAbsorptionData { id: part.id.to_owned() }),
                    }
                }
                let base = absorption
                    .zip(blend_absorption(&ref_parts))
                    .map(|(new, old)| reference.hydration_pct + new - old);
                (base, variance.sqrt())
            }
        };

        // Additions: water and its uncertainty, in points of hydration (g per 100 g of flour).
        let mut lines = Vec::with_capacity(self.additions.len());
        let (mut added, mut variance, mut bulk, mut gluten_pct) = (0.0, spread * spread, 0.0, 0.0);
        for (i, dose) in self.additions.iter().enumerate() {
            check_range(&format!("additions[{i}].pct"), dose.pct, 0.0, 100.0)?;
            let k = additions()
                .iter()
                .find(|a| a.kind == dose.kind)
                .ok_or_else(|| invalid(format!("additions[{i}].kind"), "unknown addition"))?;
            if dose.pct == 0.0 {
                continue;
            }
            added += dose.pct * k.k;
            variance += (dose.pct * (k.k_high - k.k_low) / 2.0).powi(2);
            match dose.kind.as_str() {
                VITAL_GLUTEN => gluten_pct += dose.pct,
                PSYLLIUM if dose.pct > PSYLLIUM_TESTED_MAX => {
                    notes.push(FlourNote::PsylliumBeyondRange);
                }
                PSYLLIUM => {}
                _ => bulk += dose.pct,
            }
            let grams = dose.pct / 100.0 * self.total_flour_g;
            lines.push(AdditionLine { kind: dose.kind.clone(), grams, water_g: grams * k.k });
        }
        if bulk > ADDITIONS_TESTED_MAX {
            notes.push(FlourNote::AdditionsBeyondRange);
        }

        let scald = data::SCALDS
            .iter()
            .find(|s| s.kind == self.scald)
            .ok_or_else(|| invalid("scald", "unknown scald method"))?;
        let scald_points = scald.k * self.scald_pct;
        variance += (self.scald_pct * (scald.k_high - scald.k_low) / 2.0).powi(2);
        if self.scald_pct > SCALD_TESTED_MAX {
            notes.push(FlourNote::ScaldBeyondRange);
        }

        let hydration = hydration.map(|flours| {
            let (mid, u) = (flours + added + scald_points, variance.sqrt());
            Band { low: mid - u, mid, high: mid + u }
        });
        let scale = self.total_flour_g / 100.0;
        let water_g = hydration.map(|h| Band {
            low: h.low * scale,
            mid: h.mid * scale,
            high: h.high * scale,
        });

        let gluten_k = additions().iter().find(|a| a.kind == VITAL_GLUTEN).map_or(0.0, |a| a.k);
        let gluten = match self.target_protein_pct {
            None => None,
            Some(target) => {
                // The target stays below any vital gluten's own protein (at least 40 %).
                check_range("target_protein_pct", target, 1.0, 30.0)?;
                if target <= protein {
                    notes.push(FlourNote::TargetBelowBlend);
                    None
                } else {
                    // Mass balance of protein: G = F (Pt − Pf) / (Pg − Pt).
                    let pct = 100.0 * (target - protein) / (self.gluten_protein_pct - target);
                    let grams = pct * scale;
                    Some(GlutenDose { pct, grams, extra_water_g: grams * gluten_k })
                }
            }
        };

        Ok(FlourResult {
            protein_pct: protein,
            protein_with_gluten_pct: (gluten_pct > 0.0).then(|| {
                (protein * 100.0 + self.gluten_protein_pct * gluten_pct) / (100.0 + gluten_pct)
            }),
            fibre_pct: fibre,
            absorption_pct: absorption,
            hydration,
            water_g,
            from_reference: self.reference.is_some(),
            flours_points: hydration.map_or(0.0, |h| h.mid - added - scald_points),
            additions_points: added,
            scald_points,
            additions: lines,
            gluten,
            rye_pct: rye,
            network_pct: network,
            retard_h: (retard_weight > 0.0).then(|| retard / retard_weight),
            notes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    fn share(id: &str, pct: f64) -> FlourShare {
        FlourShare { id: id.into(), pct, protein_pct: None }
    }

    fn dose(kind: &str, pct: f64) -> AdditionDose {
        AdditionDose { kind: kind.into(), pct }
    }

    #[test]
    fn every_type_of_the_table_solves_and_ids_are_unique() {
        assert!(types().len() >= 40);
        for (i, t) in types().iter().enumerate() {
            assert!(types()[..i].iter().all(|other| other.id != t.id), "{}", t.id);
            assert!(t.protein.low <= t.protein.mid && t.protein.mid <= t.protein.high, "{}", t.id);
            let r = QuickFlour { flours: vec![share(t.id, 100.0)], ..Default::default() }
                .solve()
                .unwrap();
            close(r.protein_pct, t.protein.mid, 1e-9);
            assert_eq!(r.hydration.is_some(), t.hydration.is_some(), "{}", t.id);
        }
    }

    /// The model's worked example A: 800 g T65 + 200 g rye T130, 20 g vital gluten, 100 g soaked
    /// sunflower seeds, 50 g oat bran.
    #[test]
    fn reproduces_the_models_worked_example() {
        let q = QuickFlour {
            flours: vec![share("fr_t65", 800.0), share("fr_rye_t130", 200.0)],
            additions: vec![
                dose("vital_wheat_gluten", 2.0),
                dose("seeds_sunflower_sesame_pumpkin", 10.0),
                dose("oat_bran", 5.0),
            ],
            ..Default::default()
        };
        let r = q.solve().unwrap();
        close(r.flours_points, 68.8, 1e-9);
        close(r.additions_points, 16.0, 1e-9);
        let h = r.hydration.unwrap();
        close(h.mid, 84.8, 1e-9);
        close(r.water_g.unwrap().mid, 848.0, 1e-6);
        // Blend 5.1 points + 2.5 for 20 % rye, with 0.8, 2.0 and 1.75 points for the additions.
        close(
            h.high - h.mid,
            (7.6f64.powi(2) + 0.8f64.powi(2) + 4.0 + 1.75f64.powi(2)).sqrt(),
            1e-9,
        );
        close(r.rye_pct, 20.0, 1e-9);
        // Seeds and bran are at 15 % of the flour: the edge of the tested range, not beyond.
        assert_eq!(r.notes, vec![FlourNote::RyeNeedsAcidity]);
        let lines: Vec<f64> = r.additions.iter().map(|l| l.water_g).collect();
        assert_eq!(lines, vec![30.0, 50.0, 80.0]);
    }

    #[test]
    fn vital_gluten_dose_follows_the_protein_balance() {
        // 1,000 g of flour at 11 % raised to 13 % with 75 % gluten: 32.3 g, plus 48 g of water.
        let mut q = QuickFlour {
            flours: vec![FlourShare { id: "fr_t55".into(), pct: 100.0, protein_pct: Some(11.0) }],
            target_protein_pct: Some(13.0),
            ..Default::default()
        };
        let g = q.solve().unwrap().gluten.unwrap();
        close(g.grams, 32.26, 0.01);
        close(g.extra_water_g, 48.4, 0.05);
        // Adding that gluten gives the protein wanted.
        q.additions = vec![dose("vital_wheat_gluten", g.pct)];
        close(q.solve().unwrap().protein_with_gluten_pct.unwrap(), 13.0, 1e-9);
        q.target_protein_pct = Some(10.0);
        let r = q.solve().unwrap();
        assert!(r.gluten.is_none() && r.notes.contains(&FlourNote::TargetBelowBlend));
        q.additions = vec![dose("seeds_sunflower_sesame_pumpkin", 12.0), dose("oat_bran", 4.0)];
        assert!(q.solve().unwrap().notes.contains(&FlourNote::AdditionsBeyondRange));
    }

    #[test]
    fn bag_protein_shifts_hydration_and_scald_adds_water() {
        let base = QuickFlour::default().solve().unwrap().hydration.unwrap();
        let t65 = flour_type("fr_t65").unwrap();
        close(base.mid, t65.hydration.unwrap().mid, 1e-9);
        let strong = QuickFlour {
            flours: vec![FlourShare {
                id: "fr_t65".into(),
                pct: 100.0,
                protein_pct: Some(t65.protein.mid + 2.0),
            }],
            ..Default::default()
        };
        close(strong.solve().unwrap().hydration.unwrap().mid, base.mid + 2.0, 1e-9);
        let far = QuickFlour {
            flours: vec![FlourShare { id: "fr_t65".into(), pct: 100.0, protein_pct: Some(20.0) }],
            ..Default::default()
        };
        let r = far.solve().unwrap();
        close(r.hydration.unwrap().mid, base.mid + 3.0, 1e-9);
        assert_eq!(r.notes, vec![FlourNote::ProteinFarFromType { id: "fr_t65".into() }]);
        // A yudane on 20 % of the flour: +3 points (0.15 per percent scalded).
        let scald = QuickFlour { scald_pct: 20.0, ..Default::default() }.solve().unwrap();
        close(scald.scald_points, 3.0, 1e-9);
        close(scald.hydration.unwrap().mid, base.mid + 3.0, 1e-9);
    }

    #[test]
    fn unknown_ids_and_flours_without_data_are_reported() {
        let unknown = QuickFlour { flours: vec![share("t999", 100.0)], ..Default::default() };
        assert!(
            matches!(unknown.solve(), Err(EngineError::Invalid { field, .. }) if field == "flours[0].id")
        );
        let r = QuickFlour {
            flours: vec![share("fr_t65", 80.0), share("chestnut", 20.0)],
            ..Default::default()
        }
        .solve()
        .unwrap();
        assert!(r.hydration.is_none());
        assert_eq!(r.notes, vec![FlourNote::NoHydrationData { id: "chestnut".into() }]);
        close(r.network_pct, 80.0, 1e-9);
        // The cold-proof estimate is that of the flours that form a network.
        close(r.retard_h.unwrap(), flour_type("fr_t65").unwrap().retard_h.unwrap(), 1e-9);
    }

    /// The model's worked example B: a T65 recipe that works at 68 %, with 30 % of the flour
    /// moved to T150.
    #[test]
    fn a_working_recipe_carries_its_hydration_to_a_new_blend() {
        let reference =
            Some(Reference { flours: vec![share("fr_t65", 100.0)], hydration_pct: 68.0 });
        let q = QuickFlour {
            flours: vec![share("fr_t65", 70.0), share("fr_t150", 30.0)],
            reference: reference.clone(),
            ..Default::default()
        };
        let r = q.solve().unwrap();
        let h = r.hydration.unwrap();
        close(h.mid, 70.1, 1e-9);
        close(h.high - h.mid, (0.9f64.powi(2) + 1.35f64.powi(2)).sqrt(), 1e-9);
        assert!(r.from_reference && r.notes.is_empty());
        // The same flours as the recipe: its hydration, with no uncertainty from the flours.
        let same = QuickFlour { reference: reference.clone(), ..Default::default() };
        let h = same.solve().unwrap().hydration.unwrap();
        assert_eq!((h.low, h.mid, h.high), (68.0, 68.0, 68.0));
        // Far narrower than starting from the flour types.
        let types = QuickFlour { reference: None, ..q.clone() }.solve().unwrap().hydration.unwrap();
        assert!(types.high - types.mid > 3.0 * (r.hydration.unwrap().high - 70.1));
        // A flour without an absorption value cannot take part in the transfer.
        let rice = QuickFlour {
            flours: vec![share("fr_t65", 80.0), share("rice", 20.0)],
            reference,
            ..Default::default()
        };
        let r = rice.solve().unwrap();
        assert!(r.hydration.is_none());
        assert_eq!(r.notes, vec![FlourNote::NoAbsorptionData { id: "rice".into() }]);
    }
}
