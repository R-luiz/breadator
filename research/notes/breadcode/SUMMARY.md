# The Bread Code / The Sourdough Framework — research summary

Snapshot 2026-11-24. Author: Hendrik Kleinwächter ("The Bread Code"). Generic-baker corpus: no Breadator-owner setup is recorded. Book-derived content is CC BY-SA 4.0 (adapted lines in these notes inherit it); videos, websites and Loafy are all-rights-reserved (facts only); the two GitHub repos are MIT (© Hendrik Kleinwaechter).

## 1. Outputs

| Path | Count |
|---|---|
| `data/claims-breadcode.jsonl` | **2,988 claims** — book 1,105 (all `CC-BY-SA-4.0`), videos 1,772, repos 111 |
| `data/recipes-breadcode.json` | **115 recipes** — book 7, repos 13, videos 95 |
| `data/references-breadcode.json` | 66 bibliography entries from `book/references.bib`; 44 now carry `cited_by` claim ids (book claims cite `bib:<key>` plus `doi:` when known) |
| `data/sf-*.csv` | 20 tables, listed in §8 |
| `notes/breadcode/<topic>.md` | 26 topic files (every taxonomy topic that has claims; synthesis + every claim with id, locator, evidence) |
| `notes/breadcode/tools.md` | repos, websites, Loafy, **pizza-calculator formulas (§6a)**, version drift (§5) |
| `raw/breadcode/shards/` (git-ignored) | extraction shards, `verify-<video>.json` correction files, briefs |

Evidence across claims: creator-rule 2,514 · anecdote 236 · measured 139 · literature 58 · extrapolation 41. Book claims per chapter: Ch1 15, Ch2 73, Ch3 77, Ch4 83, Ch5 26, Ch6 48, Ch7 285, Ch8 24, Ch9 52, Ch10 64, Ch11 28, Ch12 218, Glossary 93, tables/figures 19 (the preface has no usable fact). Claims by topic: starter 484, baking 468, bulk 279, proof 227, sourdough-microbiology 206, mixing 190, shaping 177, hydration 164, flour 132, gluten 110, bread-type 102, troubleshooting 97, enzymes 77, staling-storage 65, yeast 58, flavour 55, preferment 24, salt 16, dough-temperature 15, fibre 11, starch 8, scalds 8, dairy-egg 6, sugar 5, fat 3, nutrition 1.

## 2. Coverage

**Videos.** `raw/youtube/breadcode/_all.txt` lists 236 videos; no `_failed.txt` exists. All 236 have a status: **210 processed** (every one has ≥1 claim) and **26 skipped**. Publication years: 2018 14, 2019 17, 2020 66, 2021 71, 2022 19, 2023 7, 2024 11, 2025 7, 2026 24.

Skipped (reason): PcVYbH8ZkZM, yDHQL_iGLxM, Yk2-1CKcX60 (beekeeping); SVXlUvDw73E (coconut husking); uOReVJpIkew (19 s clip, music only; description gives only "105 % hydration"); bC_Z0Wnb6_s (22 s clip, no bread content); 7UqsdawpbnA (buried-starter stunt, no result shown); pXr4rNDEbbY (ocean-water starter novelty, no verifiable quantities); vFEH2PxDa44 (giveaway vlog); tXATAy92Ce8, -J58bWVV9Ks, 4kyr9BDOYfw, KX4NFkOpNAA, k3pXOKabG3o, By_Jgq4csGg (no transcript and no quantities in the description; KX4NFkOpNAA and 4kyr9BDOYfw only link out); 7-J6RHGPQ38 (channel welcome); uJMQxX6QCkM (grilled-cheese short); NGsbqe238Hk, _cvJ9LMoLVA (book crowdfunding); 9ZpaiIDXDrk (challenge winners); jEFWBtcNvU8, lBJm6vFopYU, PfLxeQjb-Fc (interview chit-chat); QfQvBGSG3os (flour shopping, ASR-garbled); 8vIxb-_zh1c, 03ZQp898-lg (app/calculator promos).

**Book.** All `\input` chapters of `book/book.tex` (Ch 1–12, preface, glossary), every table in `book/tables/`, the flowcharts in `book/figures/` and the `book/plots/*.table` data. **Repos.** All 21 recipe, basics and experiment Markdown files of the-bread-code (TEMPLATE.md and README.md excluded; the `cocoa-bread.md` body is only `TODO`) and the pizza-dough README. The calculator is documented in `tools.md` §6a.

## 3. Method and quality control

1. Haiku extraction shards: 10 book chapters, 1 repo, and 18 video bins.
2. **All 210 processed videos were then re-verified by separate haiku agents** that re-read each transcript in ≤120-line windows. Several extraction agents had worked from truncated reads, so this pass was needed. Outcome on the existing video claims: 955 kept, 430 fixed (mostly locators, plus some wording and numbers; examples: an inverted claim in KRO9EcIyXSI, an invented 80–90 % range, and a misread starter-dose band in 0aylYlvA4K8), 1 dropped. The pass also added 387 claims and fixed 60 recipes. Corrections were applied at merge, and each video claim's notes say "verified against transcript" or "verified-fix: …".
3. Automatic numeric grounding: every number in `quantities` was looked up in the cited source window (book line range ±5 lines, video paragraph ±90 s plus the description, repo file lines). 1,222 claims matched in the window, 80 matched elsewhere in the same file, and 1,550 have no numbers. 136 had values not found verbatim; nearly all are unit conversions (days→h, °F→°C) or derived percentages. These carry an `auto-check:` note.
4. Dedupe by (source, normalised claim text). Table locators were normalised to the book's table numbers (7.1 starter usage, 7.2 fermentation effects, 7.3 aliquot, 7.4 pH, 10.1 baking stages, 10.2 oven methods, 11.1 storage, 3.1–6.2 as in the CSV names).
5. **Spot check of 10 random claims (seed 20261124), checked by me against the raw sources: 10/10 supported.**

| Claim | Source checked | Result |
|---|---|---|
| breadcode-0710 non-preheated Dutch-oven air 22/77/151 °C at 1/10/25 min | `plots/non-preheated_ambient.table` rows 1, 10, 25 | exact |
| breadcode-2767 seed dough too wet to stir by hand, mix in a large pot | yt:cN_skGXZ4Es [0:47] | supported |
| breadcode-0015 sourdough can't be punched down; it overferments into a sticky mess | history/sourdough-history.tex L309–315 | supported |
| breadcode-0183 stronger flour allows longer fermentation | sourdough-starter-types.tex L84–85 | verbatim sense |
| breadcode-2754 third-party test of 25/50/75/100 % bulk rise | yt:t_-FTKXgLMU [3:18] | supported; the claim is Hendrik describing another creator's video |
| breadcode-2833 egg wash: whole egg darker, half egg + half water lighter | yt:7wyikR8qwHQ description step 21 | exact |
| breadcode-0565 practise shaping on mini batards | wheat-sourdough.tex L1274–1280 | supported |
| breadcode-2483 preheating matters for high-hydration dough (surface gel) | yt:Typy5Uzp-EM [1:05] | supported |
| breadcode-1906 3-minute method works with WW/spelt/einkorn/emmer/rye; smaller rise for WW | yt:bi-zqG5y1I8 [4:35], [14:57] | supported |
| breadcode-2006 poor oven spring causes list | yt:qRX3rulyXS8 [1:51] | verbatim list |

I also verified the five 5-minute apple-surface values of breadcode-0705 (72/47/43/23/17 °C) against the plot tables. Note that `preheated_surface.table` carries a wrong header ("#Ice Cube"); the figure legend identifies it as the preheated series.

## 4. The framework's decision model (book first, with video refinements)

1. **Ready the starter** (§7.2, `sf-starter-readiness-figure.csv`). If it was fed in the last 3 days, feed 1:5:5 and use it 6–12 h later. If it is older, feed 1:10:10 twice (≈48 h and 6–12 h before). Check that it has risen and smells right. The book prefers a stiff starter at 50–60 % hydration (more yeast, less acid).
2. **Pick hydration from the flour** (§7.4, §12.2.3). Start at ≈60 % and raise it by bassinage. Whole grain and higher protein take more water; seeds add 1–2 %.
3. **Pick inoculation from temperature and target time** (§7.5, §7.7, Table 7.1). The goal is 8–12 h of bulk. Table 7.1 gives 5/10/15 % recently fed starter at 30/25/20 °C, or 2.5/5/10 % if the starter is starving. The prose gives a second rule: 10 % for an 8 h bulk, 5 % for 12 h, down to 1 % when very hot. Avoid ≈20 %: at 25 °C it finishes in ≈5 h, before the enzymes have done their work, which is why bakers then add an autolyse.
4. **No autolyse (fermentolysis).** The low inoculation stretches bulk long enough for protease and amylase to act, so all ingredients go in together. Salt timing does not matter.
5. **Bulk endpoint: aliquot jar** (§7.9, Table 7.3). Take the sample after building initial strength. Target +25/50/100/>100 % for flour protein 8–10/10–12/12–15/>15 %. Begin at +25 % and adjust over bakes. The jar only works if it stays at the dough's temperature: use room-temperature water, keep the jar beside the dough, and avoid big ambient swings or batches over 10 kg flour. Alternatives are pH (reference curve in Table 7.4; calibrate your own) or reading the dough (bubbles, tacky not sticky, ripe-starter smell). When unsure, err slightly long. Rescue over-fermented dough in a loaf pan.
6. **Proof: choose by schedule** (§7.13). Bake within ≈3 h → room-temperature proof of 30 min–3 h. The poke dent should still be visible after 1 min; check every 15 min. Otherwise retard: fermentation stops below ≈4 °C, and the dough can be baked 6–24 h after going in (48 h sometimes works). Find your fridge time by stepping 8 → 10 → 12 … 24 h, and shorten it when the dough goes in warm. Videos add 30–60 min at room temperature before the fridge, ≈16 h at 4–6 °C or ≈24 h at 4 °C, a fridge kept below ≈6 °C, baking straight from cold, and 30–45 min in the freezer before scoring.
7. **Bake** (Ch 10). Preheat to 230 °C for 30 min, bake ≈30 min with steam, then uncovered until coloured. The loaf is done at 92 °C core. Choose the steam method by oven type (Table 10.2). A longer bake gives a less sour loaf.
8. **Diagnose** with Table 7.2 and §12.4 crumb types, then adjust inoculation, temperature or rise target for the next bake.

## 5. Top 15 insights for the Breadator calculators

1. **Starter-dose table** (Table 7.1, breadcode-0452…0457): 30/25/20 °C → 5/10/15 % (fed) or 2.5/5/10 % (starving), for 8–12 h of bulk. This is the only temperature × inoculation grid in the book. It has no duration column, so the 8–12 h target and the prose pairs "10 % ≈ 8 h, 5 % ≈ 12 h" are the calibration points. Confidence: medium (one baker).
2. **Fit Q10 from the table itself.** The table halves the dose for each 5 °C rise at equal time, so the rate roughly doubles per 5 °C (≈Q10 4). That is steeper than the usual Q10 of 2–3. Expose it as an assumption with a range.
3. **Aliquot rise target by protein** (Table 7.3, breadcode-0500–0503) as the default bulk endpoint, offered as a "start at +25 %" slider. Most 2020–22 videos default to +50 %.
4. **The book's starter % is starter mass / flour mass.** Inoculated-flour % (Wraith) differs. The calculator must ask for starter hydration and convert (tools.md §6).
5. **Stiff starter = 50–60 % hydration**, liquid = 500 %, regular = 100 % (Table 4.1). Liquid starter ferments faster per gram: ≈10 % liquid ≈ 20 % regular (ES7x28d10pg, low confidence).
6. **Readiness feed ratio from days since the last feed:** ≤3 days → 1:5:5, 6–12 h before mixing; >3 days → 1:10:10 twice.
7. **Retard model:** fermentation ≈ stops below 4 °C. Window 6–24 h (48 h max). Cool-down time depends on the dough's starting temperature (breadcode-1095), so dough temperature at fridge entry belongs in the planner.
8. **Water temperature = ambient** keeps the aliquot honest. A 2 °C dough–jar mismatch was estimated to shift bulk by 3–4 h (breadcode-1483). The engine should use dough temperature, not room temperature.
9. **Baking stage thresholds** (Table 10.1): 60 / 75 / 100 / 105–120 / 118 / 170 °C. Done at **92 °C core** in the book; videos say 92–95 °C. The bake planner should use ≥92 °C and show 95 °C as the conservative value.
10. **Steam is measured:** at 5 min the surface reached 72 °C with an ice cube, 47 spritzed, 43 preheated only, 17 unpreheated (breadcode-0705). An unpreheated pot needs ≈26 min to reach the 75 °C gel point. Recommend preheating plus steam.
11. **Hydration defaults:** 60 % for beginners on most flours; seeds +1–2 %; pan loaves up to 90–100 %; rye 80 % in a pan vs ≈60 % free-standing. A hydration-test protocol (55–75 g water per 100 g flour) exists for calibration.
12. **Salt 2 % of flour** throughout the book and videos. The old repo FAQ computes it on total dough including starter (breadcode-1124). Breadator should show salt on total flour and say which base it uses.
13. **Fresh:dry yeast = 3:1** in every source. Yeast doses run from 0.05 % (pizza, ≈12–14 h, double it below 20 °C) and 0.1 % (overnight) to 1 % (fast).
14. **pH reference** (Table 7.4): mix 6.0 → divide 4.10 → shape 4.05 → after proof 3.80 → after bake 3.90. At equal rise, liquid-starter dough is more acidic (≈3.9–4.0) than regular (≈4.2) (nPSsWa4nGgU). At doubling, a 30 °C and a 25 °C ferment had similar pH (vf5RbwlW8Xs). Usable as optional endpoints.
15. **Fermentation-effects table** (Table 7.2) gives an under/over diagnosis from crumb, alveoli, taste and oven spring. The best input for the troubleshooting module.

## 6. Book vs videos vs older material (both sides recorded; not resolved)

- **Starter dose inside the book.** Table 7.1 gives 10 % at 25 °C (fed). The §7.5 prose gives ≈5 % at ≈25 °C in summer and 10–20 % at ≈20 °C in winter (breadcode-0433). Two different rules in one chapter.
- **Starter dose, videos over time.** 2020–21 videos routinely use 20 % (o_xtVmR4vlc, dl7crk2Nvj8, 2DFvfRIywC8, MqH3GVfjfBc winter 20 %). From 2021–22 the videos say 5–10 % for overnight doughs and "10 % baseline, 2.5–5 % hot, up to 20 % cold" (0aylYlvA4K8, 36GdGT0XYwE, _P1B7PxAwk0, m7J6CBZcq8o). In 2026, _cb4vCQ2shk says "never more than 20 %" (breadcode-2100), while Loafy's planner suggests up to 30 % in a very cold kitchen (tools.md §5.2). Above 25 °C a video gives 1–5 % (breadcode-2056).
- **Aliquot targets.** Book: +25/50/100/>100 % by protein. Most videos default to ≈+50 %, and to +75–100 % only for ≈14–15 % protein (KRO9EcIyXSI says "lower than doubling" even for strong flour; qM9BXKaIr60 says aim for 75 % instead of 100 % if the dough turns sticky; 36GdGT0XYwE and -LeUYBe7HlY flag ≈50 % against the book's 100 %). The 2026 videos return to "start at +25 %". The old repo used lower tiers (+10–20 … 70–110 %).
- **Stiff-starter hydration.** Book 50–60 %. Videos give 50 % (2021–22: 1:2 water:flour), 40–50 % (msqU-ylXWUs, 2022, breadcode-1382), 60 % (_cb4vCQ2shk, 2026) and 65 % for whole-wheat strength (guest in l_PLhPWE-8E).
- **Liquid starter.** Book 500 %; UYBJZ-xmRFM uses ≈300 %; rOu-phxyQbQ uses 1:2 (200 %) and found it matured faster than 1:1.
- **Autolyse.** The book rejects it in favour of fermentolysis and has dropped it even for yeasted dough. 2020–21 videos use 1–2 h or overnight autolyse (TUkPI34Go7k, JZBtmQXKWA8, 3vr2_qqeAL8, yYkTrGHNW2w) and showed better handling after an overnight autolyse (n = 1). The website "last recipe" dropped autolyse in Oct 2026.
- **Cold-proof flavour.** Book: no detectable flavour difference; retard is for scheduling only. 02WlZa_N-vI (one-off) got more vertical spring from 30 min room + 16 h fridge. Old repo experiment: more spring after a fridge proof. b7Pd7UcZxd0 notes that freezer-before-scoring may give less flavour.
- **Retard duration.** Book: ≤24 h (48 h sometimes); step 8–24 h. Videos: "at least 16 h" (qM9BXKaIr60), 16–24 h (KRO9EcIyXSI), 8–48 h (9fWzWpL5g6w 2025), pizza dough up to 7 days (-60DxtcJ9Fo, rcOxoJk7goM).
- **Fermentation stop temperature.** Book: below ≈4 °C. Videos: keep the fridge below 6 °C (-60DxtcJ9Fo); "yeast starts above ≈6 °C" (dl7crk2Nvj8); 16 h at 6 °C ≈ 24 h at 4 °C (yYkTrGHNW2w).
- **Core temperature.** Book 92 °C. Videos 92–95 °C. Old repo and the 2020–22 par-bake videos 95 °C.
- **Oven start temperature.** BUtn4HKAiBs: a 250 °C start gave *less* spring than 230 °C (breadcode-2366). 33B6nXafahs: hot 260 °C then 230 °C gave *more* spring and bottom crust. ImVvQMvGZKE bakes covered at 250 °C then 230 °C. The book uses a flat 230 °C. Single unblinded bakes on both sides.
- **Rice flour in the banneton.** Recommended in 2020 (2tOw0eyuEsU: absorbs more, outperforms regular flour). The Nov 2026 videos (HlgA2O3J0o0, _cb4vCQ2shk) advise against it because it hides over-fermentation. Tagged as an explicit update.
- **Over-fermentation timing.** nIOPCeLPqrM (2020): one dough split in two; a 6 h bulk was excellent and 8 h was too flat (temperature not stated). This contradicts a fixed "8–12 h" expectation and supports endpoint-based control.
- **Website/repo drift** (hydration-by-protein tiers, salt base, 1:1:1 vs 1:5:5 starter build, pizza method): see `tools.md` §5. The pizza calculator code cites "section 4.3" for stiff starter; stiff starter is §4.4 in the current book.

## 7. Open questions and gaps

- No book or video gives **bulk duration as a function of temperature and inoculation**: Table 7.1 has no time axis, and the videos give single points (e.g. 5 % at room temperature overnight; 1:1:1 starter peaks in ≈4 h at 25 °C, about twice as long at 23 °C, breadcode-1355; that halving per 2 °C is implausibly steep, ASR-level). The engine's time model must come from the science corpus or Wraith's model (tools.md), with Table 7.1 as a sanity check.
- Measured claims are single-baker, n = 1 per arm, unblinded (95 video claims across ≈45 videos, plus the book's apple-probe and pH data).
- 136 claims have numbers not verbatim in the source (`auto-check` note). Mostly conversions, not individually re-verified.
- ASR transcripts garble numbers. Claims based on them are `low` confidence and say so in notes. Where the description and transcript differ, the verifiers preferred the description and noted the conflict.
- Recipe quality varies: 95 video recipes come from descriptions or transcripts, and many leave starter hydration unstated (noted per recipe). 5 recipes have `leavening: null`: 3 repo loaves with no stated base dough, and 2 video yeast doughs (TUkPI34Go7k, yvAxtECqRAs) whose yeast type is unstated. Ingredient kinds for the 8 recipes normalised at merge were inferred from names.
- Guest content (einkorn farmer Jade in wdnX3baMB38, Anatoli in MrOM_D3hnpc, Karl de Smedt in nQWlG1buMM8) is attributed in `source.author` only for wdnX3baMB38. Elsewhere, read the guest's identity from the claim notes.
- Book line locators for the ambient-temperature plot claims (breadcode-0707…0711) point to `plots/fig-temperature-ambient.tex` without a line range.

## 8. CSV tables (`research/data/`, data rows excluding header)

| File | Rows | Content |
|---|---|---|
| sf-table-3-1-bakers-math.csv | 4 | Table 3.1 baker's-math example |
| sf-table-3-2-scaled-math.csv | 4 | Table 3.2 scaled recipe |
| sf-table-4-1-starter-types.csv | 3 | Table 4.1 regular/liquid/stiff |
| sf-table-5-1-flour-types.csv | 6 | Table 5.1 flour labels US/UK/DE/FR/IT |
| sf-table-5-2-grains.csv | 2 | Table 5.2 grains and process |
| sf-table-5-3-w-value.csv | 4 | Table 5.3 W-values |
| sf-table-6-1-bread-types.csv | 5 | Table 6.1 bread-type overview |
| sf-table-6-2-flatbread-pancakes.csv | 5 | Table 6.2 flatbread/pancake formula |
| sf-table-7-1-starter-usage.csv | 3 | Table 7.1 starter % by temperature × activity |
| sf-table-7-2-fermentation-effects.csv | 5 | Table 7.2 too short / too long / perfect (**new**) |
| sf-table-7-3-aliquot.csv | 4 | Table 7.3 aliquot rise by protein |
| sf-table-7-4-ph.csv | 7 | Table 7.4 pH breakpoints |
| sf-table-10-1-baking-stages.csv | 6 | Table 10.1 baking stage temperatures |
| sf-table-10-2-oven-methods.csv | 4 | Table 10.2 oven type × steam method (**new**) |
| sf-table-11-1-storage.csv | 4 | Table 11.1 storage options |
| sf-starter-readiness-figure.csv | 6 | readiness flowchart feeds (1:5:5 / 1:10:10) |
| sf-rye-recipe.csv | 4 | Ch 8 rye formula |
| sf-flatbread-recipe.csv | 4 | recipes/flat-bread.tex formula |
| sf-history-timeline.csv | 10 | history timeline figure |
| sf-temperature-plots.csv | 264 | Ch 10 apple-probe surface/ambient series (5 variants × 2) |
