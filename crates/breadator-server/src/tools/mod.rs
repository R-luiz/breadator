//! What an assistant can call: the knowledge base (search, articles, recipes, bibliography) and
//! the engine's calculators. One registry for the site's own chat ([`crate::chat`], which offers
//! some of them to its model) and for the MCP endpoint ([`crate::mcp`], all of them).
//!
//! A model never computes and never quotes from memory: a number comes from the engine, a fact
//! from a passage. A wrong argument comes back as a message for the model to correct, never as
//! a failure of the request.

mod library;
mod passages;

use std::fmt::Write as _;
use std::sync::LazyLock;

use breadator_content::search::{Hit, Index};
use breadator_engine::ferment::Band;
use breadator_engine::flour::{self, QuickFlour};
use breadator_engine::formula::{Formula, Target};
use breadator_engine::ingredients::IngredientKind;
use breadator_engine::leaven::{QuickFerment, QuickStarter};
use breadator_engine::preferment::{Mix, QuickPreferment, TargetPlan};
use breadator_engine::quick::{QuickForm, QuickTarget};
use breadator_engine::temperature::{QuickTemperature, water_temperature};
use breadator_engine::{formula, yeast};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value, json};

pub use self::library::{article_markdown, recipe, source_list};
use crate::i18n::Lang;
use crate::knowledge::{Knowledge, cite};
use crate::state_codec;

pub const SEARCH: &str = "search_knowledge";
/// Passages returned by one search.
const MAX_PASSAGES: usize = 6;
/// Paragraphs quoted from one article.
const ARTICLE_BLOCKS: usize = 2;
/// A passage stops at the first cited statement that ends beyond this many bytes.
const PASSAGE_BYTES: usize = 900;

/// What a statement of a passage rests on. The model cites it as `[n]`; the chat stores the
/// cited ones with its answer (`chat_messages.citations`) and lists them under it, the MCP
/// endpoint returns them with the passages ([`source_list`]).
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

/// What a tool runs with.
pub struct Context<'a> {
    pub knowledge: &'a Knowledge,
    /// Language of the texts returned (passages, articles, recipes).
    pub lang: Lang,
    /// The site's origin (`PUBLIC_URL`) when results are to link to its pages. `None` for the
    /// chat: it lists the sources of an answer itself, and its model is to write no link.
    pub site: Option<&'a str>,
}

impl Context<'_> {
    /// A page of the site in the tool's language; `path` is what follows the language segment.
    fn url(&self, path: &str) -> String {
        format!("{}/{}{path}", self.site.unwrap_or(""), self.lang.code())
    }
}

/// A tool as a model is told about it.
pub struct Tool {
    pub name: &'static str,
    /// Its name for people.
    pub title: &'static str,
    pub description: &'static str,
    /// JSON Schema of its arguments (an object).
    pub parameters: Value,
    /// It returns text that exists in French and in English: the caller chooses
    /// ([`Context::lang`]).
    pub localized: bool,
}

/// Every tool, in the order they are listed.
pub static TOOLS: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    let number = |description: &str| json!({ "type": "number", "description": description });
    let text = |description: &str| json!({ "type": "string", "description": description });
    let choice = |values: &[&str], description: &str| json!({ "type": "string", "enum": values, "description": description });
    let tool = |name: &'static str,
                title: &'static str,
                description: &'static str,
                properties: Value,
                required: &[&str]| Tool {
        name,
        title,
        description,
        parameters: json!({ "type": "object", "properties": properties, "required": required }),
        localized: false,
    };
    let yeasts = ["fresh_yeast", "active_dry_yeast", "instant_yeast", "osmotolerant_yeast"];
    let rises = ["rise25", "rise50", "rise75", "doubled"];
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
    // The amount to make: of a recipe, of a full formula.
    let target = |description: &str| {
        json!({
            "type": "object",
            "description": description,
            "properties": {
                "by": choice(&["dough", "pieces", "baked", "flour"], "What `grams` is: the whole dough, one dough piece, one baked loaf, or the total flour."),
                "grams": number("Weight in grams, see `by`."),
                "count": number("Number of pieces or loaves (by = pieces or baked; default 1)."),
                "loss_pct": number("Bake loss, % (by = baked; default 14)."),
            },
            "required": ["by", "grams"],
        })
    };
    vec![
        Tool {
            localized: true,
            ..tool(
                SEARCH,
                "Search the knowledge base",
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
            )
        },
        Tool {
            localized: true,
            ..tool(
                "list_articles",
                "List the articles",
                "The articles of Breadator's knowledge base in reading order, topic by topic: slug, title, one-sentence description and page link.",
                json!({}),
                &[],
            )
        },
        Tool {
            localized: true,
            ..tool(
                "read_article",
                "Read an article",
                "The full text of one article of the knowledge base. Each statement is followed by the numbers of its sources, e.g. [3]; the list at the end gives the citation and link behind each number.",
                json!({
                    "slug": text("The article, as list_articles names it; also the last part of an article's page link."),
                }),
                &["slug"],
            )
        },
        Tool {
            localized: true,
            ..tool(
                "list_recipes",
                "List the recipes",
                "Breadator's recipes: slug, title, description, kind of bread and page link.",
                json!({}),
                &[],
            )
        },
        Tool {
            localized: true,
            ..tool(
                "get_recipe",
                "Get a recipe",
                "One recipe with its weights: the formula in baker's percentages, the grams of each stage (levain, poolish, scald) and of the final mix, the steps, the notes and the sources. `target` scales it to another amount. The `formula` returned can be changed and passed to solve_formula.",
                json!({
                    "slug": text("The recipe, as list_recipes names it."),
                    "target": target("Amount to make, instead of the recipe's own."),
                }),
                &["slug"],
            )
        },
        tool(
            "list_sources",
            "Browse the bibliography",
            "The references behind Breadator's articles and recipes (papers, books, technical documents): citation and link, 30 at a time.",
            json!({
                "query": text("Keep the references whose authors, title, journal or year contain every word (accents ignored), e.g. \"gänzle 1998\"."),
                "offset": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "References to skip (default 0), to read the next ones.",
                },
            }),
            &[],
        ),
        tool(
            "compute_formula",
            "Bread formula weights",
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
            "solve_formula",
            "Solve a full formula",
            "Exact weights of any bread formula in baker's percentages: several flours, enrichments (milk, eggs, butter, sugar), seeds, and any number of stages (levain built from a seed starter, poolish, biga, sponge, old dough, scald, soaker). Every percentage is relative to all the flour of the bread; a stage takes its flour, its liquid and its extras out of those totals, which must cover them. Returns the totals, each stage, the final mix and the dough's metrics (hydration with and without the water of milk, eggs and butter, prefermented flour, inoculation), with a link that opens the formula in Breadator's calculator. compute_formula is simpler for a lean bread with at most one levain, one poolish and one scald.",
            json!({
                "flours": {
                    "type": "array",
                    "minItems": 1,
                    "description": "The flour blend; the percentages add up to 100.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": text("Its name in this formula, e.g. bread_flour, rye."),
                            "pct": number("Share of the total flour, %."),
                        },
                        "required": ["id", "pct"],
                    },
                },
                "ingredients": {
                    "type": "array",
                    "description": "Every other ingredient of the whole bread, what goes into the stages included.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": text("Its name in this formula, e.g. water, salt, butter."),
                            "kind": choice(&INGREDIENT_KINDS, "What it is: sets the water, fat and sugar it brings. inclusion: seeds, nuts, dried fruit, cheese."),
                            "pct": number("% of the total flour."),
                            "composition": {
                                "type": "object",
                                "description": "Mass fractions (0 to 1) replacing those of the kind.",
                                "properties": { "water": { "type": "number" }, "fat": { "type": "number" }, "sugar": { "type": "number" } },
                                "required": ["water", "fat", "sugar"],
                            },
                        },
                        "required": ["id", "kind", "pct"],
                    },
                },
                "stages": {
                    "type": "array",
                    "description": "Preferments, scalds and soakers.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": text("Its name, e.g. levain."),
                            "kind": choice(&["levain", "poolish", "biga", "sponge", "pate_fermentee", "scald", "soaker"], "What the stage is."),
                            "flour_pct": number("Share of the total flour that goes into the stage, % (0 for a soaker of seeds)."),
                            "flour": text("Id of the one flour it uses; omitted: every flour, in the blend's proportions."),
                            "liquid_pct": number("Liquid of the stage, % of the stage flour: 100 for a liquid levain, a poolish or a yudane, about 500 for a tangzhong."),
                            "liquid": text("Id of the ingredient that supplies the liquid, listed in `ingredients` with enough for every stage (default: the ingredient named water)."),
                            "extras": {
                                "type": "array",
                                "description": "Other ingredients put in the stage: yeast in a poolish, salt in a biga, seeds in a soaker.",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "ingredient": text("Id of the ingredient."),
                                        "pct": number("Amount, see `basis`."),
                                        "basis": choice(&["stage_flour", "total_flour"], "% of the stage flour (default) or of the total flour."),
                                    },
                                    "required": ["ingredient", "pct"],
                                },
                            },
                            "evaporation_pct": number("Scalds: water lost while cooking, % of the stage water; added to the water to cook with."),
                            "seed": {
                                "type": "object",
                                "description": "Levain: the seed starter it is built from.",
                                "properties": {
                                    "pct": number("Seed weight, % of the fresh flour of the build (a 1:5:5 feed is 20)."),
                                    "hydration_pct": number("Hydration of the seed starter, %."),
                                },
                                "required": ["pct", "hydration_pct"],
                            },
                        },
                        "required": ["id", "kind", "flour_pct", "liquid_pct"],
                    },
                },
                "target": target("Amount to make."),
                "allowance_pct": number("Extra dough for what stays on bowls and tools, % (default 0)."),
            }),
            &["flours", "target"],
        ),
        tool(
            "water_temperature",
            "Water temperature",
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
            "Fermentation time or dose",
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
            "Starter feeding and peak",
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
            "plan_existing_preferment",
            "Bread from an existing preferment",
            "Plan a lean sourdough bread around a preferment (levain) that is already made: from the grams that went into it and its age, when it peaks, and the final mix that gives the baked weight and the bulk fermentation time wanted. When the preferment is too large or too small for both, also the mixes that use all of it: at the same weight (another bulk time) or at the same bulk time (another weight). Times come with the range 80 % of bakes fall in.",
            json!({
                "seed_g": number("Seed starter that went into the preferment, g."),
                "seed_hydration_pct": number("Hydration of that seed starter, % (default 100)."),
                "seed_state": choice(&["ripe", "from_fridge"], "State of the seed starter when it was fed (default ripe)."),
                "feed_flour_g": number("Flour added at feeding, g."),
                "feed_water_g": number("Water added at feeding, g."),
                "feed_c": number("Temperature of the preferment since feeding, °C (default 21)."),
                "age_h": number("Hours since feeding (default 8)."),
                "baked_g": number("Total baked weight wanted, g (default 800)."),
                "loss_pct": number("Bake loss, % (default 14)."),
                "hydration_pct": number("Hydration of the dough, % (default 75)."),
                "salt_pct": number("Salt, % of flour (default 2)."),
                "whole_grain_pct": number("Whole-grain flour, % of the flour (default 0)."),
                "bulk_c": number("Bulk fermentation temperature, °C (default 21): dough and room stay at it, without a fridge."),
                "bulk_h": number("Hours wanted from mixing to the end of bulk (default 8)."),
                "endpoint": choice(&rises, "Rise in the aliquot jar that ends the bulk (default rise50)."),
                "extra_hydration_pct": number("Hydration of the ripe starter added when the preferment is too small, % (default 100)."),
                "manual_vigour": number("Known vigour factor of the culture (1 = the model's reference culture). Not with `observations`."),
                "observations": {
                    "type": "array",
                    "maxItems": 4,
                    "description": "Timed events of the same culture at a steady temperature, started with ripe starter: the plan is then adjusted to its vigour.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "kind": choice(&["peak", "bulk"], "peak: a refreshed starter reaching its peak; bulk: a lean dough reaching `endpoint`. Every field of the kind is required."),
                            "observed_h": number("Hours it took."),
                            "temp_c": number("Temperature during it, °C (15 to 30)."),
                            "seed_hydration_pct": number("peak: hydration of the starter fed, %."),
                            "flour_parts": number("peak: fresh flour per part of starter (5 for 1:5:5)."),
                            "water_parts": number("peak: fresh water per part of starter."),
                            "starter_hydration_pct": number("bulk: hydration of the starter used, %."),
                            "dose_pct": number("bulk: starter weight, % of the flour added at mixing."),
                            "hydration_pct": number("bulk: hydration of the dough, %."),
                            "salt_pct": number("bulk: salt, % of flour."),
                            "whole_grain_pct": number("bulk: whole-grain flour, % of the flour."),
                            "endpoint": choice(&rises, "bulk: the rise observed."),
                        },
                        "required": ["kind", "observed_h", "temp_c"],
                    },
                },
            }),
            &["seed_g", "feed_flour_g", "feed_water_g"],
        ),
        tool(
            "blend_flour",
            "Flour blend and hydration",
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
            "Convert yeast",
            "Convert an amount of commercial yeast to another form, with the range manufacturers give.",
            json!({
                "grams": number("Amount to convert, g."),
                "from": choice(&yeasts, "Form the amount is in."),
                "to": choice(&yeasts, "Form wanted."),
            }),
            &["grams", "from", "to"],
        ),
    ]
});

/// The kinds of ingredient a full formula takes (`IngredientKind`'s codes).
const INGREDIENT_KINDS: [&str; 19] = [
    "water",
    "milk",
    "cream",
    "yogurt",
    "beer",
    "egg",
    "egg_yolk",
    "egg_white",
    "butter",
    "oil",
    "sugar",
    "honey",
    "salt",
    "fresh_yeast",
    "active_dry_yeast",
    "instant_yeast",
    "osmotolerant_yeast",
    "inclusion",
    "other",
];

pub fn get(name: &str) -> Option<&'static Tool> {
    TOOLS.iter().find(|tool| tool.name == name)
}

/// Runs the tool `name` with the model's arguments. A search adds what the `[n]` of its
/// passages stand for to `sources`. The error is a message for the model: the argument that is
/// wrong and what is accepted.
pub fn run(
    ctx: &Context,
    name: &str,
    args: &Map<String, Value>,
    sources: &mut Vec<Source>,
) -> Result<Value, String> {
    match name {
        SEARCH => search(ctx, args, sources),
        "list_articles" => library::list_articles(ctx, args),
        "read_article" => library::read_article(ctx, args),
        "list_recipes" => library::list_recipes(ctx, args),
        "get_recipe" => library::get_recipe(ctx, args),
        "list_sources" => library::list_sources(ctx, args),
        "compute_formula" => compute_formula(args),
        "solve_formula" => solve_formula(ctx, args),
        "water_temperature" => temperature(args),
        "predict_fermentation" => fermentation(args),
        "predict_starter" => starter(args),
        "plan_existing_preferment" => preferment(args),
        "blend_flour" => flour_blend(args),
        "convert_yeast" => convert_yeast(args),
        other => Err(format!("unknown tool '{other}'")),
    }
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
    solution(&form.to_formula().map_err(|e| e.to_string())?)
}

/// The solved weights of a formula.
fn solution(formula: &Formula) -> Result<Value, String> {
    let solution = formula::solve(formula).map_err(|e| e.to_string())?;
    let mut result = to_json(&solution)?;
    // Metrics that are zero (no sugar, no fat, nothing scalded…) say nothing.
    if let Some(metrics) = result.get_mut("metrics").and_then(Value::as_object_mut) {
        metrics.retain(|_, v| v.as_f64() != Some(0.0));
    }
    compact(&mut result);
    Ok(result)
}

/// The page of the formula calculator opened on `formula`.
fn calculator_url(ctx: &Context, formula: &Formula) -> String {
    ctx.url(&format!("/calculators/formula?s={}", state_codec::encode(formula)))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetArgs {
    by: QuickTarget,
    grams: f64,
    count: Option<f64>,
    loss_pct: Option<f64>,
}

/// The amount to make as a tool takes it: one piece and a 14 % bake loss unless told.
fn target(value: &Value) -> Result<Target, String> {
    let given: TargetArgs =
        serde_json::from_value(value.clone()).map_err(|e| format!("target: {e}"))?;
    let count = || match given.count.unwrap_or(1.0) {
        count if count.fract() == 0.0 && (1.0..=10_000.0).contains(&count) => Ok(count as u32),
        _ => Err("target.count: a whole number from 1 to 10000".to_owned()),
    };
    Ok(match given.by {
        QuickTarget::Flour => Target::Flour { grams: given.grams },
        QuickTarget::Dough => Target::Dough { grams: given.grams },
        QuickTarget::Pieces => Target::Pieces { count: count()?, grams: given.grams },
        QuickTarget::Baked => Target::Baked {
            count: count()?,
            grams: given.grams,
            loss_pct: given.loss_pct.unwrap_or(QuickForm::default().loss),
        },
    })
}

/// The first key of `given` that `read` does not have, as a path (`stages.0.liquid`). `read`
/// is the same value once read into its type and written back: a key missing from it is one
/// the type ignored, a misspelt optional argument that would silently keep its default.
fn ignored_key(given: &Value, read: &Value) -> Option<String> {
    match (given, read) {
        (Value::Object(given), Value::Object(read)) => {
            given.iter().find_map(|(key, value)| match read.get(key) {
                None => Some(key.clone()),
                Some(known) => ignored_key(value, known).map(|rest| format!("{key}.{rest}")),
            })
        }
        (Value::Array(given), Value::Array(read)) => {
            given.iter().zip(read).enumerate().find_map(|(i, (value, known))| {
                ignored_key(value, known).map(|rest| format!("{i}.{rest}"))
            })
        }
        _ => None,
    }
}

fn solve_formula(ctx: &Context, args: &Map<String, Value>) -> Result<Value, String> {
    let mut given = args.clone();
    let amount = given
        .get("target")
        .filter(|v| !v.is_null())
        .ok_or("target is required, e.g. {\"by\": \"dough\", \"grams\": 900}")?;
    let amount = to_json(&target(amount)?)?;
    given.insert("target".to_owned(), amount);
    let given = Value::Object(given);
    let formula: Formula = serde_json::from_value(given.clone()).map_err(|e| e.to_string())?;
    if let Some(path) = ignored_key(&given, &to_json(&formula)?) {
        return Err(format!("unknown argument '{path}'"));
    }
    let mut out = solution(&formula)?;
    out["calculator_url"] = calculator_url(ctx, &formula).into();
    Ok(out)
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

fn hours(band: Band) -> Value {
    json!({ "low": band.low_h, "mid": band.mid_h, "high": band.high_h })
}

/// What to weigh for a final mix made with a preferment.
fn preferment_mix(mix: &Mix) -> Value {
    json!({
        "preferment_g": mix.preferment_g,
        "extra_ripe_starter_g": (mix.additional_levain_g > 1e-6).then_some(mix.additional_levain_g),
        "flour_g": mix.flour_g,
        "water_g": mix.water_g,
        "salt_g": mix.salt_g,
        "dough_g": mix.dough_g,
        "baked_g": mix.baked_g,
    })
}

fn preferment(args: &Map<String, Value>) -> Result<Value, String> {
    let form: QuickPreferment = overlay(&QuickPreferment::default(), &[], args)?;
    let plan = form.solve().map_err(|e| e.to_string())?;
    let calibration = match plan.vigour {
        None => json!({ "basis": "reference_culture" }),
        Some(v) if v.n == 0 => json!({ "basis": "entered_factor", "vigour_factor": v.factor() }),
        Some(v) => {
            json!({ "basis": "observations", "vigour_factor": v.factor(), "observations": v.n })
        }
    };
    // The mix that keeps both the baked weight and the bulk time, or why there is none.
    let keep_both = match &plan.target {
        TargetPlan::Mix { mix, hours: band, extrapolated } => {
            let left_over = plan.available_g - mix.preferment_g;
            json!({
                "possible": true,
                "final_mix": preferment_mix(mix),
                "bulk_hours": hours(*band),
                "extrapolated": extrapolated,
                "preferment_left_over_g": (left_over > 1e-6).then_some(left_over),
            })
        }
        TargetPlan::WaterConflict => json!({
            "possible": false,
            "reason": "the dose this bulk time needs would bring more water than the dough's hydration allows: change the hydration or the bulk time",
        }),
        TargetPlan::NoDose { too_slow: true } => json!({
            "possible": false,
            "reason": "no dose reaches this rise so quickly: lengthen the bulk time or raise the temperature",
        }),
        TargetPlan::NoDose { too_slow: false } => json!({
            "possible": false,
            "reason": "no dose lasts so long a bulk at this temperature: shorten the bulk time or lower the temperature",
        }),
    };
    let use_all = plan.alternatives.as_ref().map(|a| {
        json!({
            "keep_baked_weight": a.same_weight.as_ref().map(|mix| json!({
                "final_mix": preferment_mix(mix),
                "prefermented_flour_pct": a.same_weight_pct,
                "bulk_hours": a.same_weight_hours.map(hours),
            })),
            "keep_bulk_time": a.same_time.as_ref().map(|mix| json!({
                "final_mix": preferment_mix(mix),
            })),
        })
    });
    let mut out = json!({
        "preferment": {
            "available_g": plan.available_g,
            "age_h": form.age_h,
            "hydration_pct": plan.preferment_hydration_pct,
            "hours_from_feeding_to_peak": hours(plan.peak),
            "peak_extrapolated": plan.peak_extrapolated,
        },
        "calibration": calibration,
        "keep_baked_weight_and_bulk_time": keep_both,
        "use_all_the_preferment": use_all,
        "inputs": to_json(&form)?,
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

/// The forms of commercial yeast, under their ingredient codes.
#[derive(Clone, Copy, Deserialize)]
enum YeastForm {
    #[serde(rename = "fresh_yeast")]
    Fresh,
    #[serde(rename = "active_dry_yeast")]
    ActiveDry,
    #[serde(rename = "instant_yeast")]
    Instant,
    #[serde(rename = "osmotolerant_yeast")]
    Osmotolerant,
}

impl From<YeastForm> for IngredientKind {
    fn from(form: YeastForm) -> Self {
        match form {
            YeastForm::Fresh => Self::FreshYeast,
            YeastForm::ActiveDry => Self::ActiveDryYeast,
            YeastForm::Instant => Self::InstantYeast,
            YeastForm::Osmotolerant => Self::OsmotolerantYeast,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct YeastArgs {
    grams: f64,
    from: YeastForm,
    to: YeastForm,
}

fn convert_yeast(args: &Map<String, Value>) -> Result<Value, String> {
    let a: YeastArgs =
        serde_json::from_value(Value::Object(args.clone())).map_err(|e| e.to_string())?;
    let converted =
        yeast::convert(a.grams, a.from.into(), a.to.into()).map_err(|e| e.to_string())?;
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
    ctx: &Context,
    args: &Map<String, Value>,
    sources: &mut Vec<Source>,
) -> Result<Value, String> {
    let (knowledge, lang) = (ctx.knowledge, ctx.lang);
    // Where a passage was found, for a caller that links to the site.
    let page = |passage: &mut Value, path: String| {
        if ctx.site.is_some() {
            passage["page"] = ctx.url(&path).into();
        }
    };
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
                        // What only closes a citation (a full stop, a table bar) is for the
                        // reader of the whole article.
                        if segment.keys.is_empty()
                            && !segment.text.chars().any(char::is_alphanumeric)
                        {
                            continue;
                        }
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
                    let mut passage = json!({ "title": heading, "text": text });
                    page(&mut passage, format!("/learn/{}", hit.id));
                    found.push(passage);
                }
            }
            "recipe" => {
                let Some(recipe) = lib.recipe(&hit.id) else { continue };
                let title = recipe.title.get(lang.code());
                let n = number(sources, recipe.sources.clone(), "recipe", &hit.id, title);
                let mut passage = json!({
                    "title": title,
                    "text": format!("{} [{n}]", recipe.description.get(lang.code()).trim()),
                });
                page(&mut passage, format!("/recipes/{}", hit.id));
                found.push(passage);
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
    use breadator_engine::formula::StageKind;

    use super::*;

    const SITE: &str = "https://breadator.test";

    /// How a call went: what the caller (chat, MCP endpoint) tells the model.
    struct Outcome {
        ok: bool,
    }

    /// A tool run over the fixture knowledge base; a refusal reads `{"error": …}`.
    fn run_in(lang: Lang, name: &str, arguments: &str) -> (Value, Outcome, Vec<Source>) {
        let knowledge = Knowledge::fixture();
        let ctx = Context { knowledge: &knowledge, lang, site: Some(SITE) };
        let Value::Object(args) = serde_json::from_str(arguments).unwrap() else {
            panic!("{arguments} is not an object")
        };
        let mut sources = Vec::new();
        let result = super::run(&ctx, name, &args, &mut sources);
        let ok = result.is_ok();
        (result.unwrap_or_else(|message| json!({ "error": message })), Outcome { ok }, sources)
    }

    fn run(name: &str, arguments: &str) -> (Value, Outcome, Vec<Source>) {
        run_in(Lang::En, name, arguments)
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
            // Only the four forms of yeast are named, not every kind of ingredient.
            (
                "convert_yeast",
                r#"{"grams":10,"from":"fresh_yeast","to":"salt"}"#,
                "unknown variant `salt`, expected one of `fresh_yeast`, `active_dry_yeast`, `instant_yeast`, `osmotolerant_yeast`",
            ),
            ("convert_yeast", r#"{"grams":10,"from":"fresh_yeast"}"#, "missing field `to`"),
            ("search_knowledge", r#"{"queries":[]}"#, "give 1 to 3"),
            ("search_knowledge", r#"{"q":"yudane"}"#, "unknown argument 'q'"),
            ("starter_plan", "{}", "unknown tool"),
            ("read_article", r#"{"slug":"croissants"}"#, "no article 'croissants'"),
            ("read_article", "{}", "missing field `slug`"),
            ("list_articles", r#"{"topic":"starch"}"#, "unknown field `topic`"),
            ("get_recipe", r#"{"slug":"baguette"}"#, "no recipe 'baguette'"),
            (
                "get_recipe",
                r#"{"slug":"milk-bread","target":{"by":"pieces","grams":90,"count":2.5}}"#,
                "target.count",
            ),
            ("list_sources", r#"{"query":"zeng","page":2}"#, "unknown field `page`"),
            (
                "plan_existing_preferment",
                r#"{"seed_g":10,"feed_flour_g":50,"feed_water_g":50,"bulk_c":45}"#,
                "bulk_c",
            ),
            (
                "plan_existing_preferment",
                r#"{"seed_g":10,"feed_flour_g":50,"feed_water_g":50,"manual_vigour":1.2,"observations":[{"kind":"peak","observed_h":5,"temp_c":24,"seed_hydration_pct":100,"flour_parts":5,"water_parts":5}]}"#,
                "observations",
            ),
            (
                "plan_existing_preferment",
                r#"{"seed_g":10,"feed_flour_g":50,"feed_water_g":50,"observations":[{"kind":"peak","observed_h":5,"temp_c":24}]}"#,
                "missing field",
            ),
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
    fn a_full_formula_is_solved_with_its_stages_and_enrichments() {
        // A brioche on a stiff levain built 1:2:1: 1000 g of flour, 200 g of it in the levain.
        let brioche = r#"{
            "flours": [{"id": "bread_flour", "pct": 100}],
            "ingredients": [
                {"id": "milk", "kind": "milk", "pct": 20},
                {"id": "water", "kind": "water", "pct": 10},
                {"id": "eggs", "kind": "egg", "pct": 30},
                {"id": "butter", "kind": "butter", "pct": 40},
                {"id": "sugar", "kind": "sugar", "pct": 12},
                {"id": "salt", "kind": "salt", "pct": 2}
            ],
            "stages": [{"id": "levain", "kind": "levain", "flour_pct": 20, "liquid_pct": 50,
                        "seed": {"pct": 50, "hydration_pct": 50}}],
            "target": {"by": "flour", "grams": 1000}
        }"#;
        let (value, record, _) = run("solve_formula", brioche);
        assert!(record.ok, "{value}");
        assert_eq!(value["metrics"]["total_flour_g"], 1000);
        assert_eq!(value["metrics"]["dough_g"], 2140);
        // Plain water only, then with the water of the milk, the eggs and the butter.
        assert_eq!(value["metrics"]["hydration_water_pct"], 10);
        let total = value["metrics"]["hydration_total_pct"].as_f64().unwrap();
        assert!((56.0..58.0).contains(&total), "{value}");
        assert_eq!(value["metrics"]["fat_pct"].as_f64().map(f64::round), Some(36.0));
        let levain = &value["stages"][0];
        assert_eq!((&levain["flour_g"], &levain["water_g"]), (&json!(200), &json!(100)));
        assert!(levain["seed"]["seed_g"].as_f64().unwrap() > 0.0, "{value}");
        // The final mix weighs the levain as one line and only the water left for it.
        let final_mix = value["final_mix"].as_array().unwrap();
        let line = |id: &str| final_mix.iter().find(|l| l["id"] == id).unwrap()["grams"].clone();
        assert_eq!(
            (line("bread_flour"), line("levain"), line("butter")),
            (800.into(), 300.into(), 400.into())
        );
        assert!(final_mix.iter().all(|l| l["id"] != "water"), "{value}");
        // The link opens the same formula in the site's calculator.
        let url = value["calculator_url"].as_str().unwrap();
        let state = url.strip_prefix("https://breadator.test/en/calculators/formula?s=").unwrap();
        let opened: Formula = state_codec::decode(state).unwrap();
        assert_eq!(opened.stages[0].kind, StageKind::Levain);
        assert_eq!(opened.target, Target::Flour { grams: 1000.0 });

        // Two loaves of 800 g baked, losing the usual 14 % in the oven.
        let lean = r#"{"flours":[{"id":"t65","pct":100}],"ingredients":[{"id":"water","kind":"water","pct":70},{"id":"salt","kind":"salt","pct":2}],"target":{"by":"baked","grams":800,"count":2}}"#;
        let (value, record, _) = run("solve_formula", lean);
        assert!(record.ok, "{value}");
        assert_eq!(value["metrics"]["dough_g"], 1860);

        for (arguments, expected) in [
            // A misspelt optional argument would otherwise silently keep its default.
            (
                lean.replace(r#""pct":70}"#, r#""pct":70,"composition":null,"percent":70}"#),
                "'ingredients.0.percent'",
            ),
            (lean.replace(r#""flours""#, r#""flour_blend":[],"flours""#), "'flour_blend'"),
            (
                lean.replace(r#","target":{"by":"baked","grams":800,"count":2}"#, ""),
                "target is required",
            ),
            (lean.replace(r#""kind":"salt""#, r#""kind":"sel""#), "unknown variant `sel`"),
            (lean.replace(r#""pct":100"#, r#""pct":90"#), "add up to 90"),
        ] {
            let (value, record, _) = run("solve_formula", &arguments);
            let message = value["error"].as_str().unwrap_or_else(|| panic!("{arguments}: {value}"));
            assert!(!record.ok && message.contains(expected), "{arguments}: {message}");
        }
    }

    #[test]
    fn a_preferment_already_made_is_planned_around() {
        // 110 g of liquid levain for 800 g baked in 8 h at 21 °C: a part of it, or all of it.
        let (value, record, _) = run(
            "plan_existing_preferment",
            r#"{"seed_g":10,"feed_flour_g":50,"feed_water_g":"50"}"#,
        );
        assert!(record.ok, "{value}");
        assert_eq!(value["preferment"]["available_g"], 110);
        assert_eq!(value["preferment"]["hydration_pct"], 100);
        let peak = &value["preferment"]["hours_from_feeding_to_peak"];
        assert!(peak["low"].as_f64() < peak["mid"].as_f64(), "{value}");
        assert_eq!(value["calibration"], json!({"basis": "reference_culture"}));
        let plan = &value["keep_baked_weight_and_bulk_time"];
        assert_eq!(plan["possible"], true, "{value}");
        let mix = &plan["final_mix"];
        assert_eq!(mix["baked_g"], 800);
        let weighed: f64 = ["preferment_g", "extra_ripe_starter_g", "flour_g", "water_g", "salt_g"]
            .iter()
            .filter_map(|k| mix[*k].as_f64())
            .sum();
        assert!((weighed - mix["dough_g"].as_f64().unwrap()).abs() < 2.0, "{value}");
        let bulk = plan["bulk_hours"]["mid"].as_f64().unwrap();
        assert!((7.5..8.5).contains(&bulk), "{value}");
        assert_eq!(value["inputs"]["bulk_h"], 8);

        // A levain far too large for the loaf: some is left over, and using all of it changes
        // the bulk time or the weight.
        let (value, record, _) = run(
            "plan_existing_preferment",
            r#"{"seed_g":5,"feed_flour_g":200,"feed_water_g":200,"baked_g":720,"loss_pct":10,"hydration_pct":58}"#,
        );
        assert!(record.ok, "{value}");
        let plan = &value["keep_baked_weight_and_bulk_time"];
        assert!(plan["preferment_left_over_g"].as_f64().unwrap() > 100.0, "{value}");
        assert!(plan["final_mix"].get("extra_ripe_starter_g").is_none(), "{value}");
        let all = &value["use_all_the_preferment"];
        assert_eq!(all["keep_baked_weight"]["final_mix"]["preferment_g"], 405);
        assert!(all["keep_bulk_time"]["final_mix"]["baked_g"].as_f64().unwrap() > 720.0, "{value}");

        // No dose ferments a dough in half an hour.
        let (value, record, _) = run(
            "plan_existing_preferment",
            r#"{"seed_g":10,"feed_flour_g":50,"feed_water_g":50,"bulk_h":0.5}"#,
        );
        assert!(record.ok, "{value}");
        let plan = &value["keep_baked_weight_and_bulk_time"];
        assert_eq!(plan["possible"], false);
        assert!(plan["reason"].as_str().unwrap().contains("lengthen the bulk time"), "{value}");

        // A culture seen peaking faster than the reference shortens the forecast.
        let reference = value["preferment"]["hours_from_feeding_to_peak"]["mid"].as_f64().unwrap();
        let (value, record, _) = run(
            "plan_existing_preferment",
            r#"{"seed_g":10,"feed_flour_g":50,"feed_water_g":50,"observations":[{"kind":"peak","observed_h":4.5,"temp_c":24,"seed_hydration_pct":100,"flour_parts":5,"water_parts":5}]}"#,
        );
        assert!(record.ok, "{value}");
        assert_eq!(value["calibration"]["basis"], "observations");
        assert_eq!(value["calibration"]["observations"], 1);
        let personal = value["preferment"]["hours_from_feeding_to_peak"]["mid"].as_f64().unwrap();
        assert!(personal < reference, "{value}");
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
        let (value, _, _) = run_in(
            Lang::Fr,
            SEARCH,
            r#"{"query":"pourquoi le yudane ébouillante la farine blanche"}"#,
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
    fn passages_name_their_page_and_their_numbers_resolve_to_citations() {
        let knowledge = Knowledge::fixture();
        let ctx = Context { knowledge: &knowledge, lang: Lang::En, site: Some(SITE) };
        let (value, _, mut sources) = run(SEARCH, r#"{"queries":["yudane"]}"#);
        // A passage read on a page of the site links to it; a statement taken from a source
        // has that source's link instead.
        let page = |start: &str| {
            let passages = value["passages"].as_array().unwrap();
            let passage = passages
                .iter()
                .find(|p| p["text"].as_str().unwrap().starts_with(start))
                .unwrap_or_else(|| panic!("{start}: {value}"));
            passage.get("page").cloned()
        };
        assert_eq!(page("Yudane scalds"), Some(json!("https://breadator.test/en/learn/yudane")));
        assert_eq!(
            page("Yudane milk bread."),
            Some(json!("https://breadator.test/en/recipes/milk-bread"))
        );
        assert_eq!(page("Yudane is flour"), None);

        // A statement of the article citing nothing would stand on the article itself.
        sources.push(Source {
            n: 3,
            keys: Vec::new(),
            kind: "article".to_owned(),
            id: "yudane".to_owned(),
            title: "Yudane".to_owned(),
        });
        let paper = cite(&knowledge.lib, "zeng2023").text;
        assert!(paper.starts_with("Zeng, F. (2023). Effect of fermentation methods"), "{paper}");
        let video = json!({
            "text": "Novita Listyani — Tangzhong video, 12:40",
            "url": "https://www.youtube.com/watch?v=ocBV6-O5SL4&t=760s",
        });
        let listed = source_list(&ctx, &sources);
        let entry = |n: usize| listed.as_array().unwrap().iter().find(|s| s["n"] == n).cloned();
        let both = sources.iter().find(|s| s.keys.len() == 2).unwrap().n;
        let alone = sources.iter().find(|s| s.keys.len() == 1).unwrap().n;
        assert_eq!(
            entry(both),
            Some(json!({
                "n": both,
                "citations": [video, {"text": paper, "url": "https://doi.org/10.1002/jsfa.12565"}],
            }))
        );
        assert_eq!(entry(alone), Some(json!({ "n": alone, "citations": [video] })));
        assert_eq!(
            entry(3),
            Some(json!({
                "n": 3,
                "page": {"title": "Yudane", "url": "https://breadator.test/en/learn/yudane"},
            }))
        );
    }

    #[test]
    fn articles_are_listed_and_read_with_their_sources() {
        let (value, record, _) = run_in(Lang::Fr, "list_articles", "{}");
        assert!(record.ok, "{value}");
        assert_eq!(
            value["articles"],
            json!([{
                "slug": "yudane",
                "title": "Le yudane",
                "description": "d",
                "topic": "Empois",
                "url": "https://breadator.test/fr/learn/yudane",
            }])
        );

        let (value, record, sources) = run("read_article", r#"{"slug":" yudane "}"#);
        assert!(record.ok && sources.is_empty(), "{value}");
        let paper = cite(&Knowledge::fixture().lib, "zeng2023").text;
        assert_eq!(
            value.as_str().unwrap(),
            format!(
                "# Yudane\n\nd\n\nScalds · 2026-11-24 · https://breadator.test/en/learn/yudane\n\n\
                 Yudane scalds part of the flour with boiling water [1][2].\n\n\
                 ## Sources\n\n\
                 [1] Novita Listyani — Tangzhong video, 12:40 https://www.youtube.com/watch?v=ocBV6-O5SL4&t=760s\n\
                 [2] {paper} https://doi.org/10.1002/jsfa.12565\n"
            )
        );
    }

    #[test]
    fn recipes_are_listed_and_scaled() {
        let (value, record, _) = run("list_recipes", "{}");
        assert!(record.ok, "{value}");
        assert_eq!(
            value["recipes"],
            json!([{
                "slug": "milk-bread",
                "title": "Milk bread",
                "description": "Yudane milk bread.",
                "bread_type": "Pan breads",
                "url": "https://breadator.test/en/recipes/milk-bread",
            }])
        );

        // As written: 900 g of dough at 100 + 70 + 2.
        let (value, record, _) = run_in(Lang::Fr, "get_recipe", r#"{"slug":"milk-bread"}"#);
        assert!(record.ok, "{value}");
        assert_eq!(value["title"], "Pain de mie");
        assert_eq!(value["url"], "https://breadator.test/fr/recipes/milk-bread");
        assert_eq!(value["weights"]["metrics"]["dough_g"], 900);
        assert_eq!(value["formula"]["target"], json!({"by": "dough", "grams": 900.0}));
        assert!(
            value["formula"].get("stages").is_none() && value.get("notes").is_none(),
            "{value}"
        );
        assert_eq!(
            value["sources"],
            json!([{
                "text": "Novita Listyani — Tangzhong video, 12:40",
                "url": "https://www.youtube.com/watch?v=ocBV6-O5SL4&t=760s",
            }])
        );

        // Scaled to 1 kg of flour; the formula returned is the scaled one and solves as is.
        let (value, record, _) =
            run("get_recipe", r#"{"slug":"milk-bread","target":{"by":"flour","grams":1000}}"#);
        assert!(record.ok, "{value}");
        assert_eq!(value["weights"]["totals"][1], json!({"id": "water", "grams": 700, "pct": 70}));
        let (again, record, _) = run("solve_formula", &value["formula"].to_string());
        assert!(record.ok, "{again}");
        assert_eq!(again["totals"], value["weights"]["totals"]);
        assert_eq!(again["calculator_url"], value["calculator_url"]);
    }

    #[test]
    fn the_bibliography_is_searched_by_words() {
        let zeng = json!({
            "text": cite(&Knowledge::fixture().lib, "zeng2023").text,
            "url": "https://doi.org/10.1002/jsfa.12565",
        });
        for arguments in ["{}", r#"{"query":"ZENG 2023 fermentation"}"#] {
            let (value, record, _) = run("list_sources", arguments);
            assert!(record.ok, "{value}");
            assert_eq!(value, json!({"total": 1, "sources": [zeng]}), "{arguments}");
        }
        let (value, _, _) = run("list_sources", r#"{"query":"zeng 1999"}"#);
        assert_eq!(value, json!({"total": 0, "sources": []}));
        let (value, _, _) = run("list_sources", r#"{"offset":30}"#);
        assert_eq!(value, json!({"total": 1, "sources": []}));
    }

    #[test]
    fn every_tool_is_listed_once_and_runs() {
        let names: Vec<&str> = TOOLS.iter().map(|tool| tool.name).collect();
        assert_eq!(
            names,
            [
                "search_knowledge",
                "list_articles",
                "read_article",
                "list_recipes",
                "get_recipe",
                "list_sources",
                "compute_formula",
                "solve_formula",
                "water_temperature",
                "predict_fermentation",
                "predict_starter",
                "plan_existing_preferment",
                "blend_flour",
                "convert_yeast",
            ]
        );
        for tool in TOOLS.iter() {
            // A name the registry lists is one `run` knows: its refusal is about arguments.
            let (value, _, _) = run(tool.name, r#"{"no_such_argument":1}"#);
            assert!(!value["error"].as_str().unwrap().contains("unknown tool"), "{}", tool.name);
            assert_eq!(tool.parameters["type"], "object", "{}", tool.name);
            for required in tool.parameters["required"].as_array().unwrap() {
                let name = required.as_str().unwrap();
                assert!(tool.parameters["properties"].get(name).is_some(), "{}: {name}", tool.name);
            }
        }
    }

    #[test]
    fn every_argument_a_schema_offers_is_accepted() {
        for tool in TOOLS.iter() {
            let name = tool.name;
            let properties = tool.parameters["properties"].as_object().unwrap();
            let base = match name {
                "compute_formula" => to_json(&QuickForm::default()).unwrap(),
                "water_temperature" => to_json(&QuickTemperature::default()).unwrap(),
                "predict_fermentation" => to_json(&QuickFerment::default()).unwrap(),
                "predict_starter" => to_json(&QuickStarter::default()).unwrap(),
                "plan_existing_preferment" => to_json(&QuickPreferment::default()).unwrap(),
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
                        "plan_existing_preferment" => {
                            overlay(&QuickPreferment::default(), &[], &probe).is_ok()
                        }
                        "blend_flour" => overlay(&QuickFlour::default(), &[], &probe).is_ok(),
                        _ => overlay(&QuickFerment::default(), &[], &probe).is_ok(),
                    };
                    assert!(accepted, "{name}: {argument} = {variant}");
                }
            }
        }

        // The full formula: each kind of ingredient and of stage, each way to give the amount.
        let solve = get("solve_formula").unwrap();
        let items = |list: &str| &solve.parameters["properties"][list]["items"]["properties"];
        for kind in items("ingredients")["kind"]["enum"].as_array().unwrap() {
            assert!(serde_json::from_value::<IngredientKind>(kind.clone()).is_ok(), "{kind}");
        }
        assert_eq!(items("ingredients")["kind"]["enum"].as_array().unwrap().len(), 19);
        for kind in items("stages")["kind"]["enum"].as_array().unwrap() {
            assert!(serde_json::from_value::<StageKind>(kind.clone()).is_ok(), "{kind}");
        }
        for by in
            solve.parameters["properties"]["target"]["properties"]["by"]["enum"].as_array().unwrap()
        {
            assert!(target(&json!({ "by": by, "grams": 500, "count": 2 })).is_ok(), "{by}");
        }
        // An observation of each kind, with the fields its schema names.
        for kind in ["peak", "bulk"] {
            let fields = get("plan_existing_preferment").unwrap().parameters["properties"]
                ["observations"]["items"]["properties"]
                .as_object()
                .unwrap();
            let mut observation = Map::new();
            for (field, schema) in fields {
                let described = schema["description"].as_str().unwrap();
                let other = if kind == "peak" { "bulk:" } else { "peak:" };
                if field != "kind" && described.starts_with(other) {
                    continue;
                }
                let value = match field.as_str() {
                    "kind" => json!(kind),
                    "endpoint" => json!("rise50"),
                    "temp_c" => json!(24),
                    "dose_pct" => json!(20),
                    "hydration_pct" => json!(70),
                    "salt_pct" => json!(2),
                    "whole_grain_pct" => json!(0),
                    "seed_hydration_pct" | "starter_hydration_pct" => json!(100),
                    // Hours observed, parts of flour and of water.
                    _ => json!(5),
                };
                observation.insert(field.clone(), value);
            }
            let arguments = json!({
                "seed_g": 10, "feed_flour_g": 50, "feed_water_g": 50,
                "observations": [observation],
            });
            let (value, record, _) = run("plan_existing_preferment", &arguments.to_string());
            assert!(record.ok, "{kind}: {value}");
        }
    }
}
