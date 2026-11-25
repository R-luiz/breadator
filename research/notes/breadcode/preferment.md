# preferment — The Bread Code / The Sourdough Framework

Claims: 24 (book 1, videos 21, repos 2). Evidence: creator-rule 24.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Claims

### preferment

- A preferment is a portion of the dough ingredients fermented before the final dough; examples are sourdough, poolish, biga, pâte fermentée and sponge. [breadcode-0868; SF Glossary > Preferment; creator-rule]
- Sourdough pizza preferment: 15 g starter, 80 g flour and 80 g water (ratio 1:5:5). — `{"starter_g": 15, "flour_g": 80, "water_g": 80, "ratio": "1:5:5"}` [breadcode-1179; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- Sourdough pizza starter refresh: add 80 g water and 80 g flour, then rise 3–5 h. — `{"water_g": 80, "flour_g": 80, "rise_h": [3, 5]}` [breadcode-1180; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- A poolish is 100% hydration (equal flour and water); 20% of the total flour is used as poolish. [breadcode-1822; yt:B_2SEmyyvso 2:49; creator-rule]
- Roughly half the flour is fermented in the starter and half is fresh flour added at mixing. — `{"fermented_flour_pct": 50, "fresh_flour_pct": 50}` [breadcode-2457; yt:cHwLBs9SRd4 2:13; creator-rule]
- Poolish, biga and sponge are the same kind of pre-ferment and differ mainly in hydration. [breadcode-2799; yt:oJs8FrlZQB0 description; creator-rule]
- Keeping a piece of dough from one bake to inoculate the next (pâte fermentée) avoids feeding a starter, and the creator says it also works for yeasted dough. [breadcode-2964; yt:DohPuCqG-YM 0:00; creator-rule]

### preferment.pate-fermentee

- Pâte fermentée (reserve dough): a piece of finished dough is kept and used to inoculate the next dough instead of a separate starter. [breadcode-2576; yt:rewSwR7-1qU 1:12; creator-rule]
- Salt in the reserve piece should match the main dough, so the microbes adapt to the same environment. [breadcode-2577; yt:rewSwR7-1qU 2:24; creator-rule]
- Hydration of the reserve piece should match the hydration of the next dough. [breadcode-2578; yt:rewSwR7-1qU 3:03; creator-rule]
- Next-day use: for 500 g flour, mix about 3 g old reserve dough with 30 g flour, 20 g water and a pinch of salt (about 53 g in total). — `{"total_flour_g": 500, "reserve_g": 3, "flour_g": 30, "water_g": 20, "starter_total_g": 53}` [breadcode-2579; yt:rewSwR7-1qU 3:38; creator-rule]
- One-week variant: 5 % inoculation. For 500 g flour, 1 g old reserve + 15 g flour + 10 g water (about 25 g) is refrigerated and mixed into the next dough. — `{"total_flour_g": 500, "inoculation_pct": 5, "reserve_g": 1, "flour_g": 15, "water_g": 10, "starter_total_g": 25}` [breadcode-2580; yt:rewSwR7-1qU 4:13; creator-rule]
- Refrigerating the reserve mixture slows its microbial activity. If it is kept at room temperature instead, use it within 1-3 days. — `{"use_within_d": [1, 3]}` [breadcode-2581; yt:rewSwR7-1qU 4:48; creator-rule]
- One-month variant: 2.5 % inoculation. For 500 g flour, 1 g old reserve + 8 g flour + 5 g water is kept in the fridge and mixed into the next dough. — `{"total_flour_g": 500, "inoculation_pct": 2.5, "reserve_g": 1, "flour_g": 8, "water_g": 5}` [breadcode-2582; yt:rewSwR7-1qU 5:25; creator-rule]
- Standard next-day reserve dose: about 10 % of flour (50 g reserve for 500 g flour). The one-week variant lowers it to 5 % and the one-month variant to 2.5 %. — `{"total_flour_g": 500, "next_day_inoculation_pct": 10, "next_day_reserve_g": 50, "week_inoculation_pct": 5, "month_inoculation_pct": 2.5}` [breadcode-2664; yt:rewSwR7-1qU 4:48; creator-rule]
- The reserve piece is cut from the finished dough after bulk fermentation, at the point the dough is ready to shape, and mixed directly into the next dough. [breadcode-2665; yt:rewSwR7-1qU 1:48; creator-rule]

### preferment.poolish

- A poolish is equal weights of flour and water with yeast (about 2 % dry or 6 % fresh of its flour), rested overnight, then mixed into the main dough at about 40 % of the main flour weight. — `{"hydration_pct": 100, "dry_yeast_pct": 2, "fresh_yeast_pct": 6, "preferment_pct_of_flour": 40}` [breadcode-2176; yt:EbO3blOpGks description; creator-rule]
- The poolish ferments overnight, roughly 10 hours in his timeline, and adds flavour without the acidity a sourdough preferment brings. — `{"preferment_h": 10}` [breadcode-2177; yt:EbO3blOpGks description chapters 1:30 and ; creator-rule]
- A poolish uses commercial yeast mixed with equal parts flour and water and gives a strong yeasty smell and taste rather than sourness. — `{"water_pct_of_flour": 100}` [breadcode-2793; yt:VlD2YuNq1w0 1:54; creator-rule]
- A poolish is equal parts flour and water with about 1 % dry yeast (of poolish weight), fermented about 24 h. — `{"water_pct_of_flour": 100, "yeast_pct_of_poolish": 1, "duration_h": 24}` [breadcode-2797; yt:oJs8FrlZQB0 description; creator-rule]
- Use poolish at about 20 % of the flour weight in the dough: 500 g flour takes 100 g poolish (50 g flour, 50 g water, 1 g dry yeast). — `{"poolish_pct_of_flour": 20}` [breadcode-2798; yt:oJs8FrlZQB0 description; creator-rule]
- A poolish was ready after about 24 h, when it had doubled and looked bubbly. At the next-morning check (roughly 12 h, inferred) it showed bubbles and had not risen much; 4 h later it had risen more but was still not ready. — `{"duration_h": 24}` [breadcode-2800; yt:oJs8FrlZQB0 0:38-1:12; creator-rule]
- A poolish can be kept in the fridge and refed to keep a yeast-based pre-ferment going, so yeast does not have to be bought each time. [breadcode-2801; yt:oJs8FrlZQB0 0:38; creator-rule]
- A poolish is made by mixing equal weights of flour and water (100 % hydration) with a small amount of commercial yeast, then left about a day before going into the final dough. — `{"water_pct_of_flour": 100, "yeast_pct_of_flour": 0.1, "ferment_h": 24}` [breadcode-2821; yt:VlD2YuNq1w0 3:09; creator-rule] *(low confidence)*
