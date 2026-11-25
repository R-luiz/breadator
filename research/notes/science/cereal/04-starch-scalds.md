# 4. Starch gelatinization and scalds

A scald pre-gelatinizes part of the flour's starch with hot water. Its flour and water stay in the recipe totals; what changes is how much water the dough carries at the same consistency, its sugars, its softness and its staling. The calculator data are in `model-flour.json` → `scalds`.

## Gelatinization temperatures

**Wheat starch in excess water.** DSC onset 60.7 °C (Woodbury 2022, 1:2 starch:water; science-flour-0082). Pre-heated to the DSC conclusion temperature (65.6–67.5 °C) wheat starch still had 1.7–2.8 J/g of gelatinization enthalpy; none remained after 71.0–73.6 °C (Wang et al. 2018, [doi:10.1038/s41598-018-21451-5](https://doi.org/10.1038/s41598-018-21451-5); science-flour-0079). With a full enthalpy of about 11.9–12.6 J/g (Yost 1985; science-flour-0080), wheat starch is only about 76–86 % gelatinized at 66–67 °C (science-flour-0081, extrapolation). Rule adopted: complete ≈ onset + 12 °C.

**Water.** Below about 66–67 % water (water:starch under 2) a second, higher endotherm appears and the transition spreads over 40–50 °C (science-flour-0079, -0080); without salt, the peak rose from 62 to 74 °C as water fell from 35 to 25 % (science-flour-0059). Flour is about 70 % starch, so a 1:2 scald is in the excess-water regime, while a 1:1 yudane (about 62 % water counting the flour's own moisture) sits just below it. A Flory–Huggins form describes melting versus water and sugars (Renzetti 2025; science-flour-0083) but its wheat parameters are incomplete in the text read, so it is not used as a formula.

**Sugar.** Onset in sucrose solutions: 60.71 (water), 65.80, 72.23, 82.35 and 98.49 °C at 15, 30, 45 and 60 % sucrose in the water phase; glucose and fructose shift less (science-flour-0082; Allan 2020 gives +3.5 to +28.9 °C at 0.5–2.0 M sucrose). Example: 20 % sugar with 60 % water → 25 % in the water phase → onset 70.1 °C, complete near 82 °C.

**Salt.** In dough, onset/peak/end moved from 60.17/70.83/79.83 °C (no salt) to 61.56/72.22/82.68 (1 % NaCl) and 61.73/72.90/84.93 °C (2 %) (Rodriguez de Marco 2022; science-flour-0058).

**Other grains.** Rice starch onset 58.9–60.0 °C, maize 64.3–65.8 °C (science-flour-0079); amylograph start 54.6 °C for wholemeal rye, 56.1 °C for white wheat, 59.8 °C buckwheat, 61.0 °C barley (science-flour-0084). Textbook ranges (wheat 51–60 °C, rice 68–78 °C; science-flour-0085) do not match these DSC values.

## Scald types

| Scald | Ratio flour:water | Temperature | Share of flour | Source |
|---|---|---|---|---|
| Tangzhong 1:5 | 1:5 | 65 °C classic; 80 °C (BAKERpedia) | 5–10 % (6.25 % in BAKERpedia's formula) | science-flour-0086, novita-0846 |
| Yudane | 1:1 boiling water poured on flour; mix ends at 55 °C | rest 1 day at 5 °C | 10–40 % tested | science-flour-0072, -0075 |
| Empois / pregelatinized flour or starch | cooked paste | 90–100 °C | 5–20 % tested | science-flour-0077, -0078 |
| Brühstück (meal, flakes, old bread) | 1:1–1:3 boiling water | rest 2–6 h | ≤ 30–50 % of cereal products | science-flour-0089 |
| Hot-soaked grain | 1:1 boiling water, ends near 60 °C | 2–4 h, enzymes active | 10–20 % | science-flour-0069, -0090 |
| Kochstück / boiled grain | TA 350–400 | 92–100 °C, enzymes destroyed | ≤ 15 % | science-flour-0069, -0089, -0090 |
| Baltic rye scald | 1:1.5–1:2.5, 92–96 °C water, 1 % malt | 90 min saccharification 75 → 55 °C, optional lactic fermentation | 10–43 % | science-flour-0087, -0088 |

## Water: accounting and shift

Accounting (science-flour-0095): F_s = s × F, W_s = r × F_s; the dough gets W_total − W_s − other stages. The scald's water caps its share: at 70 % hydration a 1:5 tangzhong can take at most 14 % of the flour, 1:2 at most 35 %, 1:1 at most 70 %. Evaporation is measured by weighing (novita-0515).

Shift at equal consistency, ΔH = k × (scalded flour %):
- 1:1 boiling-water yudane: k = 0.15 (Yamauchi 2014: 68 → 71 → 74 % at 0/20/40 %; science-flour-0072); Tsuboi 2017 kept total water at 68 % for 10–20 % (science-flour-0075).
- cooked to ≥ 75 °C: k ≈ 0.75 (extrapolation: fully gelatinized starch let pizza dough carry 1.1 g more water per g, Matsumoto 2022, science-flour-0077; flour is about 70 % starch).
- hot scald held at 60–70 °C: k ≈ 0.4 (between the two).

Worked (recomputed): 1000 g T65 at 66 % with 20 % yudane 1:1 → ΔH = 3.0 → 690 g water, 200 g of it in the scald, 490 g left. A 6.25 % tangzhong 1:5 cooked to 80 °C → ΔH = 4.69 → 706.9 g water, 312.5 g in the scald, 394.4 g left.

## Effects

- Dough: final proof +2 min (yudane 20–40 %) or +5 min (10–20 %); gas retention 130 → 119 → 108 mL; specific volume 5.99 → 5.79 → 5.37 mL/g; dough stiffer at equal water (science-flour-0072, -0075, -0077).
- Sugars: maltose 24.1 → 33.9 → 47.2 mg/g bread (science-flour-0073); saccharified rye scalds 11.8–18.2 g/100 g maltose, 25–28 g/100 g dry matter in a bakery scald (science-flour-0088).
- Softness and staling: initial hardness −10 % and −22 %, staling rate 0.97 → 0.89 × 10⁻² h⁻¹ (science-flour-0074); limiting hardness 7.67 → 5.65 kPa at 0 → 20 %, with H(t) = H_L − (H_L − H_0) exp(−A(θm − θ)³ t), θm 67.6 °C (control) and 58.8 °C (20 % yudane) (science-flour-0075, -0076). A fermented rye scald made rye bread 46 % softer but smaller; in barley bread it hardened the crumb (science-flour-0087).
- Fermentation: scalded flour is gelatinized and its enzymes partly or fully inactivated; a fermented rye scald reaches pH 3.6, TTA 12.5 on its own (science-flour-0087).

## Cross-check with Novita's claims

| Topic | Novita | Science | Verdict |
|---|---|---|---|
| Cooking temperature | 65 °C classic, about 90 % gelatinized (novita-0614, -0846, -0013); newer target 75–80 °C | 76–86 % at 66–67 °C, complete 71–74 °C | disagree on 65 °C; agree on 75–80 °C (science-flour-0081) |
| Water per % scalded | 68/71/74 % (novita-0120, -0369); +2 points per 10 % above 10 % (novita-0129); < 2 points at 30–40 % (novita-0514) | +0.15 point per % (Yamauchi); 0 (Tsuboi); ≈ +1 per % fully gelatinized starch | -0120 agrees; -0129 slightly high; -0514 contradicts (science-flour-0091) |
| Share, softness, staling | 10–20 % softest, > 30 % stales faster (thesis; novita-0005, -0481, -0495, -0826) | yudane 40 % softest and slowest staling | disagree (science-flour-0092) |
| 1:1 gelatinizes well | novita-0764, -0803 | 1:1 yudane ended mixing at 55 °C, below the 60.7 °C onset | disagree unless kept above 65 °C (science-flour-0093) |
| Rye scald | 10–30 %, 1:2–1:3 (novita-0451) | 42.9 % at 1:1.5 (Baltic bread); IREKS ≤ 20 %; Plötzblog ≤ 30–50 % | sources disagree (science-flour-0094) |
| Yudane maltose | about 2× at 40 % (novita-0114); 23 mg/g in yudane 40 (novita-0476) | 47.2 vs 24.1 mg/g | -0114 agrees; 23 mg/g is the increase, not the content (science-flour-0104) |
| Cold rest | 8–24 h in the fridge (novita-0115, -0014) | both studies: 1 day at 5 °C | agree |

## Gaps

The full text of Parenti et al. 2019 (gelatinized flour in brown wheat bread, [doi:10.1016/j.lwt.2019.02.066](https://doi.org/10.1016/j.lwt.2019.02.066)) was blocked by a captcha and two repository challenges, so its absorption shift is not used; the earlier draft's figures for it could not be re-verified. Carrillo-Navas 2016 and Day 2013 were read as abstracts only. Ratnayake 2009 (NaCl and sucrose DSC) was not retrievable.
