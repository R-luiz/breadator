# Cereal and bread science — summary

Eleven topic notes (`01`–`11` in this folder) and two machine-readable models:

| File | Content |
|---|---|
| `research/data/model-flour.json` | 40 flour types (ash, protein, fibre, absorption, enzyme activity, gluten behaviour, usual bread hydration, cold-retard class), the hydration method, scalds, vital gluten |
| `research/data/claims-science-flour.jsonl` | 104 claims `science-flour-0001`–`0104` (42 measured, 37 literature, 17 creator-rule, 8 extrapolation) |
| `research/data/references-science-flour.json` | 46 references |
| `research/data/model-bread-science.json` | enzymes, ingredients, acidity, baking, staling, nutrition |
| `research/data/claims-science-bread.jsonl` | 139 claims `science-bread-0001`–`0139` |
| `research/data/references-science-bread.json` | 45 references (31 with a DOI; the others are regulations, authority pages, trade sources) |

Every value in the models names the claims it rests on and carries an evidence label. A value
reasoned beyond the data is labelled `extrapolation` with its reasoning: in the flour table that
is 158 of 320 values, because standards fix the ash of a flour type and little else.

## What was adopted

- **Flour types**: ash ranges from the national standards (French types, DIN 10355, Italian DPR
  187/2001, US 21 CFR 137); protein and fibre from composition tables where a measured value
  exists, otherwise from the analogue of the same ash class.
- **Absorption and protein**: +1.03 absorption points per protein point at constant starch damage
  (Tipples 1978). Default absorption of common wheat when no mill value is known:
  59 + 8 (ash − 0.55) + 1.0 (protein − 11.5), ± 3 points (extrapolation).
- **Moisture**: A_M = (A₁₄ + 14)(100 − M)/86 − M, a mass balance that matches Farrand's moisture
  coefficient (−1.86 against −1.97).
- **Blends**: linear mean of the flours; against an industry table it is off by −0.8 to +2.4
  points, so wheat–rye blends carry ± 2.5 points.
- **Hydration method**: start from the flour types' usual bread hydration (mode A) or from a
  recipe that works, adding only the absorption difference (mode B); then add the water of the
  additions at equal dough consistency: vital gluten 1.5 g/g, oat bran 1.6, wheat bran 1.1,
  psyllium 9.5, ground chia 5.5, seeds and grains from soaker ratios.
- **Scalds**: extra water = k × % of flour scalded, k = 0.15 for a 1:1 yudane, about 0.4 for a
  scald held at 60–70 °C, about 0.75 for a paste cooked above 75 °C. Gelatinization finishes at
  about onset + 12 °C.
- **Vital gluten**: G = F (P_target − P_flour)/(P_gluten − P_target); 1,000 g of flour at 11 %
  raised to 13 % with 75 % gluten takes 32.3 g, plus 48 g of water.
- **Relative α-amylase activity**: liquefaction number 6000/(FN − 50), relative to a 320 s white
  flour.
- **Regulatory wording**: the β-glucan claims are quoted from Regulation (EU) 432/2012; EFSA's
  2025 refusal to lower the post-meal condition from 4 g to 2 g is recorded.
- **Mould**: USDA FSIS and BfR advise discarding bread and other moist, porous foods with visible
  mould; neither mentions starters, so "discard a mouldy starter" is an extrapolation from their
  reasoning, set beside the baking-book advice to rescue one.

## Disagreements between sources (recorded as claims, not resolved)

- **Tangzhong temperature**: 65 °C leaves starch about 76–86 % gelatinized; it finishes near
  71–74 °C, which supports the 75–80 °C target of newer practice.
- **Extra water for scalds**: +0.15 point per % scalded measured in one study, none added in
  another; a creator's rule gives +0.2 per % above 10 %.
- **Scald share and softness**: softest and slowest to stale at 40 % in one study, the opposite
  above 30 % in another. A measured 1:1 yudane ended mixing at 55 °C, below the gelatinization
  onset. Rye scald shares range from 10–30 % to 42.9 %.
- **Water binding of flour components**: coefficients differ by up to 2× between sources.
  Psyllium: 9.5 g/g in wheat dough against "no extra water" in gluten-free studies.
- **Falling number**: normal wheat is 300–360 s, 250–280 s or 230–280 s depending on the source;
  flours at 157–294 s from late-maturity amylase baked no worse in one study. Rye: 180–240 s,
  125–200 s, or a minimum of 120 s.
- **α-amylase**: stated optima run from 40–60 °C to 72–74 °C, and the heat-stability ranking
  conflicts with the inactivation figures.
- **Salt**: 1.8–2.0 % recommended by a trade source; one study finds more than 1.5 % unnecessary
  for dough quality (taste not tested).
- **Core temperature and oven loss**: 88 °C is given as enough for pan loaves, 98 °C was needed
  for shape under vacuum cooling; oven loss is 2–4 %, 4–12 % measured, or 11–18 % for baking
  plus cooling.
- **Sourdough and blood glucose**: a meta-analysis finds a small reduction at low or very low
  certainty; a later review finds the evidence insufficient; EFSA says the effect is not specific
  to sourdough; a review in healthy people finds mostly null comparisons.
- **Fructans**: a longer proof lowered them in wheat but not in rye; sourdough lowered rye
  fructans but added mannitol in wheat.
- **Flour labels**: Italian 00 is ≤ 0.55 % ash in the decree, not 0.50 %; some published label
  equivalences do not match the ash standards; two composition-table rows (T65, T170) are
  unreliable.

## Gaps

- No measured farinograph absorption for einkorn, emmer, oat or chestnut flour; no falling number
  for einkorn, emmer, oat, maize, rice or chestnut flour; no W for French T65, German types or
  typical Italian 00/0; no measured retard limit per flour type (the classes are extrapolated).
- No meta-analysis giving the GI difference between sourdough and yeast bread in GI units; no
  g/100 g resistant-starch figures for cooled, frozen or toasted bread.
- No temperature optimum for cereal phytase, no xylanase dose limits, no sugar threshold for
  ordinary yeast in lean dough, no egg dose ranges by bread type, no measured bake loss in a
  lidded pot.
- Not retrievable: Parenti 2019 (scalds), Ratnayake 2009, Rezette 2024, Codex CXS 152 and 163.
  Read as abstract only: Carrillo-Navas 2016, Day 2013. The 1963 French decree exists only as a
  scan; its ranges come from FranceAgriMer and a 2024 miller circular.
- Trade and secondary sources (BAKERpedia, IREKS, Wikipedia) are labelled with medium or low
  confidence; one ascorbic-acid limit quoted from a trade source is unchecked.
