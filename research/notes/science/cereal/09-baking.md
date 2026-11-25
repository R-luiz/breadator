# 9. Baking: core temperature, steam and bake loss

Machine-readable values: `model-bread-science.json`, section `baking`; claims `science-bread-0073` to `0093`.

## What happens as the dough heats

IREKS lists the events by dough temperature ([IREKS, Processes during baking](https://www.ireks-kompendium.com/en/the-baking-process/processes-in-the-dough-and-the-baked-goods-during-the-baking-process), table):

| dough temperature | event |
|---|---|
| 30–35 °C | yeast and lactic bacteria still make gas and acid |
| 35–45 °C | enzyme and yeast activity rise; oven spring starts |
| 50–60 °C | yeast and acid bacteria die |
| 50 °C → 65–76 °C | rye starch gelatinises |
| 60 °C | wheat starch starts to gelatinise; β-amylase inactivated |
| 60–80 °C | gluten proteins denature |
| 75 °C | α-amylase inactivated (all enzymes above 80 °C) |
| 78 °C | ethanol evaporates |
| 78–89 °C | wheat starch gelatinisation ends |
| 98 °C | water evaporates; crust starts |
| 100–120 °C | light dextrins, Maillard melanoidins |
| 130–140 °C | sucrose caramelises; dark dextrins |
| 150–200 °C | roasting to carbonisation |

The crumb centre cannot pass 100 °C while it holds water. In a 1.7 kg mixed rye bread it reaches almost 100 °C after about 40 min and stays there, while 1 mm under the surface climbs past 180 °C; the crumb can be cut cleanly after 20 min but needs about 50 min more for an elastic, edible crumb (same page, after Klingler 1995).

## Core temperature

Measured:

- Pan white bread, 230 °C, 35 min: crumb maximum 97 °C ([Reichenberger 2019](https://doi.org/10.1007/s12161-019-01673-7)).
- Part-baked sourdough breads re-baked at 230 °C: 79 °C after 9.5 min, 96 °C after 17 min ([Temkov 2024](https://doi.org/10.3389/fnut.2024.1370086), section 3.1).
- Small toast loaves (150 × 45 × 45 mm tins): minutes to reach a 90 / 95 / 98 °C core were 24.0 / 26.8 / 28.8 at 200 °C, 21.2 / 22.5 / 26.8 at 220 °C and 18.8 / 21.3 / 28.3 at 240 °C. Only loaves baked to 98 °C held their shape through vacuum cooling ([Kinner 2021](https://doi.org/10.1007/s13197-020-04945-x)).

Practitioner tests ([King Arthur, Using a thermometer with yeast bread](https://www.kingarthurbaking.com/blog/2017/04/07/using-a-thermometer-with-yeast-bread)): pan loaves, a dense whole-grain boule and soft rolls were fully baked at 190 °F (88 °C); loaves taken to 205–210 °F (96–99 °C) were drier and seemed less fresh after two days. Baguettes reach 212 °F (100 °C) before their crust is right, so they are judged by colour. King Arthur advises 205–210 °F (96–99 °C) for rye, 205 °F (96 °C) for gluten-free bread, and about 5 °F (3 °C) less at high altitude.

Endpoints for the app (extrapolation): wheat pan loaves and rolls 88–96 °C (88 °C softest, 96–98 °C firmer and shape-stable); hearth loaves 96–99 °C; rye and gluten-free 96–99 °C followed by a long rest; thin crusty breads by crust colour.

## Times and temperatures (IREKS guide values)

From [IREKS, Guiding values for baking times and initial baking temperatures](https://www.ireks-kompendium.com/en/the-baking-process/guiding-values-for-baking-times-initial-baking-temperatures-and-capacities), commercial deck (and rack) ovens with falling heat:

| product | time | initial temperature, deck (rack) |
|---|---|---|
| rolls | 18–22 min | 220–240 °C (240–260 °C) |
| wheat and toast bread | 28–35 min | 210–230 °C (220–240 °C) |
| mixed-grain bread | 45–60 min | 240–250 °C (260–270 °C) |
| rye / mixed rye, free-standing 1000 g | 50–60 min | 250–270 °C (270–280 °C) |
| same, 2000 g / 3000 g | 60–70 / 80–90 min | |
| rye / mixed rye, touching, 1500 g | 75 min | 230–240 °C (mixed rye) |
| pumpernickel | 16–24 h | 100–180 °C |

Cold-retarded pieces should reach a core of 15–24 °C before baking, depending on size and shape; too cold gives hollow spaces in the crumb and uneven crust cracks, too warm an over-aerated crumb with little shine ([IREKS, Core temperatures of various baked goods](https://www.ireks-kompendium.com/en/cold-dough-methods/core-temperatures-of-various-baked-goods)).

## Steam

Steam condenses on the cold dough, heats the surface fast and keeps it extensible, so the loaf expands evenly; without it the crust tears irregularly and stays matt. Rye and mixed rye breads need the steam let out after a short time or the crust cracks. More steam for under-proofed pieces and hot ovens, less for over-proofed pieces and cool ovens (IREKS scale 1 to 5, after Schünemann 2016; same IREKS page as above).

## Bake loss

Bake loss % = 100 × (dough − loaf) / dough; say whether the loaf was weighed hot or cooled.

IREKS mixed rye bread, baking plus cooling ([IREKS, Baking loss](https://www.ireks-kompendium.com/en/the-baking-process/135-scaling-weight/baking-loss/baked-goods-yield)):

| baked loaf | dough | loss |
|---:|---:|---:|
| 500 g | 600 g | 16.7 % |
| 750 g | 875 g | 14.3 % |
| 1000 g | 1150 g | 13.0 % |
| 1250 g | 1425 g | 12.3 % |
| 1500 g | 1700 g | 11.8 % |
| 1750 g | 1975 g | 11.4 % |
| 2000 g | 2250 g | 11.1 % |
| 49 g roll | 60 g | 18.0 % |

The loaf rows follow exactly loss = 50 g + 0.1 × loaf weight, i.e. dough = 1.1 × loaf + 50 g, for 500–2000 g; the roll does not fit the line. IREKS bread yields per 100 flour: wheat 136–141, rolls 130–133, mixed rye 80/20 159–162, rye 160–164, coarse rye 166–169.

Drivers (IREKS; [Hamelman 2004](https://app.ckbk.com/section/brea32715c01s012ss001/moisture-loss-in-baked-bread)): free-standing loaves lose more than tin or touching loaves (more crust); small pieces more than large; long thin shapes more than round; longer bakes more; a cooler oven, which needs a longer bake, more. Hamelman: losses can reach 20 % or more.

By shape and vessel, dough to cooled loaf ([Busby, How much weight does bread lose](https://www.busbysbakery.com/bread-weight-lost-when-baked/)): white tin 950 → 820 g (14 %), small white tin 13 %, whole-wheat tin 15 %, small whole-wheat tin 16 %, baguette 550 → 420 g (24 %), artisan loaf 24 %, sourdough 700 → 530 g (24 %); whole-grain doughs lose 2–3 % more. Measured small toast tins lost 4–12 % in the oven, near a literature standard of 10 % (Kinner 2021).

No measured figure for lidded pots (Dutch ovens) was found. Default for the app (extrapolation, low confidence): open hearth loaves of 0.5–1 kg 20–24 %, tin loaves 13–16 %, a covered first phase somewhere below the open bake.

## Disagreements

- **Core temperature for wheat loaves**: 88 °C is enough (King Arthur) vs 98 °C needed for shape stability under vacuum cooling (Kinner); full bakes reach 97–100 °C anyway (Reichenberger, IREKS).
- **How much is lost in the oven**: Busby says 2–4 % in the oven, 2–4 % on cooling and 10–25 % in total from dough (proofing and handling included); Kinner measured 4–12 % in the oven alone on small tins, and IREKS gives 11–18 % for baking plus cooling.

## Gaps

Bake loss of covered pots; core-temperature endpoints from a controlled study for enriched or rye breads; oven-side steam quantities for home ovens.
