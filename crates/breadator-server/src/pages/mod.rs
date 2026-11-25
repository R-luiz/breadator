//! Pages and JSON API.

pub mod api;
pub mod fermentation;
pub mod flour;
pub mod formula;
pub mod home;
pub mod learn;
pub mod recipes;
pub mod starter;
pub mod temperature;
pub mod yeast;

use breadator_engine::{EngineError, Warning};
use fluent_templates::fluent_bundle::FluentValue;

use crate::i18n::Tr;

/// Display name of a formula entry: a translation for known ids (`water`, `levain`…), else the
/// id itself (user-named flours and ingredients).
pub fn entry_name(tr: &Tr, id: &str) -> String {
    tr.try_t(&format!("entry-{id}")).unwrap_or_else(|| id.to_owned())
}

/// Label of a form field named in an engine error (`field-water`, `field-target-grams`…), else
/// the raw field path. Fluent ids cannot contain dots, so `target.grams` looks up `target-grams`.
fn field_name(tr: &Tr, field: &str) -> String {
    tr.try_t(&format!("field-{}", field.replace('.', "-"))).unwrap_or_else(|| field.to_owned())
}

pub fn describe_error(tr: &Tr, e: &EngineError) -> String {
    let n = |v: f64| FluentValue::from(tr.num(v, 1));
    match e {
        EngineError::OutOfRange { field, value, min, max } => tr.t_args(
            "err-out-of-range",
            &[
                ("field", field_name(tr, field).into()),
                ("value", n(*value)),
                ("min", n(*min)),
                ("max", n(*max)),
            ],
        ),
        EngineError::FlourSum { sum } => tr.t_args("err-flour-sum", &[("sum", n(*sum))]),
        EngineError::NoFlour => tr.t("err-no-flour"),
        EngineError::DuplicateId { id } => tr.t_args("err-duplicate", &[("id", id.clone().into())]),
        EngineError::UnknownIngredient { id, .. } => {
            tr.t_args("err-unknown", &[("id", id.clone().into())])
        }
        EngineError::Overallocated { ingredient, needed_g, available_g } => tr.t_args(
            "err-overallocated",
            &[
                ("ingredient", entry_name(tr, ingredient).into()),
                ("needed", tr.g(*needed_g).into()),
                ("available", tr.g(*available_g).into()),
            ],
        ),
        EngineError::SeedTooWet { stage } => {
            tr.t_args("err-seed-too-wet", &[("stage", entry_name(tr, stage).into())])
        }
        EngineError::Invalid { field, reason } => tr.t_args(
            "err-invalid",
            &[("field", field_name(tr, field).into()), ("reason", reason.clone().into())],
        ),
    }
}

pub fn describe_warning(tr: &Tr, w: &Warning) -> String {
    match w {
        Warning::WaterHotForYeast { water_c } => {
            tr.t_args("warn-water-hot", &[("t", tr.num(*water_c, 1).into())])
        }
        Warning::IceNeeded { water_c } => {
            tr.t_args("warn-ice-needed", &[("t", tr.num(*water_c, 1).into())])
        }
        Warning::TargetUnreachable { best_dough_c } => {
            tr.t_args("warn-unreachable", &[("t", tr.num(*best_dough_c, 1).into())])
        }
        Warning::ScaldEvaporationUnknown { stage } => {
            tr.t_args("warn-scald-evaporation", &[("stage", entry_name(tr, stage).into())])
        }
    }
}
