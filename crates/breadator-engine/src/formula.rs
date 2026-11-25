//! Baker's math with stages.
//!
//! A [`Formula`] is the *total* formula: every percentage is relative to all the flour of the
//! bread, whichever stage it ends up in (flour = 100 %). Stages (levain, poolish, biga, sponge,
//! pâte fermentée, scald, soaker) take their share out of those totals; the final mix is what
//! remains plus the stages themselves. Solving scales the formula to a [`Target`].

use serde::{Deserialize, Serialize};

use crate::error::{EngineError, Warning};
use crate::ingredients::{Composition, IngredientKind};
use crate::{EPS, check_range};

/// Upper bound for any single baker's percentage; far above any real dough.
const MAX_PCT: f64 = 1000.0;
/// Upper bound for any mass in grams (a 1-tonne batch).
const MAX_G: f64 = 1.0e6;
/// More entries than any bread has. A formula can come from anyone (the public API, a shared
/// link): these also bound the work it can ask for.
const MAX_FLOURS: usize = 16;
const MAX_INGREDIENTS: usize = 48;
const MAX_STAGES: usize = 12;
const MAX_STAGE_EXTRAS: usize = 16;
const MAX_ID_LEN: usize = 64;

fn check_len(field: &str, len: usize, max: usize) -> Result<(), EngineError> {
    if len > max {
        return Err(EngineError::Invalid {
            field: field.to_owned(),
            reason: format!("more than {max} entries"),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Formula {
    /// The flour blend; percentages add up to 100.
    pub flours: Vec<Flour>,
    /// Every non-flour ingredient of the bread, in % of total flour.
    #[serde(default)]
    pub ingredients: Vec<Ingredient>,
    #[serde(default)]
    pub stages: Vec<Stage>,
    pub target: Target,
    /// Extra dough made to cover what stays on bowls and tools, % of the target dough.
    #[serde(default)]
    pub allowance_pct: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flour {
    pub id: String,
    pub pct: f64,
    /// Flour type from the flour database (e.g. `fr_t65`, `us_bread`, `rye_whole`); used for advice.
    #[serde(default)]
    pub kind: Option<String>,
    /// Protein content of this flour as sold, %.
    #[serde(default)]
    pub protein_pct: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: String,
    pub kind: IngredientKind,
    pub pct: f64,
    /// Overrides the kind's default composition.
    #[serde(default)]
    pub composition: Option<Composition>,
}

impl Ingredient {
    pub fn composition(&self) -> Composition {
        self.composition.unwrap_or_else(|| self.kind.default_composition())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageKind {
    Levain,
    Poolish,
    Biga,
    Sponge,
    PateFermentee,
    Scald,
    Soaker,
}

impl StageKind {
    fn is_preferment(self) -> bool {
        matches!(
            self,
            Self::Levain | Self::Poolish | Self::Biga | Self::Sponge | Self::PateFermentee
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stage {
    pub id: String,
    pub kind: StageKind,
    /// Share of the total flour that goes into this stage, % (prefermented or scalded flour).
    #[serde(default)]
    pub flour_pct: f64,
    /// Flour of the blend this stage uses; `None` takes every flour in blend proportions.
    #[serde(default)]
    pub flour: Option<String>,
    /// Stage liquid, % of the stage flour (the stage's hydration).
    #[serde(default)]
    pub liquid_pct: f64,
    /// Ingredient that supplies the stage liquid.
    #[serde(default = "default_liquid")]
    pub liquid: String,
    /// Other ingredients put in this stage (yeast in a poolish, salt in a biga, seeds in a soaker).
    #[serde(default)]
    pub extras: Vec<StageExtra>,
    /// Scalds and cooked soakers: water lost while cooking, % of the stage's water. The solver
    /// adds it to the water to cook with, so the cooked stage matches the formula.
    #[serde(default)]
    pub evaporation_pct: Option<f64>,
    /// Levain only: the seed starter it is built from.
    #[serde(default)]
    pub seed: Option<Seed>,
}

fn default_liquid() -> String {
    "water".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageExtra {
    pub ingredient: String,
    pub pct: f64,
    #[serde(default)]
    pub basis: Basis,
}

/// What a stage percentage is relative to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Basis {
    /// % of the flour in the stage (e.g. 0.1 % yeast in a poolish).
    #[default]
    StageFlour,
    /// % of the total flour of the bread (e.g. 15 % seeds in a soaker).
    TotalFlour,
}

/// Seed starter used to build a levain, as in a seed:flour:water feeding ratio.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Seed {
    /// Seed mass, % of the fresh flour of the build (1:5:5 → 20).
    pub pct: f64,
    /// Hydration of the seed starter, %.
    pub hydration_pct: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum Target {
    /// Total flour of the bread, all stages included.
    Flour { grams: f64 },
    /// Total dough mass.
    Dough { grams: f64 },
    /// `count` dough pieces of `grams` each.
    Pieces { count: u32, grams: f64 },
    /// `count` breads weighing `grams` each after baking, losing `loss_pct` of the dough in the oven.
    Baked { count: u32, grams: f64, loss_pct: f64 },
}

/// Inoculation conventions for a levain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Inoculation {
    /// Levain mass / flour of the final mix (flour not already in the levain). Used by most
    /// recipes and by *The Sourdough Framework* Table 7.1.
    FinalMixFlour,
    /// Levain mass / total flour of the bread.
    TotalFlour,
}

/// Share of the total flour in a levain (`Stage::flour_pct`) from an inoculation percentage.
pub fn levain_flour_pct(
    inoculation_pct: f64,
    levain_hydration_pct: f64,
    basis: Inoculation,
) -> Result<f64, EngineError> {
    check_range("inoculation_pct", inoculation_pct, 0.0, MAX_PCT)?;
    check_range("levain_hydration_pct", levain_hydration_pct, 0.0, MAX_PCT)?;
    let i = inoculation_pct / 100.0;
    let h = levain_hydration_pct / 100.0;
    let p = match basis {
        // levain = p·F·(1+h) and final-mix flour = (1−p)·F  ⇒  i = p(1+h)/(1−p)
        Inoculation::FinalMixFlour => i / (1.0 + h + i),
        // levain = p·F·(1+h)  ⇒  i = p(1+h)
        Inoculation::TotalFlour => i / (1.0 + h),
    };
    if p > 1.0 + EPS {
        return Err(EngineError::Invalid {
            field: "inoculation_pct".into(),
            reason: "the levain would hold more flour than the whole bread".into(),
        });
    }
    Ok(p * 100.0)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Line {
    pub id: String,
    pub grams: f64,
    /// % of the total flour of the bread.
    pub pct: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SeedSolution {
    pub seed_g: f64,
    pub fresh_flour_g: f64,
    pub fresh_water_g: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StageSolution {
    pub id: String,
    pub kind: StageKind,
    pub flour_g: f64,
    pub water_g: f64,
    /// Stage mass as it goes into the final mix (after cooking for scalds).
    pub mass_g: f64,
    /// Stage hydration counting the water of every liquid in it, %.
    pub hydration_pct: f64,
    /// What goes into the stage (flours and ingredients), cooking compensation excluded.
    pub lines: Vec<Line>,
    /// Extra water to cook with, lost by evaporation.
    pub evaporation_water_g: f64,
    pub seed: Option<SeedSolution>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LevainMetrics {
    pub mass_g: f64,
    pub hydration_pct: f64,
    /// Levain mass / final-mix flour, %; `None` when all the flour is in the levain.
    pub inoculation_final_mix_pct: Option<f64>,
    /// Levain mass / total flour, %.
    pub inoculation_total_flour_pct: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Metrics {
    pub total_flour_g: f64,
    pub dough_g: f64,
    /// Plain water only / total flour, %.
    pub hydration_water_pct: f64,
    /// Water from every ingredient (milk, eggs, butter, honey…) / total flour, %.
    pub hydration_total_pct: f64,
    /// German "Teigausbeute": dough mass per 100 g flour, counting total water.
    pub dough_yield: f64,
    pub prefermented_flour_pct: f64,
    pub scalded_flour_pct: f64,
    pub salt_pct: f64,
    pub sugar_pct: f64,
    pub fat_pct: f64,
    /// Yeast of each kind, % of total flour.
    pub yeast: Vec<Line>,
    pub levains: Vec<LevainMetrics>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Solution {
    /// Totals of the whole bread: flours first, then ingredients in formula order.
    pub totals: Vec<Line>,
    pub stages: Vec<StageSolution>,
    /// What to weigh for the final mix: remaining ingredients, then each stage as one line.
    pub final_mix: Vec<Line>,
    pub metrics: Metrics,
    pub warnings: Vec<Warning>,
}

/// Grams of each formula entry (flours then ingredients, same order as `Formula`).
struct Ledger {
    ids: Vec<String>,
    total: Vec<f64>,
    used: Vec<f64>,
}

impl Ledger {
    fn index(&self, field: &str, id: &str) -> Result<usize, EngineError> {
        self.ids.iter().position(|x| x == id).ok_or_else(|| EngineError::UnknownIngredient {
            field: field.to_owned(),
            id: id.to_owned(),
        })
    }
}

pub fn solve(formula: &Formula) -> Result<Solution, EngineError> {
    validate(formula)?;
    let n_flours = formula.flours.len();
    let total_flour = total_flour_g(formula)?;
    let f = total_flour / 100.0; // grams per baker's percent

    let mut ledger = Ledger {
        ids: formula
            .flours
            .iter()
            .map(|x| x.id.clone())
            .chain(formula.ingredients.iter().map(|x| x.id.clone()))
            .collect(),
        total: formula
            .flours
            .iter()
            .map(|x| x.pct * f)
            .chain(formula.ingredients.iter().map(|x| x.pct * f))
            .collect(),
        used: vec![0.0; n_flours + formula.ingredients.len()],
    };
    let composition = |idx: usize| -> Composition {
        if idx < n_flours {
            Composition { water: 0.0, fat: 0.0, sugar: 0.0 }
        } else {
            formula.ingredients[idx - n_flours].composition()
        }
    };

    let mut warnings = Vec::new();
    let mut stages = Vec::with_capacity(formula.stages.len());
    for (si, stage) in formula.stages.iter().enumerate() {
        let field = |name: &str| format!("stages[{si}].{name}");
        let stage_flour = stage.flour_pct * f;
        let mut alloc: Vec<(usize, f64)> = Vec::new();

        match &stage.flour {
            Some(id) => alloc.push((ledger.index(&field("flour"), id)?, stage_flour)),
            None => {
                for (i, fl) in formula.flours.iter().enumerate() {
                    alloc.push((i, stage_flour * fl.pct / 100.0));
                }
            }
        }
        if stage.liquid_pct > 0.0 {
            let li = ledger.index(&field("liquid"), &stage.liquid)?;
            if li < n_flours {
                return Err(EngineError::Invalid {
                    field: field("liquid"),
                    reason: "a flour cannot be a stage liquid".into(),
                });
            }
            alloc.push((li, stage_flour * stage.liquid_pct / 100.0));
        }
        for (ei, extra) in stage.extras.iter().enumerate() {
            let idx =
                ledger.index(&field(&format!("extras[{ei}].ingredient")), &extra.ingredient)?;
            let grams = match extra.basis {
                Basis::StageFlour => stage_flour * extra.pct / 100.0,
                Basis::TotalFlour => extra.pct * f,
            };
            alloc.push((idx, grams));
        }

        let mut lines = Vec::with_capacity(alloc.len());
        let (mut water, mut mass) = (0.0, 0.0);
        for &(idx, grams) in &alloc {
            ledger.used[idx] += grams;
            water += grams * composition(idx).water;
            mass += grams;
            match lines.iter_mut().find(|l: &&mut Line| l.id == ledger.ids[idx]) {
                Some(line) => {
                    line.grams += grams;
                    line.pct = line.grams / f;
                }
                None => lines.push(Line { id: ledger.ids[idx].clone(), grams, pct: grams / f }),
            }
        }

        let evaporation_water_g = match stage.evaporation_pct {
            Some(e) => water * e / 100.0,
            None => {
                if stage.kind == StageKind::Scald {
                    warnings.push(Warning::ScaldEvaporationUnknown { stage: stage.id.clone() });
                }
                0.0
            }
        };

        let seed = match (&stage.seed, stage.kind) {
            (Some(seed), StageKind::Levain) => Some(split_seed(stage, seed, stage_flour, water)?),
            _ => None,
        };

        stages.push(StageSolution {
            id: stage.id.clone(),
            kind: stage.kind,
            flour_g: stage_flour,
            water_g: water,
            mass_g: mass,
            hydration_pct: if stage_flour > 0.0 { water / stage_flour * 100.0 } else { 0.0 },
            lines,
            evaporation_water_g,
            seed,
        });
    }

    for i in 0..ledger.ids.len() {
        let (used, total) = (ledger.used[i], ledger.total[i]);
        if used > total * (1.0 + 1e-9) + EPS {
            return Err(EngineError::Overallocated {
                ingredient: ledger.ids[i].clone(),
                needed_g: used,
                available_g: total,
            });
        }
    }

    let totals: Vec<Line> = ledger
        .ids
        .iter()
        .zip(&ledger.total)
        .map(|(id, &g)| Line { id: id.clone(), grams: g, pct: g / f })
        .collect();
    let mut final_mix: Vec<Line> = ledger
        .ids
        .iter()
        .enumerate()
        .filter_map(|(i, id)| {
            let g = (ledger.total[i] - ledger.used[i]).max(0.0);
            (g > EPS).then(|| Line { id: id.clone(), grams: g, pct: g / f })
        })
        .collect();
    final_mix.extend(stages.iter().map(|s| Line {
        id: s.id.clone(),
        grams: s.mass_g,
        pct: s.mass_g / f,
    }));

    let metrics = metrics(formula, &ledger, &stages, total_flour);
    Ok(Solution { totals, stages, final_mix, metrics, warnings })
}

fn split_seed(
    stage: &Stage,
    seed: &Seed,
    stage_flour: f64,
    stage_water: f64,
) -> Result<SeedSolution, EngineError> {
    let s = seed.pct / 100.0;
    let hs = seed.hydration_pct / 100.0;
    // stage flour = fresh + seed flour = fresh·(1 + s/(1+hs))
    let fresh_flour = stage_flour / (1.0 + s / (1.0 + hs));
    let seed_g = s * fresh_flour;
    let fresh_water = stage_water - seed_g * hs / (1.0 + hs);
    if fresh_water < -EPS {
        return Err(EngineError::SeedTooWet { stage: stage.id.clone() });
    }
    Ok(SeedSolution { seed_g, fresh_flour_g: fresh_flour, fresh_water_g: fresh_water.max(0.0) })
}

fn total_flour_g(formula: &Formula) -> Result<f64, EngineError> {
    let dough_per_flour = 1.0 + formula.ingredients.iter().map(|i| i.pct).sum::<f64>() / 100.0;
    let allowance = 1.0 + formula.allowance_pct / 100.0;
    let flour = match formula.target {
        Target::Flour { grams } => {
            check_range("target.grams", grams, EPS, MAX_G)?;
            return Ok(grams);
        }
        Target::Dough { grams } => {
            check_range("target.grams", grams, EPS, MAX_G)?;
            grams * allowance / dough_per_flour
        }
        Target::Pieces { count, grams } => {
            check_range("target.count", f64::from(count), 1.0, 10_000.0)?;
            check_range("target.grams", grams, EPS, MAX_G)?;
            f64::from(count) * grams * allowance / dough_per_flour
        }
        Target::Baked { count, grams, loss_pct } => {
            check_range("target.count", f64::from(count), 1.0, 10_000.0)?;
            check_range("target.grams", grams, EPS, MAX_G)?;
            check_range("target.loss_pct", loss_pct, 0.0, 60.0)?;
            f64::from(count) * grams / (1.0 - loss_pct / 100.0) * allowance / dough_per_flour
        }
    };
    check_range("total_flour_g", flour, EPS, MAX_G)?;
    Ok(flour)
}

fn validate(formula: &Formula) -> Result<(), EngineError> {
    if formula.flours.is_empty() {
        return Err(EngineError::NoFlour);
    }
    check_len("flours", formula.flours.len(), MAX_FLOURS)?;
    check_len("ingredients", formula.ingredients.len(), MAX_INGREDIENTS)?;
    check_len("stages", formula.stages.len(), MAX_STAGES)?;
    for (i, stage) in formula.stages.iter().enumerate() {
        check_len(&format!("stages[{i}].extras"), stage.extras.len(), MAX_STAGE_EXTRAS)?;
    }
    let mut seen: Vec<&str> = Vec::new();
    let ids = formula
        .flours
        .iter()
        .map(|f| f.id.as_str())
        .chain(formula.ingredients.iter().map(|i| i.id.as_str()))
        .chain(formula.stages.iter().map(|s| s.id.as_str()));
    for id in ids {
        if id.is_empty() {
            return Err(EngineError::Invalid { field: "id".into(), reason: "empty id".into() });
        }
        if id.len() > MAX_ID_LEN {
            return Err(EngineError::Invalid {
                field: "id".into(),
                reason: format!("longer than {MAX_ID_LEN} bytes"),
            });
        }
        if seen.contains(&id) {
            return Err(EngineError::DuplicateId { id: id.to_owned() });
        }
        seen.push(id);
    }
    for (i, fl) in formula.flours.iter().enumerate() {
        check_range(&format!("flours[{i}].pct"), fl.pct, 0.0, 100.0)?;
        if let Some(p) = fl.protein_pct {
            check_range(&format!("flours[{i}].protein_pct"), p, 0.0, 100.0)?;
        }
    }
    let sum: f64 = formula.flours.iter().map(|f| f.pct).sum();
    if (sum - 100.0).abs() > 0.01 {
        return Err(EngineError::FlourSum { sum });
    }
    for (i, ing) in formula.ingredients.iter().enumerate() {
        check_range(&format!("ingredients[{i}].pct"), ing.pct, 0.0, MAX_PCT)?;
        if let Some(c) = ing.composition {
            for (name, v) in [("water", c.water), ("fat", c.fat), ("sugar", c.sugar)] {
                check_range(&format!("ingredients[{i}].composition.{name}"), v, 0.0, 1.0)?;
            }
            if c.water + c.fat + c.sugar > 1.0 + 1e-9 {
                return Err(EngineError::Invalid {
                    field: format!("ingredients[{i}].composition"),
                    reason: "fractions add up to more than 1".into(),
                });
            }
        }
    }
    check_range("allowance_pct", formula.allowance_pct, 0.0, 50.0)?;
    for (i, st) in formula.stages.iter().enumerate() {
        check_range(&format!("stages[{i}].flour_pct"), st.flour_pct, 0.0, 100.0)?;
        check_range(&format!("stages[{i}].liquid_pct"), st.liquid_pct, 0.0, MAX_PCT)?;
        if let Some(e) = st.evaporation_pct {
            check_range(&format!("stages[{i}].evaporation_pct"), e, 0.0, 90.0)?;
        }
        for (ei, ex) in st.extras.iter().enumerate() {
            check_range(&format!("stages[{i}].extras[{ei}].pct"), ex.pct, 0.0, MAX_PCT)?;
            if ex.basis == Basis::StageFlour && st.flour_pct <= 0.0 && ex.pct > 0.0 {
                return Err(EngineError::Invalid {
                    field: format!("stages[{i}].extras[{ei}].basis"),
                    reason: "a stage without flour needs total_flour percentages".into(),
                });
            }
        }
        if let Some(seed) = st.seed {
            check_range(&format!("stages[{i}].seed.pct"), seed.pct, 0.0, MAX_PCT)?;
            check_range(
                &format!("stages[{i}].seed.hydration_pct"),
                seed.hydration_pct,
                0.0,
                MAX_PCT,
            )?;
            if st.kind != StageKind::Levain {
                return Err(EngineError::Invalid {
                    field: format!("stages[{i}].seed"),
                    reason: "only a levain has a seed".into(),
                });
            }
        }
    }
    Ok(())
}

fn metrics(
    formula: &Formula,
    ledger: &Ledger,
    stages: &[StageSolution],
    total_flour: f64,
) -> Metrics {
    let n_flours = formula.flours.len();
    let pct = |g: f64| g / total_flour * 100.0;
    let (mut water_only, mut water_all, mut salt, mut sugar, mut fat) = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut yeast: Vec<Line> = Vec::new();
    let mut ingredient_mass = 0.0;
    for (j, ing) in formula.ingredients.iter().enumerate() {
        let grams = ledger.total[n_flours + j];
        ingredient_mass += grams;
        let c = ing.composition();
        water_all += grams * c.water;
        sugar += grams * c.sugar;
        fat += grams * c.fat;
        match ing.kind {
            IngredientKind::Water => water_only += grams,
            IngredientKind::Salt => salt += grams,
            k if k.is_yeast() => yeast.push(Line { id: ing.id.clone(), grams, pct: pct(grams) }),
            _ => {}
        }
    }
    let flour_in = |kinds: &[StageKind]| -> f64 {
        stages.iter().filter(|s| kinds.contains(&s.kind)).map(|s| s.flour_g).sum()
    };
    let prefermented: f64 =
        stages.iter().filter(|s| s.kind.is_preferment()).map(|s| s.flour_g).sum();
    let levains = stages
        .iter()
        .filter(|s| s.kind == StageKind::Levain)
        .map(|s| LevainMetrics {
            mass_g: s.mass_g,
            hydration_pct: s.hydration_pct,
            inoculation_final_mix_pct: (total_flour - s.flour_g > EPS)
                .then(|| s.mass_g / (total_flour - s.flour_g) * 100.0),
            inoculation_total_flour_pct: pct(s.mass_g),
        })
        .collect();
    Metrics {
        total_flour_g: total_flour,
        dough_g: total_flour + ingredient_mass,
        hydration_water_pct: pct(water_only),
        hydration_total_pct: pct(water_all),
        dough_yield: 100.0 + pct(water_all),
        prefermented_flour_pct: pct(prefermented),
        scalded_flour_pct: pct(flour_in(&[StageKind::Scald])),
        salt_pct: pct(salt),
        sugar_pct: pct(sugar),
        fat_pct: pct(fat),
        yeast,
        levains,
    }
}
