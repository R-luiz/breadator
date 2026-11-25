# flour — The Bread Code / The Sourdough Framework

Claims: 132 (book 63, videos 66, repos 3). Evidence: creator-rule 116, anecdote 12, literature 2, measured 2.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

Country label equivalence (Table 5.1, `sf-table-5-1-flour-types.csv`), grain suitability (Table 5.2) and W-values (Table 5.3) are tabulated. Protein (≈80 % gluten) sets the aliquot target and water uptake; whole-grain flours carry more enzymes and bran, speeding autolysis/fermentation and absorbing more water. Ancient grains (einkorn, emmer, spelt, khorasan) have weaker gluten; for whole wheat the videos aim for a smaller rise (bi-zqG5y1I8); the einkorn farmer interview (wdnX3baMB38, guest Jade) adds falling number and storage notes.

## Claims

### flour

- Supermarket flour is blended from several sources rather than single origin, so bakeries can get the same product from year to year. [breadcode-0007; SF Ch 1 The history of sourdough; creator-rule]
- Grains have been bred over time toward higher gluten content, so modern bread contains more gluten than ancient bread (the author's account). [breadcode-0010; SF Ch 1 The history of sourdough; creator-rule]
- Milled flour can no longer sprout, but the enzymes that trigger sprouting remain present, so milling at too high a temperature can damage some of them. [breadcode-0017; SF Ch 2 How sourdough works; creator-rule]
- A recent study found that milling flour at home with a small mill had no significant negative impact on bread quality compared with flour from temperature-regulated large-scale mills. [breadcode-0018; SF Ch 2 How sourdough works; literature]
- Grinding grain exposes the seed contents, which is ideal for sourdough microorganisms, while the intact seed normally shields the germ from pathogens. [breadcode-0019; SF Ch 2 How sourdough works; creator-rule]
- For bootstrapping a starter, use whole flour (whole wheat, rye, spelt or other), because the wild microbes sit on the grain's hull/surface, so whole flour carries more of them than refined flour. [breadcode-0093; SF Ch 3 Making a sourdough starter; creator-rule]
- Gluten-free whole flours such as rice or corn can also start a starter; the flour can be changed later. [breadcode-0095; SF Ch 3 Making a sourdough starter; creator-rule]
- Some non-organic flours may contain treatments that block microbe growth; if the starter fails, switch flour and wait about ten days before retrying. — `{"retry_wait_h": 240}` [breadcode-0134; SF Ch 3 Making a sourdough starter; creator-rule]
- Flour, water and salt are the essential ingredients of wheat sourdough; additions such as seeds are optional. [breadcode-0380; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Adding 20-30 % whole-wheat flour (of total flour) gives a hearty flavour. — `{"whole_wheat_pct_of_flour": [20, 30]}` [breadcode-0381; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Whole-wheat can be used at 100 % of flour, but then use a strong whole-wheat flour milled from higher-protein wheat. — `{"whole_wheat_pct_of_flour": 100}` [breadcode-0382; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- If whole-wheat is omitted, replace its quantity with bread flour. [breadcode-0383; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Bread flour, with more gluten than all-purpose or cake flour, is recommended, especially for beginners, and is essential for freestanding loaves. [breadcode-0385; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Salt is about 2 % of total flour weight. — `{"salt_pct_of_flour": [2, 2]}` [breadcode-0386; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Powder mix-ins integrate homogeneously and may replace some of the flour. [breadcode-0713; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Non-wheat flours such as chickpea, corn, hemp and potato can partially replace wheat flour. [breadcode-0728; SF Ch 9 Mix-ins §9.2 Examples > Flours; creator-rule]
- In Germany, all-purpose flour corresponds to type 550. — `{"type_number_DE": 550}` [breadcode-0800; SF Glossary > All Purpose Flour; creator-rule]
- The alveograph inflates a piece of dough like a balloon until it bursts, and the resulting curve describes the balance of dough elasticity and extensibility, used to judge a flour's baking performance. [breadcode-0802; SF Glossary > Alveograph; creator-rule]
- Bread flour has more gluten and can therefore ferment for a longer period. [breadcode-0822; SF Glossary > Bread Flour; creator-rule]
- The P/L value is the ratio of the alveograph's P (pressure, a measure of dough tenacity) to L (length, extensibility). [breadcode-0865; SF Glossary > P/L Value; creator-rule]
- A low P/L value means the dough is more extensible than resistant, which suits products such as pizza or ciabatta. [breadcode-0866; SF Glossary > P/L Value; creator-rule]
- A high P/L value means the dough has more strength than extensibility, which suits products that need volume and structure. [breadcode-0867; SF Glossary > P/L Value; creator-rule]
- The W-value, from the Chopin alveograph test, is the energy required to blow a dough bubble until it bursts and indicates a flour's ability to withstand fermentation and baking. [breadcode-0883; SF Glossary > W-Value; creator-rule]
- A high W-value means a stronger flour suited to high-volume bread and longer fermentation; a low W-value suits cakes and pastries. [breadcode-0884; SF Glossary > W-Value; creator-rule]
- Organic unbleached flour is recommended for a starter, because industrial flour may be treated with fungicides. [breadcode-0913; SF Ch 12 Troubleshooting §12.1.5 My new starter doesn't rise at all; creator-rule]
- Whole wheat flour contains more enzymes than white flour, which accelerated the tearing in the experiment. [breadcode-0968; SF Ch 12 Troubleshooting §12.2.4 My dough completely tears after a long f; creator-rule]
- Dusting the dough with non-rice flour before placing it in the banneton is the author's practice. [breadcode-1007; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out when removin; creator-rule] *(low confidence)*
- Einkorn and emmer doughs behave sticky like rye; in the same recipe the einkorn loaf rose slightly higher in the oven. [breadcode-1136; repo:the-bread-code experiments/einkorn-vs-emmer.md; measured] *(low confidence)*
- Einkorn crumb was yellowish and emmer crumb looked rye-like; taste was judged equal. [breadcode-1137; repo:the-bread-code experiments/einkorn-vs-emmer.md; measured] *(low confidence)*
- Corn flour has no gluten, so the broa structure comes from rye. [breadcode-1171; repo:the-bread-code recipes/sourdough/portuguese-broa-corn-bread.md; creator-rule]
- Flour with more protein can be fermented longer and inflates more, giving better crumb and oven spring; many whole wheat flours from cooler climates have less protein, so test each new flour. [breadcode-1265; yt:ImVvQMvGZKE 4:06; creator-rule]
- Milling roughly 5 cups of grain yields about 12 cups of flour; volume measures are imprecise because flour density varies. — `{"grain_cups": 5, "flour_cups": 12}` [breadcode-1289; yt:wdnX3baMB38 54:10; anecdote]
- Grain quality cannot be judged by looking at it; before buying more, test-bake a small amount and reorder if the result is good. [breadcode-1291; yt:wdnX3baMB38 53:00; creator-rule]
- Replacing about 10 % of a usual flour with freshly milled whole grain is a low-risk way to test flavour. — `{"substitution_pct": 10}` [breadcode-1292; yt:wdnX3baMB38 58:23; creator-rule]
- A stiff starter with more yeast and less gluten-eating bacteria lets a lower-protein flour give good bread. [breadcode-1384; yt:msqU-ylXWUs 8:12; creator-rule]
- Lighter flour types keep longer (about a year), while darker flour types go off sooner. [breadcode-1399; yt:KooRWvS5c-A 44:32; anecdote] *(low confidence)*
- German flour type numbers (e.g. 550, 812, 1050) indicate how dark or bright the flour is, which reflects its mineral (ash) content. The number says nothing about baking quality. [breadcode-1400; yt:KooRWvS5c-A 37:09-37:46; creator-rule]
- The grain is stored for roughly 6 to 8 weeks before milling; he says this post-harvest rest improves the baking properties of the flour. — `{"rest_weeks": [6, 8]}` [breadcode-1405; yt:KooRWvS5c-A 34:40; creator-rule] *(low confidence)*
- German type 1150 is the typical bread flour, and he says it is also used for dark breads and sourdough. [breadcode-1407; yt:KooRWvS5c-A 38:23; creator-rule] *(low confidence)*
- Water absorption rises with flour protein, mostly because gluten (about 80 % of wheat protein) takes up water, so the water amount must match the flour. [breadcode-1441; yt:l0nAMbbYziI 4:04; creator-rule]
- Long autolysis had a negative effect in his whole-wheat tests, so he skips it for whole-wheat dough. [breadcode-1482; yt:JZBtmQXKWA8 0:37; 1:25; 31:35; anecdote] *(low confidence)*
- Whole-wheat ciabatta skips autolyse because whole wheat has more enzymes and everything runs faster. [breadcode-1501; yt:HSdOUZq5i6U 28:35; creator-rule]
- The flour used has about 13 % protein. — `{"protein_pct": 13}` [breadcode-1553; yt:36GdGT0XYwE 1:14; creator-rule]
- Use a high-protein flour for laminated sourdough, since its stronger gluten tolerates the longer, cooler fermentation. [breadcode-1625; yt:dY9GeoZX8DY 13:56; creator-rule]
- German Type 550 flour (the US all-purpose equivalent) has about 11–12 % protein. — `{"flour_protein_pct": [11, 12]}` [breadcode-1652; yt:0kFYyj3WKpE description; 2:32; creator-rule]
- A high-hydration demonstration dough used flour of about 14 % protein; with a lower-protein flour, reduce the water. — `{"flour_protein_pct": 14}` [breadcode-1663; yt:wHL44ONu3so 0:00; creator-rule]
- Bran cuts the CO₂ bubbles during expansion rather than the gluten during mixing, so high-bran dough needs a longer mix. [breadcode-1690; yt:l_PLhPWE-8E 20:57–22:09; anecdote] *(low confidence)*
- Higher-protein flour tolerates a longer fermentation; as gluten degrades during fermentation, weak flour turns into a slack blob. [breadcode-1694; yt:9fWzWpL5g6w 3:04; creator-rule]
- Low-protein wheat, comparable to cake flour, is a poor choice for freestanding sourdough. [breadcode-1759; yt:9fWzWpL5g6w 3:41; creator-rule]
- Spelt is treated the same as wheat in this roll recipe. [breadcode-2043; yt:OcOpIISBK6c description; creator-rule]
- Rye, emmer, einkorn and old spelt doughs use about 240 g water (description gives no flour weight), are mixed without kneading, are refrigerated before baking; the description both says to shape them elongated and says shaping is not possible. — `{"water_g": 240}` [breadcode-2044; yt:OcOpIISBK6c description; creator-rule] *(low confidence)*
- Rye in the starter makes the dough stickier; he made a stiffer dough so it would not slump out of the glass bowl. [breadcode-2125; yt:6x9ic4YIni8 [9:07]-[9:41]; creator-rule]
- The flour used for this roughly 77 % hydration sticky dough has about 15 % protein. — `{"protein_pct": 15, "hydration_pct": 77}` [breadcode-2141; yt:4VJQqOtwP_E [1:06]; creator-rule]
- About 80% of wheat protein is gluten, so a flour with more protein can absorb more water. — `{"gluten_pct_of_protein": 80}` [breadcode-2335; yt:-6dlWqFlFSo 2:50; creator-rule]
- A weak, cheap flour over-fermented after about 10 h, and the dough collapsed. — `{"hours": 10}` [breadcode-2355; yt:WOfTMskT44Y 2:29; anecdote] *(low confidence)*
- Cheap cake flour tasted fine in the creator's test. [breadcode-2356; yt:WOfTMskT44Y 7:33; anecdote] *(low confidence)*
- The creator would not buy cheap flour long-term, because of the flour's treatment and inconsistent standardisation. [breadcode-2357; yt:WOfTMskT44Y 8:10; anecdote] *(low confidence)*
- Cake-flour dough deflated noticeably during shaping and retained little gas, which is unusual for him. [breadcode-2711; yt:lG9ERQFFn5I 2:26; anecdote]
- Kneading is for wheat dough; for rye, emmer, spelt or einkorn he says no kneading is needed because these flours have almost no gluten. [breadcode-2744; yt:Yki-0UoPIn4 description; creator-rule] *(low confidence)*
- Einkorn and emmer contain almost no gluten, so they give little dough strength and are only stirred together, not kneaded. [breadcode-2846; yt:FMP2-Qa8vXk 0:00; creator-rule]
- Rye has almost no gluten, so an open crumb is very hard to achieve; rye breads need only stirring. [breadcode-2847; yt:FMP2-Qa8vXk 0:40; creator-rule]
- Spelt absorbs a lot of water and bakes like wheat; most spelt sold today is a wheat-spelt mix rather than old spelt. [breadcode-2848; yt:FMP2-Qa8vXk 0:00; creator-rule]
- Wheat has a lot of gluten, which allows an open crumb and good oven spring; for oven spring, wheat is the choice. [breadcode-2849; yt:FMP2-Qa8vXk 1:18; creator-rule]
- Flours absorb water at different rates because milling, grain type and growing conditions differ, so one hydration figure does not fit every flour. [breadcode-2974; yt:2TukiEN8VQM 0:00; creator-rule]

### flour.absorption

- A low-gluten flour absorbs less water, so its dough hydration should be reduced. [breadcode-0223; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]

### flour.ancient-grain-yield

- Einkorn and emmer give about 10% of wheat's yield per area, so they are scarce and expensive. He estimates their flour costs about eight times the cheapest supermarket flour. — `{"yield_vs_wheat_pct": 10, "price_ratio_vs_cheapest": 8}` [breadcode-1923; yt:PehhnV1lW2o description (yield); 7:43 (pri; creator-rule] *(low confidence)*

### flour.ash-type

- In the German system a flour type number is the mg of ash left after burning 100 g of flour; T405 leaves 405 mg. — `{"ash_mg_per_100g_flour": 405}` [breadcode-0251; SF Ch 5 Flour types; creator-rule]
- A higher type number means more hull and mineral content, closer to whole flour, and generally a heartier taste; the scale differs by grain. [breadcode-0252; SF Ch 5 Flour types; creator-rule]

### flour.corusan

- Corusan needs more mixing than other ancient grains to activate its gluten, and then bakes into a soft crumb. [breadcode-1283; yt:wdnX3baMB38 49:23; creator-rule]

### flour.einkorn

- For bakers with digestive concerns around gluten, the speaker suggests starting with einkorn or Corusan; einkorn's gluten differs from modern wheat's. [breadcode-1282; yt:wdnX3baMB38 47:37; creator-rule]
- Einkorn sourdough with three ingredients: 500 g einkorn, 100 g starter, 10 g salt and 325 g water (65 % hydration, 20 % starter, 2 % salt). — `{"flour_g": 500, "starter_g": 100, "salt_g": 10, "water_g": 325, "hydration_pct": 65, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-2700; yt:wz7tysfSDFM description; creator-rule]
- Einkorn dough has no strong gluten network, so shaping means tucking the dough together and avoiding sticking rather than building strength. [breadcode-2701; yt:wz7tysfSDFM 2:29; creator-rule]
- Rice flour in the banneton keeps einkorn dough from sticking. [breadcode-2702; yt:wz7tysfSDFM 1:53; creator-rule]

### flour.falling-number

- A fast drop of the stirring rod in a flour-water test gives a low falling number; a slow drop gives a high one. [breadcode-1277; yt:wdnX3baMB38 41:29; creator-rule]
- A grain's falling number can shift between seasons without an obvious cause; a lab retest returned the same value, which the speaker attributed to that year's growing conditions. [breadcode-1278; yt:wdnX3baMB38 42:45; anecdote]
- Spelt falling numbers this year were about 350, against roughly 400 in typical years; the speaker's producer did not know the cause. — `{"falling_number_typical": 400, "falling_number_this_year": 350}` [breadcode-1359; yt:wdnX3baMB38 41:29; anecdote]

### flour.gluten-free

- Grains such as rye, emmer, einkorn, rice and corn are homogenised only: no kneading, no stretch and fold, no shaping. [breadcode-0254; SF Ch 5 Flour types; creator-rule]
- In low- or no-gluten flours the main fermentation concern is acidity (avoid an over-sour bread), not gluten degradation. [breadcode-0258; SF Ch 5 Flour types; creator-rule]

### flour.grade

- Germany grades wheat as A (good quality, can be blended to improve weaker flour), B (average, general baking), C (poor baking quality, e.g. sprouted; feed or biomass) and E (elite, only from optimal growing conditions). [breadcode-0262; SF Ch 5 Flour types; creator-rule]
- Large mills blend flours and blends are not labelled, so a pack's name does not fully reveal its properties. [breadcode-0264; SF Ch 5 Flour types; creator-rule]

### flour.other-grains

- Flatbread dough made with einkorn is low in gluten, so it traps less CO2 than wheat; more water or some wheat flour makes it fluffier. [breadcode-0320; SF Ch 6 Bread types; creator-rule]
- Non-wheat flours (rye, emmer, einkorn) need no kneading, no stretch-and-fold, and no special handling for flatbread; over-fermentation is not a structural concern for them. [breadcode-0325; SF Ch 8 Non wheat sourdough; creator-rule]
- Corn flour has no gluten to develop, so this bread only needs mixing until homogeneous, not kneading. [breadcode-2609; yt:bjLCHRInmLY 0:38; creator-rule]
- He says rye, einkorn and emmer cannot develop a usable gluten network, so mixing alone is enough for those doughs. [breadcode-2628; yt:ep0mua88Sa8 3:35; creator-rule]
- Einkorn has no gluten to develop; mixing is all that is needed. [breadcode-2656; yt:nlqVo9sxTlQ 1:17; creator-rule]
- Corn-and-rye broa can be made with 70 % corn flour and 30 % rye flour instead of 50/50; he calls it a matter of taste. — `{"corn_flour_pct": 70, "rye_flour_pct": 30}` [breadcode-2657; yt:bjLCHRInmLY 0:38; creator-rule]
- A non-wheat sourdough loaf takes about 5 minutes of hands-on time, per the creator's book summary. — `{"hands_on_min": 5}` [breadcode-2753; yt:JgsvLGfSSt4 2:30; creator-rule] *(low confidence)*

### flour.protein

- Fermentations longer than about 8 hours call for a flour with higher gluten content; with weaker flour the dough can be broken down so much that it tears when stretched. — `{"bulk_h": 8}` [breadcode-0039; SF Ch 2 How sourdough works; creator-rule]
- With more bacterial activity, the microbes consume more gluten; so a free-standing loaf needs a flour with more gluten, because more gluten allows more of it to be broken down while dough integrity is kept. [breadcode-0170; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- A low inoculation rate (about 1 %) requires a stronger wheat flour because enzymatic breakdown of gluten may have degraded the gluten significantly before the starter regrows. — `{"starter_pct_of_flour": 1}` [breadcode-0181; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- A stronger flour with more gluten allows a longer fermentation before most of its gluten is broken down. [breadcode-0183; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Regular starter suits strong wheat or spelt flour, and also works with rye, emmer and einkorn. [breadcode-0188; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- A regular starter with a weak, low-gluten flour may cause problems because the substantial bacterial activity breaks gluten down quickly. [breadcode-0189; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- Bread flour (strong flour) has higher protein and gluten; it suits sourdough because the dough tolerates a longer leavening period. [breadcode-0259; SF Ch 5 Flour types; creator-rule]
- For cakes a low-gluten flour is preferable, since strong gluten binding makes the cake chewy. [breadcode-0261; SF Ch 5 Flour types; creator-rule]
- The grade is usually not printed on flour packs; protein content can serve as an indicator of quality. [breadcode-0263; SF Ch 5 Flour types; creator-rule]
- For freestanding loaves a flour with at least 12 % protein is recommended for long fermentation. — `{"protein_pct_min": 12}` [breadcode-0982; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Hendrik's flour for this pizza is about 14 % protein; he says higher protein in general is better for this dough, so check the flour packaging. — `{"protein_pct": 14}` [breadcode-1326; yt:-60DxtcJ9Fo 5:31; creator-rule]
- He quotes about 13.3 g protein for the einkorn flour he used. — `{"protein_g": 13.3}` [breadcode-1947; yt:PehhnV1lW2o 2:04; creator-rule] *(low confidence)*
- A long, low-inoculum fermentation needs a strong flour with high protein, because flour weakens in contact with water over a long time. [breadcode-2220; yt:ngcISK7ZDjc 4:46; creator-rule]
- Use a high-protein flour, about 12 % protein, for a pizza dough so it builds a strong gluten network. — `{"flour_protein_pct": 12}` [breadcode-2271; yt:-rwuxy5KS6Q 1:50; creator-rule]
- The creator says that in wheat doughs most of the protein is gluten, and that the more gluten a flour has, the longer the dough can ferment, because lactic acid bacteria eat gluten. Weaker flour therefore suits a stiffer starter. — `{"gluten_share_of_protein_pct": 80}` [breadcode-2412; yt:WOfTMskT44Y description; creator-rule] *(low confidence)*
- A stiff starter works with inexpensive low-protein flour. [breadcode-2519; yt:MqH3GVfjfBc 5:24; creator-rule]
- For more tang with a stiff starter, use a higher-protein flour, because longer bacterial fermentation needs gluten strength to hold. [breadcode-2520; yt:MqH3GVfjfBc 7:49; creator-rule]
- A liquid starter needs a strong, high-gluten flour to survive long ferments; a stiff starter works with weaker flour. [breadcode-2530; yt:nPSsWa4nGgU 4:20; creator-rule]
- How much a sample rises depends on the flour's protein content (he does not give numbers here). [breadcode-2569; yt:gMbZeUIVzZY 4:49; creator-rule]
- Use high-protein bread flour for pizza: more protein builds a stronger gluten network that traps more gas for a fluffier crust. [breadcode-2715; yt:ziMMCphF054 0:00; creator-rule]
- The flour for the 92 % hydration test is described as 14 % protein; the transcript's '40 %' is an ASR error. — `{"protein_pct": 14}` [breadcode-2940; yt:nbw-p6_L36w description; anecdote]

### flour.rye

- Whole wheat flour can replace any amount of bread flour and gives a more complex, nutty taste. [breadcode-0724; SF Ch 9 Mix-ins §9.2 Examples > Flours; creator-rule]
- Rye flour adds a hearty, nutty and malty taste. [breadcode-0725; SF Ch 9 Mix-ins §9.2 Examples > Flours; creator-rule]
- Rye doughs stay flat because pentosans (arabinoxylan and beta-glucan) keep the dough from forming a gluten network that traps CO2 as in wheat. [breadcode-0971; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; literature]
- Rye in the starter makes the dough stickier, so expect more sticking with a rye starter at high hydration. [breadcode-2142; yt:4VJQqOtwP_E [1:06]-[1:40]; creator-rule]
- Rye dough develops almost no gluten; mixing into a homogeneous mass is enough, with no kneading. [breadcode-2584; yt:nsyjVa5SSQI 0:40; creator-rule]

### flour.single-source

- Single-source flour varies year to year with growing conditions; a very hot season can lower yield and kernel fill, which changes baking quality. [breadcode-1279; yt:wdnX3baMB38 38:27; creator-rule]
- A flour that behaves differently from last season is usually handled by adjusting process (proof time, water temperature, fermentation), not by changing the recipe. [breadcode-1280; yt:wdnX3baMB38 46:26; creator-rule]

### flour.spelt

- Spelt, like wheat with more than about 70 % wheat, needs homogenising, kneading, stretch and fold, and shaping. — `{"wheat_pct_min": 70}` [breadcode-0255; SF Ch 5 Flour types; creator-rule]
- Spelt sits between ancient grains and modern wheat: it develops strong gluten and rises well. [breadcode-1284; yt:wdnX3baMB38 48:47; creator-rule]

### flour.turmeric-addition

- Adding about 1% turmeric changes the colour of the crumb and gives a slight taste; the creator says it can be left out. — `{"turmeric_pct": 1}` [breadcode-1899; yt:xmt3eXzOQLM 3:02; anecdote] *(low confidence)*

### flour.type-classification

- Flour names describe which layers of the seed remain; the whitest flour is mostly the starch part of the seed. [breadcode-0250; SF Ch 5 Flour types; creator-rule]
- Across countries, the same flour name maps to different types: e.g. T405 (DE), T45 (FR) and 00 (IT) are the white-flour grades in each system; bread flour can be T405/T550, T45/T55 or 00/0. [breadcode-0253; SF Ch 5 Flour types; creator-rule]

### flour.w-value

- The Italian W-value measures a dough's resistance to kneading; higher W means more gluten, more elasticity and a longer fermentation. [breadcode-0265; SF Ch 5 Flour types; creator-rule]
- A high-W flour resists microbial inflation, so it needs a long fermentation; the long fermentation also lets microbes add flavour. [breadcode-0266; SF Ch 5 Flour types; creator-rule]
- W 0–150: hydration about 50 %, very short fermentation, used for cookies. — `{"w_value": [0, 150], "hydration_pct": 50}` [breadcode-0267; SF Ch 5 Flour types; creator-rule]
- W 150–250: hydration 50–60 %, short to medium fermentation, used for cakes, bread and pizza. — `{"w_value": [150, 250], "hydration_pct": [50, 60]}` [breadcode-0268; SF Ch 5 Flour types; creator-rule]
- W 250–350: hydration 60–70 %, long fermentation, used for bread and pizza. — `{"w_value": [250, 350], "hydration_pct": [60, 70]}` [breadcode-0269; SF Ch 5 Flour types; creator-rule]
- W 350+: hydration 70–90 %, very long fermentation, used for bread and pizza. — `{"w_value_min": 350, "hydration_pct": [70, 90]}` [breadcode-0270; SF Ch 5 Flour types; creator-rule]

### flour.whole-grain

- Adding 10 to 20 % whole-wheat flour (baker's %, flour = 100 %) to a white-flour dough significantly improves its enzymatic activity; this is the author's practice. — `{"whole_wheat_pct_of_flour": [10, 20]}` [breadcode-0045; SF Ch 2 How sourdough works; creator-rule]
- Whole-grain flour is the whole seed ground into smaller pieces; whiter flours have larger hull layers removed by mills and sieves. [breadcode-0249; SF Ch 5 Flour types; creator-rule]
- Doughs with more whole-wheat flour need a longer soak: bran absorbs water quickly, which activates enzymes and leaves less free water for gluten bonding. [breadcode-0471; SF Ch 7 Wheat sourdough §7.8 Dough strength > Whole-wheat soaking; creator-rule]
- With whole-wheat doughs, either wait longer before proceeding or add slightly more water. [breadcode-0472; SF Ch 7 Wheat sourdough §7.8 Dough strength > Whole-wheat soaking; creator-rule]
- A sourdough loaf made with half whole-rye flour and half rye berries gives a hearty, tangy flavour and a moist crumb; the berries are typically boiled 10 min to soften, and a thermometer is advised to judge doneness. — `{"whole_rye_flour_pct": 50, "rye_berries_pct": 50, "boil_min": 10}` [breadcode-0756; SF Ch 9 Mix-ins §9.2 Examples > Seeds and nuts (Whole-rye figure); creator-rule]

### flour.whole-grain-variants

- The 3-minute method works with whole wheat, spelt, einkorn, emmer and rye. For whole wheat, use a smaller size increase before baking; rye dough is stickier than wheat dough. [breadcode-1906; yt:bi-zqG5y1I8 4:35; 14:57; 15:55; creator-rule] *(low confidence)*
