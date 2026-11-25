//! Ingredient kinds and their default composition.
//!
//! Composition values (mass fractions) come from USDA FoodData Central, SR Legacy
//! (`usda-fdc-sr-legacy` in the research references): whole milk 01077, heavy cream 01053,
//! plain whole-milk yogurt 01116, whole egg 01123, yolk 01125, white 01124, salted butter 01001,
//! honey 19296, granulated sugar 19335, compressed yeast 18374, active dry yeast 18375, regular beer
//! 14003. Rounded to two decimals; any ingredient can override them.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IngredientKind {
    Water,
    Milk,
    Cream,
    Yogurt,
    Beer,
    Egg,
    EggYolk,
    EggWhite,
    Butter,
    Oil,
    Sugar,
    Honey,
    Salt,
    FreshYeast,
    ActiveDryYeast,
    InstantYeast,
    /// Instant yeast selected for doughs above ~5–10 % sugar.
    OsmotolerantYeast,
    /// Seeds, nuts, dried fruit, cheese…: never counted as flour or liquid.
    Inclusion,
    Other,
}

/// Mass fractions (0..=1) of the components that matter to dough maths.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Composition {
    /// Free water: counted in the total hydration.
    pub water: f64,
    pub fat: f64,
    /// Sugars; lactose (milk) is included but baker's yeast does not ferment it.
    pub sugar: f64,
}

impl IngredientKind {
    pub fn default_composition(self) -> Composition {
        let c = |water, fat, sugar| Composition { water, fat, sugar };
        match self {
            Self::Water => c(1.0, 0.0, 0.0),
            Self::Milk => c(0.88, 0.03, 0.05),
            Self::Cream => c(0.58, 0.36, 0.03),
            Self::Yogurt => c(0.88, 0.03, 0.04),
            Self::Beer => c(0.92, 0.0, 0.0),
            Self::Egg => c(0.76, 0.10, 0.0),
            Self::EggYolk => c(0.52, 0.27, 0.01),
            Self::EggWhite => c(0.88, 0.0, 0.0),
            Self::Butter => c(0.16, 0.81, 0.0),
            Self::Oil => c(0.0, 1.0, 0.0),
            Self::Sugar => c(0.0, 0.0, 1.0),
            Self::Honey => c(0.17, 0.0, 0.82),
            // Yeast water is real but conventionally left out of hydration (1–3 % of flour).
            Self::FreshYeast
            | Self::ActiveDryYeast
            | Self::InstantYeast
            | Self::OsmotolerantYeast => c(0.0, 0.0, 0.0),
            Self::Salt | Self::Inclusion | Self::Other => c(0.0, 0.0, 0.0),
        }
    }

    pub fn is_yeast(self) -> bool {
        matches!(
            self,
            Self::FreshYeast | Self::ActiveDryYeast | Self::InstantYeast | Self::OsmotolerantYeast
        )
    }
}
