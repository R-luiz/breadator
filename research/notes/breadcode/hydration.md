# hydration — The Bread Code / The Sourdough Framework

Claims: 164 (book 43, videos 110, repos 11). Evidence: creator-rule 139, measured 11, anecdote 10, extrapolation 3, literature 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

- Water need depends on flour: whole grain absorbs more (outer layers), higher protein holds more. Beginners: ≈60 % (book §12.2.3, Loafy default); the author's own default is ≈70 % for his flour. Test: five 100 g flour portions with 55–75 g water, rest 15 min, windowpane.
- High hydration = more extensible → bigger oven spring and open crumb, but stickier, longer kneading, more folds, faster gluten breakdown (more bacterial activity); slow fermentation gives similar extensibility with less risk. Use bassinage (start ≈60 %, add water after gluten forms; a low-gluten flour reached ≈80 %).
- Seeds: +1–2 % water. Loaf pan bread tolerates 90–100 %. Rye: 80 % for pan, ≈60 % for free-standing.
- Website/old repo protein tiers (<10 % → 55, 10–12 → 60, 13–14 → 65, >15 → 70 %) are in `tools.md`; they are not in the book.

## Claims

### hydration

- Baker's math expresses every ingredient as a percentage of flour weight; flour is the 100 % baseline. [breadcode-0089; SF Ch 3 Making a sourdough starter; creator-rule]
- Example pizza-type dough in baker's %: flour 1000 g (100 %), water 600 g (60 %), starter 100 g (10 %), salt 20 g (2 %). — `{"flour_g": 1000, "water_g": 600, "starter_g": 100, "salt_g": 20, "water_pct": 60, "starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-0090; SF Ch 3 Making a sourdough starter; creator-rule]
- To rescale a baker's-% formula, multiply the flour weight by each percentage: with 1400 g flour, water is 840 g (60 %), starter 140 g (10 %), salt 28 g (2 %). — `{"flour_g": 1400, "water_g": 840, "starter_g": 140, "salt_g": 28, "water_pct": 60, "starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-0091; SF Ch 3 Making a sourdough starter; creator-rule]
- Any flour quantity (e.g. 50 kg) can be scaled by recalculating grams from the baker's percentages, instead of scaling each ingredient ad hoc. [breadcode-0092; SF Ch 3 Making a sourdough starter; creator-rule]
- A liquid starter must be shaken to homogenise all its ingredients before it is measured. [breadcode-0205; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- Bran absorbs more water, so whole-wheat or rye flour often needs more water to make a stiff starter. [breadcode-0209; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- A very wet flatbread dough gives a fluffier result because much water evaporates in the pan; very high water gives a pancake-like texture. [breadcode-0278; SF Ch 6 Bread types; creator-rule]
- Loaf pan bread can use 90–100 % hydration; a very wet dough gives an airier, chewier crumb. — `{"hydration_pct": [90, 100]}` [breadcode-0311; SF Ch 6 Bread types; creator-rule]
- A wheat flatbread at about 60 % hydration is drier and harder to mix; wheat gluten lets it puff in the pan. — `{"hydration_pct": 60}` [breadcode-0321; SF Ch 6 Bread types; creator-rule]
- Rye loaf formula in baker's %: whole rye flour 100 %, water 80 %, starter 20 %, salt 2 % (yields 2 loaves from 1000 g flour). — `{"water_pct": 80, "starter_pct_of_flour": 20, "salt_pct": 2, "flour_g": 1000}` [breadcode-0326; SF Ch 8 Non wheat sourdough; creator-rule]
- A rye dough at 80 % hydration is too wet for free-standing shaping; lower it to about 60 % for free-standing loaves. — `{"water_pct": [60, 80]}` [breadcode-0328; SF Ch 8 Non wheat sourdough; creator-rule]
- Water for bread flour in this chapter's example ranges from 60 % to 90 % of flour weight. — `{"hydration_pct": [60, 90]}` [breadcode-0387; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Water need depends on the flour and is not fixed. [breadcode-0389; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Whole-wheat flour, whose bran layers absorb water initially, needs slightly more water. [breadcode-0390; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Highly hydrated dough is more extensible (easier to stretch) and is used for fluffier bread. [breadcode-0392; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- High-hydration dough is stickier and harder to handle. [breadcode-0395; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- High-hydration crumb can feel sticky; this is a matter of taste. [breadcode-0403; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- By bassinage, a low-gluten flour was pushed to about 80 % hydration in the author's experience. — `{"hydration_pct": 80}` [breadcode-0405; SF Ch 7 Wheat sourdough §7.4 Hydration; anecdote]
- Keep adding water until the dough has the right consistency; the feel for it develops with experience. [breadcode-0406; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- The author's default hydration is about 70 % for his flour; the sweet spot is flour-specific and should be tested. — `{"hydration_pct": [70, 70]}` [breadcode-0417; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Test each new flour's hydration capacity with a series of 100 g flour batches at increasing water levels. [breadcode-0418; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Hydration test: five 100 g flour portions with 55, 60, 65, 70 and 75 g water. — `{"flour_g": 100, "water_g": [55, 75]}` [breadcode-0419; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- After mixing until no dry chunks remain, rest the dough 15 min, then stretch it thin for a windowpane test. — `{"rest_min": 15}` [breadcode-0420; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Choose a hydration below the first mixture whose dough tears before a windowpane forms. [breadcode-0421; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Water is the cheapest dough ingredient; higher hydration lowers cost per dough weight but increases labour and failure risk. [breadcode-0422; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- An over-hydrated dough at the start makes gluten chains harder to align; start with lower hydration and add water later (bassinage) if needed. [breadcode-0474; SF Ch 7 Wheat sourdough §7.8 Dough strength > Bassinage; creator-rule]
- Bassinage: gluten forms at lower hydration, then extensibility is increased by adding water and kneading again; a useful trick for lower-gluten flour. [breadcode-0475; SF Ch 7 Wheat sourdough §7.8 Dough strength > Bassinage; literature]
- Liquid mix-ins integrate homogeneously into the dough and may replace some or all of the water. [breadcode-0712; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Seeds such as flax, sunflower and sesame dehydrate the dough slightly during fermentation; adding about 1–2 % more water is advised. — `{"added_water_pct": [1, 2]}` [breadcode-0718; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Some mix-ins absorb a lot of water, so water must be increased to keep the same consistency. [breadcode-0719; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Other liquids can replace some or all of the water, changing taste and texture. [breadcode-0730; SF Ch 9 Mix-ins §9.2 Examples > Liquids; creator-rule]
- Tomato juice can replace the water in a dough with 20 % rye flour. — `{"rye_flour_pct": 20}` [breadcode-0733; SF Ch 9 Mix-ins §9.2 Examples > Combinations; creator-rule]
- Whole soaked berries release water into the dough, so a little more flour is needed. [breadcode-0739; SF Ch 9 Mix-ins §9.3 Techniques; creator-rule]
- Pumpkin purée can substitute part of the dough's water; add any extra water gradually because the purée releases its own liquid over time. [breadcode-0749; SF Ch 9 Mix-ins §9.1 Categories (Pumpkin purée figure); creator-rule]
- Mashed pumpkin adds flavour and hydration to soft pull-apart sourdough buns. [breadcode-0750; SF Ch 9 Mix-ins (chapter intro, pumpkin figure); creator-rule]
- Baker's math expresses every ingredient as a percentage of the total flour weight, which is 100 %, so recipes scale easily. — `{"flour_pct": 100}` [breadcode-0807; SF Glossary > Baker's math; creator-rule]
- Example: for 500 g of flour, 60 % water is 300 g, 10 % starter is 50 g and 2 % salt is 10 g. — `{"flour_g": 500, "water_pct": 60, "starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-0808; SF Glossary > Baker's math; creator-rule]
- Dough hydration is the water weight as a percentage of flour weight; higher hydration gives a wetter, stickier dough and lower hydration a firmer one. [breadcode-0830; SF Glossary > Dough Hydration; creator-rule]
- Example: 375 g of water with 500 g of flour gives 75 % hydration. — `{"flour_g": 500, "water_g": 375, "hydration_pct": 75}` [breadcode-0831; SF Glossary > Dough Hydration; creator-rule]
- For beginners, use about 60 % hydration: 60 g of water per 100 g of flour, a ballpark figure that works for most flours. — `{"hydration_pct": 60}` [breadcode-0960; SF Ch 12 Troubleshooting §12.2.3 What's a good level of water (hydration); creator-rule]
- Lower hydration makes dough easier to handle and gives more yeast fermentation, which lowers the risk of over-fermentation. [breadcode-0962; SF Ch 12 Troubleshooting §12.2.3 What's a good level of water (hydration); creator-rule]
- Using less water gives a more elastic dough right away; this underlies no-knead sourdough. [breadcode-1061; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Not enoug; creator-rule]
- About 70 % hydration is a good target: fluffier crumb with still-manageable dough; bake with at least 70 % for bubbles. — `{"hydration_pct": 70}` [breadcode-1128; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- In a side-by-side test of 60–100 % hydration, the creator judged 70 % best and 80 % second. — `{"hydration_range_pct": [60, 100], "best_hydration_pct": 70, "second_hydration_pct": 80, "dry_yeast_pct": 0.25, "salt_pct": 2, "flour_g": 150}` [breadcode-1135; repo:the-bread-code experiments/different-hydration-levels.md; measured]
- Basic bread: 500 g flour, 325 g water (65 %), 10 g salt (2 %), 1 g dry yeast (0.2 %) or 3 g fresh yeast (0.6 %). — `{"flour_g": 500, "water_g": 325, "hydration_pct": 65, "salt_pct": 2, "dry_yeast_pct": 0.2, "fresh_yeast_pct": 0.6}` [breadcode-1150; repo:the-bread-code recipes/savory/basic-bread.md; creator-rule]
- Basic sourdough water is set by flour protein: about 275 g at <10 % protein, 300 g at 10–12 %, 325 g at 13–14 %, 350 g at >15 % (per 500 g flour). — `{"water_g_protein_lt10": 275, "water_g_protein_10_12": 300, "water_g_protein_13_14": 325, "water_g_protein_gt15": 350, "flour_g": 500}` [breadcode-1159; repo:the-bread-code recipes/sourdough/basic-sour-dough.md; creator-rule]
- Mischbrot: 250 g rye and 250 g wheat, 100 g starter (20 % of flour), 325 g water (65 %), 10 g salt (2 %). — `{"rye_flour_g": 250, "wheat_flour_g": 250, "starter_g": 100, "starter_pct": 20, "water_pct": 65, "salt_pct": 2}` [breadcode-1164; repo:the-bread-code recipes/sourdough/mischbrot-aka-graubrot.md; creator-rule]
- Sourdough pasta total hydration is 42 %: 105 g added water over 250 g flour, counting the starter's flour and water. — `{"hydration_pct": 42}` [breadcode-1178; repo:the-bread-code recipes/sourdough/sourdough-pasta.md; extrapolation]
- Sourdough pizza main dough: 290 g flour, 130 g water, 10 g salt and 12 g olive oil (optional, for an electric oven). — `{"flour_g": 290, "water_g": 130, "salt_g": 10, "oil_g": 12}` [breadcode-1181; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- Computed from the sourdough pizza ingredient list: total water 290 g on 450 g flour = about 64 %, salt about 2.2 %, oil about 2.7 % of flour. — `{"hydration_pct": 64, "salt_pct": 2.2, "oil_pct": 2.7}` [breadcode-1186; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; extrapolation]
- Neapolitan-style pizza dough: 60 % water, 2 % salt, 0.05 % dry or 0.15 % fresh yeast; optionally 5 % stiff starter. — `{"water_pct": 60, "salt_pct": 2, "dry_yeast_pct": 0.05, "fresh_yeast_pct": 0.15, "starter_pct": 5}` [breadcode-1204; repo:pizza-dough README.md; creator-rule]
- Pizza dough example: 200 g flour with 120 g water, 4 g salt, 0.1 g dry or 0.3 g fresh yeast. — `{"flour_g": 200, "water_g": 120, "salt_g": 4, "dry_yeast_g": 0.1, "fresh_yeast_g": 0.3}` [breadcode-1205; repo:pizza-dough README.md; creator-rule]
- Pizza dough example: 250 g dough from 154 g flour, 93 g water, 3 g salt, and 0.08 g dry yeast, 0.23 g fresh yeast, or 8 g starter. — `{"flour_g": 154, "water_g": 93, "salt_g": 3, "dry_yeast_g": 0.08, "fresh_yeast_g": 0.23, "starter_g": 8}` [breadcode-1206; repo:pizza-dough README.md; creator-rule]
- Less water makes the gluten network tighter; for a stiffer dough, knead less, not more. [breadcode-1219; yt:MrOM_D3hnpc 59:35; creator-rule]
- Low-hydration dough builds strength with less work, so fewer stretch-and-folds are needed. [breadcode-1223; yt:MrOM_D3hnpc 1:05:39; creator-rule]
- Less handling of a wetter dough gives more extensibility and bubbles, which suits open-crumb breads, but the dough is harder to shape into a freestanding loaf. [breadcode-1224; yt:MrOM_D3hnpc 1:08:00; anecdote] *(low confidence)*
- A single comparison of 60 % and 80 % hydration breads looked and tasted very similar, so the extra effort was judged marginal. [breadcode-1240; yt:MrOM_D3hnpc 1:59:43; measured] *(low confidence)*
- Low-hydration dough (about 60–65 %) is easier to handle than 80 % and rises similarly. — `{"hydration_pct": [60, 65]}` [breadcode-1242; yt:MrOM_D3hnpc 2:03:31; creator-rule]
- Hydration choice depends on the flour; test a range (75–90 %) with your own flour rather than assuming one value. [breadcode-1247; yt:ImVvQMvGZKE 5:57; creator-rule]
- Test a flour's water absorption: mix it with several water levels, rest about an hour, then check for a gluten network to find the right hydration. — `{"rest_h": 1}` [breadcode-1293; yt:KRO9EcIyXSI 6:48; creator-rule]
- A strong bread flour blended with whole wheat can take around 80 % hydration. — `{"hydration_pct": 80}` [breadcode-1294; yt:KRO9EcIyXSI 6:12; creator-rule]
- Whole-wheat flour absorbs more water than white flour, so whole-grain doughs can be run wetter. [breadcode-1295; yt:KRO9EcIyXSI 6:12; creator-rule]
- More water makes dough more extensible and easier for yeast to inflate; a very stiff dough resists gas expansion. [breadcode-1296; yt:KRO9EcIyXSI 8:37; creator-rule]
- Pizza dough at 60 % hydration with 2 % salt and 5 to 20 % starter gives a workable, stiff dough. — `{"hydration_pct": 60, "salt_pct": 2, "starter_pct_of_flour": [5, 20]}` [breadcode-1318; yt:-60DxtcJ9Fo description; creator-rule]
- Whole-wheat dough absorbs more water than white flour; he used 80 % hydration, then 90 %, and reported good gluten development after kneading the 90 % dough. — `{"hydration_pct": [80, 90]}` [breadcode-1343; yt:013Qw3BojM4 7:07; creator-rule]
- A low-gluten rye and lentil-flour mix (1000 g flour mix) can be made with 900 g water (90 % hydration), 18 g salt and 90 g starter, stirred by hand with no kneading. — `{"flour_g": 1000, "water_g": 900, "salt_g": 18, "starter_g": 90, "hydration_pct": 90}` [breadcode-1353; yt:013Qw3BojM4 8:21-9:34; creator-rule]
- Choose the water level from the flour's protein content; for a 15 % protein flour he uses 70 % hydration (280 g water on 400 g flour). — `{"hydration_pct": 70}` [breadcode-1364; yt:o_xtVmR4vlc 5:00; creator-rule]
- Higher-protein flour takes more water, and a no-knead dough should use slightly less water than a kneaded one. [breadcode-1365; yt:o_xtVmR4vlc 5:00; creator-rule]
- Choose water by flour gluten: about 60 % for low-gluten flour, 70 % for medium (10–12 % protein), 80 % or more for high-gluten flour. — `{"hydration_pct": [60, 80]}` [breadcode-1379; yt:msqU-ylXWUs 5:30; creator-rule]
- Use 65 % water for a 10 % protein flour; if the flour is 12 % protein, use 70 %. — `{"hydration_pct": [65, 70]}` [breadcode-1388; yt:NMglhwp2lNs description; creator-rule]
- Flour around 14-15 % protein can take 80-85 % water; the 65 % recipe is chosen for ease, not maximum absorption. — `{"hydration_pct": [80, 85], "flour_protein_pct": [14, 15]}` [breadcode-1419; yt:NMglhwp2lNs 3:05; creator-rule]
- Very light sourdough pizza can be made at about 95 % hydration with room-temperature fermentation and no refrigeration, but this is hard to reproduce. — `{"hydration_pct": 95}` [breadcode-1432; yt:WxFE3bxrTSM 10:33; anecdote] *(low confidence)*
- For about 10 % protein flour, start near 55 % hydration (check the flour packaging). — `{"protein_pct": 10, "hydration_pct": 55}` [breadcode-1438; yt:l0nAMbbYziI 3:31; creator-rule]
- For about 12 % protein flour, use about 60 % hydration; the Neapolitan regulations also specify 60 %. — `{"protein_pct": 12, "hydration_pct": 60}` [breadcode-1439; yt:l0nAMbbYziI 3:31; creator-rule]
- For 14 % protein flour or more, use about 65 % hydration. — `{"protein_pct": 14, "hydration_pct": 65}` [breadcode-1440; yt:l0nAMbbYziI 3:31; creator-rule]
- For beginners, start near 65 % hydration; this is easier to handle than higher levels. — `{"hydration_pct": 65}` [breadcode-1463; yt:qM9BXKaIr60 2:52; 3:24; creator-rule]
- At about 90 % hydration the dough tore quickly and needed expert handling, so he advises against it for beginners. — `{"hydration_pct": 90}` [breadcode-1464; yt:qM9BXKaIr60 5:05; 5:40; anecdote]
- The right water amount depends on your flour, so test your flour rather than copying a hydration figure from the internet. [breadcode-1465; yt:qM9BXKaIr60 3:57; creator-rule]
- Starter carries its own water and flour, so it raises the effective dough hydration; count it when setting hydration. [breadcode-1487; yt:JZBtmQXKWA8 6:31; 7:08; creator-rule]
- A liquid starter adds water on top of the flour and raises dough hydration; count that water in your hydration calculation. [breadcode-1514; yt:HLPNdyGCSPk 3:20; creator-rule]
- Adding a lot of flour during shaping of a high-hydration dough changes its hydration; unless you account for it, avoid heavy flouring. [breadcode-1523; yt:rGL8MnW6jEU 0:00; anecdote]
- The demonstration dough is 80 % hydration, with starter at 20 % and salt at 2 % of flour weight. — `{"hydration_pct": 80, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-1549; yt:wHL44ONu3so description; creator-rule]
- Use about 65 % water for a flour with about 10 % protein; his usual sourdough ratio is about 80 %, reduced because this flour has less protein. — `{"hydration_pct": 65, "flour_protein_pct": 10}` [breadcode-1606; yt:0kFYyj3WKpE 1:49; 2:32; description; creator-rule]
- The right hydration depends on the flour: test several bowls with increasing water during autolyse and use the highest water level that still gives a windowpane. [breadcode-1665; yt:wHL44ONu3so 7:44; creator-rule]
- A local organic whole-wheat flour absorbed about 80 % water in his test. — `{"hydration_pct": 80}` [breadcode-1707; yt:2DFvfRIywC8 1:11; anecdote]
- Hendrik uses 70 % water for a bread flour of about 15 % protein; higher-protein flour absorbs more water. — `{"hydration_pct": 70, "flour_protein_pct": 15}` [breadcode-1712; yt:dl7crk2Nvj8 5:40; creator-rule]
- Same flour and starter at 50 %, 100 % and 500 % hydration: the 50 % stiff starter inflated a balloon fastest; pH about 4.5 for stiff, about 3.9–4.0 for the others. — `{"hydration_pct": [50, 100, 500], "ph_range": [3.9, 4.5]}` [breadcode-1722; yt:lNCHzD8HxBA 8:52; pH at 10:07; measured]
- For his flour (about 15 % protein) he uses 70 % water (280 g per 400 g flour); higher-protein flour takes more water, lower-protein flour less. — `{"hydration_pct": 70, "flour_protein_pct": 15}` [breadcode-1737; yt:vyDTKFtfIb0 2:27–3:02; creator-rule]
- Iteration 3 is labelled 110 % hydration in the description: 550 g water on a 500 g whole-wheat base. Computed on the full 520 g of flour (450 g whole wheat plus 70 g vital wheat gluten) it is 105.8 %. — `{"water_g": 550, "total_flour_g": 520, "hydration_pct": 105.8, "stated_hydration_pct": 110}` [breadcode-1748; yt:2DFvfRIywC8 description (recipe list, iter; anecdote]
- A liquid starter counted as water raised the final hydration of the dough to about 90%. [breadcode-1812; yt:dQEebR6pIng 5:00; creator-rule]
- Per-loaf recipe: 500 g flour, 325 g water (65 % hydration), 100 g starter (20 %), 10 g salt (2 %). He says every flour is unique, so the right amount of water depends on the flour and on feel. — `{"hydration_pct": 65, "starter_pct_of_flour": 20, "salt_pct_of_flour": 2, "flour_g_per_loaf": 500}` [breadcode-1828; yt:3vr2_qqeAL8 description; 3:27; creator-rule]
- Hydration is water per 100 g flour. In the creator's test with 50 % whole wheat + 50 % strong bread flour, 90 % held together best; 95 %, 100 % and 105 % tore. — `{"hydration_pct_best": 90, "hydration_pct_tore": [95, 100, 105]}` [breadcode-1835; yt:BnvhhxQZQ14 4:30; measured]
- In the creator's autolyse test, 100 g flour was mixed with water and rested about 1 h before stretch-and-fold testing. — `{"flour_g": 100, "autolyse_h": 1}` [breadcode-1867; yt:BnvhhxQZQ14 2:16; measured]
- Water values from recipes are flour-specific: the same hydration can be too wet for one flour, so the water should be adjusted to the flour being used. — `{"hydration_pct": 80}` [breadcode-1941; yt:G0nVQy-L3Bk 10:29; creator-rule]
- Dough made with 500 g flour and 400 g water (80 % hydration) turned into a soup that was hard to handle and had to be rescued by adding flour; 500 g flour with 300 g water (60 %) was easy to handle from the start. — `{"flour_g": 500, "water_g": [300, 400], "hydration_pct": [60, 80]}` [breadcode-1948; yt:t3GkfPxQxik 1:15; 4:54; measured]
- For an overnight sourdough, about 65 % hydration keeps the dough workable; a higher hydration over a long ferment gets sticky because acid weakens the gluten. — `{"hydration_pct": 65}` [breadcode-1969; yt:APstLJpWOiE 0:37; creator-rule]
- For a 15 % protein flour he used about 80 % hydration (640 g water on 800 g flour) so the dough had more room to expand, at the cost of harder handling. — `{"hydration_pct": 80}` [breadcode-2019; yt:lJ38fY3ZEQk description; creator-rule]
- With whole-meal flour a slightly larger amount of water may be needed to get extensibility in the dough. [breadcode-2033; yt:m7J6CBZcq8o 7:44; creator-rule] *(low confidence)*
- Wheat roll dough water ranges 60–80 % of flour weight, depending on the flour; lower-protein flour at the low end, stronger flour higher. — `{"hydration_pct": [60, 80]}` [breadcode-2041; yt:OcOpIISBK6c description; transcript [1:48]; creator-rule]
- Water needs depend on the flour: whole wheat and high-gluten flours absorb more water. [breadcode-2061; yt:0aylYlvA4K8 [6:08]; creator-rule]
- A 100 % hydration whole-wheat loaf: 400 g whole wheat, 400 g water, 20 g starter (5 %), 8 g salt (2 %). — `{"hydration_pct": 100, "starter_pct_of_flour": 5, "salt_pct": 2}` [breadcode-2074; yt:MPWypFY0wyE description; [0:00]; creator-rule]
- An earlier video put 85 % as the sweet spot for 100 g flour, below full 100 % hydration. — `{"hydration_pct": 85}` [breadcode-2075; yt:MPWypFY0wyE [0:35]; creator-rule] *(low confidence)*
- Water is 325 g per 500 g flour, i.e. 65 % of flour weight (400 g bread flour + 100 g whole wheat). — `{"hydration_pct": 65, "water_g": 325, "flour_g": 500}` [breadcode-2084; yt:gOxCUh_kyMQ [2:44]; creator-rule]
- Water should scale with flour protein; the video flour is about 15 % protein. — `{"protein_pct": 15}` [breadcode-2085; yt:gOxCUh_kyMQ [3:20]; creator-rule]
- Formula: 500 g flour, 375 g water, 100 g rye starter (20 %), 10 g salt (2 %), final hydration about 77 %. — `{"hydration_pct": 77, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-2105; yt:4VJQqOtwP_E description; [0:00]; creator-rule]
- Basic loaf: 400 g flour, 240 g water (60 %), 8 g salt (2 %) and 40–80 g starter (10–20 %). — `{"hydration_pct": 60, "salt_pct": 2, "starter_pct_of_flour": [10, 20]}` [breadcode-2118; yt:_P1B7PxAwk0 [7:44]; creator-rule]
- Suggested hydration by flour protein: about 10 % protein gives 65 % and about 12 % gives 70 %. His 14 % strong bread flour is used with 75 % water in the main dough (plus the poolish water, about 79 % overall). — `{"hydration_pct_at_10pct_protein": 65, "hydration_pct_at_12pct_protein": 70, "main_dough_water_pct_at_14pct_protein": 75, "total_hydration_pct": 79.2}` [breadcode-2178; yt:EbO3blOpGks description; creator-rule]
- A high-water dough becomes very elastic and gives a fluffy crumb, but it is harder to handle and sticky. — `{"hydration_pct": 75}` [breadcode-2179; yt:EbO3blOpGks 1:08; creator-rule]
- For most flours, about 65 % hydration is a good starting point; a sticky dough with an open crumb can mean too much water for that flour. — `{"hydration_pct": 65}` [breadcode-2251; yt:oAFjGTWMgUo 4:59; creator-rule]
- The one-hour pizza dough is 60 % hydration: 412 g water to 687 g flour. — `{"hydration_pct": 60}` [breadcode-2290; yt:-rwuxy5KS6Q description; creator-rule]
- Water needs depend on the flour: if the dough looks overly dry add a little more water; wholemeal flour usually needs a tiny bit more. [breadcode-2291; yt:-rwuxy5KS6Q 6:34; creator-rule]
- The water quantity depends on the flour's gluten content, so a generic hydration figure from the internet may not suit a given flour. [breadcode-2315; yt:sFQZIKb_iIA 5:32; creator-rule]
- For a flour of about 15% protein, a hydration of about 70% is a suitable starting point, and the right figure depends on the flour. — `{"hydration_pct": 70, "protein_pct": 15}` [breadcode-2320; yt:JoAG7R9QESA 1:08; creator-rule]
- The creator considers about 55% hydration better for this cracker dough than about 50%. — `{"hydration_pct": 55}` [breadcode-2330; yt:urHYzIktD9M 3:27; anecdote] *(low confidence)*
- More water needs more kneading to develop strength. [breadcode-2341; yt:HNQbwvZIAYc 0:31; creator-rule]
- A very wet discard dough baked in a loaf pan gives an open crumb, because water evaporates during the bake. [breadcode-2347; yt:7V3FyVzzVUI 1:45; creator-rule]
- To find a flour's absorption, prepare small bowls with different water levels and test each for a windowpane after about an hour; the highest level that still gives a windowpane is the limit, and beginners should stay lower. — `{"rest_h": 1, "test_bowls": 5}` [breadcode-2374; yt:-6dlWqFlFSo 2:50; creator-rule]
- Bassinage (holding back part of the water and adding it after the initial mix) let the creator push a dough to about 105% hydration, which he finds more extensible but very hard to divide and shape. — `{"hydration_pct": 105}` [breadcode-2376; yt:0v1QhtyUic4 description; 5:31-6:47; creator-rule]
- The discard dough is a stirred batter at about 100 % hydration; it cannot be kneaded and is mixed with a spoon until no flour chunks remain, usually about one minute. — `{"hydration_pct": 100, "mix_min": 1}` [breadcode-2385; yt:7V3FyVzzVUI 4:06; creator-rule]
- A normal sourdough dough is about 100 % hydration (equal parts flour and water by weight). — `{"hydration_pct": 100}` [breadcode-2402; yt:sFQZIKb_iIA 1:15; creator-rule]
- A liquid, converted starter is so highly hydrated that it is easy to add too much water to the dough; water must be adjusted with care. [breadcode-2424; yt:HgcGcG7rdZM 4:25; creator-rule]
- For a home oven without a very hot oven, raise naan hydration to about 80 % (80 g water per 100 g flour) so the naan doesn't dry out during a 3-4 minute broil. — `{"hydration_pct": 80, "bake_min": [3, 4]}` [breadcode-2446; yt:sH99_IGh0ic description; extrapolation] *(low confidence)*
- Flour absorption depends mainly on its protein (gluten) content and on how much bran (outer grain layer) it contains, so the water a flour needs changes from one flour to another. [breadcode-2449; yt:s1gM_jziXcI 0:33; creator-rule]
- Published water percentages differ (for example 70 % or 80 %) because they depend on the flour; test each new flour rather than copying a figure. — `{"hydration_pct": [70, 80]}` [breadcode-2450; yt:s1gM_jziXcI 0:00; creator-rule]
- Hydration test: 40 g flour per bowl with water at 65, 70, 75, 80 and 85 % (26, 28, 30, 32 and 34 g), rest 30 min (autolyse), then pull each dough with wet hands; the level where the dough holds a windowpane is a safe hydration. — `{"flour_g": 40, "hydration_pct": [65, 85], "water_g": [26, 34], "autolyse_min": 30}` [breadcode-2451; yt:s1gM_jziXcI 2:59; measured]
- In the test with that flour, 65–80 % gave a windowpane (80 % with technique), 85 % tore too much and was not usable; 75 % was the easier good level. — `{"hydration_safe_pct": [65, 80], "hydration_fail_pct": 85, "hydration_easier_pct": 75}` [breadcode-2452; yt:s1gM_jziXcI 6:30; measured]
- Beginners should stay about 5 points below the test limit (for example 70 % instead of 75 %), because high-hydration dough needs more technique. — `{"hydration_pct": 70, "test_hydration_pct": 75}` [breadcode-2453; yt:s1gM_jziXcI 7:04; creator-rule]
- He advises going lower in the water range (600–800 g) because it simplifies everything afterwards. — `{"hydration_pct": [60, 80]}` [breadcode-2498; yt:0h63OloQQd8 0:35; creator-rule]
- The dough in this boule and batard demo is about 72 % hydration: 72 g water per 100 g flour. — `{"hydration_pct": 72}` [breadcode-2537; yt:2tOw0eyuEsU description; creator-rule]
- Recommended water test series: prepare several small doughs at 60, 65, 70, 75 and 80 % water (85 % was also tested in the video for a high-protein flour), using a small flour base (40 g) to reduce waste, rest 30 min, then pull each dough to check for a windowpane. — `{"hydration_pct": [60, 85], "flour_g": 40, "autolyse_min": 30}` [breadcode-2562; yt:s1gM_jziXcI description; creator-rule]
- Demo overnight sourdough: 400 g bread flour, 300 g water (75 %), 40 g starter (10 %), 8 g salt (2 %). — `{"flour_g": 400, "water_pct": 75, "starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-2570; yt:gMbZeUIVzZY description; creator-rule]
- 3-minute sourdough: 900 g bread flour, 900 g water (100 % hydration), 18 g salt (2 %), and 45–180 g starter (5–20 %) per the description. — `{"flour_g": 900, "water_pct": 100, "salt_pct": 2, "starter_pct_of_flour": [5, 20]}` [breadcode-2598; yt:P0-EGk_1SYo description; creator-rule]
- At about 75 % hydration a dough is relatively sticky; a dome shape reduces contact with the fingers and sticking. — `{"hydration_pct": 75}` [breadcode-2620; yt:HYCaVzqOveU 0:32; creator-rule]
- Water needed depends on the flour's protein: higher-protein flour absorbs more water, so adjust water to the flour you use. [breadcode-2621; yt:ep0mua88Sa8 0:00; creator-rule]
- To find water for an unknown flour, test 100 g portions with different water amounts and pick the most water that still shows a window pane; go lower if unsure. — `{"test_flour_g": 100}` [breadcode-2622; yt:ep0mua88Sa8 0:36; creator-rule]
- Water in a written recipe is not fixed: check the consistency with your own flour, since milled einkorn needed more flour than the recipe water. [breadcode-2653; yt:nlqVo9sxTlQ 1:17 (too much water, added fl; creator-rule]
- Wetting hands between steps stops very wet dough (about 80 % hydration) from sticking to them. — `{"hydration_pct": 80}` [breadcode-2674; yt:tWIn8NHJkFU description; creator-rule]
- Demo dough: 80 % hydration, 50 % bread flour and 50 % strong whole-wheat flour, 20 % starter and 2 % salt (baker's %). — `{"hydration_pct": 80, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-2675; yt:tWIn8NHJkFU description; creator-rule]
- Coca-Cola replaced all the water: 320 g cola on 400 g flour (80 %). The dough came together, though sticky and sugary to the hands, and its test piece doubled. — `{"cola_pct_of_flour": 80}` [breadcode-2686; yt:0oKA_iTytiw 1:21; anecdote]
- At 85 % hydration the dough is very sticky: he dusted the surface with flour and tucked the dough with a scraper, and it still stuck once. — `{"hydration_pct": 85}` [breadcode-2699; yt:bBSuFq346YM 1:54; creator-rule]
- With very fine cake flour the dough was too crumbly and he added water; he says he originally used too little water. [breadcode-2710; yt:lG9ERQFFn5I 1:17; anecdote]
- Neapolitan pizza dough for two pizzas: 280 g bread flour, 168 g water (60 %), 5 g salt (2 %) and about 1 g dry yeast. — `{"hydration_pct": 60, "salt_pct": 2, "yeast_g": 1}` [breadcode-2714; yt:ziMMCphF054 description; creator-rule]
- Lazy wholegrain bread: 500 g wholegrain flour, 475 g water (95 %), 10 g salt (2 %) and 100 g starter (20 %); no kneading. — `{"flour_g": 500, "water_g": 475, "salt_pct": 2, "starter_pct_of_flour": 20, "hydration_pct": 95}` [breadcode-2729; yt:2NmD9KVyZCg 1:05; creator-rule]
- A 95 % hydration wholegrain dough cannot be shaped on a banneton; it works only in a loaf pan. — `{"hydration_pct": 95}` [breadcode-2734; yt:2NmD9KVyZCg 8:01; creator-rule]
- High hydration (95 %) gives a moist crumb. — `{"hydration_pct": 95}` [breadcode-2737; yt:2NmD9KVyZCg 8:01; creator-rule]
- This seed dough is too wet to stir by hand; the creator mixes everything together in a large pot. [breadcode-2767; yt:cN_skGXZ4Es 0:47; creator-rule]
- Bun dough with 65 % water on flour and 40 % starter gives about 70 % total hydration when starter water is counted. — `{"hydration_flour_pct": 65, "starter_pct_of_flour": 40, "total_hydration_pct": 70}` [breadcode-2810; yt:nIQEiSGGw2Q description; creator-rule]
- At about 70 % hydration, line the banneton with a towel or linen to stop the dough sticking. — `{"hydration_pct": 70}` [breadcode-2814; yt:g2Wx3JSiPvo description; creator-rule]
- At about 65 % hydration he prefers an unlined banneton, which adds a nice visual pattern to the loaf. — `{"hydration_pct": 65}` [breadcode-2815; yt:g2Wx3JSiPvo description; creator-rule]
- A 92 % total hydration dough = 90 % base water + 2 % from a 20 % sourdough addition. — `{"hydration_pct": 92, "base_hydration_pct": 90, "starter_pct_of_flour": 20, "starter_water_pct": 2}` [breadcode-2843; yt:nbw-p6_L36w description; creator-rule]
- A wetter dough is more elastic and can be stretched more, which helps an open crumb. [breadcode-2844; yt:nbw-p6_L36w description; creator-rule]
- Sourdough buns at about 65 % hydration; he lowers hydration for buns because they ferment on the tray, and too high hydration makes the dough elastic and weakens oven spring. — `{"hydration_pct": 65}` [breadcode-2875; yt:HHKGRwH76ls description; creator-rule]
- Hydration (baker's math) is water weight divided by flour weight; 60 % means 60 g water per 100 g flour. — `{"hydration_pct": 60}` [breadcode-2884; yt:yvAxtECqRAs description; creator-rule]
- Stronger flours absorb more water, so the right hydration depends on the flour. [breadcode-2885; yt:yvAxtECqRAs description; creator-rule]
- In the fixed loaf (150 g flour, 2 % salt, about 0.5 % yeast), the 60 % hydration loaf rose a lot in the oven in the five-loaf comparison (his visual judgement). — `{"hydration_pct": 60, "flour_g": 150, "salt_pct": 2, "yeast_pct": 0.5}` [breadcode-2886; yt:yvAxtECqRAs 0:40; measured]
- At 90 % and 100 % hydration the loaves could not hold their shape; 80 % had large bubbles; 70 % looked best. — `{"shape_fail_hydration_pct": [90, 100], "best_visual_hydration_pct": 70}` [breadcode-2887; yt:yvAxtECqRAs 0:40; measured]
- A high-hydration dough (about 80 %) helps an open crumb but is harder to handle and needs better technique. — `{"hydration_pct": 80}` [breadcode-2961; yt:m0EyfGp70Hw description; creator-rule]

### hydration.flour-absorption

- Flours closer to whole grain (more bran) and higher-protein flours absorb more water, so the same water percentage behaves differently with different flours. [breadcode-1886; yt:t3GkfPxQxik 1:52; creator-rule]

### hydration.open-crumb-recipe

- The creator's open-crumb wheat loaf is about 80% hydration, built from 500 g strong bread flour and 400 g water with starter and salt, and gets its structure from bench kneading, lamination and coil folds. — `{"flour_g": 500, "water_g": 400, "hydration_pct": 80}` [breadcode-1890; yt:U9_GVHuI1Ro 1:09; creator-rule]

### hydration.over-fermentation

- Higher hydration shifts fermentation toward bacterial acid production and away from yeast gas, and gluten degrades faster; lower hydration gives more yeast activity and lower risk of over-fermentation. [breadcode-1887; yt:t3GkfPxQxik 22:49; creator-rule]

### hydration.over-hydration

- Overhydrated wheat dough is much harder to knead, stretch and shape than a lower-hydration dough made from the same flour, and it sticks to the bench, hands and banneton. — `{"flour_g": 500, "water_g": [300, 400]}` [breadcode-1884; yt:t3GkfPxQxik 1:15; measured]

### hydration.same-day-recipe

- Same-day recipe: 400 g bread flour (15% protein) and 100 g wholemeal, 100 g rye starter (20%), 350 g water (70%) and 10 g salt; with 12% protein flour, the creator suggests 65% water. — `{"flour_g": 500, "water_g": 350, "hydration_pct": 70, "salt_g": 10, "starter_pct": 20}` [breadcode-1912; yt:b7Pd7UcZxd0 description; creator-rule]

### hydration.slow-fermentation

- Slow fermentation gives the extensibility of high hydration without very wet dough; slow it with less starter or a cooler environment. [breadcode-0407; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]

### hydration.wheat-baseline

- The creator calls about 60% hydration the sweet spot for many wheat doughs and advises starting lower than high-hydration recipes unless the baker is experienced. — `{"hydration_pct": 60}` [breadcode-1885; yt:t3GkfPxQxik 3:08; creator-rule]
