//! Browser bindings of `breadator-engine`.
//!
//! Every export takes the engine input as a JSON string and returns a JSON string:
//! `{"ok": <result>}` or `{"error": <EngineError>}` (same shapes as the server's JSON API), so the
//! islands share one decoding path with server-rendered data.

use breadator_engine::EngineError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Reply<T> {
    Ok(T),
    Error(EngineError),
}

fn call<I: DeserializeOwned, O: Serialize>(
    input: &str,
    f: impl FnOnce(I) -> Result<O, EngineError>,
) -> String {
    let reply = match serde_json::from_str::<I>(input) {
        Ok(i) => match f(i) {
            Ok(o) => Reply::Ok(o),
            Err(e) => Reply::Error(e),
        },
        Err(e) => {
            Reply::Error(EngineError::Invalid { field: "input".into(), reason: e.to_string() })
        }
    };
    serde_json::to_string(&reply).unwrap_or_else(|e| {
        format!(r#"{{"error":{{"code":"invalid","field":"output","reason":"{e}"}}}}"#)
    })
}

#[wasm_bindgen]
pub fn solve_formula(input: &str) -> String {
    call(input, |f: breadator_engine::formula::Formula| breadator_engine::formula::solve(&f))
}

#[derive(Serialize)]
struct QuickReply {
    formula: breadator_engine::formula::Formula,
    solution: breadator_engine::formula::Solution,
}

/// The calculator's simple form: builds the formula and solves it.
#[wasm_bindgen]
pub fn solve_quick(input: &str) -> String {
    call(input, |q: breadator_engine::quick::QuickForm| {
        let formula = q.to_formula()?;
        let solution = breadator_engine::formula::solve(&formula)?;
        Ok(QuickReply { formula, solution })
    })
}

#[wasm_bindgen]
pub fn water_temperature(input: &str) -> String {
    call(input, |i: breadator_engine::temperature::WaterTempInput| {
        breadator_engine::temperature::water_temperature(&i)
    })
}

/// The water-temperature calculator's simple form.
#[wasm_bindgen]
pub fn water_temperature_quick(input: &str) -> String {
    call(input, |q: breadator_engine::temperature::QuickTemperature| {
        breadator_engine::temperature::water_temperature(&q.to_input())
    })
}

#[derive(serde::Deserialize)]
struct YeastInput {
    grams: f64,
    from: breadator_engine::ingredients::IngredientKind,
    to: breadator_engine::ingredients::IngredientKind,
}

#[wasm_bindgen]
pub fn convert_yeast(input: &str) -> String {
    call(input, |i: YeastInput| breadator_engine::yeast::convert(i.grams, i.from, i.to))
}

/// The fermentation calculator: time for a dose, or dose for a time.
#[wasm_bindgen]
pub fn ferment_quick(input: &str) -> String {
    call(input, |q: breadator_engine::leaven::QuickFerment| q.solve())
}

/// The starter calculator: peak time for a feed, or the feed for a peak time.
#[wasm_bindgen]
pub fn starter_quick(input: &str) -> String {
    call(input, |q: breadator_engine::leaven::QuickStarter| q.solve())
}

/// The flour calculator: protein and starting hydration of a blend with its additions.
#[wasm_bindgen]
pub fn flour_quick(input: &str) -> String {
    call(input, |q: breadator_engine::flour::QuickFlour| q.solve())
}

#[cfg(test)]
mod tests {
    #[test]
    fn replies_are_tagged_json() {
        let ok = super::convert_yeast(r#"{"grams":10,"from":"fresh_yeast","to":"instant_yeast"}"#);
        assert!(ok.starts_with(r#"{"ok":{"grams":3.3"#), "{ok}");
        let err = super::solve_formula("{}");
        assert!(err.starts_with(r#"{"error":{"code":"invalid""#), "{err}");
    }
}
