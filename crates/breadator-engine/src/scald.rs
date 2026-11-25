//! Scalds: flour (or grains) pre-gelatinized with hot liquid before the final mix.
//!
//! A scald is a [`Stage`] of kind [`StageKind::Scald`]: its flour and liquid come out of the
//! formula's totals like any stage, so the bread's overall hydration does not change. What changes
//! is how the dough behaves: gelatinized starch binds water, so a scalded dough feels stiffer at
//! the same hydration and usually takes more water. Presets give the usual liquid:flour ratio and
//! temperature of each method; the scalded share of the flour stays the baker's choice.

use serde::{Deserialize, Serialize};

use crate::check_range;
use crate::error::EngineError;
use crate::formula::{Stage, StageKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaldMethod {
    /// Chinese/Japanese water roux: flour cooked in ~5 parts liquid to ~65 °C.
    Tangzhong,
    /// Japanese: flour mixed with about its weight of boiling water, rested (often overnight, cold).
    Yudane,
    /// French "empois": flour cooked with liquid into a paste.
    Empois,
    /// German Kochstück: flour or meal cooked in water to a paste.
    Kochstueck,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct ScaldPreset {
    pub method: ScaldMethod,
    /// Liquid per gram of scalded flour.
    pub liquid_ratio: f64,
    /// Temperature the paste is brought to, or of the liquid poured on, °C.
    pub temp_c: f64,
    /// Usual share of the total flour that is scalded, % (low, high).
    pub usual_flour_pct: (f64, f64),
}

pub fn preset(method: ScaldMethod) -> ScaldPreset {
    match method {
        ScaldMethod::Tangzhong => {
            ScaldPreset { method, liquid_ratio: 5.0, temp_c: 65.0, usual_flour_pct: (5.0, 10.0) }
        }
        ScaldMethod::Yudane => {
            ScaldPreset { method, liquid_ratio: 1.0, temp_c: 100.0, usual_flour_pct: (10.0, 25.0) }
        }
        ScaldMethod::Empois => {
            ScaldPreset { method, liquid_ratio: 5.0, temp_c: 65.0, usual_flour_pct: (5.0, 10.0) }
        }
        ScaldMethod::Kochstueck => {
            ScaldPreset { method, liquid_ratio: 5.0, temp_c: 90.0, usual_flour_pct: (3.0, 10.0) }
        }
    }
}

/// Builds the scald stage of a formula from a preset.
pub fn scald_stage(
    id: &str,
    method: ScaldMethod,
    flour_pct: f64,
    flour: Option<String>,
    liquid: &str,
    evaporation_pct: Option<f64>,
) -> Result<Stage, EngineError> {
    check_range("flour_pct", flour_pct, 0.0, 100.0)?;
    let p = preset(method);
    Ok(Stage {
        id: id.to_owned(),
        kind: StageKind::Scald,
        flour_pct,
        flour,
        liquid_pct: p.liquid_ratio * 100.0,
        liquid: liquid.to_owned(),
        extras: Vec::new(),
        evaporation_pct,
        seed: None,
    })
}
