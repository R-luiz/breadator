use serde::Serialize;

/// Input the engine cannot compute with. `code` (the serde tag) is stable and translated by the UI.
#[derive(Debug, Clone, PartialEq, Serialize, thiserror::Error)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum EngineError {
    #[error("{field}: {value} is outside {min}..={max}")]
    OutOfRange { field: String, value: f64, min: f64, max: f64 },
    #[error("flour percentages add up to {sum}, not 100")]
    FlourSum { sum: f64 },
    #[error("the formula has no flour")]
    NoFlour,
    #[error("id '{id}' is used twice")]
    DuplicateId { id: String },
    #[error("{field} refers to unknown ingredient '{id}'")]
    UnknownIngredient { field: String, id: String },
    #[error("stages use {needed_g:.2} g of '{ingredient}' but the formula has {available_g:.2} g")]
    Overallocated { ingredient: String, needed_g: f64, available_g: f64 },
    #[error("levain '{stage}': the seed brings more water than the levain's hydration allows")]
    SeedTooWet { stage: String },
    #[error("{field}: {reason}")]
    Invalid { field: String, reason: String },
}

/// Something the user should know about a valid input; `code` is stable and translated by the UI.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Warning {
    /// Water this warm damages yeast cells it touches; mix it with the flour first.
    WaterHotForYeast { water_c: f64 },
    /// No liquid water can be cold enough: part of the water has to be ice.
    IceNeeded { water_c: f64 },
    /// Even all the water as ice cannot reach the desired dough temperature.
    TargetUnreachable { best_dough_c: f64 },
    /// Evaporation was not given for a cooked scald; weigh it after cooking and top up with water.
    ScaldEvaporationUnknown { stage: String },
}
