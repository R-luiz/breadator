//! Breadator's bread-making engine.
//!
//! Pure computation shared by the server, the browser (WebAssembly) and the chatbot's tools.
//! Units: grams, degrees Celsius, hours; percentages are baker's percentages (total flour = 100)
//! unless a field says otherwise. Every entry point validates its input and returns
//! [`EngineError`] instead of panicking; soft problems come back as [`Warning`]s with stable codes
//! that the user interface translates.

pub mod error;
pub mod ferment;
pub mod flour;
pub mod formula;
pub mod ingredients;
pub mod leaven;
pub mod preferment;
pub mod quick;
pub mod scald;
pub mod temperature;
pub mod yeast;

pub use error::{EngineError, Warning};

/// Relative tolerance used when checking that allocations fit in their totals.
pub(crate) const EPS: f64 = 1e-9;

/// Rejects NaN, infinities and values outside `min..=max`.
pub(crate) fn check_range(field: &str, value: f64, min: f64, max: f64) -> Result<(), EngineError> {
    if value.is_finite() && value >= min && value <= max {
        Ok(())
    } else {
        Err(EngineError::OutOfRange { field: field.to_owned(), value, min, max })
    }
}
