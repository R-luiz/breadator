//! The tools the chat's model may call: some of the site's tools ([`crate::tools`]) in the format
//! of OpenRouter's `tools`, and what the chat keeps of each call.
//!
//! Whatever the model sends, a tool answers with JSON: a wrong argument comes back as
//! `{"error": …}` for the model to correct, never as a failure of the request.

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::tools::{self, SEARCH};
pub use crate::tools::{Context, Source};

/// The tools offered, in the order the model was benchmarked with
/// (`research/notes/chat-model/REPORT.md`).
const NAMES: [&str; 7] = [
    SEARCH,
    "compute_formula",
    "water_temperature",
    "predict_fermentation",
    "predict_starter",
    "blend_flour",
    "convert_yeast",
];

/// A tool the answer called, stored with it (`chat_messages.tool_calls`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolRecord {
    pub name: String,
    pub arguments: Value,
    pub ok: bool,
}

/// Progress line shown while a tool runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Search,
    Compute,
}

pub fn kind(name: &str) -> Kind {
    if name == SEARCH { Kind::Search } else { Kind::Compute }
}

/// The `tools` array of the request.
pub static SCHEMAS: LazyLock<Value> = LazyLock::new(|| {
    NAMES
        .iter()
        .filter_map(|name| tools::get(name))
        .map(|tool| {
            json!({
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": tool.parameters,
                },
            })
        })
        .collect()
});

/// Runs the tool `name` with the model's `arguments` (JSON text). The first value is what goes
/// back to the model. A search adds the sources of its passages to `sources`.
pub fn execute(
    ctx: &Context,
    name: &str,
    arguments: &str,
    sources: &mut Vec<Source>,
) -> (Value, ToolRecord) {
    let parsed = if arguments.trim().is_empty() {
        Ok(Value::Object(Map::new()))
    } else {
        serde_json::from_str::<Value>(arguments)
    };
    let (result, recorded) = match parsed {
        Ok(Value::Object(args)) => {
            let result = if NAMES.contains(&name) {
                tools::run(ctx, name, &args, sources)
            } else {
                Err(format!("unknown tool '{name}'"))
            };
            (result, Value::Object(args))
        }
        Ok(other) => (Err("the arguments must be a JSON object".to_owned()), other),
        Err(e) => (
            Err(format!("the arguments are not valid JSON: {e}")),
            Value::String(arguments.chars().take(500).collect()),
        ),
    };
    let ok = result.is_ok();
    let value = result.unwrap_or_else(|message| json!({ "error": message }));
    (value, ToolRecord { name: name.to_owned(), arguments: recorded, ok })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;
    use crate::knowledge::Knowledge;

    fn run(name: &str, arguments: &str) -> (Value, ToolRecord) {
        let knowledge = Knowledge::fixture();
        let ctx = Context { knowledge: &knowledge, lang: Lang::En, site: None };
        execute(&ctx, name, arguments, &mut Vec::new())
    }

    #[test]
    fn passages_reach_the_model_without_links() {
        // The page lists an answer's sources itself; the model is given none to write.
        let (value, record) = run(SEARCH, r#"{"queries":["yudane"]}"#);
        assert!(record.ok, "{value}");
        let passages = value["passages"].as_array().unwrap();
        assert_eq!(passages.len(), 3, "{value}");
        for passage in passages {
            let fields: Vec<&str> =
                passage.as_object().unwrap().keys().map(String::as_str).collect();
            assert!(
                fields == ["text", "title"] || fields == ["evidence", "text", "title"],
                "{passage}"
            );
        }
        assert!(value.get("sources").is_none(), "{value}");
    }

    #[test]
    fn the_model_is_offered_its_seven_tools() {
        let names: Vec<&str> = SCHEMAS
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["function"]["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, NAMES);
        for tool in SCHEMAS.as_array().unwrap() {
            assert_eq!(tool["function"]["parameters"]["type"], "object", "{tool}");
            assert!(tool["function"]["description"].as_str().is_some_and(|d| d.len() > 20));
        }
    }

    #[test]
    fn a_call_is_recorded_with_its_arguments_and_outcome() {
        let (value, record) =
            run("convert_yeast", r#"{"grams":21,"from":"fresh_yeast","to":"instant_yeast"}"#);
        assert_eq!(value["grams"], 6.93);
        assert!(record.ok);
        assert_eq!(record.name, "convert_yeast");
        assert_eq!(record.arguments["from"], "fresh_yeast");
        // No arguments at all is an empty object.
        let (value, record) = run("predict_starter", "  ");
        assert!(record.ok, "{value}");
    }

    #[test]
    fn what_cannot_run_comes_back_as_an_error_for_the_model() {
        for (name, arguments, expected) in [
            ("compute_formula", r#"{"by":"dough","#, "not valid JSON"),
            ("compute_formula", "[1, 2]", "must be a JSON object"),
            ("compute_formula", r#"{"by":"dough","grams":900,"hydration":70}"#, "'hydration'"),
            ("starter_plan", "{}", "unknown tool"),
            // A tool of the site that the chat's model was not given.
            ("list_recipes", "{}", "unknown tool"),
        ] {
            let (value, record) = run(name, arguments);
            assert!(!record.ok, "{name} {arguments}: {value}");
            let message = value["error"].as_str().unwrap_or_else(|| panic!("{name}: {value}"));
            assert!(message.contains(expected), "{name} {arguments}: {message}");
        }
    }
}
