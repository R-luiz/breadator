# Breadator content — how to write it

Articles and recipes are compiled into the site and validated: run
`cargo test -p breadator-content --test repo_content` after every change, from the repository
root (on Windows, see the note on `PATH` in the README).
It fails on any unknown citation, missing translation or invalid recipe formula.

## Audience and voice

- Home bakers of every level, worldwide, metric units. Explain the mechanism, then what to do
  with it. Never assume one person's kitchen, flour brand, oven or starter.
- Plain, precise sentences. Numbers with their conditions ("at 24 °C dough temperature",
  "for a 100 % hydration starter"). Ranges rather than false precision.
- When sources disagree, say so and give both sides with their evidence; do not pick silently.
  Mark single home tests as such ("in one side-by-side test…").
- No marketing, no filler, no emoji. 500–1,200 words per article; one idea per section.
- Link the calculators where they help: `/fr/calculators/formula`, `/fermentation`, `/starter`,
  `/flour`, `/temperature`, `/yeast` (use the article's language prefix).

## Languages

Every article exists in `fr.md` and `en.md` with the same structure, the same citations in the
same order, the same `topic` and `order`. Write each language natively, not word-for-word.
French uses bakery terms: pointage, apprêt, façonnage, rabat, bassinage, autolyse,
fermentolyse, frasage, pétrissage, levain chef, rafraîchi, bocal témoin, température de base,
taux d'ensemencement, farine préfermentée, empois, grigne, alvéolage, mie, croûte, pâte bâtarde.

## Citations

- `[@id]` after the sentence it supports; several: `[@a; @b]`. Every paragraph that states a
  fact, number or rule carries at least one citation.
- Ids come from `research/data/claims.jsonl` (claim ids such as `novita-0042`,
  `breadcode-1102`, `science-ferm-0007`) or `research/data/bibliography.json` (keys such as
  `zeng2023`). Prefer the bibliography key of the paper when a claim cites one (`cites` field);
  cite the claim id for a creator's experiment or rule.
- Never cite or use anything under `research/notes/project/` or `research/data/*-project.*`
  (private).
- Text adapted from *The Sourdough Framework* (claims with `"license": "CC-BY-SA-4.0"`) is
  allowed with its citation; everything else is paraphrased: no sentence copied from a video
  transcript or web page.

## Article file

`content/articles/<slug>/<lang>.md` — slug in lowercase ASCII with dashes, identical for both
languages.

```markdown
+++
title = "Tangzhong, yudane and other scalds"
description = "One sentence shown in search results and on the topic index."
topic = "scalds"
order = 1
updated = "2026-11-24"
+++
Opening paragraph: what it is and why it matters [@novita-0123].

## Section heading
…
```

`topic` comes from the taxonomy in `research/CONVENTIONS.md` (`flour`, `starch`, `enzymes`,
`starter`, `preferment`, `scalds`, `mixing`, `dough-temperature`, `bulk`, `proof`, `baking`,
`staling-storage`, `troubleshooting`, `yeast`, `sourdough-microbiology`, `bread-type`…).

## Recipe file

`content/recipes/<slug>.toml`: an engine formula (same JSON shape as the formula calculator's
`?s=` state, written as TOML) plus translated text. Every quantity comes from a source recipe in
`research/data/recipes-*.json` (not the private project); convert to baker's percentages with all
flour = 100 % and stages carved from the totals (see `crates/breadator-engine/src/formula.rs`).

```toml
bread_type = "enriched-pan"
sources = ["novita-0456", "yamauchi2014"]
title = { fr = "Pain de mie au tangzhong", en = "Tangzhong milk bread" }
description = { fr = "…", en = "…" }
steps = [
  { fr = "…", en = "…" },
]
notes = { fr = "…", en = "…" }

[formula]
target = { by = "pieces", count = 1, grams = 900 }
flours = [{ id = "bread_flour", pct = 100 }]
ingredients = [
  { id = "milk", kind = "milk", pct = 70 },
  { id = "salt", kind = "salt", pct = 1.8 },
  { id = "yeast", kind = "instant_yeast", pct = 1 },
]
stages = [
  { id = "scald", kind = "scald", flour_pct = 10, liquid_pct = 200, liquid = "milk", evaporation_pct = 10 },
]
```

Ingredient kinds: water, milk, cream, yogurt, beer, egg, egg_yolk, egg_white, butter, oil, sugar,
honey, salt, fresh_yeast, active_dry_yeast, instant_yeast, osmotolerant_yeast, inclusion, other.
Stage kinds: levain, poolish, biga, sponge, pate_fermentee, scald, soaker. Steps give
temperatures and durations with their conditions and point to the fermentation calculator for
the bulk and proof times rather than fixing a clock time.
