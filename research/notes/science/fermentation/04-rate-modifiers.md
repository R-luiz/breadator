# 4. Formula modifiers

Each modifier multiplies the fermentation rate, so time is divided by it. The reference dough is lean: 2 % salt, 65–75 % hydration, white flour. Factors multiply together. A range is meant as an 80 % interval and widens the prediction band (note 8). Where the evidence is thin, the engine still gives a number, labels it, and offers a one-bake calibration instead.

| ingredient | applies to | multiplier | range | evidence | confidence |
|---|---|---|---|---|---|
| salt 0 / 1 / 1.5 / 2 / 2.5 / 3 % | yeast | 1.09 / 1.07 / 1.04 / 1.00 / 0.95 / 0.88 | ±5 % | measured | medium |
| salt 0 / 1 / 2 / 3 % | sourdough | 1.18 / 1.09 / 1.00 / 0.91 | 1.06–1.21 at 0 % | creator-rule | medium |
| sugar 0 / 5 / 10 / 15 / 20 / 30 % | regular yeast | 1.00 / 1.00 / 0.80 / 0.70 / 0.60 / 0.40 | 0.7–1.0 at 10 %, 0.5–0.8 at 20 % | literature | medium |
| sugar 0 / 10 / 20 / 30 % | osmotolerant yeast | 0.57 / 0.78 / 1.00 / 1.25 | ±15 % | literature | medium |
| sugar 10 / 20 % | sourdough | 0.8 / 0.5 | 0.6–1.0 / 0.3–0.8 | extrapolation | low |
| fat 0–10 / 10–30 / above 30 % | both | 1.00 / 0.90 / 0.80 | 0.95–1.05 / 0.75–1.0 / 0.6–1.0 | extrapolation | low |
| milk, eggs | both | 1.00 | 0.9–1.05 | extrapolation | low |
| cinnamon mixed in, 1 % / 3 % of flour | both | 0.80 / 0.60 | 0.5–1.0 / 0.3–0.9 | extrapolation | low |
| cocoa, 5–10 % of flour | both | 0.90 | 0.75–1.0 | extrapolation | low |
| whole grain share w (0–1) | sourdough | 1 + 0.3 w | 1 to 1 + 0.6 w | measured (one test) | low |
| whole grain share w | yeast | 1 + 0.1 w | 1 to 1 + 0.25 w | extrapolation | low |
| scalded flour, 5–20 % of the flour | both | 1.00 | 0.95–1.10 | extrapolation | low |
| hydration H | sourdough dough and starter refresh | 1 + 0.005 (H − 75), kept within 0.85–1.15 | slope 0–0.015 per % | extrapolation | low |
| hydration H | yeast | 1 + 0.004 (H − 65), kept within 0.9–1.1 | slope 0–0.01 per % | literature (direction) | low |

## Salt

- **Yeast.** Total gas over 3 h fell from 1518 mL without salt to 1487 mL at 1.5 % and 1272 mL at 3.0 % for one flour, and from 1584 to 1519 and 1242 mL for another ([Carcea et al. 2020](https://pmc.ncbi.nlm.nih.gov/articles/PMC7404662/), rheofermentometer results, Figure 9). That is 0.96–0.98 of the salt-free gas at 1.5 % and 0.78–0.84 at 3 %. The engine's curve is (1 − 0.021 s²) rescaled to 1 at 2 % salt.
- **Sourdough.** Wraith's table has salted doughs 15 % slower to double and 21 % slower in bulk than unsalted ones ([table data](https://www.the-bread-code.io/fermentation-table/)). In broth the lactobacilli stopped at 4 % NaCl while the yeast tolerated 8 % ([Gänzle et al. 1998](https://pmc.ncbi.nlm.nih.gov/articles/PMC106434/), abstract). Those are percentages of broth. In a 75 % hydration dough, 2 % salt on flour is about 2.7 % of the water.

## Sugar and osmotolerant yeast

A manufacturer's figure plots relative yeast activity against sugar from 0 to 30 % of flour ([Lallemand Baking Update 1/3](http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/1_3YEAST.PDF), "Effect of sugar on yeast activity"; values read from the figure, ±5):

- lean-dough yeast: about 100 % at 0, 80 % at 10 %, 60 % at 20 %, 40 % at 30 %;
- general-purpose yeast: 75 % at 0, 100 % at 10 %, 60 % at 30 %;
- sugar-tolerant yeast: 57 % at 0, 78 % at 10 %, 100 % at 20 %, 125 % at 30 %.

The lean and sugar-tolerant lines cross near 10 % sugar, the general-purpose line near 17 %. The same maker advises more yeast above 10 % sugar ([Baking Update 2/9](http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/2_9INST.PDF)) and states that sugar raises gas production up to about 5 % and lowers it beyond ([Baking Update 1/4](http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/1_4DOSE.PDF)). The engine's threshold for recommending osmotolerant yeast is 10 % sugar (range 8–17 %).

The osmotic load is real: a dough with 18 % sugar and 45 % water had a water activity of 0.95 against 0.97 at 6 % sugar and 52 % water ([Aslankoohi et al. 2015](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0119364), Results). Sugar and water changed together there, and the strain mattered, so this is not a multiplier by itself.

For sourdough no dose-response was found. The values in the table copy the shape of the lean-yeast line; they are a low-confidence estimate, to be replaced by a recipe-specific vigour once a bake is logged.

## Fat, milk, eggs

No measurement of the effect of fat, milk or egg on gas rate was read. The manufacturer's list of factors names proof temperature, water absorption, sugar, salt, calcium propionate and fibre, not fat (Baking Update 1/4). Rich doughs are slow mostly through their sugar, their lower water and their cold ingredients. The heat balance of note 6 deals with the last; the hydration term counts milk as about 88 % water and egg as about 75 % water (general composition figures, not sourced here).

## Spices and cocoa

Cinnamon contains antifungal compounds, and bakers report slow doughs when it is mixed in. No dose-response measurement in dough could be read for this corpus. The table values are reasoned estimates with wide ranges. The safest advice is in the handling: keep the spice in a filling or swirl, where it does not touch most of the yeast.

## Whole grain and bran

One pressure-vessel test fed a starter 1:1:1 with bread flour, whole wheat or rye at 20 °C. Whole wheat made almost twice the CO2 early, peaked later (14 h against 12 h) and finished with less ([The Sourdough Journey](https://thesourdoughjourney.com/white-wheat-or-rye-what-is-the-best-flour-to-feed-your-starter/)). Bran also reduces gas retention (Baking Update 1/4). The engine speeds the rate modestly and advises a lower rise target for whole-grain doughs, rather than a longer wait.

## Scalded flour

A tangzhong or yudane changes the dough mostly through temperature and water binding. It enters the heat balance as an ingredient at its own temperature. No measured effect on gas rate was found; the multiplier stays at 1.

## Hydration

Higher dough yield speeds acidification (Gänzle et al. 1998, Discussion), and higher absorption raises gas production while lowering retention (Baking Update 1/4). Neither gives a number. Craig's stiff, salty pizza dough runs at half the reference vigour (note 3), which mixes hydration, salt, culture and endpoint. The slopes in the table are small on purpose.

## Acidity

- Sourdough: no multiplier inside the rise window. The bacteria stop near pH 3.8 and the yeast is held back by undissociated acetic acid near 0.15 mol/L (Gänzle et al. 1998, Tables 3–4). The engine stops predicting a rise once ι · e^G passes about 1.5.
- Yeast in an acidic dough (hybrid with levain): 0.9 between pH 3.8 and 4.5 (range 0.7–1.0), an extrapolation from the sourdough yeast.

## Hybrid doughs

Run the yeast rise and the sourdough rise side by side and add the rises. Never add the times.
