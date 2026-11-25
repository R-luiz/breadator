# 1. Commercial yeast: dose, temperature and time

The engine predicts the time for a yeasted dough to double from the yeast dose and the dough temperature. The numbers come from one field-tuned chart and are checked against independent tables.

## The law

τ_doubled = A · d^(−a) · exp(−c (T − 25)) · ((Tmax − 25) / (Tmax − T))^b

- d is the dose in instant-dry-yeast equivalent, as a percentage of the flour that ferments.
- T is the dough temperature in °C.
- A = 0.91 h, a = 0.72, c = 0.190 per °C, b = 1.32, Tmax = 45 °C.

The exponential term carries the low and middle temperatures. The last factor is the cardinal correction: it flattens the curve toward an optimum at Tmax − b/c ≈ 38 °C and brings the rate to zero at Tmax. Examples, in hours to double:

| instant yeast | 4 °C | 10 °C | 18 °C | 21 °C | 25 °C | 30 °C |
|---|---:|---:|---:|---:|---:|---:|
| 0.05 % | 165 | 65 | 20 | 13 | 7.9 | 4.4 |
| 0.1 % | 100 | 39 | 12 | 8.0 | 4.8 | 2.7 |
| 0.3 % | 45 | 18 | 5.5 | 3.6 | 2.2 | 1.2 |
| 1.0 % | 19 | 7.5 | 2.3 | 1.5 | 0.9 | 0.5 |

Below about 45 minutes the law is outside its data; the engine adds a 0.2 h lag (range 0.1–0.4 h, an assumption) and flags the result.

## Where the numbers come from

**The chart.** TXCraig1's final chart on pizzamaking.com lists hours for 17 doses (0.003–0.32 % instant yeast) at every degree from 35 to 95 °F ([topic 26831, reply #260, 22 Aug 2015](https://www.pizzamaking.com/forum/index.php?topic=26831.0); read from the [Wayback copy of the chart image](http://web.archive.org/web/20151010043650/http://www.pizzamaking.com/forum/index.php?action=dlattach;topic=26831.0;attach=221784;image)). The hours are total fermentation, bulk plus balls, to "ready to bake" (reply #5 and #11). It is a model, not an experiment: Craig started from a 25 °C dose curve, applied a growth curve of the Gänzle form with Tmax = 45 °C, and adjusted it to forum feedback, last "more in line with cold fermentation observations" (replies #4 and #188). All 918 cells are transcribed in `fits/data/yeast_craig_chart.csv`.

**The cf-calculator formula.** The source of [cf-calculator](https://raw.githubusercontent.com/derwille1/cf-calculator/main/index.html) (line 119) does contain hours = 30.91 × IDY%^−0.72 × exp(−0.110 × (°F − 35)). It is a regression of the same chart near fridge temperature. Below 8 °C it matches the chart within 5 %. At 18 °C it is too short by a factor of 2, at 30 °C by a factor of 4. It must not be used above about 8 °C.

**The fit** (`fits/fit_yeast.py`, 753 cells of 4 h or more; shorter cells are distorted by rounding and by the chart's 2 h floor):

| temperature form | fitted parameters | rms error (ln h) | below 8 °C | 8–28 °C | 28–35 °C |
|---|---:|---:|---:|---:|---:|
| cf-calculator formula | 0 | 0.885 | 0.049 | 0.729 | 1.683 |
| one exponential (Q10 3.9) | 3 | 0.103 | 0.129 | 0.065 | 0.184 |
| CTMI | 5 | 0.265 | 0.262 | 0.229 | 0.397 |
| exponential × cardinal factor (engine) | 4 | 0.052 | 0.044 | 0.052 | 0.059 |

Only the last form stays within the chart's rounding everywhere. A CTMI cannot follow the chart's near-exponential cold tail, and its optimum runs into its maximum because the chart never turns down below 35 °C.

## Checks against independent sources

Residuals are in `fits/yeast-validation-residuals.csv`.

- **Doubling statements.** Five rows with an explicit "doubled" endpoint (0.05–0.33 % instant yeast, 21–24 °C) sit at a median 1.04 times the engine. One deliberately slow 24 h recipe is the outlier. These rows come from a [third-party compilation](https://github.com/RomaricErnst/baker-hub/blob/main/docs/yeast-model/DATASET.csv) and were not re-read at their primary pages.
- **Dose exponent.** A French training table for poolish gives 20–25 g of fresh yeast per kg of flour for 2 h down to 1 g for 18 h at 22–24 °C ([Technomitron, poolish table](https://technomitron.aainb.com/la-fermentation/les-methodes-de-fermentation-sur-levure/)). Its exponent is 0.70. A second table (3 % compressed yeast for 1–2 h down to 0.25 % for 12–15 h at 26–29 °C, [BAKERpedia, Poolish](https://bakerpedia.com/processes/poolish/)) gives 0.90.
- **Ripe poolish.** A ripe poolish takes about 1.9 times as long as doubling a dough with the same dose (quartiles 1.4–2.2, 48 rows; 1.9 for the French table, 1.9 for the [Weekend Bakery schedule](https://www.weekendbakery.com/posts/more-artisan-bread-baking-tips-poolish-biga/), 3.2 for BAKERpedia).
- **Recipe schedules** (bulk plus proof as printed) run 1.2–1.6 times the doubling time; calculator tables with no stated endpoint run 2.8 times.

## Temperature sensitivity: two readings

The chart implies a time Q10 of 3.9 at 20 °C, 3.4 at 25 °C and 2.7 at 30 °C. Sources that list several temperatures themselves imply about 2.1 between 18 and 30 °C (fixed-effects regression over seven groups, `yeast-fit.json`, `within_source_temperature_slope`). A French rule of thumb equates 1 °C on the dough with 10 % of the yeast weight ([Technomitron](https://technomitron.aainb.com/petrissage/methodes-de-petrissage/), "Utilité du chiffre de base"), which is a Q10 of 2.0–2.6. The engine keeps the chart's curve, because it is the only complete one, and carries the gap as an uncertainty of 3 % per °C away from the temperature the baker calibrated at.

## Biology behind the curve

- Ten *Saccharomyces cerevisiae* strains grew between 0.4–5.0 °C and 42.0–46.1 °C, with optima of 30.0–34.8 °C and optimal rates of 0.28–0.45 per hour ([Salvadó et al. 2011](https://pmc.ncbi.nlm.nih.gov/articles/PMC3067424/), Table 2). None was a baker's strain, and growth in broth is not gas production in dough.
- Gas production continues below the growth minimum. A commercial baker's yeast produced more than 40 mL of CO2 per gram of pressed yeast after 6–7 days at 3–5 °C in a maltose medium ([US patent 5,827,724](https://patents.google.com/patent/US5827724A/en), description of Fig. 7). The fridge slows fermentation; it does not stop it.
- At a bakery dose the gas production stays roughly level for three hours (58.5, 65.5 and 57.5 mL/g measured after 60, 120 and 180 min with 3 % yeast at 30 °C; [Codină et al. 2013](https://www.agriculturejournals.cz/publicFiles/216_2012-CJFS.pdf), Table 2). This is why the engine treats the rise of a yeasted dough as close to linear (note 8).

## Limits

- The chart is one person's model. Its 25 °C baseline was a guess that he then checked against a few known points (reply #4).
- Its cold rows were tuned to schedules that include the cool-down from mixing temperature. The engine integrates the cool-down itself, so part of the warm start may be counted twice. Published cold schedules end at a median 75 % of the engine's progress to doubling (quartiles 54–133 %, 41 schedules), a scatter of a factor 2.2.
- Doses above 1 % instant yeast and temperatures above 35 °C are extrapolation.
