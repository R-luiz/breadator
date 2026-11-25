# bread-type — The Bread Code / The Sourdough Framework

Claims: 102 (book 24, videos 65, repos 13). Evidence: creator-rule 82, anecdote 16, measured 3, extrapolation 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Claims

### bread-type

- Bread types are classified by baking technique: flatbread (stove), loaf pan (oven), free-standing (oven). [breadcode-0275; SF Ch 6 Bread types; creator-rule]
- Broa de milho (Portuguese) is made from half rye and half corn flour. — `{"rye_flour_pct": 50, "corn_flour_pct": 50}` [breadcode-0729; SF Ch 9 Mix-ins §9.2 Examples > Flours; creator-rule]
- Bolo do caco (Madeira) uses a 50/50 split of wheat flour and mashed potato. — `{"wheat_flour_pct": 50, "mashed_potato_pct": 50}` [breadcode-0731; SF Ch 9 Mix-ins §9.2 Examples > Liquids; creator-rule]
- Stollen is a traditional German sweet Christmas bread with candied lemon, candied orange and raisins; the mix-ins are soaked in rum before being added to the dough. [breadcode-0747; SF Ch 9 Mix-ins §9.2 Examples > Seeds and nuts (Stollen caption); creator-rule]
- Stollen matures after baking for up to 6 months, during which the candied ingredients release aroma into the crumb. — `{"maturation_months_max": 6}` [breadcode-0748; SF Ch 9 Mix-ins §9.2 Examples > Seeds and nuts (Stollen caption); creator-rule]
- In one experiment the author merged wheat, rye, spelt and einkorn doughs into a single dough, which came out layered with different colours, textures and flavours. [breadcode-0755; SF Ch 9 Mix-ins §9.3 Techniques > Swirled colors (footnote); anecdote] *(low confidence)*
- Rye's low gluten content makes pure rye breads dense, so rye is often combined with wheat; pure rye breads are usually made with a sourdough process. [breadcode-0875; SF Glossary > Rye; creator-rule]
- Dough at about 60 % hydration can be shaped into bread, buns, pizzas and baguettes. — `{"hydration_pct": 60}` [breadcode-0961; SF Ch 12 Troubleshooting §12.2.3 What's a good level of water (hydration); creator-rule]
- For an everyday sourdough sandwich loaf he uses 750 g of flour for two loaves, mostly bread flour (about 80 %) with about 20 % whole wheat, rye or spelt, for a fluffier crumb. — `{"total_flour_g": 750, "loaves": 2, "bread_flour_pct": 80, "other_flour_pct": 20}` [breadcode-1435; yt:WxFE3bxrTSM 29:11; creator-rule]
- Use 60–80 g of tomato sauce per pizza and apply it quickly, because wet dough becomes hard to load. — `{"sauce_g": [60, 80]}` [breadcode-1458; yt:l0nAMbbYziI 25:10; 25:43; creator-rule]
- Semolina flour works well as a peel dusting; use slightly more than seems needed so the pizza slides off. [breadcode-1459; yt:l0nAMbbYziI 20:28; creator-rule]
- For German-style rye and for emmer or einkorn breads, starter acidity matters little, because open crumb and oven spring are not the goal. [breadcode-1588; yt:yYkTrGHNW2w 28:31; creator-rule]
- Sourdough croissant dough uses about 25 % milk and 25 % water, 10 % sugar, 10 % butter, 10 % starter and 2 % salt, all on flour weight. — `{"milk_pct": 25, "water_pct": 25, "sugar_pct": 10, "butter_pct": 10, "starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-1621; yt:dY9GeoZX8DY description; creator-rule]
- Sequential iterations at 80 % hydration: iteration 1 over-fermented and went flat; iteration 2 had a crumb that was not fluffy enough. — `{"hydration_pct": 80}` [breadcode-1708; yt:2DFvfRIywC8 1:11 (iteration 1); 1:46 (iter; anecdote] *(low confidence)*
- A true sourdough bread contains only flour, water and salt; baking powder or added yeast means it is not real sourdough. [breadcode-1762; yt:9fWzWpL5g6w 4:50; creator-rule]
- Starter dough can be baked in a covered pan on medium heat without preheating; the creator recommends a lid so the steam stays in. [breadcode-2324; yt:sB81OLY25Jo 8:38; creator-rule]
- In a pan on a single heat source, a wheat starter bake cooks mainly in the centre, so it has to be rotated several times. [breadcode-2325; yt:sB81OLY25Jo 9:51; anecdote] *(low confidence)*
- Rye starter is sticky, so only a little water should be used. [breadcode-2326; yt:sB81OLY25Jo 6:01; anecdote] *(low confidence)*
- Thicker starter versions were preferred in the creator's tasting. [breadcode-2327; yt:sB81OLY25Jo 13:50; anecdote] *(low confidence)*
- A discard cracker dough can rest about 24 h at room temperature before baking. — `{"hours": 24}` [breadcode-2328; yt:urHYzIktD9M 5:44; creator-rule]

### bread-type.barbecue

- Loading a soft, sticky dough onto a hot barbecue with a pizza peel failed; loading the dough by hand and pulling it on the hot surface to round it worked. [breadcode-2187; yt:EbO3blOpGks 13:53; creator-rule]

### bread-type.cake

- Banana bread: 2–3 soft bananas, 150 g sugar, 125 g butter, 5 ml vanilla, 2 beaten eggs, 1 tbsp yogurt, 250 g wheat flour, ½ tsp salt, 1 tsp bicarbonate of soda. — `{"sugar_g": 150, "butter_g": 125, "vanilla_ml": 5, "yogurt_tbsp": 1, "flour_g": 250, "salt_tsp": 0.5, "bicarb_tsp": 1, "bananas": [2, 3]}` [breadcode-1192; repo:the-bread-code recipes/sweet/banana-bread.md; creator-rule]
- Banana bread ratios: sugar about 60 % and butter about 50 % of flour weight. — `{"sugar_pct": 60, "butter_pct": 50}` [breadcode-1193; repo:the-bread-code recipes/sweet/banana-bread.md; extrapolation]
- Fluffier banana bread uses 2–3 tsp bicarbonate of soda instead of 1 tsp. — `{"bicarb_tsp": [2, 3]}` [breadcode-1195; repo:the-bread-code recipes/sweet/banana-bread.md; creator-rule] *(low confidence)*
- Date-walnut loaf: 100 g walnuts and 100 g dates added to the dough. — `{"walnut_g": 100, "date_g": 100}` [breadcode-1196; repo:the-bread-code recipes/sweet/date-walnut-bread.md; creator-rule]

### bread-type.enriched

- Burger buns: per 600 g flour, about 45 % milk, 10 % sourdough, 10 % butter, 10 % sugar, 2 % salt and 2 % dry yeast (or 6 % fresh yeast). — `{"flour_g": 600, "milk_pct": 45, "sourdough_pct": 10, "butter_pct": 10, "sugar_pct": 10, "salt_pct": 2, "dry_yeast_pct": 2, "fresh_yeast_pct": 6}` [breadcode-1155; repo:the-bread-code recipes/savory/burger-buns.md; creator-rule]
- Full-milk chocolate loaf: 100 g full-milk chocolate, melted and stirred fully into the dough. — `{"chocolate_g": 100}` [breadcode-1200; repo:the-bread-code recipes/sweet/full-milk-chocolate-bread.md; creator-rule]
- Frozen-dough formula: 350 g bread flour and 50 g whole wheat (400 g flour), 300 g water (75 %), 40 g starter (10 %) and 8 g salt (2 %). — `{"flour_g": 400, "water_g": 300, "hydration_pct": 75, "starter_g": 40, "starter_pct": 10, "salt_g": 8, "salt_pct": 2}` [breadcode-2479; yt:Typy5Uzp-EM description; creator-rule]
- Sourdough brioche bun formula (700 g flour): milk 55 % of flour, butter 20 %, sugar 10 %, salt 2 %, sourdough 10 %, dry yeast 2 % (or fresh yeast 6 %), and 1 egg plus 1 yolk per 600 g flour. — `{"flour_g": 700, "milk_pct": 55, "butter_pct": 20, "sugar_pct": 10, "salt_pct": 2, "sourdough_pct": 10, "yeast_dry_pct": 2, "yeast_fresh_pct": 6}` [breadcode-2545; yt:Dx7xM8IThCI description; creator-rule]
- Alternative ratio for these brioche buns: one egg per 400 g flour instead of one per 600 g; the dough is a bit stickier. — `{"egg_per_flour_g": 400}` [breadcode-2560; yt:Dx7xM8IThCI description; creator-rule]

### bread-type.flatbread

- Flatbread needs no oven, only a stove; it is the least effort bread type. [breadcode-0276; SF Ch 6 Bread types; creator-rule]
- Flatbread can use any flour, including gluten-free corn or rice flour; adding a little wheat flour makes it fluffier because gluten traps gas. [breadcode-0277; SF Ch 6 Bread types; creator-rule]
- Flatbread recipe ratios (baker's %): flour 100 %, water up to 100 %, starter 5–20 %, salt 2 %. — `{"water_pct": [0, 100], "starter_pct_of_flour": [5, 20], "salt_pct": 2}` [breadcode-0279; SF Ch 6 Bread types; creator-rule]
- Pancake ratios (baker's %): flour 100 %, water 300 %, starter 5–20 %, salt 2 %. — `{"water_pct": 300, "starter_pct_of_flour": [5, 20], "salt_pct": 2}` [breadcode-0280; SF Ch 6 Bread types; creator-rule]
- Flatbread is ready when the dough has risen by at least 50 %; pancake batter is ready when bubbles are visible on the surface. — `{"rise_pct": 50}` [breadcode-0281; SF Ch 6 Bread types; creator-rule]
- Flatbread should be about 1 cm thick; pancakes about 0.1–0.5 cm, depending on preference. — `{"thickness_cm": 1}` [breadcode-0297; SF Ch 6 Bread types; creator-rule]
- Fermentation for flatbread takes 4–24 h depending on temperature and starter activity. — `{"bulk_h": [4, 24]}` [breadcode-0301; SF Ch 6 Bread types; creator-rule]
- Flatbread: pan, fire or barbecue cooking, working time about 3 min, all flours, very easy, low cost. — `{"working_min": 3}` [breadcode-0318; SF Ch 6 Bread types; creator-rule]
- Naan for one piece: 75 g flour, 45 g water and 1.5 g salt (60 % hydration, 2 % salt); no yeast. — `{"flour_g": 75, "water_g": 45, "salt_g": 1.5, "hydration_pct": 60, "salt_pct": 2}` [breadcode-2442; yt:sH99_IGh0ic description; creator-rule]
- Roti is the whole-grain version of naan: use wholemeal flour with the same method; the water level may need adjusting. [breadcode-2448; yt:sH99_IGh0ic description; creator-rule]

### bread-type.freestanding

- For free-standing sourdough aim for higher protein; low gluten makes the loaf collapse faster, so a loaf pan, skillet or flatbread is easier. [breadcode-0271; SF Ch 5 Flour types; creator-rule]
- Free-standing bread requires gluten flour, an oven, about 60 min working time, and is the most difficult and costly type. — `{"working_min": 60}` [breadcode-0317; SF Ch 6 Bread types; creator-rule]

### bread-type.fried

- Deep-fried sourdough formula: 450 g bread flour + 50 g wholewheat (500 g flour), 375 g water (75 %), 100 g stiff starter, 10 g salt (2 %). — `{"total_flour_g": 500, "water_pct": 75, "salt_pct": 2}` [breadcode-2603; yt:rzakvUe7eIE description; creator-rule]
- He aimed for about 175 °C (≈350 °F) oil for frying, with about 5 L of oil; the oil took about 30 min to reach temperature. — `{"fry_c": 175, "oil_l": 5}` [breadcode-2605; yt:rzakvUe7eIE 1:57; creator-rule] *(low confidence)*
- A whole loaf fried about 15 min browned very dark, and its doneness was hard to judge from outside. — `{"fry_min": 15}` [breadcode-2606; yt:rzakvUe7eIE 5:08; anecdote] *(low confidence)*

### bread-type.laminated

- Sourdough croissant dough: 400 g bread flour, 100 g milk, 100 g water, 40 g butter, 40 g sugar, 40 g starter, 8 g salt, 4 g malt extract and about 1 g yeast (baker's % 100/25/25/10/10/10/2/1/0.25). — `{"flour_g": 400, "milk_pct": 25, "water_pct": 25, "butter_pct": 10, "sugar_pct": 10, "starter_pct_of_flour": 10, "salt_pct": 2, "malt_extract_pct": 1, "yeast_pc` [breadcode-2827; yt:7wyikR8qwHQ description; creator-rule]
- Lamination: one 4-fold after encasing the butter, chill about 1 h wrapped in foil until the butter is cold, then a 2-fold (8 layers), chill, then a 4-fold (32 layers). — `{"chill_h": 1, "fold_sequence": [4, 2, 4], "layers_final": 32}` [breadcode-2830; yt:7wyikR8qwHQ description; creator-rule]

### bread-type.lean-hearth

- A wheat dough with degraded gluten flattens during baking and loses the typical fluffy crumb structure. [breadcode-0182; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- In his test, the boule with a tight pre-shape had an even crumb, while the batard was too wild; he thinks the batard needed a longer fermentation. [breadcode-2233; yt:iEfLO_Cl0uE 11:22; anecdote] *(low confidence)*
- Shaping-video formula: 100 % all-purpose wheat flour, 70 % hydration, 2 % salt and 40 % sourdough (160 g per 400 g flour). — `{"hydration_pct": 70, "salt_pct": 2, "starter_pct_of_flour": 40, "flour_g": 400, "starter_g": 160}` [breadcode-2282; yt:5--bR1mPiZE description; creator-rule]
- Base dough for two loaves: 1000 g flour, 600–800 g water, 20 g salt (2 %) and 100 g stiff starter (10 %); without starter use 1 g dry yeast (0.1 %). — `{"flour_g": 1000, "water_g": [600, 800], "salt_g": 20, "salt_pct": 2, "starter_g": 100, "starter_pct": 10, "yeast_g_alt": 1, "yeast_pct_alt": 0.1}` [breadcode-2497; yt:0h63OloQQd8 description; creator-rule]
- The 30 s pre-bake also made scoring easy for baguettes and rolls that are proofed at room temperature, in one trial. — `{"pre_bake_s": 30}` [breadcode-2510; yt:0h63OloQQd8 8:28; anecdote] *(low confidence)*
- Overnight 50/50 bread flour and whole wheat at about 81 % total hydration, with 10 % starter, gave an open crumb (his result). — `{"whole_wheat_pct": 50, "hydration_pct": 81, "starter_pct_of_flour": 10}` [breadcode-2897; yt:MyGMF5sLg1U description; anecdote]

### bread-type.loaf-pan

- Loaf pan bread: oven, working time about 5 min, all flours, easy, medium cost. — `{"working_min": 5}` [breadcode-0319; SF Ch 6 Bread types; creator-rule]

### bread-type.other

- Broa de milho (Portuguese corn bread): 250 g corn flour, 250 g rye flour, 350 g water (70 %), 100 g sourdough (20 %), 10 g salt (2 %). — `{"corn_flour_g": 250, "rye_flour_g": 250, "water_pct": 70, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-2608; yt:bjLCHRInmLY description; creator-rule]
- Fresh sourdough pasta dough: 200 g flour, 100 g water (or 2 medium eggs), 40 g stiff starter and 2 g salt, about 50 % hydration. — `{"flour_g": 200, "water_g": 100, "starter_g": 40, "salt_g": 2, "hydration_pct": 50, "salt_pct": 1}` [breadcode-2783; yt:A5blYerC8VE description; creator-rule]

### bread-type.pan

- Sourdough sandwich loaf: 500 g dough = 200 g leaven + 300 g white flour, with the water set so the dough is about 65 % hydration (leaven water discounted); salt about 1.2 % of 500 g (6 g). — `{"leaven_g": 200, "white_flour_g": 300, "hydration_pct": 65, "salt_g": 6, "dough_g": 500}` [breadcode-1187; repo:the-bread-code recipes/sourdough/sourdough-sandwich.md; creator-rule]
- Dough can ferment and proof entirely in a loaf pan without stretch-and-folds or shaping, giving a sandwich loaf. [breadcode-1335; yt:013Qw3BojM4 2:55; measured]
- If the dough is too sticky to build strength, proof it in a loaf pan and bake it in the pan. [breadcode-1673; yt:wHL44ONu3so 31:34; creator-rule]
- In a loaf pan the pan supports the dough, so a sandwich loaf is forgiving: a very wet mix is easy to stir and starter readiness matters less. [breadcode-2764; yt:Rv1AnZ4rxHs 2:48; creator-rule] *(low confidence)*
- In a metal loaf pan, greasing with oil released the emmer-spelt sandwich loaf cleanly, while parchment stuck; butter also works. — `{"hydration_pct": 70}` [breadcode-2860; yt:4j5PGepJSKM 1:23; measured]
- Two-minute sourdough sandwich loaf: 600 g flour, 300 g water (50 %), 60 g stiff starter (10 %), 8 g salt (2 %). Dissolve starter in water, add flour and salt, mix until no dry flour, place in an oiled pan and proof until doubled. — `{"flour_g": 600, "water_g": 300, "water_pct": 50, "starter_g": 60, "starter_pct": 10, "salt_g": 8, "salt_pct": 2}` [breadcode-2879; yt:l5FtMycJna4 description; creator-rule]

### bread-type.pasta

- Sourdough pasta: 75 % semolina and 25 % starter by mass, 42 % hydration. — `{"semolina_pct": 75, "starter_pct": 25, "hydration_pct": 42}` [breadcode-1173; repo:the-bread-code recipes/sourdough/sourdough-pasta.md; creator-rule]
- Sourdough pasta 250 g batch: 188 g semolina, 125 g starter, 42.5 g water. — `{"semolina_g": 188, "starter_g": 125, "water_g": 42.5}` [breadcode-1174; repo:the-bread-code recipes/sourdough/sourdough-pasta.md; creator-rule]
- Sourdough pasta: knead about 15 min, then rest about 1 h refrigerated. — `{"knead_min": 15, "rest_h": 1}` [breadcode-1175; repo:the-bread-code recipes/sourdough/sourdough-pasta.md; creator-rule]
- Cook fresh sourdough pasta 2–3 min, or dried pasta 6–11 min, in 2 L of boiling water. — `{"cook_fresh_min": [2, 3], "cook_dried_min": [6, 11], "water_l": 2}` [breadcode-1176; repo:the-bread-code recipes/sourdough/sourdough-pasta.md; creator-rule]

### bread-type.pizza

- A long, slow fermentation of Neapolitan pizza dough lets protease make the dough extensible and easy to inflate, which gives a soft, airy edge. [breadcode-0038; SF Ch 2 How sourdough works; creator-rule]
- Traditional sourdough pizza dough has a working window of about 30 to 90 minutes before it deteriorates, due to continued protease activity and bacterial by-products; yeast-leavened commercial dough stays workable longer. — `{"window_min": [30, 90]}` [breadcode-0040; SF Ch 2 How sourdough works; creator-rule]
- Neapolitan pizza sauce: about 1 g salt per 100 g canned San Marzano tomatoes, crushed with a fork; nothing else added. — `{"salt_g_per_100g_tomato": 1}` [breadcode-1349; yt:-60DxtcJ9Fo 20:23; creator-rule]
- Use about 50 to 60 g of tomato sauce for one Neapolitan-style pizza. — `{"tomato_sauce_g": [50, 60]}` [breadcode-1350; yt:-60DxtcJ9Fo 24:58; creator-rule]
- One-hour Neapolitan dough: 142 g bread or all-purpose flour, 85 g warm water (60 %), 3 g salt, 3 g dry yeast (or 9 g fresh), 9 g sugar; the ball is about 230 g. — `{"hydration_pct": 60, "salt_pct": 2.1, "dry_yeast_pct": 2.1, "sugar_pct": 6.3, "ball_g": 230}` [breadcode-2259; yt:xLYxPneBx-M description; creator-rule]
- In an informal taste comparison, the creator found his one-hour dough very similar to his usual 24-hour overnight dough; in the video he says the 24-hour version tasted better. [breadcode-2260; yt:xLYxPneBx-M description; 10:48; measured] *(low confidence)*
- For a sourdough pizza dough, use 5 to 10 % sourdough starter by flour weight and let it ferment overnight. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-2280; yt:-rwuxy5KS6Q 7:53; creator-rule]
- The one-hour pizza dough reaches about 80 % of the quality of a slowly fermented dough; slow fermentation is the key to the best flavour. [breadcode-2281; yt:-rwuxy5KS6Q 12:22; 13:36; creator-rule] *(low confidence)*
- A sourdough-leavened pizza dough takes about 8 to 10 hours to be ready, so the creator uses yeast for same-day dough. — `{"ferment_h": [8, 10]}` [breadcode-2289; yt:-rwuxy5KS6Q 0:36; creator-rule]
- For a larger home oven, a final Neapolitan dough ball of about 250 g is typical; the one-hour recipe targets about 230 g for a standard home oven. — `{"ball_g": 250}` [breadcode-2304; yt:xLYxPneBx-M description; 7:37; creator-rule]
- For a cast-iron Neapolitan-style pizza he uses dough balls of about 200 g instead of his usual ~250 g, so the base fits the pan. — `{"dough_ball_g": 200, "usual_dough_ball_g": 250}` [breadcode-2433; yt:HSEdU6_fscs 1:39; creator-rule]
- Olive oil followed by semolina, spread evenly in the cast iron, stops the pizza sticking. [breadcode-2434; yt:HSEdU6_fscs 1:39; creator-rule]
- Charring the base on the stovetop at maximum heat, rotating the pan about every 30 s, for about three minutes (stove-dependent) gives an even charred base. — `{"stove_char_min": 3, "rotate_every_s": 30}` [breadcode-2435; yt:HSEdU6_fscs 3:24; creator-rule]
- Adding the mozzarella only after the stovetop charring, so it melts in the oven rather than on the stove, gave better cheese than adding it before the charring. [breadcode-2437; yt:HSEdU6_fscs 5:04; anecdote]
- Too much tomato sauce and olive oil made the pizza leak in the cast iron; less of both was needed. [breadcode-2438; yt:HSEdU6_fscs 5:04; anecdote]
- A blowtorch on the rim after about four minutes of stovetop charring produced the leopard spotting of Neapolitan pizza; he judged the added flavour small. — `{"stove_char_min": 4}` [breadcode-2439; yt:HSEdU6_fscs 6:53; anecdote]
- Cast-iron stovetop-then-oven pizza gave a better base than his earlier bakes on a steel or a stone. [breadcode-2440; yt:HSEdU6_fscs 8:00; anecdote]
- In a cast-iron pizza, keep the centre slightly thicker: a very thin centre showed through and had to be stitched. [breadcode-2441; yt:HSEdU6_fscs 2:50; anecdote]

### bread-type.rye

- Non-wheat sourdough uses non-wheat flour (all except spelt): mix, bulk ferment to the desired acidity, shape or pan, short proof, bake. [breadcode-0323; SF Ch 8 Non wheat sourdough; creator-rule]
- Wetter rye dough evaporates more water in the bake, giving a more open crumb; rye is never as fluffy as wheat sourdough. [breadcode-0343; SF Ch 8 Non wheat sourdough; creator-rule]
- Franconian bread: 450 g rye and 50 g wheat (90 % rye of 500 g) with spices — 1 tbsp each of anise, caraway, fennel and coriander (1 tsp light, 2 tbsp strong). — `{"rye_flour_g": 450, "wheat_flour_g": 50, "rye_pct": 90, "each_spice_tbsp": 1}` [breadcode-1163; repo:the-bread-code recipes/sourdough/franconian-bread.md; creator-rule]
- Broa (Portuguese corn bread): 200 g rye and 200 g corn, 80 g starter (20 %), 280 g water (70 %), 8 g salt (2 %). — `{"rye_flour_g": 200, "corn_flour_g": 200, "starter_g": 80, "starter_pct": 20, "water_pct": 70, "salt_pct": 2}` [breadcode-1170; repo:the-bread-code recipes/sourdough/portuguese-broa-corn-bread.md; creator-rule]
- Per the creator, the low-gluten rye-blend dough is easier to inflate than the wheat dough because it has less gluten; he also remarked that one sample showed no large air pockets (sample not clearly identified). [breadcode-1342; yt:013Qw3BojM4 12:01; anecdote] *(low confidence)*
- Rye, emmer and corn breads need less protein strength for long fermentation and are usually baked in a loaf pan. [breadcode-2221; yt:ngcISK7ZDjc 5:20; creator-rule]
- In wheat loaves with rye, keep rye to about 20 % of the flour to keep a good rise. — `{"rye_pct_of_flour_max": 20}` [breadcode-2246; yt:oAFjGTWMgUo 1:49; creator-rule]
- Rye doughs need no gluten development: mix and stir. [breadcode-2247; yt:oAFjGTWMgUo 6:08; creator-rule]
- Rye cannot build the gluten network needed for an open crumb, so a rye loaf stays tight and dense. [breadcode-2265; yt:-LeUYBe7HlY 3:48; creator-rule]
- Schwarzbrot formula: 400 g sourdough starter (discard is fine), 400 g whole rye kernels, 200 g whole rye flour, 200 g water and 8 g salt (2 % of flour). — `{"starter_g": 400, "rye_kernels_g": 400, "rye_flour_g": 200, "water_g": 200, "salt_g": 8, "salt_pct": 2}` [breadcode-2455; yt:cHwLBs9SRd4 description; creator-rule]
- 100 % rye sourdough (Roggen-Sauerteigbrot): 500 g rye flour, 350 g water (70 %), 100 g rye sourdough (20 %), 10 g salt (2 %); mixed, not kneaded. — `{"rye_flour_g": 500, "water_pct": 70, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-2583; yt:nsyjVa5SSQI description; creator-rule]
- Mischbrot (50 % rye, 50 % wheat): 250 g rye flour, 250 g wheat flour, 100 g sourdough (20 %), 325 g warm water (65 %), 10 g salt (2 %). — `{"rye_pct": 50, "wheat_pct": 50, "starter_pct_of_flour": 20, "water_pct": 65, "salt_pct": 2}` [breadcode-2592; yt:l9g4AI_z3uY description; creator-rule]
- Mixing rye with wheat gives a more open crumb than 100 % rye, which is denser. [breadcode-2593; yt:l9g4AI_z3uY description; creator-rule]
- For a seed-heavy rye loaf, set the formula on the soaked seed mass: 80 % wholegrain rye flour, 95 % water, 20 % sourdough and 2 % salt. — `{"rye_flour_pct": 80, "water_pct": 95, "starter_pct": 20, "salt_pct": 2}` [breadcode-2766; yt:cN_skGXZ4Es description; creator-rule]
- For rye bread, dusting with the loaf's own flour does not work because rye is very sticky; he uses wheat or spelt flour instead. [breadcode-2812; yt:g2Wx3JSiPvo 1:21; creator-rule]
- Mischbrot formula: 250 g whole rye, 250 g whole wheat, 350 g water, 100 g rye sourdough and 10 g salt (50/50 rye and wheat). — `{"rye_pct": 50, "wheat_pct": 50, "water_pct": 70, "starter_pct_of_flour": 20, "salt_pct": 2, "total_flour_g": 500}` [breadcode-2840; yt:di4QatuNB7Y description; creator-rule]
- Mischbrot has a fairly dense crumb, less dense than all-rye sourdough, and a hearty flavour. [breadcode-2841; yt:di4QatuNB7Y description; creator-rule]

### bread-type.yeast-only

- In the creator's informal taste panel, the yeast-only and mild-sourdough breads tasted similar, and friends could not tell a major difference between them; the liquid-starter bread stood out as the most flavourful. [breadcode-2293; yt:bJxpCVc2wYE 11:46; anecdote] *(low confidence)*
