# 8. Acidity: pH and degree of acidity

pH says how strong the acid is (free hydrogen ions); the degree of acidity (Säuregrad, the German form of TTA) says how much acid there is, free and bound. IREKS measures both on 10 g of sourdough or crumb mashed with 5 mL acetone in 100 mL distilled water; the degree of acidity is the mL of 0.1 M NaOH that neutralise it ([IREKS, pH value and degree of acidity](https://www.ireks-kompendium.com/en/rye-sourdoughs/basics-of-acidification/ph-value-and-degree-of-acidity)). The two do not convert into each other: mineral-rich flour buffers the acid, so it builds high acidity while the pH falls slowly. Machine-readable values: `model-bread-science.json`, section `acidity`; claims `science-bread-0062` to `0072`.

## Guide values (IREKS)

Rye sourdoughs (DY = dough yield = 100 + water per 100 flour):

| sourdough | degree of acidity | pH |
|---|---|---|
| refreshed sour, DY 190 | 6–8 | 4.0–4.3 |
| basic sour, DY 160 | 16–20 | 3.9–4.1 |
| full sour, DY 180 | 11–13 | 4.0–4.2 |
| basic sour, DY 200 | 14–16 | 3.6–3.9 |
| full sour, DY 230 | 13–15 | 3.8–4.1 |
| long-time sour, DY 200 | 25–30 | 3.5–3.9 |

Bread doughs and finished breads:

| bread | dough: acidity | dough: pH | bread: acidity | bread: pH |
|---|---|---|---|---|
| coarse rye (Schrotbrot) | 9–14 | 4.2–4.8 | 11–14 | 4.0–4.5 |
| rye | 7–10 | 4.2–4.5 | 8–10 | 4.2–4.5 |
| mixed rye (more rye than wheat) | 6–8 | 4.2–4.6 | 7–9 | 4.2–4.6 |
| mixed wheat | 5–7 | 5.0–5.2 | 5–7 | 4.9–5.2 |
| wheat | 4–5 | 5.2–5.6 | 2–5 | 5.2–5.6 |

Measured on commercial German breads: wheat pan bread pH 5.48, TTA 4.18 mL; rye-wheat (70:30) sourdough bread pH 4.48, TTA 9.11 mL ([Alpers 2021](https://doi.org/10.3390/foods10010076), Table 1) — both inside the IREKS rows.

## Wheat sourdough

Wheat sourdoughs typically reach pH 3.5–4.3. After 4 h at 30 °C sourdoughs measured 4.51–4.73 and their breads 4.45–4.74; after 10 h, 3.30–3.50 and 3.20–3.40 ([Islam 2024](https://doi.org/10.3390/foods13132132), section 4.1). A ripe sourdough sits at pH 3.8–4.3 ([Wikipedia (de), Sauerteig](https://de.wikipedia.org/wiki/Sauerteig)). The Sourdough Framework's example process (pH 6.0 at mixing, 3.8 after proof, 3.9 after baking) is already in the corpus as `research/data/sf-table-7-4-ph.csv`.

## What steers acidity

- **Temperature.** At 30 °C and above, homofermentative lactic bacteria dominate and make only lactic acid (mild bread); at 22–25 °C heterofermentative bacteria add acetic acid (stronger taste) ([IREKS, Raw material and process parameters](https://www.ireks-kompendium.com/en/rye-sourdoughs/raw-material-and-process-parameters-in-sourdough-methods)).
- **Firmness.** Soft, warm sours (DY 190–220) form mainly lactic acid; firm, cool sours favour acetic acid and tolerate a wider ripeness window.
- **Flour.** Higher-ash rye (type 1370) buffers more and reaches higher acidity than type 815 with the same process.
- **Starter dose.** Too much starter drops the starting pH so fast that the enzymatic breakdown wanted in the sour is cut short; short processes need more starter than long ones.

## Why rye needs acid

Above about 20 % rye, sourdough or acidifier improves aroma, crumb and keeping: lower pH raises the solubility and water uptake of rye pentosans, makes the crumb elastic and sliceable, and improves microbial stability ([IREKS, Why does rye sourdough have to be acidified?](https://www.ireks-kompendium.com/en/rye-sourdoughs/basics-of-acidification/why-does-rye-sourdough-have-to-be-acidified)). The main reason is the amylase: its pH optimum is 5–5.5, and acid protects the starch framework (note 5, with the falling-number link and the 5 % acidifier demonstration). Moderate acidity also unlocks phytase: pH 5.5 already gives 70 % phytate breakdown in whole wheat (note 5).

## Targets for the calculator (extrapolation)

Read the dough target from the rye share using the IREKS rows: wheat 5.2–5.6, mixed wheat 5.0–5.2, mixed rye 4.2–4.6, rye 4.2–4.5. An all-wheat sourdough bread usually sits lower, about 3.8–4.5, because there acidity is a flavour choice, not a crumb necessity. Confidence low: the wheat-sourdough range is pieced together from Islam 2024, Alpers 2021 and The Sourdough Framework, not from one table.

## Disagreements

Ripe-sourdough pH is given as 3.8–4.3 (Wikipedia de), 3.5–4.3 (Islam 2024) and down to 3.5 for long rye sours (IREKS). The ranges overlap; they describe different processes rather than contradict each other.

## Gaps

No source gave a conversion between degree of acidity and pH for a given flour, nor a measured lactic/acetic ratio target for home sourdough.
