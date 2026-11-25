# Breadator research corpus — conventions

Breadator (`breadator.robinluiz.fr`) is a public web app on bread making: calculators (baker's
math, desired dough temperature, bulk/proof time across temperature profiles, yeast and levain
dosing, preferments, scalds such as tangzhong / yudane / empois, hydration), a knowledge base
(enzymes, starch, gluten, fibres, acidity…), recipes and troubleshooting. This folder holds
the research that feeds it. Every research agent writes in the same shapes so the corpora can
be merged.

## Layout

| Path | Content | Published? |
|---|---|---|
| `raw/<source>/…` | fetched originals: transcripts, pages, XML, PDFs, API dumps | **never** (git-ignored, copyright) |
| `notes/<source>/SUMMARY.md` | coverage (processed / skipped and why), key insights, disagreements, open questions | yes |
| `notes/<source>/<topic>.md` | synthesized notes per topic, every claim cited | yes |
| `data/claims-<source>.jsonl` | one claim per line (schema below) | yes |
| `data/recipes-<source>.json` | normalized recipes (schema below) | yes |
| `data/references-<source>.json` | papers/books cited: `{key, authors, year, title, venue, doi, url, oa_url, cited_by:[claim ids]}` | yes |
| `data/*.csv` | numeric tables (e.g. Sourdough Framework Table 7.1), with a `source` column | yes |

`<source>` ∈ `novita`, `breadcode`, `science`, `project` (the owner's Claude project), `landscape`.

**Exception — `project` is private.** Everything derived from the owner's Claude project
(`notes/project/`, `data/*-project.*`, including `bakes-project.csv`) holds conversation ids,
titles and personal bakes: it is git-ignored, never published, and `tools/merge_corpus.py` does not
merge it. Use it locally to write content and to validate models (aggregate residuals only); public
content cites the primary sources the project relied on, never the project itself.

## Audience rule — generic, not personal

The app serves many bakers. **Never record the site owner's personal setup as a fact**: his
kitchen temperature, his flour brands, his mixer, his oven, his starter's vigour, his bakeware,
his inventory or his diet goals. Keep the *mechanism or lesson* behind a personal observation,
generalized and labelled (e.g. "a starter can be more vigorous than the Table 7.1 baseline →
calibrate inoculation from your own bakes", not "the stiff T65 levain is more vigorous").

## Evidence rules

- Every claim carries a source with a locator: video id + `mm:ss`, page URL + section,
  DOI + table/figure, *The Sourdough Framework* chapter/section/table, project conversation id.
- Label evidence honestly (`evidence` field): `measured` (controlled experiment shown or
  published), `literature` (cites a paper — record the paper), `creator-rule` (a creator's rule of
  thumb or practice), `anecdote`, `extrapolation` (reasoned beyond the data).
- When sources disagree, record both claims and add the disagreement to `SUMMARY.md`. Do not
  resolve silently.
- Narration-level numbers that cannot be traced to a primary source get `confidence: "low"` and
  a note.
- Units: grams, °C, minutes/hours, baker's percentages (flour = 100). Convert and keep the
  original in `notes` when converting.

## Copyright rules

- Raw material stays in `raw/`. Notes **paraphrase**; quotes ≤ one short sentence, in quotation
  marks, with locator.
- Quantities, formulas, temperatures, durations and ingredient lists are facts: record them exactly.
- *The Sourdough Framework* is CC BY-SA 4.0: adapted text is allowed with attribution, and
  anything adapted from it inherits CC BY-SA. Mark such claims with `"license": "CC-BY-SA-4.0"`.
- Videos, Novita Listyani's site and other web pages are all-rights-reserved: facts only.

## Topic taxonomy (`topic` field; dotted sub-topics allowed, e.g. `scalds.yudane`)

`flour` (protein, ash/type T45–T150, absorption, damaged starch, whole grain, rye, spelt,
einkorn, other grains) · `gluten` · `starch` (gelatinization, retrogradation, damaged) · `enzymes`
(amylases, proteases, xylanases, phytase, lipoxygenase, β-glucanase, malt, glucose oxidase) ·
`fibre` (bran, arabinoxylans, β-glucan, psyllium, seeds, soakers) · `hydration` · `salt` ·
`sugar` · `fat` · `dairy-egg` · `yeast` (types, conversions, dosing vs time/temperature,
osmotolerance) · `sourdough-microbiology` (LAB, wild yeasts, acids, pH/TTA) · `starter`
(maintenance, ratios, hydration, peak timing, stiff vs liquid) · `preferment` (poolish, biga,
sponge, pâte fermentée, levain-levure, refrigerated sponge) · `scalds` (tangzhong, yudane,
empois, Brühstück, porridge) · `mixing` (autolyse, fermentolyse, bassinage, friction,
windowpane, folds) · `dough-temperature` (DDT, base temperature, friction factor) · `bulk`
(endpoints, aliquot jar, rise %, time models) · `shaping` · `proof` (final proof, cold retard,
fridge cooling) · `baking` (oven, steam, vessel, core temperature, bake loss) · `staling-storage`
· `nutrition` (GI, resistant starch, FODMAP, β-glucan, minerals/phytate) · `troubleshooting` ·
`flavour` · `bread-type` (lean hearth, pan, enriched, laminated, pizza, flatbread, rye, gluten-free,
steamed, fried).

## Claim schema — `data/claims-<source>.jsonl`

```json
{"id": "novita-0001",
 "topic": "scalds.tangzhong",
 "claim": "Paraphrased statement.",
 "quantities": {"flour_to_water": "1:5", "temp_c": 65},
 "conditions": "white bread flour, 5% of total flour scalded",
 "source": {"type": "video|web|paper|book|project", "ref": "youtube:ocBV6-O5SL4",
            "locator": "12:40", "author": "Novita Listyani", "url": "https://…"},
 "evidence": "measured|literature|creator-rule|anecdote|extrapolation",
 "cites": ["doi:10.1002/jsfa.12565"],
 "confidence": "high|medium|low",
 "license": null,
 "notes": ""}
```

## Recipe schema — `data/recipes-<source>.json` (array)

```json
{"id": "novita-rsd-whole-wheat-18h",
 "title": "…",
 "source": {"type": "video", "ref": "youtube:DtsBfGiq1H0", "url": "…",
            "locator": "description; 03:12–07:40", "author": "Novita Listyani"},
 "bread_type": "enriched-pan",
 "leavening": "instant-yeast|active-dry-yeast|fresh-yeast|sourdough|hybrid|chemical|none",
 "total_flour_g": 375,
 "ingredients": [
   {"name": "whole wheat flour", "kind": "flour|water|milk|egg|fat|sugar|salt|yeast|levain|inclusion|other",
    "grams": 170, "baker_pct": 45.3, "stage": "poolish|levain|scald|soaker|final|filling|topping",
    "notes": "≥13.5% protein"}],
 "stages": [
   {"name": "poolish", "kind": "preferment|scald|soaker|levain-build",
    "flour_g": 170, "water_g": 170, "yeast_g": 2, "temp_c": 4, "duration_h": 18, "notes": ""}],
 "totals": {"hydration_pct": 77, "salt_pct": 1.7, "sugar_pct": 5.3, "fat_pct": 6.7,
            "prefermented_flour_pct": 45, "scalded_flour_pct": 10,
            "hydration_note": "counts milk water"},
 "process": [
   {"step": "mix|autolyse|bulk|fold|divide|preshape|bench|shape|proof|retard|bake|cool",
    "temp_c": 29, "duration_min": 90, "endpoint": "doubled", "notes": ""}],
 "bake": {"oven_c": [200, 185], "minutes": [20, 18], "steam": "covered|injected|none",
          "vessel": "loaf pan", "core_c": 96},
 "yield": {"dough_g": 723, "pieces": 1},
 "quantities_from": "description|on-screen|transcript|page|xml",
 "claims": ["novita-0042"],
 "notes": ""}
```

Fill what the source gives; leave the rest `null`. Never invent quantities; when a value is
inferred, say how in `notes`.
