//! The model's tools, run on the server: the knowledge base search and the engine's calculators.
//!
//! The model never computes and never quotes from memory: a number comes from the engine, a
//! fact from a passage. Whatever the model sends, a tool answers with JSON: a wrong argument
//! comes back as `{"error": …}` for the model to correct, never as a failure of the request.

use std::fmt::Write as _;
use std::sync::LazyLock;

use breadator_content::search::{Hit, Index};
use breadator_engine::flour::{self, QuickFlour};
use breadator_engine::ingredients::IngredientKind;
use breadator_engine::leaven::{QuickFerment, QuickStarter};
use breadator_engine::quick::QuickForm;
use breadator_engine::temperature::{QuickTemperature, water_temperature};
use breadator_engine::{formula, yeast};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value, json};

use super::passages;
use crate::i18n::Lang;
use crate::knowledge::{Knowledge, cite};

const SEARCH: &str = "search_knowledge";
/// Passages returned by one search.
const MAX_PASSAGES: usize = 6;
/// Paragraphs quoted from one article.
const ARTICLE_BLOCKS: usize = 2;
/// A passage stops at the first cited statement that ends beyond this many bytes.
const PASSAGE_BYTES: usize = 900;

/// What a statement of a passage rests on. The model cites it as `[n]`; cited ones are stored
/// with the answer (`chat_messages.citations`) and listed under it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub n: usize,
    /// Bibliography keys and claim ids, as in an article's `[@key]`. Empty when the passage
    /// cites nothing itself: the article or recipe page is then the source.
    pub keys: Vec<String>,
    /// Where the passage was found: `article`, `recipe` or `claim`, and its slug or id.
    pub kind: String,
    pub id: String,
    /// Title of the article or recipe.
    pub title: String,
}

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
    let number = |description: &str| json!({ "type": "number", "description": description });
    let choice = |values: &[&str], description: &str| json!({ "type": "string", "enum": values, "description": description });
    let tool = |name: &str, description: &str, properties: Value, required: &[&str]| {
        json!({
            "type": "function",
            "function": {
                "name": name,
                "description": description,
                "parameters": { "type": "object", "properties": properties, "required": required },
            },
        })
    };
    let yeasts = ["fresh_yeast", "active_dry_yeast", "instant_yeast", "osmotolerant_yeast"];
    let flour_ids: Vec<&str> = flour::types().iter().map(|t| t.id).collect();
    let addition_kinds: Vec<&str> = flour::additions().iter().map(|a| a.kind).collect();
    // A list of flours with their shares: a blend, or the flours of a reference recipe.
    let blend = |description: &str| {
        json!({
            "type": "array",
            "description": description,
            "items": {
                "type": "object",
                "properties": {
                    "id": choice(&flour_ids, "Flour type (fr_ = French T types, de_ = German, it_ = Italian, us_ = US)."),
                    "pct": number("Share of the blend."),
                    "protein_pct": number("Protein printed on the bag, % (optional)."),
                },
                "required": ["id", "pct"],
            },
        })
    };
    json!([
        tool(
            SEARCH,
            "Search Breadator's knowledge base: articles, recipes and statements taken from papers, books and bakers' documented tests. Returns passages in which each statement is followed by its source number, e.g. [3].",
            json!({
                "queries": {
                    "type": "array",
                    "items": { "type": "string" },
                    "minItems": 1,
                    "maxItems": 3,
                    "description": "1 to 3 keyword queries of 1 to 4 words, e.g. [\"salt yeast fermentation\", \"sel levure fermentation\"]. Most sources are in English: always include an English query.",
                },
            }),
            &["queries"],
        ),
        tool(
            "compute_formula",
            "Exact weights of a bread formula given in baker's percentages (all the flour of the bread = 100 %, the flour inside the levain, poolish and scald included). Returns the totals, each stage and the final mix, in grams.",
            json!({
                "by": choice(&["dough", "pieces", "baked", "flour"], "What `grams` is: the whole dough, one dough piece, one baked loaf, or the total flour."),
                "grams": number("Weight in grams, see `by`."),
                "count": number("Number of pieces or loaves (by = pieces or baked)."),
                "loss": number("Bake loss, % (by = baked; default 14)."),
                "water": number("Total water, % of flour (the hydration)."),
                "salt": number("% of flour (default 2)."),
                "yeast_kind": choice(&yeasts, "Commercial yeast form; required with `yeast`."),
                "yeast": number("Commercial yeast, % of flour."),
                "levain": number("Levain (ripe starter) weight, % of the flour added at the final mix. 0 or omitted: no levain."),
                "levain_hyd": number("Hydration of the levain, % (default 100)."),
                "flour2": { "type": "string", "description": "Name of a second flour, e.g. rye." },
                "flour2_pct": number("Second flour, % of total flour."),
                "poolish": number("Flour prefermented in a 100 % hydration poolish, % of total flour."),
                "poolish_yeast": number("Yeast in the poolish, % of the poolish flour (default 0.1)."),
                "scald_method": choice(&["tangzhong", "yudane", "empois", "kochstueck"], "Required with `scald`."),
                "scald": number("Scalded flour, % of total flour."),
            }),
            &["by", "grams", "water"],
        ),
        tool(
            "water_temperature",
            "Water temperature that gives the desired dough temperature after mixing.",
            json!({
                "method": choice(&["balance", "classic"], "balance (default): heat balance from the weights; classic: base-temperature rule with a friction factor."),
                "desired": number("Desired dough temperature, °C."),
                "flour_c": number("Flour temperature, °C."),
                "room_c": number("Room temperature, °C (classic)."),
                "flour_g": number("Flour of the final mix, g (balance; default 500)."),
                "water_g": number("Water of the final mix, g (balance; default 350)."),
                "pref_g": number("Levain or preferment added, g (default 0: none)."),
                "pref_c": number("Its temperature, °C."),
                "pref_hyd": number("Its hydration, % (default 100)."),
                "rise": number("Temperature rise caused by mixing, °C (balance; default 0)."),
                "friction": number("Friction factor, °C (classic; default 0)."),
                "tap_c": number("Coldest tap water available, °C: if the water must be colder, the result gives ice and water weights."),
            }),
            &["desired", "flour_c"],
        ),
        tool(
            "predict_fermentation",
            "Time for a dough to reach an endpoint, or the dose of leavening that gets it there in a given time, from Breadator's fermentation model. Returns hours with the range 80 % of bakes fall in.",
            json!({
                "solve": choice(&["time", "dose"], "time (default): hours for `dose_pct`; dose: the dose reaching the endpoint in `target_h`."),
                "leavening": choice(&["sourdough", "instant_yeast", "active_dry_yeast", "fresh_yeast", "osmotolerant_yeast"], "Default sourdough."),
                "dose_pct": number("Sourdough: ripe starter weight, % of the flour added (default 20). Yeast: % of flour, in its own form."),
                "starter_hydration_pct": number("Default 100."),
                "starter_state": choice(&["ripe", "from_fridge"], "Default ripe."),
                "target_h": number("Hours from mixing to the endpoint (solve = dose)."),
                "endpoint": choice(&["rise25", "rise50", "rise75", "doubled", "tripled", "ripe_poolish"], "Volume rise that ends the fermentation. Sourdough: rise25 to doubled (default rise50). Yeast: rise50 to tripled, or ripe_poolish."),
                "dough_c": number("Dough temperature after mixing, °C. With a single temperature, give it here."),
                "room_c": number("Room temperature, °C (default: same as dough_c)."),
                "dough_g": number("Dough weight, g (default 900)."),
                "container": choice(&["tub", "banneton", "thick_bowl", "pizza_tray", "bare_tray"], "Default tub."),
                "warm_h": number("Hours at room temperature before the fridge."),
                "fridge_h": number("Hours in the fridge (default 0: none); the dough then finishes at room temperature."),
                "fridge_c": number("Fridge temperature, °C (default 4)."),
                "dough": {
                    "type": "object",
                    "description": "Dough composition, % of flour, when it is not a lean white dough with 2 % salt and 70 % hydration.",
                    "properties": {
                        "salt_pct": { "type": "number" },
                        "sugar_pct": { "type": "number" },
                        "fat_pct": { "type": "number" },
                        "hydration_pct": { "type": "number" },
                        "whole_grain_pct": { "type": "number" },
                        "milk": { "type": "boolean" },
                        "eggs": { "type": "boolean" },
                    },
                },
            }),
            &["leavening"],
        ),
        tool(
            "predict_starter",
            "When a refreshed (fed) sourdough starter peaks for a feeding ratio and temperature, or the feeding ratio that makes it peak after a given time, from Breadator's fermentation model; optionally the grams to weigh. Returns hours with the range 80 % of refreshes fall in.",
            json!({
                "solve": choice(&["time", "dose"], "time (default): when it peaks with flour_parts and water_parts; dose: the feeding ratio peaking at target_h."),
                "flour_parts": number("Fresh flour per part of starter, e.g. 5 for a 1:5:5 feed (solve = time; default 5)."),
                "water_parts": number("Fresh water per part of starter, e.g. 5 for a 1:5:5 feed (solve = time; default 5)."),
                "seed_hydration_pct": number("Hydration of the starter being fed, % (default 100)."),
                "hydration_pct": number("Hydration wanted for the refreshed starter, % (solve = dose; default 100)."),
                "temp_c": number("Temperature of the starter, °C (default 24)."),
                "target_h": number("Hours from feeding to peak (solve = dose)."),
                "seed_state": choice(&["ripe", "from_fridge"], "State of the starter being fed (default ripe)."),
                "need_g": number("Grams of refreshed starter wanted: the result then gives the seed, flour and water to weigh (default 0: none)."),
            }),
            &[],
        ),
        tool(
            "blend_flour",
            "Protein of a blend of flour types and a hydration to start from with its range, from Breadator's flour table; the water that bran, seeds, psyllium, vital gluten or a scald add; the vital gluten that reaches a protein target. With `reference` (a recipe that already works) the hydration is carried over from that recipe, which is more precise.",
            json!({
                "flours": blend("The blend. Shares are relative: 80 and 20, or grams."),
                "total_flour_g": number("Total flour, g (default 1000)."),
                "reference": {
                    "type": "object",
                    "description": "A recipe the user already makes with success.",
                    "properties": {
                        "flours": blend("The flours of that recipe."),
                        "hydration_pct": number("The hydration of that recipe, %."),
                    },
                },
                "additions": {
                    "type": "array",
                    "description": "What is added to the flour.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "kind": choice(&addition_kinds, "The addition."),
                            "pct": number("% of the flour."),
                        },
                        "required": ["kind", "pct"],
                    },
                },
                "scald_pct": number("Flour scalded, % of the total flour (default 0)."),
                "scald": choice(&["yudane", "hot", "cooked"], "yudane: boiling water, about 1:1 (default); hot: held at 60-70 °C; cooked: paste cooked to 75 °C or more (tangzhong)."),
                "target_protein_pct": number("Protein wanted for the blend, %: the result then gives the vital gluten to add."),
                "gluten_protein_pct": number("Protein of the vital gluten, % (default 75)."),
            }),
            &["flours"],
        ),
        tool(
            "convert_yeast",
            "Convert an amount of commercial yeast to another form, with the range manufacturers give.",
            json!({
                "grams": number("Amount to convert, g."),
                "from": choice(&yeasts, "Form the amount is in."),
                "to": choice(&yeasts, "Form wanted."),
            }),
            &["grams", "from", "to"],
        ),
    ])
});

/// Runs the tool `name` with the model's `arguments` (JSON text). The first value is what goes
/// back to the model. A search adds the sources of its passages to `sources`.
pub fn execute(
    knowledge: &Knowledge,
    lang: Lang,
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
            let result = match name {
                SEARCH => search(knowledge, lang, &args, sources),
                "compute_formula" => compute_formula(&args),
                "water_temperature" => temperature(&args),
                "predict_fermentation" => fermentation(&args),
                "convert_yeast" => convert_yeast(&args),
                "predict_starter" => starter(&args),
                "blend_flour" => flour_blend(&args),
                other => Err(format!("unknown tool '{other}'")),
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

// ---- Arguments ---------------------------------------------------------------------------

/// `base` with the model's arguments laid over it. An argument that `base` does not have is an
/// error naming the ones it has: a misspelt name must not silently fall back to a default.
fn overlay<T: Serialize + DeserializeOwned>(
    base: &T,
    hidden: &[&str],
    args: &Map<String, Value>,
) -> Result<T, String> {
    let Ok(Value::Object(mut fields)) = serde_json::to_value(base) else {
        return Err("the tool is unavailable".to_owned());
    };
    for name in hidden {
        fields.remove(*name);
    }
    merge(&mut fields, args, "")?;
    serde_json::from_value(Value::Object(fields)).map_err(|e| e.to_string())
}

fn merge(
    fields: &mut Map<String, Value>,
    args: &Map<String, Value>,
    path: &str,
) -> Result<(), String> {
    for (name, value) in args {
        // `null` is how some models write "not given".
        if value.is_null() {
            continue;
        }
        let Some(slot) = fields.get_mut(name) else {
            let known: Vec<&str> = fields.keys().map(String::as_str).collect();
            return Err(format!(
                "unknown argument '{path}{name}'; the arguments are: {}",
                known.join(", ")
            ));
        };
        if let (Value::Object(inner), Value::Object(given)) = (&mut *slot, value) {
            merge(inner, given, &format!("{path}{name}."))?;
            continue;
        }
        // "70" for 70: some models quote numbers. An optional value has no default to tell
        // its type (`null`): none of the tools' optional arguments is text, so a quoted number
        // there is a number too.
        let number = match (&*slot, value) {
            (Value::Number(_) | Value::Null, Value::String(text)) => {
                text.trim().parse::<f64>().ok().and_then(Number::from_f64)
            }
            _ => None,
        };
        *slot = number.map_or_else(|| value.clone(), Value::Number);
    }
    Ok(())
}

/// Numbers as a baker reads them (whole grams from 100 g, three decimals under 1), which is also
/// fewer tokens; nulls and empty lists dropped.
fn compact(value: &mut Value) {
    match value {
        Value::Number(n) if n.is_f64() => {
            let x = n.as_f64().unwrap_or(0.0);
            let decimals = match x.abs() {
                a if a >= 100.0 => 0,
                a if a >= 10.0 => 1,
                a if a >= 1.0 => 2,
                _ => 3,
            };
            let scale = 10f64.powi(decimals);
            let rounded = (x * scale).round() / scale;
            *value = if rounded.fract() == 0.0 && rounded.abs() < 1e15 {
                Value::from(rounded as i64)
            } else {
                Value::from(rounded)
            };
        }
        Value::Array(items) => items.iter_mut().for_each(compact),
        Value::Object(fields) => {
            fields.retain(|_, v| !v.is_null() && v.as_array().is_none_or(|a| !a.is_empty()));
            fields.values_mut().for_each(compact);
        }
        _ => {}
    }
}

fn to_json<T: Serialize>(value: &T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|e| e.to_string())
}

// ---- Engine ------------------------------------------------------------------------------

fn compute_formula(args: &Map<String, Value>) -> Result<Value, String> {
    // The calculator's form starts with a 20 % levain; a call that names none means none.
    let base = QuickForm { levain: 0.0, ..QuickForm::default() };
    let form: QuickForm = overlay(&base, &[], args)?;
    if form.yeast > 0.0 && form.yeast_kind.is_none() {
        return Err("yeast_kind is required when yeast is above 0".to_owned());
    }
    if form.scald > 0.0 && form.scald_method.is_none() {
        return Err("scald_method is required when scald is above 0".to_owned());
    }
    let formula = form.to_formula().map_err(|e| e.to_string())?;
    let solution = formula::solve(&formula).map_err(|e| e.to_string())?;
    let mut result = to_json(&solution)?;
    // Metrics that are zero (no sugar, no fat, nothing scalded…) say nothing.
    if let Some(metrics) = result.get_mut("metrics").and_then(Value::as_object_mut) {
        metrics.retain(|_, v| v.as_f64() != Some(0.0));
    }
    compact(&mut result);
    Ok(result)
}

fn temperature(args: &Map<String, Value>) -> Result<Value, String> {
    let form: QuickTemperature = overlay(&QuickTemperature::default(), &[], args)?;
    let result = water_temperature(&form.to_input()).map_err(|e| e.to_string())?;
    let mut out = to_json(&result)?;
    // The inputs as computed, defaults included: the answer has to state its assumptions.
    out["inputs"] = to_json(&form)?;
    compact(&mut out);
    Ok(out)
}

fn fermentation(args: &Map<String, Value>) -> Result<Value, String> {
    let mut args = args.clone();
    // One temperature given: the dough stays at it (the form's own defaults differ).
    let given =
        |args: &Map<String, Value>, name: &str| args.get(name).filter(|v| !v.is_null()).cloned();
    match (given(&args, "dough_c"), given(&args, "room_c")) {
        (Some(dough), None) => {
            args.insert("room_c".to_owned(), dough);
        }
        (None, Some(room)) => {
            args.insert("dough_c".to_owned(), room);
        }
        _ => {}
    }
    // Personal calibration belongs to the calculator page, not to the model.
    let hidden = ["vigour", "anchor_c"];
    let form: QuickFerment = overlay(&QuickFerment::default(), &hidden, &args)?;
    let result = form.solve().map_err(|e| e.to_string())?;
    let mut inputs = to_json(&form)?;
    if let Some(fields) = inputs.as_object_mut() {
        for name in hidden {
            fields.remove(name);
        }
    }
    let factors: Vec<Value> = result
        .modifiers
        .iter()
        .map(|m| json!({ "code": m.code, "rate_factor": m.factor }))
        .collect();
    let mut out = json!({
        "hours": { "low": result.hours.low_h, "mid": result.hours.mid_h, "high": result.hours.high_h },
        "endpoint_phase": result.endpoint_phase,
        "dose_pct": result.dose_pct,
        "mean_dough_temp_c": result.mean_temp_c,
        "factors": factors,
        "notes": result.notes,
        "inputs": inputs,
    });
    compact(&mut out);
    Ok(out)
}

fn starter(args: &Map<String, Value>) -> Result<Value, String> {
    // Personal calibration belongs to the calculator page, not to the model.
    let hidden = ["vigour"];
    let form: QuickStarter = overlay(&QuickStarter::default(), &hidden, args)?;
    let result = form.solve().map_err(|e| e.to_string())?;
    let mut inputs = to_json(&form)?;
    if let Some(fields) = inputs.as_object_mut() {
        for name in hidden {
            fields.remove(name);
        }
    }
    let build = result
        .build
        .map(|(seed, flour, water)| json!({ "seed_g": seed, "flour_g": flour, "water_g": water }));
    let mut out = json!({
        "hours_to_peak": {
            "low": result.hours.low_h,
            "mid": result.hours.mid_h,
            "high": result.hours.high_h,
        },
        "feed": {
            "starter_parts": 1,
            "flour_parts": result.flour_parts,
            "water_parts": result.water_parts,
        },
        "refreshed_hydration_pct": result.hydration_pct,
        "seed_share_of_flour_pct": result.iota * 100.0,
        "build": build,
        "notes": result.notes,
        "inputs": inputs,
    });
    compact(&mut out);
    Ok(out)
}

fn flour_blend(args: &Map<String, Value>) -> Result<Value, String> {
    let form: QuickFlour = overlay(&QuickFlour::default(), &[], args)?;
    let result = form.solve().map_err(|e| e.to_string())?;
    let mut out = json!({ "result": to_json(&result)?, "inputs": to_json(&form)? });
    compact(&mut out);
    Ok(out)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct YeastArgs {
    grams: f64,
    from: IngredientKind,
    to: IngredientKind,
}

fn convert_yeast(args: &Map<String, Value>) -> Result<Value, String> {
    let a: YeastArgs =
        serde_json::from_value(Value::Object(args.clone())).map_err(|e| e.to_string())?;
    let converted = yeast::convert(a.grams, a.from, a.to).map_err(|e| e.to_string())?;
    let mut out = to_json(&converted)?;
    compact(&mut out);
    Ok(out)
}

// ---- Knowledge base ----------------------------------------------------------------------

/// Words that carry no meaning in a keyword query (the index requires every word of a query).
const STOPWORDS: [&str; 44] = [
    "the", "an", "of", "in", "on", "for", "to", "and", "or", "is", "are", "my", "why", "how",
    "what", "does", "do", "with", "le", "la", "les", "un", "une", "des", "du", "de", "et", "ou",
    "est", "mon", "ma", "mes", "pourquoi", "comment", "que", "qui", "dans", "pour", "avec", "au",
    "aux", "en", "sur", "ce",
];

fn words(query: &str) -> Vec<&str> {
    query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() >= 2 && !STOPWORDS.contains(&w.to_lowercase().as_str()))
        .take(8)
        .collect()
}

/// Hits for a query whose words the index requires all at once: the whole query first, then
/// with one word left out, then pairs of neighbours, then (if still nearly empty) single words,
/// longest first.
fn relaxed(index: &Index, query: &str, lang: &str, limit: usize) -> Vec<Hit> {
    let words = words(query);
    let mut hits: Vec<Hit> = Vec::new();
    let add = |terms: &[&str], hits: &mut Vec<Hit>| {
        if hits.len() >= limit || terms.is_empty() {
            return;
        }
        match index.search(&terms.join(" "), lang, limit) {
            Ok(found) => {
                for hit in found {
                    if hits.len() < limit
                        && !hits.iter().any(|h| h.kind == hit.kind && h.id == hit.id)
                    {
                        hits.push(hit);
                    }
                }
            }
            Err(e) => tracing::warn!(error = %e, "knowledge search failed"),
        }
    };
    add(&words, &mut hits);
    if words.len() > 2 {
        for skipped in (0..words.len()).rev() {
            let rest: Vec<&str> =
                words.iter().enumerate().filter(|(i, _)| *i != skipped).map(|(_, w)| *w).collect();
            add(&rest, &mut hits);
        }
    }
    if words.len() > 3 {
        for pair in words.windows(2) {
            add(pair, &mut hits);
        }
    }
    if words.len() > 1 && hits.len() < 3 {
        let mut singles = words.clone();
        singles.sort_by_key(|w| std::cmp::Reverse(w.chars().count()));
        for word in singles {
            add(&[word], &mut hits);
        }
    }
    hits
}

/// The number of the source made of `keys` (or, without keys, of the page `kind`/`id`), new or
/// already given in this answer.
fn number(
    sources: &mut Vec<Source>,
    keys: Vec<String>,
    kind: &str,
    id: &str,
    title: &str,
) -> usize {
    let same = |s: &&Source| {
        if keys.is_empty() {
            s.keys.is_empty() && s.kind == kind && s.id == id
        } else {
            s.keys == keys
        }
    };
    if let Some(found) = sources.iter().find(same) {
        return found.n;
    }
    let n = sources.len() + 1;
    sources.push(Source {
        n,
        keys,
        kind: kind.to_owned(),
        id: id.to_owned(),
        title: title.to_owned(),
    });
    n
}

fn search(
    knowledge: &Knowledge,
    lang: Lang,
    args: &Map<String, Value>,
    sources: &mut Vec<Source>,
) -> Result<Value, String> {
    if let Some(unknown) = args.keys().find(|k| !matches!(k.as_str(), "queries" | "query")) {
        return Err(format!("unknown argument '{unknown}'; the argument is: queries"));
    }
    let queries: Vec<String> = args
        .get("queries")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .chain(args.get("query"))
        .filter_map(Value::as_str)
        .map(|q| q.trim().chars().take(200).collect::<String>())
        .filter(|q| !q.is_empty())
        .take(3)
        .collect();
    if queries.is_empty() {
        return Err("queries: give 1 to 3 keyword queries as an array of strings".to_owned());
    }
    let lists: Vec<Vec<Hit>> =
        queries.iter().map(|q| relaxed(&knowledge.index, q, lang.code(), MAX_PASSAGES)).collect();
    // Best hits of each query in turn.
    let mut hits: Vec<&Hit> = Vec::new();
    for rank in 0..MAX_PASSAGES {
        for hit in lists.iter().filter_map(|list| list.get(rank)) {
            if hits.len() < MAX_PASSAGES
                && !hits.iter().any(|h| h.kind == hit.kind && h.id == hit.id)
            {
                hits.push(hit);
            }
        }
    }
    let terms: Vec<String> = queries
        .iter()
        .flat_map(|q| words(q))
        .filter(|w| w.chars().count() >= 3)
        .map(passages::fold)
        .collect();

    let lib = &knowledge.lib;
    let mut found = Vec::new();
    for hit in hits {
        match hit.kind.as_str() {
            "claim" => {
                let Some(claim) = lib.claim(&hit.id) else { continue };
                let n = number(sources, vec![claim.id.clone()], "claim", &claim.id, "");
                found.push(json!({
                    "title": cite(lib, &claim.id).text,
                    "evidence": claim.evidence,
                    "text": format!("{} [{n}]", claim.claim.trim()),
                }));
            }
            "article" => {
                let Some(article) = lib.article(&hit.id, lang.code()) else { continue };
                let title = &article.front.title;
                let blocks = passages::blocks(&article.html, &article.citations);
                for block in passages::best(&blocks, &terms, ARTICLE_BLOCKS) {
                    let mut text = String::new();
                    let mut cited = false;
                    for segment in &block.segments {
                        if text.len() > PASSAGE_BYTES {
                            text.push('…');
                            break;
                        }
                        if !text.is_empty() && segment.text.starts_with(char::is_alphanumeric) {
                            text.push(' ');
                        }
                        text.push_str(&segment.text);
                        if !segment.keys.is_empty() {
                            let n =
                                number(sources, segment.keys.clone(), "article", &hit.id, title);
                            let _ = write!(text, " [{n}]");
                            cited = true;
                        }
                    }
                    // A paragraph that cites nothing: the article itself is the source.
                    if !cited {
                        let n = number(sources, Vec::new(), "article", &hit.id, title);
                        let _ = write!(text, " [{n}]");
                    }
                    let heading = if block.heading.is_empty() {
                        title.clone()
                    } else {
                        format!("{title} › {}", block.heading)
                    };
                    found.push(json!({ "title": heading, "text": text }));
                }
            }
            "recipe" => {
                let Some(recipe) = lib.recipe(&hit.id) else { continue };
                let title = recipe.title.get(lang.code());
                let n = number(sources, recipe.sources.clone(), "recipe", &hit.id, title);
                found.push(json!({
                    "title": title,
                    "text": format!("{} [{n}]", recipe.description.get(lang.code()).trim()),
                }));
            }
            _ => {}
        }
    }
    if found.is_empty() {
        return Ok(json!({
            "passages": [],
            "note": "Nothing found. Search again with other or fewer keywords, in English too.",
        }));
    }
    Ok(json!({ "passages": found }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(name: &str, arguments: &str) -> (Value, ToolRecord, Vec<Source>) {
        let knowledge = Knowledge::fixture();
        let mut sources = Vec::new();
        let (value, record) = execute(&knowledge, Lang::En, name, arguments, &mut sources);
        (value, record, sources)
    }

    #[test]
    fn formula_weights_come_from_the_engine() {
        let (value, record, _) = run(
            "compute_formula",
            r#"{"by":"dough","grams":1730,"water":"70","salt":2,"yeast_kind":"instant_yeast","yeast":1,"levain":null}"#,
        );
        assert!(record.ok, "{value}");
        // 1730 g of dough at 100 + 70 + 2 + 1: 1000 g of flour, and no levain unless asked.
        assert_eq!(
            value["totals"],
            json!([
                {"id": "flour", "grams": 1000, "pct": 100},
                {"id": "water", "grams": 700, "pct": 70},
                {"id": "salt", "grams": 20, "pct": 2},
                {"id": "yeast", "grams": 10, "pct": 1},
            ])
        );
        assert!(value.get("stages").is_none(), "{value}");
        assert_eq!(value["metrics"]["dough_g"], 1730);
        assert!(value["metrics"].get("sugar_pct").is_none());

        let (value, _, _) =
            run("compute_formula", r#"{"by":"flour","grams":500,"water":75,"levain":20}"#);
        assert_eq!(value["stages"][0]["id"], "levain");
        // 20 % of the 454.5 g of flour added at the final mix; whole grams only from 100 g.
        assert_eq!(value["stages"][0]["mass_g"], 90.9);
    }

    #[test]
    fn wrong_arguments_come_back_as_errors() {
        for (name, arguments, expected) in [
            (
                "compute_formula",
                r#"{"by":"dough","grams":900,"hydration":70}"#,
                "unknown argument 'hydration'",
            ),
            ("compute_formula", r#"{"by":"loaf","grams":900}"#, "unknown variant `loaf`"),
            (
                "compute_formula",
                r#"{"by":"dough","grams":900,"water":70,"yeast":1}"#,
                "yeast_kind is required",
            ),
            ("compute_formula", r#"{"by":"dough","grams":900,"water":-5}"#, "water"),
            ("compute_formula", r#"{"by":"dough","grams":"heavy"}"#, "invalid type"),
            ("compute_formula", r#"{"by":"dough","#, "not valid JSON"),
            ("compute_formula", "[1, 2]", "must be a JSON object"),
            (
                "predict_fermentation",
                r#"{"leavening":"sourdough","dough":{"salt":2}}"#,
                "unknown argument 'dough.salt'",
            ),
            (
                "predict_fermentation",
                r#"{"leavening":"instant_yeast","endpoint":"rise25"}"#,
                "endpoint",
            ),
            (
                "predict_fermentation",
                r#"{"leavening":"sourdough","vigour":{"log_mean":3}}"#,
                "unknown argument 'vigour'",
            ),
            ("water_temperature", r#"{"desired":24,"flour_c":20,"water_g":0}"#, "water_g"),
            (
                "convert_yeast",
                r#"{"grams":10,"from":"fresh_yeast","to":"salt"}"#,
                "not a commercial yeast",
            ),
            ("convert_yeast", r#"{"grams":10,"from":"fresh_yeast"}"#, "missing field `to`"),
            ("search_knowledge", r#"{"queries":[]}"#, "give 1 to 3"),
            ("search_knowledge", r#"{"q":"yudane"}"#, "unknown argument 'q'"),
            ("starter_plan", "{}", "unknown tool"),
        ] {
            let (value, record, _) = run(name, arguments);
            assert!(!record.ok, "{name} {arguments}: {value}");
            let message = value["error"].as_str().unwrap_or_else(|| panic!("{name}: {value}"));
            assert!(message.contains(expected), "{name} {arguments}: {message}");
        }
    }

    #[test]
    fn temperature_fermentation_and_yeast() {
        let (value, record, _) =
            run("water_temperature", r#"{"desired":25,"flour_c":20,"flour_g":1000,"water_g":700}"#);
        assert!(record.ok, "{value}");
        let water = value["water_c"].as_f64().unwrap();
        assert!((27.0..40.0).contains(&water), "{value}");
        assert_eq!(value["inputs"]["flour_g"], 1000);

        // A single temperature is the dough's and the room's.
        let (value, record, _) =
            run("predict_fermentation", r#"{"leavening":"sourdough","dose_pct":20,"dough_c":26}"#);
        assert!(record.ok, "{value}");
        assert_eq!(value["inputs"]["room_c"], 26);
        let (low, mid, high) = (
            value["hours"]["low"].as_f64().unwrap(),
            value["hours"]["mid"].as_f64().unwrap(),
            value["hours"]["high"].as_f64().unwrap(),
        );
        assert!(low < mid && mid < high && (1.0..24.0).contains(&mid), "{value}");
        assert!(value["inputs"].get("vigour").is_none());
        let (colder, _, _) =
            run("predict_fermentation", r#"{"leavening":"sourdough","dose_pct":20,"room_c":20}"#);
        assert!(colder["hours"]["mid"].as_f64().unwrap() > mid, "{colder}");

        let (value, record, _) =
            run("convert_yeast", r#"{"grams":21,"from":"fresh_yeast","to":"instant_yeast"}"#);
        assert!(record.ok);
        assert_eq!(value, json!({"grams": 6.93, "low_g": 5.25, "high_g": 8.82}));
    }

    #[test]
    fn starter_peak_and_feed_come_from_the_engine() {
        // The engine's own reference: a liquid 1:5:5 feed peaks in about 6.6 h at 24 °C.
        let (value, record, _) =
            run("predict_starter", r#"{"flour_parts":5,"water_parts":5,"temp_c":24}"#);
        assert!(record.ok, "{value}");
        let mid = value["hours_to_peak"]["mid"].as_f64().unwrap();
        assert!((6.0..7.2).contains(&mid), "{value}");
        assert_eq!(value["feed"], json!({"starter_parts": 1, "flour_parts": 5, "water_parts": 5}));
        assert_eq!(value["refreshed_hydration_pct"], 100);
        assert!(value.get("build").is_none() && value["inputs"].get("vigour").is_none(), "{value}");

        // The feed peaking in 12 h is leaner than 1:5:5 and comes with the grams to weigh.
        let (value, record, _) =
            run("predict_starter", r#"{"solve":"dose","target_h":12,"temp_c":24,"need_g":120}"#);
        assert!(record.ok, "{value}");
        assert!(value["feed"]["flour_parts"].as_f64().unwrap() > 5.0, "{value}");
        let build = &value["build"];
        let total: f64 =
            ["seed_g", "flour_g", "water_g"].iter().map(|k| build[k].as_f64().unwrap()).sum();
        assert!((total - 120.0).abs() < 1.0, "{value}");

        let (value, record, _) = run("predict_starter", r#"{"temp_c":60}"#);
        assert!(!record.ok && value["error"].as_str().unwrap().contains("temp_c"), "{value}");
    }

    #[test]
    fn flour_blends_come_from_the_engine() {
        // A recipe at 68 % with T65, 30 % of the flour moved to T150: 70.1 %, as the flour
        // model's worked example.
        let (value, record, _) = run(
            "blend_flour",
            r#"{"flours":[{"id":"fr_t65","pct":70},{"id":"fr_t150","pct":30}],"reference":{"flours":[{"id":"fr_t65","pct":100}],"hydration_pct":68},"target_protein_pct":"13"}"#,
        );
        assert!(record.ok, "{value}");
        assert_eq!(value["result"]["hydration"]["mid"], 70.1);
        assert_eq!(value["result"]["from_reference"], true);
        assert!(value["result"]["gluten"]["grams"].as_f64().unwrap() > 0.0, "{value}");

        // A flour type the table does not have is the model's to correct.
        let (value, record, _) = run("blend_flour", r#"{"flours":[{"id":"t65","pct":100}]}"#);
        assert!(!record.ok && value["error"].as_str().unwrap().contains("flours[0].id"), "{value}");
    }

    #[test]
    fn passages_carry_numbered_sources() {
        let (value, record, sources) = run(SEARCH, r#"{"queries":["yudane"]}"#);
        assert!(record.ok, "{value}");
        let passages = value["passages"].as_array().unwrap();
        assert_eq!(passages.len(), 3, "{value}");
        let texts: Vec<&str> = passages.iter().map(|p| p["text"].as_str().unwrap()).collect();
        // The article's statement keeps the two keys it cites; the claim and the recipe (which
        // rests on that claim) share one source number.
        let article = texts.iter().find(|t| t.starts_with("Yudane scalds")).unwrap();
        let claim = texts.iter().find(|t| t.starts_with("Yudane is flour")).unwrap();
        let recipe = texts.iter().find(|t| t.starts_with("Yudane milk bread.")).unwrap();
        let marker = |text: &str| -> usize {
            text.rsplit_once('[').unwrap().1.trim_end_matches(']').parse().unwrap()
        };
        let keys = |n: usize| sources.iter().find(|s| s.n == n).unwrap().keys.clone();
        assert_eq!(keys(marker(article)), vec!["fixture-0001", "zeng2023"]);
        assert_eq!(keys(marker(claim)), vec!["fixture-0001"]);
        assert_eq!(marker(recipe), marker(claim));
        assert_eq!(sources.len(), 2);
        let claim_passage = passages.iter().find(|p| p.get("evidence").is_some()).unwrap();
        assert_eq!(claim_passage["title"], "Novita Listyani — Tangzhong video, 12:40");
        assert_eq!(claim_passage["evidence"], "creator-rule");

        // French pages for a French search; words the index cannot find all at once still hit.
        let knowledge = Knowledge::fixture();
        let mut sources = Vec::new();
        let (value, _) = execute(
            &knowledge,
            Lang::Fr,
            SEARCH,
            r#"{"query":"pourquoi le yudane ébouillante la farine blanche"}"#,
            &mut sources,
        );
        let first = value["passages"][0]["text"].as_str().unwrap();
        assert!(first.starts_with("Le yudane ébouillante une partie de la farine [1]"), "{value}");
        assert_eq!(value["passages"][0]["title"], "Le yudane");

        let (value, record, sources) = run(SEARCH, r#"{"queries":["croissant lamination"]}"#);
        assert!(record.ok && sources.is_empty());
        assert_eq!(value["passages"], json!([]));
        assert!(value["note"].as_str().unwrap().contains("Nothing found"));
    }

    #[test]
    fn schemas_name_the_six_tools() {
        let names: Vec<&str> = SCHEMAS
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["function"]["name"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "search_knowledge",
                "compute_formula",
                "water_temperature",
                "predict_fermentation",
                "predict_starter",
                "blend_flour",
                "convert_yeast"
            ]
        );
        // Every argument a schema offers is one the tool accepts.
        for tool in SCHEMAS.as_array().unwrap() {
            let name = tool["function"]["name"].as_str().unwrap();
            let properties = tool["function"]["parameters"]["properties"].as_object().unwrap();
            let base = match name {
                "compute_formula" => to_json(&QuickForm::default()).unwrap(),
                "water_temperature" => to_json(&QuickTemperature::default()).unwrap(),
                "predict_fermentation" => to_json(&QuickFerment::default()).unwrap(),
                "predict_starter" => to_json(&QuickStarter::default()).unwrap(),
                // With a reference, so that its fields are there to check.
                "blend_flour" => to_json(&QuickFlour {
                    reference: Some(flour::Reference { flours: Vec::new(), hydration_pct: 68.0 }),
                    ..Default::default()
                })
                .unwrap(),
                _ => continue,
            };
            for (argument, schema) in properties {
                assert!(base.get(argument).is_some(), "{name}: {argument}");
                for nested in schema["properties"].as_object().into_iter().flatten() {
                    assert!(
                        base[argument].get(nested.0).is_some(),
                        "{name}: {argument}.{}",
                        nested.0
                    );
                }
                for variant in schema["enum"].as_array().into_iter().flatten() {
                    let mut probe = Map::new();
                    probe.insert(argument.clone(), variant.clone());
                    let accepted = match name {
                        "compute_formula" => overlay(&QuickForm::default(), &[], &probe).is_ok(),
                        "water_temperature" => {
                            overlay(&QuickTemperature::default(), &[], &probe).is_ok()
                        }
                        "predict_starter" => overlay(&QuickStarter::default(), &[], &probe).is_ok(),
                        "blend_flour" => overlay(&QuickFlour::default(), &[], &probe).is_ok(),
                        _ => overlay(&QuickFerment::default(), &[], &probe).is_ok(),
                    };
                    assert!(accepted, "{name}: {argument} = {variant}");
                }
            }
        }
    }
}
