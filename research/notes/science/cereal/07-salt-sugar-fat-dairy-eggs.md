# 7. Salt, sugar, fat, dairy and eggs

These ingredients change dough strength, fermentation speed, browning and keeping, and most of them bring water that counts toward hydration. Machine-readable values: `research/data/model-bread-science.json`, section `ingredients`; claims `science-bread-0048` to `0061` (malt syrup composition is `0029`, in note 5). Fermentation-rate effects of salt and sugar live in the fermentation model (`model-fermentation.json`, modifiers `salt` and `sugar`).

## Salt

- **Range.** 1.8–2.0 % of flour in bread, 0.75–1.25 % in cookies and cakes, where it only seasons ([BAKERpedia, Salt](https://bakerpedia.com/ingredients/salt/)).
- **Dough.** In 176 Italian soft-wheat flours, 1.5 % salt lowered farinograph water absorption by a few points in 95 % of samples, lengthened development time in 78 % and raised stability in 70 %, lowered softening in 72 %, and raised alveograph W by up to 86 % ([Carcea 2020](https://doi.org/10.3390/foods9070952), Figures 4–7). Part of the strengthening comes from the lower water uptake.
- **How much is needed.** Carcea concludes that 1.5 % (the Italian average) is enough for medium bread flours to tolerate longer mixing and fermentation, that strong flours work without salt and become too tenacious at 3.0 %, and that more than 1.5 % does not improve bread-making quality. Taste was not assessed.
- **Fermentation.** Rheofermentometer data of the same study drive the fermentation model's salt factor: about ×1.04 at 1.5 %, ×1.00 at 2 %, ×0.88 at 3 %.

## Sugar, honey and malt syrup

- Sugar feeds yeast, sweetens, holds water, browns (Maillard reaction, caramelisation) and raises the temperatures at which starch gelatinises and proteins set ([BAKERpedia, Sugar](https://bakerpedia.com/ingredients/sugar/)).
- Sweeteners raise starch gelatinisation temperature; wheat starch (onset 60.8 °C in water) is among the most affected, fructose raises it least, isomalt and isomaltulose most, and the shift grows with concentration ([Allan 2020](https://doi.org/10.3390/foods9060757), DSC at 0.5–4 M). The numerical shifts belong to the scald/DSC part of the corpus (`model-cereal.json`).
- High sugar slows yeast osmotically: handled by the fermentation model; osmotolerant yeast is the usual answer.
- Honey replaces sugar 1:1 by weight if the recipe's liquid falls by 15–18 % of the honey weight; bake about 14 °C cooler to limit browning; pH 3.5–4.5 ([BAKERpedia, Honey](https://bakerpedia.com/ingredients/honey/)).

## Fat

- Shortening: 3–5 % of flour in bread (sliceability, moistness, volume, oven spring), up to 12 % in yeasted doughnuts; it shortens gluten strands and slows retrogradation; 100 g butter ≈ 80 g shortening ([BAKERpedia, Shortening](https://bakerpedia.com/ingredients/shortening/)).
- Butter is about 80 % fat; the rest is water and milk solids ([BAKERpedia, Butter](https://bakerpedia.com/ingredients/butter/)). Ciqual measures 15.4 g water per 100 g in 82 %-fat unsalted butter.

## Milk and eggs

- Milk instead of water raises water absorption, raises dough pH (slower, more tolerant fermentation), browns faster through lactose, and gives more volume, a darker crust and longer shelf life. 100 g whole milk = 8.5 g non-fat dry milk + 3.5 g butter + 88 g water ([BAKERpedia, Milk](https://bakerpedia.com/ingredients/milk/)).
- Eggs: liquid whole egg 75 % moisture (dry matter 53.2 % protein, 46.0 % fat), whites 88 %, yolks 49 % (dry matter 62 % fat); they add colour, flavour, structure, emulsification (yolk lecithin) and tenderness ([BAKERpedia, Eggs spec sheet](https://bakerpedia.com/ingredients/eggs/)).

## Counting water

Water per 100 g ([Ciqual 2020](https://ciqual.anses.fr/), foods 19024, 19042, 22000, 22002, 22001, 16400, 31008, 19054, 31016):

| ingredient | water (g/100 g) |
|---|---:|
| whole milk, pasteurised | 89.1 |
| semi-skimmed milk | 89.4 |
| whole egg, raw | 76.3 |
| yolk, raw | 55.0 |
| white, raw | 87.6 |
| butter, 82 % fat | 15.4 |
| honey | 17.6 |
| skimmed milk powder | 1.95 |
| white sugar | 0.17 |

Hydration bookkeeping (extrapolation from these figures): dough water = water + 0.89 × milk + 0.76 × whole egg + 0.55 × yolk + 0.88 × white + 0.15 × butter + 0.18 × honey + 0.21 × malt syrup. An enriched dough still feels softer than an all-water dough at the same hydration, because fat and yolk tenderise and sugar competes for water.

## Disagreements

- Salt: 1.8–2.0 % recommended by BAKERpedia vs "more than 1.5 % unnecessary" for dough quality (Carcea). The first is a taste-and-practice range, the second a rheology result.
- Yolk moisture: 49 % (BAKERpedia) vs 55 % (Ciqual, raw yolk). The model uses Ciqual.

## Gaps

No primary source was reached on scalding milk for bread (the whey-protein effect on loaf volume), on egg dose ranges by bread type, or on the dose at which sugar starts to slow ordinary instant yeast in a lean dough (the fermentation model uses a trade chart).
