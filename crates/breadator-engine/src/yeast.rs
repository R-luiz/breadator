//! Conversions between commercial yeast forms.
//!
//! Each form has a fresh-yeast equivalence factor: grams of that form replacing 1 g of fresh
//! (compressed) yeast. Sources disagree (professional guidance 3–4:1 fresh:dry, consumer packs
//! 2.4:1), so every conversion carries a range: `research/data/model-fermentation.json`,
//! `commercial_yeast.yeast_equivalence` (Lallemand Baking Updates 1/6 and 2/9, Red Star,
//! Fleischmann's): fresh → instant ×0.33 (0.25–0.42), active dry → instant ×0.75 (0.67–1.0).
//! Sourdough has no fixed yeast equivalent: its timing comes from the fermentation model.

use serde::Serialize;

use crate::check_range;
use crate::error::EngineError;
use crate::ingredients::IngredientKind;

/// Grams of a yeast form replacing 1 g of fresh yeast: (typical, low, high).
pub fn fresh_equivalence(kind: IngredientKind) -> Option<(f64, f64, f64)> {
    match kind {
        IngredientKind::FreshYeast => Some((1.0, 1.0, 1.0)),
        // 0.33 g instant per g fresh, and 1.0–1.49 g active dry per g instant.
        IngredientKind::ActiveDryYeast => Some((0.44, 0.33, 0.50)),
        IngredientKind::InstantYeast | IngredientKind::OsmotolerantYeast => {
            Some((0.33, 0.25, 0.42))
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Conversion {
    pub grams: f64,
    pub low_g: f64,
    pub high_g: f64,
}

pub fn convert(
    grams: f64,
    from: IngredientKind,
    to: IngredientKind,
) -> Result<Conversion, EngineError> {
    check_range("grams", grams, 0.0, 1.0e5)?;
    let not_yeast = |field: &str, kind: IngredientKind| EngineError::Invalid {
        field: field.to_owned(),
        reason: format!("{kind:?} is not a commercial yeast"),
    };
    let (f_from, from_lo, from_hi) =
        fresh_equivalence(from).ok_or_else(|| not_yeast("from", from))?;
    let (f_to, to_lo, to_hi) = fresh_equivalence(to).ok_or_else(|| not_yeast("to", to))?;
    let fresh = grams / f_from;
    Ok(Conversion {
        grams: fresh * f_to,
        // Widest plausible spread: the source form at its strongest/weakest, the target at its weakest/strongest.
        low_g: grams / from_hi * to_lo,
        high_g: grams / from_lo * to_hi,
    })
}
