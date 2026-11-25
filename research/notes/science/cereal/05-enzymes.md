# 5. Enzymes and malt

Flour, malt and added enzymes change dough and crumb through starch, protein, fibre and lipid breakdown. Each enzyme works in a window of pH, temperature, water and time, and stops when the bake heats it past its inactivation point. Values here are reference points; activity always depends on dose and on the temperature history of the dough. Machine-readable values: `research/data/model-bread-science.json`, section `enzymes`; claims `science-bread-0001` to `0047`.

## Amylases

**Three sources, three heat stabilities.** Cereal α-amylase has low purity, medium heat resistance and many side activities; fungal α-amylase has low heat stability; bacterial α-amylase is extremely heat stable and easy to over-dose. Only cereal amylase registers in the falling-number test ([BAKERpedia, Amylase](https://bakerpedia.com/ingredients/amylase/), comparison table).

| | optimum | inactivated | source |
|---|---|---|---|
| β-amylase (cereal, malt) | 60–65 °C (rye scald); 62–67 °C (brewing rest) | 60 °C in the dough | [Murniece 2024](https://doi.org/10.3390/foods13132077), introduction; [Wikipedia, Mashing](https://en.wikipedia.org/wiki/Mashing); [IREKS, processes during baking](https://www.ireks-kompendium.com/en/the-baking-process/processes-in-the-dough-and-the-baked-goods-during-the-baking-process) |
| α-amylase (cereal, malt) | 40–60 °C, pH 5.5–6.0 (dough); 72–74 °C (rye scald); 71–72 °C (brewing) | 75 °C in the dough; "denatures quickly above 78 °C" (brewing) | BAKERpedia; Murniece 2024; IREKS; Wikipedia |
| fungal α-amylase (*Aspergillus oryzae*) | — | about 80 °C (Rebholz citing Cauvain); no residual activity after baking | [Rebholz 2021](https://doi.org/10.1007/s00217-020-03657-y), discussion |
| bacterial α-amylase (*Bacillus amyloliquefaciens*) | 70 °C, still active at 90 °C | survives: 4.0 % of applied activity in the crumb | Rebholz 2021 |
| maltogenic amylase (*Geobacillus stearothermophilus*) | — | in buffer 50 % left after 30 min at 80 °C, none at 90 °C; in bread 17.8 ± 1.24 % survives | [Reichenberger 2019](https://doi.org/10.1007/s12161-019-01673-7) |

In Reichenberger's pan white bread (230 °C, 35 min) the crumb peaked at 97 °C; the flour's and yeast's own amylases were fully inactivated, while the added maltogenic amylase survived, apparently protected by the starch matrix and low water. Rebholz found the surviving bacterial amylase still producing maltopentaose to maltooctaose over 96 h of storage, which is why bacterial amylases cause gummy crumb when over-dosed.

**Sound wheat has little α-amylase.** Unmalted wheat flour carries considerable β-amylase but little α-amylase, too little to matter unless the grain sprouted or has late-maturity amylase (Rebholz 2021, introduction). α-Amylase only attacks damaged or gelatinised starch (BAKERpedia).

**Doses.** Added α-amylase: 20–60 ppm of flour (BAKERpedia). Manufacturer doses for the five preparations in Rebholz were 10–50 mg/kg (fungal) and 50–600 mg/kg (bacterial), and their activity per gram differed more than 40-fold (Table 1): a dose in mg/kg means nothing without the product's activity. General bakery enzymes: pH 4.0–7.5, 35–60 °C, 50–100 ppm ([BAKERpedia, Enzymes](https://bakerpedia.com/ingredients/enzymes/)).

## Falling number

The falling number is the time in seconds, including 60 s of stirring, for a stirrer to fall through a paste of 7 g flour and 25 mL water in a boiling-water bath ([IREKS, Falling number](https://www.ireks-kompendium.com/en/flour-analysis/analysis-methods-for-testing-the-starch-quality-and-the-enzyme-activity/falling-number); ICC 107/1). More α-amylase liquefies the gelatinising paste and gives a lower number. It is an indirect measure that also depends on the starch itself.

| flour | high activity | normal | low activity | source |
|---|---|---|---|---|
| wheat type 550 | 240–300 s | 300–360 s | 360–420 s | IREKS |
| wheat (industrial) | — | > 250 s; target 250–280 s | > 350 s: add amylase or malt | [BAKERpedia, Falling number test](https://bakerpedia.com/processes/falling-number-test/) |
| wheat | < 220 s moist crumb; < 150 s sprouted | 230–280 s ("optimal 200–250 s" elsewhere on the page) | > 300 s dry crumb | [Wikipedia (de), Fallzahl](https://de.wikipedia.org/wiki/Fallzahl) |
| wheat grain receival | below 250–350 s by grade (300 s Australia) | | | [Newberry 2018](https://doi.org/10.3389/fpls.2018.01356) |
| rye types 997/1150 | 120–180 s | 180–240 s | 240–300 s | IREKS |
| rye | | 125–200 s for high-quality rye bread | | Murniece 2024 |
| rye | minimum 120 s | | | Wikipedia (de) |

The amylograph (80 g flour in 450 mL water, +1.5 °C/min to 95 °C) gives the same picture: rye 250–450 / 450–650 / 650–900 AU with peaks at 58–63 / 63–68 / 68–73 °C; wheat 550 300–600 / 600–900 / 900–1200 AU at 75–80 / 80–85 / 85–90 °C (high / normal / low activity; [IREKS, Amylograph](https://www.ireks-kompendium.com/en/flour-analysis/analysis-methods-for-testing-the-starch-quality-and-the-enzyme-activity/amylograph)). For enzyme-rich rye IREKS advises a direct dough, more acidifier, slightly more salt, shorter bulk and proof and a hot oven; for enzyme-poor rye darker flour, softer dough and longer bulk.

**A low falling number is not always a fault.** Newberry 2018 baked flours from wheat with late-maturity α-amylase (falling numbers 157–294 s) and found no consistent link with oven spring, loaf volume, slice area, gas cells or crumb firmness; transgenic lines near 60 s with 30-fold amylase also baked normally. Pre-harvest sprouting is a different defect and is not covered by this result.

## Rye: amylase and acid

Rye has more α-amylase than wheat, and rye starch gelatinises at lower temperatures (from about 50 °C, done by 65–76 °C), so the amylase optimum overlaps gelatinisation. Unchecked, the enzyme breaks the starch framework, which then cannot hold the dough water: damp crumb, crust separation. α-Amylase works best at pH 5–5.5; sourdough acid lowers the pH and inhibits it ([IREKS, Why does rye sourdough have to be acidified?](https://www.ireks-kompendium.com/en/rye-sourdoughs/basics-of-acidification/why-does-rye-sourdough-have-to-be-acidified)). In an IREKS bake with type 1150 rye of falling number about 120 s, a dough acidifier (degree of acidity 200) gave a stable, sliceable crumb only from 5 % of the flour. A German textbook quoted by [Wikipedia (de), Sauerteig](https://de.wikipedia.org/wiki/Sauerteig) puts the point where sourdough acids largely stop enzyme activity at pH 4.1. The acidity targets themselves are in note 8.

Since the 1970s hybrid rye has made flours enzyme-poor and sprout-resistant: today's faults are more often small volume and dry crumb; less acid is needed, the flours take up to 10 % more water than 1960s rye, and bread-dough temperature should stay at or below 28 °C ([IREKS, Changes in the rye flour quality](https://www.ireks-kompendium.com/en/rye-sourdoughs/changes-in-the-rye-flour-quality-and-influence-on-the-sourdough-method)).

## Malt

Only enzyme-active (diastatic) malt adds amylase; kilned or inactive malt adds sugar, colour and flavour ([IREKS, Effect of malt flour and malt extract](https://www.ireks-kompendium.com/en/malt-flours-and-malt-extracts/effect-of-malt-flour-and-malt-extract)). Doses:

- Malt additive in general: 0.2–2 % of flour, depending on diastatic power and flour (Murniece 2024).
- Diastatic malt **syrup**, US formulas (baker's %): white pan 0.5–1.5, French/Italian 0.5–2.0, hard rolls 1.25, wheat rolls 2.7, light rye 0.5, pumpernickel 1.6, sweet goods 1.5–3.0, whole wheat 5–9, pretzels 1.5–6.0; US law allows malted barley flour in wheat flour up to 0.75 % ([BAKERpedia, Diastatic malt](https://bakerpedia.com/ingredients/diastatic-malt/)). Syrup doses are not dry-powder doses.
- Malt syrup per 100 g: 71.4 g carbohydrate, 21.1 g water, 6.2 g protein; malt extract is about 80 % solids with about 12 % glucose and 45 % maltose.
- Too much: soft dough, side-wall collapse (keyholing), sticky crumb in long fermentations ([BAKERpedia, Malt](https://bakerpedia.com/ingredients/malt/)).

## Other enzymes and improvers

- **Proteases**: 0.1–0.5 % of flour (e.g. 0.25 % powder or 75 HU/100 g flour), pH 4.5–6.5; excess wrecks gas retention ([BAKERpedia, Protease](https://bakerpedia.com/ingredients/protease/)). Brewing protease rests sit at 50–54 °C.
- **Xylanases** (EC 3.2.1.8) turn water-unextractable arabinoxylans into larger water-extractable ones: better handling, gas retention and crumb; IREKS adds higher water binding and better freshkeeping ([BAKERpedia, Xylanase](https://bakerpedia.com/ingredients/xylanase/); [IREKS, Enzymes](https://www.ireks-kompendium.com/en/improvers-and-mixes/enzymes)). No dose or over-dose threshold was found.
- **Lipoxygenase**: enzyme-active soy flour at no more than 0.5 %; bleaches carotenoids, strengthens dough, can give off-flavours ([BAKERpedia, Lipoxygenase](https://bakerpedia.com/ingredients/lipoxygenase/)).
- **Ascorbic acid**: mills add 5–30 ppm (0.5–3.0 g per 100 kg); flour enzyme turns it into dehydroascorbic acid, which oxidises glutathione ([IREKS, Ascorbic acid](https://www.ireks-kompendium.com/en/improvers-and-mixes/technological-effect-of-additives/ascorbic-acid)). Bread use up to 150 ppm, US limit 200 ppm; needs mixing oxygen ([BAKERpedia, Ascorbic acid](https://bakerpedia.com/ingredients/ascorbic-acid/)). BAKERpedia's "EU/UK 200 ppm except wholemeal" was not checked against EU law.
- **Glucose oxidase** makes hydrogen peroxide from glucose and oxygen, so it acts mainly during mixing ([BAKERpedia, Glucose oxidase](https://bakerpedia.com/ingredients/glucose-oxidase/)).
- **Lipases** prefer pH about 8 and above 45 °C; over-dose gives rancid notes and gas-cell collapse ([BAKERpedia, Lipase](https://bakerpedia.com/ingredients/lipase/)).

## Phytase

Cereal phytase works best at pH 4.3–5.5 (an early study: 4.3–4.6) ([Ribet 2026](https://doi.org/10.3389/fnut.2026.1777596), table and mineral section). Measured reductions:

- Whole-wheat dough brought to about pH 5.5 by sourdough or lactic acid lost 70 % of its phytate, against 40 % without leavening or acid: the flour's own phytase did most of the work ([Leenhardt 2005](https://doi.org/10.1021/jf049193q)).
- Whole-wheat bread: −62 % with sourdough, −38 % with yeast; pre-fermenting the bran raised it to about 90 %; traditional French bread kept 40 % ([Lopez 2001](https://doi.org/10.1021/jf001255z)).
- Long sourdough beats short yeast fermentation; added fungal phytase can remove all of it ([Brouns 2021](https://doi.org/10.3390/nu14010025), section 4).
- Cooking or roasting destroys phytase; low-phytase grains (maize, rice, oats, millet) need longer fermentation or a high-phytase grain ([Nkhata 2018](https://doi.org/10.1002/fsn3.846)). By extension, a scalded or porridge fraction has no phytase of its own left (labelled extrapolation).

No temperature optimum for cereal phytase was found in the sources read. What phytate reduction means for mineral status is in note 11.

## β-glucanase

Brewers rest malt at 40–45 °C for β-glucanase. EFSA excludes enzymatically degraded oat or barley β-glucans from its evidence because lower molecular weight and viscosity lower the post-meal glucose effect ([EFSA 2025](https://doi.org/10.2903/j.efsa.2025.9630), methods, citing Wood et al. 1994). No study measuring how much flour β-glucanase degrades β-glucan in bread was found; so the β-glucan of a finished bread, not of its flour, is what counts for the EU claims (extrapolation, low confidence; note 11).

## Disagreements

- Wheat falling-number targets differ by source: 300–360 s normal (IREKS), 250–280 s industrial target (BAKERpedia), 230–280 s normal but "200–250 s optimal" on the same page (Wikipedia de); Newberry found no baking penalty at 157–294 s from late-maturity amylase.
- Rye: 180–240 s normal (IREKS) vs 125–200 s good (Murniece) vs minimum 120 s (Wikipedia de).
- α-Amylase "optimum": 40–60 °C (BAKERpedia, dough) vs 72–74 °C (Murniece, scald) vs 71–72 °C (brewing). Short assays favour high values; long dough processes act at lower temperatures.
- Heat stability: BAKERpedia ranks fungal amylase least heat-stable, yet Rebholz (citing Cauvain) gives about 80 °C for fungal inactivation and IREKS 75 °C for cereal α-amylase. No source measured both.

## Gaps

Temperature optimum of cereal phytase; xylanase doses and over-dose limits; endogenous β-glucanase action in bread; lipoxygenase activity of plain wheat flour; a measured inactivation curve of cereal α-amylase in dough.
