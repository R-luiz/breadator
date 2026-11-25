//! The quick formula: the common bread in a dozen fields (the calculator's simple form).
//!
//! Covers a size target, one or two flours, water, salt, commercial yeast, a levain given by its
//! inoculation, a poolish and one scald. Shared by the server (no-JavaScript form) and the browser
//! (WASM island) so both build exactly the same [`Formula`].

use serde::{Deserialize, Serialize};

use crate::check_range;
use crate::error::EngineError;
use crate::formula::{
    Basis, Flour, Formula, Ingredient, Inoculation, Stage, StageExtra, StageKind, Target,
    levain_flour_pct,
};
use crate::ingredients::IngredientKind;
use crate::scald::{ScaldMethod, scald_stage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickTarget {
    Dough,
    Pieces,
    Baked,
    Flour,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickForm {
    pub by: QuickTarget,
    /// Dough, piece, loaf or flour weight depending on `by`, g.
    pub grams: f64,
    /// Pieces or loaves (`pieces`, `baked`).
    pub count: f64,
    /// Bake loss for `baked`, %.
    pub loss: f64,
    pub flour1: String,
    /// Empty when there is a single flour.
    pub flour2: String,
    pub flour2_pct: f64,
    pub water: f64,
    pub salt: f64,
    /// `None` without commercial yeast.
    pub yeast_kind: Option<IngredientKind>,
    /// Total commercial yeast, % of flour.
    pub yeast: f64,
    /// Levain mass, % of the final-mix flour (0 = no levain).
    pub levain: f64,
    pub levain_hyd: f64,
    /// Flour fermented in a 100 % hydration poolish, % of total flour (0 = none).
    pub poolish: f64,
    /// Yeast in the poolish, % of its flour (taken from the total yeast).
    pub poolish_yeast: f64,
    pub scald_method: Option<ScaldMethod>,
    /// Scalded flour, % of total flour.
    pub scald: f64,
}

impl Default for QuickForm {
    fn default() -> Self {
        Self {
            by: QuickTarget::Dough,
            grams: 900.0,
            count: 1.0,
            loss: 14.0,
            flour1: "flour".to_owned(),
            flour2: String::new(),
            flour2_pct: 0.0,
            water: 70.0,
            salt: 2.0,
            yeast_kind: None,
            yeast: 0.0,
            levain: 20.0,
            levain_hyd: 100.0,
            poolish: 0.0,
            poolish_yeast: 0.1,
            scald_method: None,
            scald: 0.0,
        }
    }
}

impl QuickForm {
    /// Builds the formula. Errors name the form's own fields (`flour2_pct`, `levain`…), so the
    /// interface can point at the field the user typed in.
    pub fn to_formula(&self) -> Result<Formula, EngineError> {
        for (field, value, max) in [
            ("flour2_pct", self.flour2_pct, 100.0),
            ("poolish", self.poolish, 100.0),
            ("scald", self.scald, 100.0),
            ("water", self.water, 1000.0),
            ("salt", self.salt, 100.0),
            ("yeast", self.yeast, 100.0),
            ("poolish_yeast", self.poolish_yeast, 100.0),
            ("levain", self.levain, 1000.0),
            ("levain_hyd", self.levain_hyd, 1000.0),
        ] {
            check_range(field, value, 0.0, max)?;
        }
        let count = || -> Result<u32, EngineError> {
            if self.count.fract() == 0.0 && (1.0..=10_000.0).contains(&self.count) {
                Ok(self.count as u32)
            } else {
                Err(EngineError::OutOfRange {
                    field: "count".into(),
                    value: self.count,
                    min: 1.0,
                    max: 10_000.0,
                })
            }
        };
        let target = match self.by {
            QuickTarget::Dough => Target::Dough { grams: self.grams },
            QuickTarget::Pieces => Target::Pieces { count: count()?, grams: self.grams },
            QuickTarget::Baked => {
                Target::Baked { count: count()?, grams: self.grams, loss_pct: self.loss }
            }
            QuickTarget::Flour => Target::Flour { grams: self.grams },
        };
        let flour = |id: &str, pct| Flour { id: id.to_owned(), pct, kind: None, protein_pct: None };
        let mut flours = vec![flour(self.flour1.trim(), 100.0 - self.flour2_pct)];
        if self.flour2_pct > 0.0 {
            let name = self.flour2.trim();
            flours.push(flour(if name.is_empty() { "flour2" } else { name }, self.flour2_pct));
        }
        let ing =
            |id: &str, kind, pct| Ingredient { id: id.to_owned(), kind, pct, composition: None };
        let mut ingredients = vec![
            ing("water", IngredientKind::Water, self.water),
            ing("salt", IngredientKind::Salt, self.salt),
        ];
        if let Some(kind) = self.yeast_kind {
            if !kind.is_yeast() {
                return Err(EngineError::Invalid {
                    field: "yeast_kind".into(),
                    reason: "not a commercial yeast".into(),
                });
            }
            ingredients.push(ing("yeast", kind, self.yeast));
        }
        let mut stages = Vec::new();
        if self.levain > 0.0 {
            stages.push(Stage {
                id: "levain".into(),
                kind: StageKind::Levain,
                flour_pct: levain_flour_pct(
                    self.levain,
                    self.levain_hyd,
                    Inoculation::FinalMixFlour,
                )?,
                flour: None,
                liquid_pct: self.levain_hyd,
                liquid: "water".into(),
                extras: vec![],
                evaporation_pct: None,
                seed: None,
            });
        }
        if self.poolish > 0.0 {
            let extras = match self.yeast_kind {
                Some(_) if self.poolish_yeast > 0.0 => {
                    vec![StageExtra {
                        ingredient: "yeast".into(),
                        pct: self.poolish_yeast,
                        basis: Basis::StageFlour,
                    }]
                }
                _ => vec![],
            };
            stages.push(Stage {
                id: "poolish".into(),
                kind: StageKind::Poolish,
                flour_pct: self.poolish,
                flour: None,
                liquid_pct: 100.0,
                liquid: "water".into(),
                extras,
                evaporation_pct: None,
                seed: None,
            });
        }
        if let Some(method) = self.scald_method.filter(|_| self.scald > 0.0) {
            stages.push(scald_stage("scald", method, self.scald, None, "water", None)?);
        }
        Ok(Formula { flours, ingredients, stages, target, allowance_pct: 0.0 })
    }
}
