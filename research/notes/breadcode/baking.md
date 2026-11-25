# baking — The Bread Code / The Sourdough Framework

Claims: 468 (book 139, videos 306, repos 23). Evidence: creator-rule 378, measured 48, anecdote 32, literature 5, extrapolation 5.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

- **Stages (Table 10.1):** microbes die ≈60 °C; surface gel ≈75 °C (needed for oven spring); water evaporates 100 °C; Maillard 105–120 °C once the surface is dry; acetic acid boils ≈118 °C; caramelisation ≈170 °C.
- **Standard bake:** preheat 230 °C for 30 min; bake ≈30 min with steam (Dutch oven, inverted tray or steam oven), then remove steam and bake ≥10 min (5-min increments) to colour. Done at ≈92 °C core (book; pan and rye loaves too); videos use 92–95 °C and the old repo 95 °C. Pan loaves ≈200 °C, free-standing ≈230 °C. Longer baking evaporates more acid → less sour loaf.
- **Steam is measured** (book Ch 10 plots, apple-probe surface temperature, n = 1 per arm): at 5 min the surface was 72 °C with an ice cube in a preheated Dutch oven vs 47 °C spritzed, 43 °C plain preheated, 23 °C bottom-only, 17 °C unpreheated; an unpreheated pot needed ≈26 min to reach 75 °C. Table 10.2 maps oven types to methods (gas or always-on fan → Dutch oven only).
- Video experiments (NrKHO2xZTEU, 9yI7QRpAgow, NAjBY7S_u04, 0h63OloQQd8, P59OfgJ4Dps) test spritzing, half-preheat, ice cube vs spritz and "bake 30 s then score" — see claims below.

## Claims

### baking

- Steaming the dough in the oven gives oven spring; the second baking stage builds the crust. [breadcode-0353; SF Ch 7 Wheat sourdough §7.1 The process; creator-rule]
- During baking the dough's water and acids evaporate, gas expands, and the loaf rises (oven spring) while the outer dough sets into a gel-like crust. [breadcode-0647; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Microbes in the dough are killed at about 60 °C (140 °F). — `{"temp_c": 60}` [breadcode-0648; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- At about 75 °C the dough surface forms a gel that holds structure, stays extensible and retains gas; this gel is needed for oven spring. — `{"temp_c": 75}` [breadcode-0650; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Water starts evaporating from the dough at about 100 °C; higher hydration leaves more water in the baked crumb. — `{"temp_c": 100}` [breadcode-0651; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Maillard browning starts once the surface has dried, roughly at 105–120 °C, and speeds up as the crust gets hotter. — `{"temp_c_range": [105, 120]}` [breadcode-0657; SF Ch 10 Baking §10.1 The process of baking; literature]
- Caramelization begins at about 170 °C; crust colour is then mainly a function of bake time. — `{"temp_c": 170}` [breadcode-0658; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- A loaf can be returned to the oven later to darken the crust further, so building less crust than wanted is the safer choice. [breadcode-0659; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Doneness is best judged by core temperature: about 92 °C (197 °F) at the core means the loaf can come out. — `{"core_c": 92}` [breadcode-0660; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Taking loaves out at 92 °C core can leave the crust underbaked, since the crust is not yet built at that point. — `{"core_c": 92}` [breadcode-0661; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Judging doneness by the outside is hard, especially in large loaf pans; author repeatedly ended with semi-baked loaves before using a thermometer. [breadcode-0662; SF Ch 10 Baking §10.1 The process of baking; anecdote]
- Fully baked bread is effectively sterile, because baking heat kills the microorganisms. [breadcode-0663; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Steam delays early crust formation, so the dough keeps expanding as its water evaporates in the first stage of the bake. [breadcode-0665; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- The aim is to keep crust from forming until expansion stops, which happens once most microbes die and the evaporating water can no longer be held in gas cells. [breadcode-0666; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Steam condenses on the dough surface and heats it quickly to about 75 °C, forming the gel that keeps the dough extensible. — `{"temp_c": 75}` [breadcode-0668; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Without steam the surface skips the gel stage and goes straight to the Maillard zone, which caps oven spring. [breadcode-0669; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Keep the dough in the gel stage as long as possible for maximum expansion; the author checks this by removing the loaf after about 5 minutes. — `{"gel_check_min": 5}` [breadcode-0670; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Too little steam produces large air pockets just under the crust, because the crust stops expansion and gas pockets merge. [breadcode-0672; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Too high a bake temperature gives the same large air pockets under the crust as too little steam. [breadcode-0673; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Steam softens the crust and blocks Maillard and caramelization, so the steam source is removed for the second stage of the bake. [breadcode-0674; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Steaming the whole bake gives a soft crust; the author notes this is possible if a soft crust is wanted. [breadcode-0675; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- General workflow: preheat the oven to 230 °C for 30 min, score the dough, choose a steaming method (inverted tray, Dutch oven or steam oven), then bake about 30 min with steam. — `{"oven_c": 230, "preheat_min": 30, "steam_bake_min": 30}` [breadcode-0676; SF Ch 10 Baking §10.3 Building up steam; creator-rule]
- After the steamed phase, remove the steam source and bake a further 10–30 min depending on desired crust colour. — `{"finish_min": [10, 30]}` [breadcode-0677; SF Ch 10 Baking §10.3 Building up steam; creator-rule]
- A tray of rocks plus a covering tray greatly improves steam in a home oven and lets the bread rise more in the first stage. [breadcode-0678; SF Ch 10 Baking §10.3 Building up steam; creator-rule]
- A Dutch oven is not fully sealed, but the evaporating water fills it with steam, which lets the dough rise further and makes home steaming easy. [breadcode-0679; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- Preheat the Dutch oven properly so the dough does not stick; semolina or parchment paper also help. — `{"preheat_min": 30, "oven_c": 230}` [breadcode-0680; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- Spritzing the dough with water before covering the Dutch oven adds surface steam. [breadcode-0681; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- A small ice cube placed beside the dough adds extra steam in a Dutch oven. [breadcode-0682; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- Dutch-oven flowchart: preheat 30 min at 230 °C, load and score, spritz, close, and bake 30 min covered at 230 °C. — `{"oven_c": 230, "preheat_min": 30, "covered_bake_min": 30}` [breadcode-0683; SF figures/fig-dutch-oven-process.tex; creator-rule]
- Dutch-oven decision rule: when the core reads 92 °C, remove the lid and bake 5 min at a time until the crust is dark enough; if the core is below 92 °C, wait 5 min and check again. — `{"core_c": 92, "wait_min": 5, "extra_uncovered_min": 5}` [breadcode-0684; SF figures/fig-dutch-oven-process.tex; creator-rule]
- A Dutch oven holds one loaf at a time, limiting output per bake. — `{"pieces": 1}` [breadcode-0685; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- Baking several loaves in one bake saves kneading, energy and a separate preheat per loaf. [breadcode-0686; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- Cast-iron Dutch ovens can weigh up to about 10 kg, are hot and dangerous to handle, need heat-resistant gloves, and need occasional seasoning. — `{"max_weight_kg": 10}` [breadcode-0687; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- The inverted tray method works like a Dutch oven: a second tray placed upside down over the dough traps steam from the dough and a water source. [breadcode-0688; SF Ch 10 Baking §10.3 Building up steam > Inverted tray method; creator-rule]
- Equipment for the inverted tray method: two trays, a heat-resistant bowl, boiling water, oven gloves, and optionally parchment paper. [breadcode-0689; SF Ch 10 Baking §10.3 Building up steam > Inverted tray method; creator-rule]
- Inverted tray method: preheat the oven to 230 °C and preheat one tray at the same time. — `{"oven_c": 230}` [breadcode-0690; SF Ch 10 Baking §10.3 Building up steam > Inverted tray method; creator-rule]
- Inverted tray method: load loaves onto the hot tray (on parchment or semolina), cover with the cold tray upside down, and place a bowl of boiling water in the oven for steam; rocks are optional. [breadcode-0691; SF Ch 10 Baking §10.3 Building up steam > Inverted tray method; creator-rule]
- Inverted tray method: after 30 min remove the top tray and water bowl, then bake about 15–25 min more for the desired crust colour. — `{"steam_bake_min": 30, "finish_min": [15, 25]}` [breadcode-0692; SF Ch 10 Baking §10.3 Building up steam > Inverted tray method; creator-rule]
- Inverted tray flowchart: bake 30 min or until core reaches 92 °C, remove steam source and top tray, then bake at least 10 more min or until crust is desired colour. — `{"steam_bake_min": 30, "core_c": 92, "finish_min": [10, null]}` [breadcode-0693; SF figures/fig-inverted-tray-method.tex; creator-rule]
- Author's own inverted-tray batch was about six loaves (two freestanding, four in loaf pans); generalized here, not a recommended batch size. — `{"pieces": 6}` [breadcode-0694; SF Ch 10 Baking §10.3 Building up steam > Inverted tray method; creator-rule]
- Most home ovens vent steam and are not sealed, which dries out bread; roasting vegetables suits that, bread needs as much steam as possible. [breadcode-0695; SF Ch 10 Baking §10.4 Conclusions; creator-rule]
- In gas ovens and convection ovens whose fan cannot be turned off, a Dutch oven is the only steaming method listed. [breadcode-0696; SF Ch 10 Baking §10.4 Conclusions; creator-rule]
- In a convection oven whose fan can be disabled, the inverted tray method is a cost-efficient option. [breadcode-0697; SF Ch 10 Baking §10.4 Conclusions; creator-rule]
- In a steam oven, activate steam; an extra tray over the dough gives indirect heat, keeps the dough in the gel zone longer and gives more oven spring. [breadcode-0698; SF Ch 10 Baking §10.4 Conclusions; creator-rule]
- Oven matrix: plain ovens do not steam; inverted tray works in steam ovens and fan-off convection; Dutch oven works in gas, fan-on convection, fan-off convection and steam ovens. [breadcode-0699; SF Table 10.2; creator-rule]
- Ice cube in a preheated Dutch oven: apple surface 13 °C at 1 min, 66 °C at 3 min, 86 °C at 10 min, 99 °C at 20 min; reached 75 °C at about 6 min. — `{"surface_c_by_min": {"1": 13, "3": 66, "10": 86, "20": 99}, "time_to_75c_min": 6}` [breadcode-0700; SF Ch 10 Baking §10.1, Figure: surface temperature; measured]
- Plain preheated Dutch oven: apple surface 16 °C at 2 min, 51 °C at 6 min, 74 °C at 10 min, 85 °C at 13–15 min (series ends at 85 °C). — `{"surface_c_by_min": {"2": 16, "6": 51, "10": 74, "15": 85}}` [breadcode-0701; SF Ch 10 Baking §10.1, Figure: surface temperature; measured]
- Non-preheated Dutch oven: apple surface 26 °C at 10 min and 75 °C at 26 min, still rising at the end of the run. — `{"surface_c_by_min": {"10": 26, "26": 75}}` [breadcode-0702; SF Ch 10 Baking §10.1, Figure: surface temperature; measured]
- Preheated-bottom curve: apple surface 0 °C at 1 min, 42 °C at 9 min, 74 °C at 17 min, 100 °C at 26 min. — `{"surface_c_by_min": {"9": 42, "17": 74, "26": 100}}` [breadcode-0703; SF Ch 10 Baking §10.1, Figure: surface temperature; measured] *(low confidence)*
- Spritzed preheated Dutch oven: apple surface 13 °C at 2 min, 40 °C at 4 min, 74 °C at 10 min, 100 °C at 28 min, 103 °C at 35 min. — `{"surface_c_by_min": {"2": 13, "4": 40, "10": 74, "28": 100, "35": 103}}` [breadcode-0704; SF Ch 10 Baking §10.1, Figure: surface temperature; measured]
- At 5 min the apple surface was 72 °C with an ice cube, 47 °C spritzed, 43 °C plain preheated, 23 °C bottom-only preheated and 17 °C without preheat. — `{"surface_c_at_5min": {"ice_cube": 72, "spritz": 47, "preheated": 43, "preheated_bottom": 23, "non_preheated": 17}}` [breadcode-0705; SF Ch 10 Baking §10.1, Figure: surface temperature; measured]
- The ice cube raised apple surface temperature fastest among the variants; at 10 min it was about 12 °C above plain preheat (86 °C vs 74 °C). — `{"surface_c_delta_at_10min": 12}` [breadcode-0706; SF Ch 10 Baking §10.1, Figure: surface temperature; measured]
- Interior air with ice cube in a preheated Dutch oven: 95 °C at 1 min, 165 °C at 3 min, and a plateau near 182 °C from about 8 min to the end of the run. — `{"ambient_c_by_min": {"1": 95, "3": 165}, "ambient_plateau_c": 182}` [breadcode-0707; SF Ch 10 Baking §10.1, Figure: ambient temperature; measured]
- Interior air in a plain preheated Dutch oven: 110 °C at 1 min, 181 °C at 3 min, then 188–190 °C from about 5 to 15 min. — `{"ambient_c_by_min": {"1": 110, "3": 181}, "ambient_plateau_c": 189}` [breadcode-0708; SF Ch 10 Baking §10.1, Figure: ambient temperature; measured]
- Interior air with spritzing: 31 °C at 1 min, 168 °C at 3 min, plateau about 189–190 °C from about 5 to 35 min. — `{"ambient_c_by_min": {"1": 31, "3": 168}, "ambient_plateau_c": 190}` [breadcode-0709; SF Ch 10 Baking §10.1, Figure: ambient temperature; measured]
- Interior air with no preheat: 22 °C at 1 min, 77 °C at 10 min, 151 °C at 25 min, with no plateau in the run. — `{"ambient_c_by_min": {"1": 22, "10": 77, "25": 151}}` [breadcode-0710; SF Ch 10 Baking §10.1, Figure: ambient temperature; measured]
- Derived from the tabulated curves: a preheated Dutch oven reached near-plateau interior air in about 3–5 min, versus about 25 min to reach 151 °C without preheat. — `{"time_to_plateau_min_preheated": [3, 5], "time_to_151c_min_no_preheat": 25}` [breadcode-0711; SF Ch 10 Baking §10.1, Figure: ambient temperature; extrapolation] *(low confidence)*
- Chunks can bake differently: cheese melts and leaves holes, and vegetables can char where they show through the crust. [breadcode-0722; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- In the first phase of baking the dough's internal temperature rises and yeast activity increases, producing rapid carbon dioxide that causes oven spring. [breadcode-0809; SF Glossary > Baking; creator-rule]
- As baking temperature climbs, the dough's proteins, mainly gluten, coagulate and set, which gives the loaf its structure. [breadcode-0810; SF Glossary > Baking; creator-rule]
- Crust browning comes from caramelization of sugars and the Maillard reaction between amino acids and reducing sugars; this also creates flavour and aroma. [breadcode-0812; SF Glossary > Baking; creator-rule]
- Some acids produced in fermentation evaporate during baking; a longer bake concentrates the remaining acids less and lowers the tang. [breadcode-0813; SF Glossary > Baking; creator-rule]
- Water in the dough turns to steam and evaporates during baking; the steam adds to oven spring and helps gelatinize starch. [breadcode-0814; SF Glossary > Baking; creator-rule]
- The outer layer dries and hardens into a crust, which acts as a barrier that keeps the inner crumb moist. [breadcode-0815; SF Glossary > Baking; creator-rule]
- A Dutch oven, a heavy lidded pot, traps steam during the early phase of baking, which helps create a crusty exterior. [breadcode-0833; SF Glossary > Dutch Oven; creator-rule]
- The Maillard reaction is a browning reaction between reducing sugars and amino acids that produces many tastes and aromas depending on reactants and conditions. [breadcode-0851; SF Glossary > Maillard Reaction; creator-rule]
- Maillard reactions occur readily above 150 °C and still occur, more slowly, below that temperature. — `{"temp_c": [150, null]}` [breadcode-0852; SF Glossary > Maillard Reaction; creator-rule]
- The optimal Maillard reaction rate is at pH 6.0–8.0, and the reaction favours alkaline conditions. — `{"ph": [6.0, 8.0]}` [breadcode-0853; SF Glossary > Maillard Reaction; creator-rule]
- Oven spring is the rapid rise of dough in the early oven phase, caused by expansion of trapped gases and water. [breadcode-0859; SF Glossary > Oven Spring; creator-rule]
- After about 30 min at about 230 °C the crust is still pale and only part of the water and acetic acid has left; continued baking drives more acetic acid off with the steam. — `{"oven_c": 230, "bake_min": 30}` [breadcode-0990; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Stopping the bake after about 30 min typically leaves a core temperature around 95 °C. — `{"core_c": 95, "bake_min": 30}` [breadcode-0991; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- The crust darkens and gains aroma (Maillard reaction) only when the loaf is baked again after cooling. [breadcode-0992; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Adding baking powder neutralises lactic acid and gives a much milder dough (author's unconventional approach). [breadcode-1003; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; literature] *(low confidence)*
- Crust colour forms in the second stage of baking, after the oven's steam source is removed: the Maillard reaction, then caramelisation, each giving a different aroma. [breadcode-1013; SF Ch 12 Troubleshooting §12.3 Bread > My crust becomes chewy; creator-rule]
- A too-sour over-fermented loaf can be improved by baking it longer to boil off acidity. [breadcode-1035; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Baking too hot constrains expansion: starch sets early and the crust closes, so alveoli merge into larger structures near the surface and the crumb is less fluffy. [breadcode-1064; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked too; creator-rule]
- Baking too cold with a high-hydration, extensible dough makes it flatten; starch gelatinisation is needed to hold the structure. [breadcode-1066; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked too; creator-rule]
- The author's sweet spot for maximum oven spring is about 230 °C (446 °F). — `{"oven_c": 230}` [breadcode-1067; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked too; creator-rule]
- Displayed oven temperature may differ from the actual oven temperature; test the oven. [breadcode-1068; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked too; literature]
- Around 230 °C gives a nicely coloured crust while the interior still bakes fluffy; too hot browns the crust before the centre is done, too cool gives a pale, less crisp crust. — `{"bake_c": 230}` [breadcode-1106; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Bread is done when the centre of the thickest part reaches about 95 °C. — `{"core_c": 95}` [breadcode-1107; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- A hollow sound when the loaf is knocked suggests gas bubbles inside; it is a secondary check next to crust colour and core temperature. [breadcode-1108; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Score at an angle rather than vertically: vertical cuts can block each other and rise less, angled cuts open freely and rise more. [breadcode-1112; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Score a large X in the centre of the surface; cuts too far to the sides make the dough spread like a pancake. [breadcode-1113; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Create steam for the first half of the bake, then remove the water tray (or the dutch oven lid) at about 50 % of the bake time to get a thin crust overall. — `{"steam_removal_pct_of_bake": 50}` [breadcode-1114; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Boil the water before putting it in the oven tray for maximum steam. [breadcode-1115; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- A crust that is too thick usually means too little early steam or too high a temperature; a very strong oven may need a lower setting. [breadcode-1116; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- A covered vessel (dutch oven or a pizza stone with a large lid) traps moisture from the dough, keeps the crust soft early on, and gives an even heat; no parchment is needed. [breadcode-1131; repo:the-bread-code basics/tools.md; creator-rule]
- A pizza stone stores a lot of heat and gives it directly to the dough on contact, which produces a strong bottom crust. [breadcode-1132; repo:the-bread-code basics/tools.md; creator-rule]
- The maximum oven temperature needed is about 230 °C, so an oven reaching 250 °C is ideal. — `{"oven_needed_max_c": 230, "oven_ideal_max_c": 250}` [breadcode-1133; repo:the-bread-code basics/tools.md; creator-rule]
- A loaf pan oiled with oil stuck less than parchment-lined; the creator preferred the oiled pan. [breadcode-1138; repo:the-bread-code experiments/parchment-paper-vs-oiled-loaf-pan.md; measured]
- Vertical X scoring gave less rise and a pancake-like loaf; angled X scoring gave a taller loaf with the same crumb. [breadcode-1140; repo:the-bread-code experiments/scoring-vertically-vs-angle.md; measured]
- Basic bread bake: preheat to 230 °C, steam for 20–25 min covered, then about 20 min uncovered. — `{"bake_c": 230, "steam_min": [20, 25], "uncovered_min": 20}` [breadcode-1153; repo:the-bread-code recipes/savory/basic-bread.md; creator-rule]
- Basic bread: score at 45° with about 1 cm depth. — `{"score_angle_deg": 45, "score_depth_cm": 1}` [breadcode-1154; repo:the-bread-code recipes/savory/basic-bread.md; creator-rule]
- Burger buns: spray the buns and the oven every 2 min for the first 10 min, preheat to 230 °C for at least 10 min, bake about 20 min. — `{"bake_c": 230, "preheat_min": 10, "bake_min": 20, "spray_interval_min": 2, "spray_window_min": 10}` [breadcode-1157; repo:the-bread-code recipes/savory/burger-buns.md; creator-rule]
- Basic sourdough bake: preheat at least 30 min to 230 °C, bake 30 min covered and then 15–20 min uncovered; the core should reach 95 °C. — `{"bake_c": 230, "preheat_min": 30, "covered_min": 30, "uncovered_min": [15, 20], "core_c": 95}` [breadcode-1162; repo:the-bread-code recipes/sourdough/basic-sour-dough.md; creator-rule]
- Mischbrot bake: 25 min covered and 20 min uncovered at 230 °C, then rest 30 min before cutting. — `{"bake_covered_min": 25, "bake_uncovered_min": 20, "bake_c": 230, "rest_min": 30}` [breadcode-1167; repo:the-bread-code recipes/sourdough/mischbrot-aka-graubrot.md; creator-rule]
- Sourdough sandwich bake: 200 °C (400 °F) for 45 min with a 1–2 cm score down the centre; do not exceed 450 °F (about 230 °C) because nonstick pans degrade. — `{"bake_c": 200, "bake_min": 45, "score_depth_cm": [1, 2], "max_c": 230}` [breadcode-1190; repo:the-bread-code recipes/sourdough/sourdough-sandwich.md; creator-rule]
- Spray the loaf in a dry climate to keep the surface from drying. [breadcode-1191; repo:the-bread-code recipes/sourdough/sourdough-sandwich.md; creator-rule] *(low confidence)*
- Banana bread bakes at 150 °C with air circulation for 1 h in a 20 cm tin. — `{"bake_c": 150, "bake_min": 60, "tin_cm": 20}` [breadcode-1194; repo:the-bread-code recipes/sweet/banana-bread.md; creator-rule]
- A faster bake leaves a chocolate loaf sweeter. [breadcode-1203; repo:the-bread-code recipes/sweet/full-milk-chocolate-bread.md; creator-rule] *(low confidence)*
- A stone oven at about 450 °C works for pizza; in a home oven preheat to maximum, use the grill to brown the top, and the edge browns first. — `{"stone_oven_c": 450}` [breadcode-1216; repo:pizza-dough README.md; creator-rule]
- Pouring a large volume of boiling water over hot rocks in one motion produces much more steam than slow kettle pouring. [breadcode-1235; yt:MrOM_D3hnpc 1:40:47; anecdote]
- A fan running during the first bake dries the surface and forms a crust too early; if the oven cannot turn the fan off, bake in a covered loaf pan or Dutch oven. [breadcode-1236; yt:MrOM_D3hnpc 1:46:16; creator-rule]
- Running the fan at the end of baking, after the loaf has risen, gives a quicker dark crust. [breadcode-1237; yt:MrOM_D3hnpc 1:51:45; creator-rule]
- Bake with the cover on about 30 minutes, then remove it; removing the parchment later and turning the loaf gives an even crust. [breadcode-1244; yt:MrOM_D3hnpc 2:05:20; creator-rule]
- Spraying water on the dough loosens its surface and makes it spread; pouring water on hot rocks works better. [breadcode-1245; yt:MrOM_D3hnpc 2:07:17; anecdote] *(low confidence)*
- Preheating the oven matters, even more for wet dough: in his bake comparison the preheated oven gave much more oven spring. He bakes in a preheated Dutch oven (32:37). [breadcode-1252; yt:ImVvQMvGZKE 2:54; measured]
- Bake covered at 250 °C (upper and lower heat) for the first 25 min to maximise oven spring, then remove the lid and lower the temperature to 230 °C. — `{"temp_covered_c": 250, "covered_min": 25, "temp_uncovered_c": 230}` [breadcode-1253; yt:ImVvQMvGZKE 33:13; creator-rule]
- In his side-by-side test, baking with a hotter oven at the start of the bake gave more oven spring than a constant temperature. [breadcode-1262; yt:ImVvQMvGZKE 2:54; measured]
- Bake with steam for about 30 minutes, then without steam for about 15 minutes, and judge the crust by colour. [breadcode-1269; yt:MrOM_D3hnpc 1:49:58; creator-rule]
- Bake at 230 °C as the default; 250 °C was briefly used to heat the stone and then lowered. [breadcode-1270; yt:MrOM_D3hnpc 1:45:42; creator-rule]
- Use about 2 % salt on flour weight (EU baker's-math recommendation). [breadcode-1271; yt:MrOM_D3hnpc 38:47; creator-rule]
- Bake with steam at 230 °C for about 25 min, then dry-bake 15 to 20 min for colour; a core of about 92 °C indicates done. — `{"oven_c": 230, "steam_min": 25, "dry_min": [15, 20], "core_c": 92}` [breadcode-1310; yt:KRO9EcIyXSI 40:31; creator-rule]
- Par-bake: steam-bake 25 min, cool, refrigerate, then finish at 230 °C for about 15 min when needed. — `{"oven_c": 230, "finish_min": 15}` [breadcode-1311; yt:KRO9EcIyXSI 41:06; creator-rule]
- A second tray or bowl placed above the loaf shields it from top heat and traps steam, which helps oven spring. [breadcode-1312; yt:KRO9EcIyXSI 45:27; creator-rule]
- A rack under a preheated tray keeps it hot enough to give bottom heat at the start of a bake. [breadcode-1313; yt:KRO9EcIyXSI 46:43; creator-rule]
- Dust semolina on top of the dough, not the tray, to stop sticking. [breadcode-1314; yt:KRO9EcIyXSI 47:17; creator-rule]
- Neapolitan-style pizza bakes at about 500 °C for about 90 s; a home oven at about 250 °C needs the base pre-baked before toppings. — `{"oven_c": [250, 500], "bake_s": 90}` [breadcode-1331; yt:-60DxtcJ9Fo 20:58; 21:35; creator-rule]
- A cast-iron pan heated on the stove and then finished in a hot oven gives good pizza in a home oven. [breadcode-1332; yt:-60DxtcJ9Fo 22:07; creator-rule] *(low confidence)*
- Grease the loaf pan with nonstick vegetable oil so the dough does not stick; he later said he needed more of it. [breadcode-1337; yt:013Qw3BojM4 5:55; creator-rule]
- Bake a loaf-pan wheat loaf at 230 °C until the core reaches 95 °C; no oven preheat is needed in this method. — `{"oven_c": 230, "core_c": 95}` [breadcode-1339; yt:013Qw3BojM4 15:02; creator-rule]
- Remove the loaf-pan lid partway through to get colour on top. [breadcode-1340; yt:013Qw3BojM4 17:30; creator-rule]
- In a home oven with a broiler, set the oven to maximum and finish the pizza close to the broiler for about 60 to 90 seconds. — `{"broil_s": [60, 90]}` [breadcode-1351; yt:-60DxtcJ9Fo 27:27; creator-rule]
- Preheat the oven for 30 minutes; turn off the fan once the dough is in because the fan removes steam. — `{"preheat_min": 30}` [breadcode-1375; yt:o_xtVmR4vlc 24:44; creator-rule]
- Bake to a core temperature of about 92–95 °C; he removes the steam source at roughly the halfway point, then bakes longer for crust colour. — `{"core_c": [92, 95]}` [breadcode-1376; yt:o_xtVmR4vlc 27:39; creator-rule]
- Bake at 230 °C with top and bottom heat, no fan, on a preheated stone with a steam tray: about 20 min covered (steam tray in), then about 20 min uncovered (tray removed). — `{"oven_c": 230, "covered_min": 20, "uncovered_min": 20}` [breadcode-1393; yt:NMglhwp2lNs 36:48-37:27; creator-rule]
- Bake the loaf at about 230 °C; higher oven heat forms the crust too early and stops the dough rising. — `{"oven_c": 230}` [breadcode-1410; yt:msqU-ylXWUs 23:07; creator-rule]
- Keep steam in for the first stage of the bake (about 30 minutes, until the core reaches 92 °C), then remove the steam source (top tray and water). — `{"core_c": 92, "steam_stage_min": 30}` [breadcode-1411; yt:msqU-ylXWUs 22:00; creator-rule]
- The second, uncovered stage of the bake can last 10 to 30 minutes, depending on how dark the crust should be; skipping it gives a very soft crust. — `{"uncovered_stage_min": [10, 30]}` [breadcode-1412; yt:msqU-ylXWUs 22:00; creator-rule]
- In the demonstrated loaf the total oven time was about 50 minutes (30 minutes with steam plus 20 minutes more for colour). — `{"bake_total_min": 50}` [breadcode-1413; yt:msqU-ylXWUs 27:31; anecdote]
- A baking stone or Dutch oven may give slightly better oven spring and crumb, roughly 1–2 %; a tray is fine. — `{"gain_pct": [1, 2]}` [breadcode-1414; yt:msqU-ylXWUs 23:38; anecdote] *(low confidence)*
- Preheat the oven for about 30 min before baking, with the stone and steam tray in place. — `{"preheat_min": 30}` [breadcode-1423; yt:NMglhwp2lNs 34:52; creator-rule]
- A Neapolitan pizza should bake for at most about 90 seconds so the toppings stay partly raw. — `{"bake_max_s": 90}` [breadcode-1452; yt:l0nAMbbYziI 18:44; 19:17; creator-rule]
- In a home oven, preheat to maximum for at least 30 minutes and use the broiler if available. — `{"preheat_min": 30}` [breadcode-1453; yt:l0nAMbbYziI 18:44; creator-rule]
- A stone Neapolitan oven runs around 450 °C with strong top heat; a home broiler approximates that top heat. — `{"temp_c": 450}` [breadcode-1454; yt:l0nAMbbYziI 19:17; creator-rule]
- A steel plate transfers heat to the base faster than a stone and gave darker base spotting in his bake. [breadcode-1455; yt:l0nAMbbYziI 29:04; 29:36; measured]
- A gas-fired pizza oven produced leopard spotting but needed timed turning every ~12 s after an initial 20 s; he found it hard to operate. [breadcode-1456; yt:l0nAMbbYziI 29:04; 30:45; anecdote] *(low confidence)*
- Steam is needed in the first half of the bake; without it a crust sets early and limits oven spring. [breadcode-1476; yt:qM9BXKaIr60 32:36; creator-rule]
- Crumb looked about equally open in the autolysed and non-autolysed loaves; the author saw more vertical oven spring in one loaf and said shaping may explain it. [breadcode-1479; yt:JZBtmQXKWA8 25:15; 26:33; 27:12; 27:48; measured] *(low confidence)*
- Preheat a baking stone for about 30 minutes at the oven's maximum temperature before baking ciabatta (his oven reaches about 270 °C). — `{"preheat_min": 30, "temp_c": 270}` [breadcode-1497; yt:HSdOUZq5i6U 20:29; creator-rule]
- Bake ciabatta at 230 °C: about 10 minutes with steam and a top tray for indirect heat, then remove the steam and tray and bake about 10 minutes more. — `{"oven_c": 230, "steam_min": 10, "bake_min": 20}` [breadcode-1498; yt:HSdOUZq5i6U 21:05; 21:39; 22:12; creator-rule]
- Measure the real oven temperature with a separate thermometer; the oven display can differ from actual temperature. [breadcode-1511; yt:HLPNdyGCSPk 16:24; 16:57; creator-rule]
- Bake at 230 °C with a water spritz for 25 minutes, then remove the water and bake about 15 minutes more for colour. — `{"oven_c": 230, "steam_min": 25, "extra_min": 15}` [breadcode-1512; yt:HLPNdyGCSPk 17:30; 18:40; 20:22; creator-rule]
- A core reading above about 95 °C was taken as done on one loaf; the author then baked about 15 more minutes only for crust colour. — `{"core_c": 95, "extra_min": 15}` [breadcode-1526; yt:HLPNdyGCSPk 19:14; 20:22; creator-rule]
- The oven was preheated to its maximum setting so the loaf gets more oven spring at the start of the bake; semolina on top keeps the loaf from sticking to the peel. [breadcode-1534; yt:JZBtmQXKWA8 23:36; creator-rule] *(low confidence)*
- The author bakes the sourdough on a stone at 230 °C with steam for 25 minutes, then removes the steam to finish the crust. — `{"oven_c": 230, "steam_min": 25}` [breadcode-1542; yt:LQPCXMpxO4c 18:05; 18:42; creator-rule]
- Bake at 230 °C with steam for the first 25 minutes, then remove the steam to colour the crust. — `{"oven_c": 230, "steam_min": 25}` [breadcode-1561; yt:36GdGT0XYwE 27:12; creator-rule]
- Simulate a Dutch oven: put a heat-resistant metal vessel with a few rocks on the stone, add boiling water to it, and place a tray above to shield the dough from the top elements; turn the oven fan off so the steam is not blown away. [breadcode-1562; yt:36GdGT0XYwE 23:58; creator-rule] *(low confidence)*
- Score the loaf at about 45 degrees. — `{"score_angle_deg": 45}` [breadcode-1563; yt:36GdGT0XYwE 26:24; creator-rule]
- Cold dough taken straight from the fridge is easier to score and bake. [breadcode-1587; yt:yYkTrGHNW2w 26:48; creator-rule]
- Preheat the cast-iron Dutch oven to about 260 °C (500 °F), then lower the oven to about 216–220 °C (425 °F) after loading the loaf. — `{"preheat_c": 260, "bake_c": [216, 220]}` [breadcode-1591; yt:nQWlG1buMM8 3:58; anecdote]
- A lower bake temperature is reported to give a thinner crust, but the speakers did not verify this. [breadcode-1592; yt:nQWlG1buMM8 3:26; anecdote] *(low confidence)*
- A few ice cubes in a covered pot add a little extra steam; spraying water was less effective in this experience. [breadcode-1595; yt:nQWlG1buMM8 9:07; anecdote] *(low confidence)*
- Blisters on the crust were more common with an unfloured loaf in a linen-lined banneton. [breadcode-1597; yt:nQWlG1buMM8 11:23; anecdote] *(low confidence)*
- Technique matters more than equipment; a kitchen scale is the one tool needed, since changing one parameter at a time shows its effect. [breadcode-1605; yt:0kFYyj3WKpE 1:13; creator-rule]
- Preheat the oven at its maximum temperature for about 15 minutes; then bake at 230 °C with the fan off, covering the loaf with a second tray and adding boiling water after loading to trap steam. — `{"preheat_min": 15, "bake_oven_c": 230, "fan": "off"}` [breadcode-1616; yt:0kFYyj3WKpE 20:16; 23:18; creator-rule]
- Bake about 45 minutes at 230 °C, steaming for the first half, then remove the steam so the crust can colour. — `{"oven_c": 230, "bake_min": 45, "steam_min": 25}` [breadcode-1617; yt:0kFYyj3WKpE 24:08; 24:45; creator-rule]
- Score at about 45 degrees with a swift stroke; a cold dough from the fridge is easier to score. — `{"score_angle_deg": 45}` [breadcode-1618; yt:0kFYyj3WKpE 21:26; 22:02; creator-rule]
- Pure egg wash was chosen to test for a darker crust than half egg, half water; the creator compared the two side by side during the bake but did not state a clear colour difference. [breadcode-1633; yt:dY9GeoZX8DY 23:21; anecdote] *(low confidence)*
- Preheat the oven to maximum for about 20 minutes, then reduce to 200 °C and add boiling water to a tray below the bake for steam. — `{"preheat_min": 20, "oven_c": 200}` [breadcode-1634; yt:dY9GeoZX8DY 23:21; creator-rule]
- Chill the shaped dough in the freezer for about 30–45 minutes before scoring; a cooler surface tears less and gives a better ear. — `{"freezer_min": [30, 45]}` [breadcode-1643; yt:WOZLvsV1M8s 6:39–7:12; creator-rule]
- Preheat the oven to about 230 °C for 30 minutes with a tray or stone; then place the loaf on it and cover it with a preheated glass bowl to simulate a Dutch oven. A heavy metal pot does the same job and is safer. — `{"preheat_min": 30, "oven_c": 230}` [breadcode-1644; yt:WOZLvsV1M8s 8:18–9:24; creator-rule]
- A loaf is baked through when its core reaches at least 95 °C; the crust can then be finished later. — `{"core_c": 95}` [breadcode-1645; yt:WOZLvsV1M8s 10:31; creator-rule]
- Reheat a refrigerated loaf by placing it in a non-preheated oven, then heating at about 230 °C for roughly 15 minutes or until the colour is right. — `{"oven_c": 230, "reheat_min": 15}` [breadcode-1648; yt:WOZLvsV1M8s 11:38; creator-rule]
- Before scoring, bake the loaves with steam for about 30 seconds so the surface dries slightly, which makes scoring easier. — `{"pre_score_bake_s": 30}` [breadcode-1656; yt:36GdGT0XYwE 25:11; creator-rule]
- Use as much steam as possible during baking: without it a crust forms early and stops the loaf expanding. [breadcode-1704; yt:9fWzWpL5g6w 19:06; creator-rule]
- A fridge-proofed loaf can be baked 1 to 3 h late without major problems. — `{"bake_after_h": [1, 3]}` [breadcode-1716; yt:dl7crk2Nvj8 16:45; creator-rule]
- Don't trust the oven's dial: check with a thermometer. [breadcode-1732; yt:9yI7QRpAgow 10:55; creator-rule]
- In a side-by-side bake, the water-spritzed loaf had a better ear, a slightly taller loaf and more blisters than the unspritzed control; the crumb difference was judged small. [breadcode-1734; yt:9yI7QRpAgow 18:36–23:00; measured]
- The unspritzed loaf formed its crust quickly and rose less. [breadcode-1735; yt:9yI7QRpAgow 18:36; measured]
- Brushing water on versus spritzing made no major difference to blisters. [breadcode-1736; yt:9yI7QRpAgow 21:09; measured] *(low confidence)*
- Preheat the oven about 30 min before baking at 230 °C with steam. — `{"bake_c": 230, "preheat_min": 30}` [breadcode-1742; yt:vyDTKFtfIb0 17:48–18:24; creator-rule]
- The loaf is done at a core temperature of 95 °C (200 °F). — `{"core_c": 95}` [breadcode-1744; yt:vyDTKFtfIb0 20:14; creator-rule]
- Bake 25 min with steam (covered), then 20 min without steam, for a 110 % whole-wheat loaf in a Dutch oven. Oven temperature is not stated. — `{"bake_covered_min": 25, "bake_uncovered_min": 20}` [breadcode-1753; yt:2DFvfRIywC8 description (instructions step; creator-rule]
- A Dutch oven, or a lower bake temperature, can stand in for a tray-and-stone steam setup. [breadcode-1760; yt:9fWzWpL5g6w 19:42; creator-rule]
- Dutch-oven bake used: 25 minutes covered at 230 °C, then lid removed for 15–20 more minutes to colour the crust. — `{"oven_c": 230, "covered_min": 25, "uncovered_min": [15, 20]}` [breadcode-1763; yt:9yI7QRpAgow 10:55; 16:43; description; creator-rule]
- A covered Dutch oven traps steam so the crust sets more slowly and the loaf keeps rising; this is the main reason to bake in one. [breadcode-1766; yt:9yI7QRpAgow 12:12; creator-rule]
- Loaves can be part-baked (about half time), bagged and refrigerated, then finished later for fresh-baked bread. [breadcode-1767; yt:9yI7QRpAgow 14:05; creator-rule]
- Preheat the oven and Dutch oven for about 30 minutes at about 270 °C before baking. — `{"preheat_min": 30, "preheat_c": 270}` [breadcode-1780; yt:dl7crk2Nvj8 17:18; creator-rule]
- Bake at 230 °C with steam for the first 25 minutes, then remove the steam source and bake a further 15 to 20 minutes until the desired colour. — `{"bake_c": 230, "steam_min": 25, "dry_bake_min": [15, 20]}` [breadcode-1781; yt:dl7crk2Nvj8 18:25; creator-rule]
- Steam during the first part of the bake prevents an early crust so the dough can expand; the crust is developed after steam is removed. — `{"steam_min": 25}` [breadcode-1782; yt:dl7crk2Nvj8 17:51; creator-rule]
- Steam is essential for the first stage of the bake: boiling water poured into a preheated tray or bowl under the loaf (or a second loaf pan on top) creates a steamy environment. [breadcode-1799; yt:vyDTKFtfIb0 18:24–19:02; creator-rule]
- For a soft-crust sandwich loaf keep steam on for the whole bake; for a crustier loaf remove steam after about 30 minutes. — `{"steam_off_min": 30}` [breadcode-1800; yt:vyDTKFtfIb0 20:14; creator-rule]
- Bake at up to 230 °C with the Dutch oven lid on for 25 minutes, then remove the lid for crust colour. [breadcode-1815; yt:dQEebR6pIng 15:16; creator-rule]
- Bake at 230 °C with fan off and a tray of boiling water for steam. Just before loading, cut the cold dough with a razor and spritz it with water. — `{"oven_c": 230}` [breadcode-1827; yt:B_2SEmyyvso 16:43; creator-rule]
- On a stone, spray with steam every minute for the first 10 minutes. [breadcode-1833; yt:3vr2_qqeAL8 19:14; creator-rule]
- In a non-blinded single bake, the Dutch oven gave more vertical spring than the stone. [breadcode-1834; yt:3vr2_qqeAL8 20:24; measured] *(low confidence)*
- Score at 45 degrees with a sharp lame. [breadcode-1843; yt:BnvhhxQZQ14 14:13; creator-rule]
- Steam: preheat a large bowl about 30 min, pour boiling water under a tray, and keep it for about the first half of the bake (~20 min). Then remove the water and tray. — `{"steam_min": 20, "preheat_bowl_min": 30}` [breadcode-1844; yt:BnvhhxQZQ14 16:35; creator-rule] *(low confidence)*
- Load steam, then reduce the oven temperature and switch the fan off (fan would blow the steam away). Bake 25 min with steam, then remove the steam source. Finish the bake, about 10 min more. — `{"steam_bake_min": 25, "bake_total_extra_min": 10}` [breadcode-1845; yt:BnvhhxQZQ14 18:24; creator-rule]
- Cast-iron pan on the stove, then oven; semolina dusting and a broiler finish. [breadcode-1851; yt:rcOxoJk7goM 15:18; creator-rule]
- The fifth, largest layer was too big for the Dutch oven, so it was baked on a preheated stone with no cover; water was added for steam, and the creator says a covering tray would have trapped steam better. [breadcode-1852; yt:1Dbbfq5SKi0 17:40-18:56; creator-rule] *(low confidence)*
- A covered Dutch oven traps the steam from the dough and the added water, which he links to stronger oven spring and crust. [breadcode-1864; yt:3vr2_qqeAL8 20:24; creator-rule]
- Water sprayed on the dough in the first stage of the bake prevents an early crust so the loaf can keep expanding. [breadcode-1865; yt:3vr2_qqeAL8 18:38; creator-rule]
- Preheat a large bowl for about 30 min, then pour boiling water into it under a tray to trap steam for the first half of the bake. — `{"preheat_bowl_min": 30}` [breadcode-1866; yt:BnvhhxQZQ14 16:35; creator-rule]
- Bake the sandwich loaf at 230 °C until the core reaches 95 °C, which the creator estimates takes 30 to 50 min depending on the oven. — `{"oven_c": 230, "core_c": 95, "bake_min": [30, 50]}` [breadcode-1869; yt:B_2SEmyyvso description; creator-rule]
- Preheat the whole oven about 30 min, with the steam tray hot in place, before baking. — `{"preheat_oven_min": 30}` [breadcode-1871; yt:B_2SEmyyvso 16:43; creator-rule]
- In a cast-iron pan, bake the pizza on the stove at maximum heat until steam and a slight burnt smell appear, rotate it, then finish in a maximally heated oven and use the broiler for about 2 minutes to brown the top. — `{"broil_min": 2}` [breadcode-1875; yt:rcOxoJk7goM 15:18; 15:53; creator-rule]
- At the end of the bake, the creator checked the core of a bucket-dough loaf and read above 97 °C; he said baking longer would give more crust darkening. — `{"core_c": 97}` [breadcode-1944; yt:MetItcI3Yks 6:18; anecdote] *(low confidence)*
- Bake with steam for the first 30 min at 230 °C: boiling water is poured into a bowl preheated in the oven and covered by an additional tray to trap steam. — `{"oven_c": 230, "steam_min": 30}` [breadcode-1960; yt:agCpWuDgAwg 9:38; creator-rule]
- Remove the steam once the loaf core reaches about 80 °C; afterwards he planned about 10–15 more min of baking without steam. — `{"core_c": 80, "dry_bake_min": [10, 15]}` [breadcode-1961; yt:agCpWuDgAwg 9:38; creator-rule]
- A whole wheat loaf is done at a core temperature of about 95 °C. — `{"core_c": 95}` [breadcode-1962; yt:agCpWuDgAwg 11:18; creator-rule]
- Too little steam produces large air pockets near the top of the crust. [breadcode-1967; yt:HgIKSVMkbFg 12:40; creator-rule]
- Bake covered for 25 min at 230 °C, then uncovered for about 20 min. — `{"oven_c": 230, "covered_min": 25, "uncovered_min": 20}` [breadcode-1973; yt:APstLJpWOiE 14:13; creator-rule]
- A preheated Dutch oven gives more vertical oven spring and a better bottom than a cold oven with the same dough. — `{"preheat_min": 30}` [breadcode-1977; yt:lJ38fY3ZEQk 12:00; measured]
- In the cold-start arm the loaf was still puffy after 60 min in the Dutch oven; when moved onto a rag for bottom browning, its bottom burned and stuck. — `{"bake_min": 60}` [breadcode-1978; yt:lJ38fY3ZEQk 9:35; measured]
- In his small side-by-side bake, the dough with a ~20 g ice cube had far more crust blisters than the one with a ~70–80 g cube; he did not show a clear oven-spring difference. — `{"ice_g": [20, 80], "oven_c": 230}` [breadcode-1990; yt:0BA-Lka_QMI 13:32; measured] *(low confidence)*
- In his crumb reading, large air pockets near the top are typically a sign of too little steam, not too much; his large-ice-cube explanation was a hypothesis he did not confirm. [breadcode-1991; yt:0BA-Lka_QMI 14:44; creator-rule]
- Bake at 230 C for the first 25 min with steam from boiling water placed below a tray that shields the loaves from top heat. — `{"oven_c": 230, "steam_min": 25}` [breadcode-1999; yt:vf5RbwlW8Xs 10:37; creator-rule]
- Bake covered 25 min at 230 °C, then uncovered 20–30 min, until the dough core reaches about 92 °C. — `{"oven_c": 230, "covered_min": 25, "uncovered_min": [20, 30], "core_c": 92}` [breadcode-2003; yt:02WlZa_N-vI 8:48; creator-rule]
- Score a dome at about 45° to its surface; scoring off-centre on a dome needs a near-horizontal blade. — `{"angle_deg": 45}` [breadcode-2004; yt:qRX3rulyXS8 10:15; creator-rule]
- Score depth about 0.5 cm for a puffy high-hydration dough and about 1 cm for a stiffer dough. — `{"depth_cm": [0.5, 1]}` [breadcode-2005; yt:qRX3rulyXS8 12:47; creator-rule]
- For steam, preheat the Dutch oven about 30 min, then add boiling water to a heatproof dish placed on a lower rack under the loaf, with a second tray on top of the loaf to trap steam. — `{"preheat_min": 30}` [breadcode-2008; yt:02WlZa_N-vI 7:04; creator-rule]
- In his ice-cube Dutch-oven bake he baked about 25 minutes first, checked the crust, then baked about 15 minutes more. — `{"covered_min": 25, "further_min": 15, "oven_c": 230}` [breadcode-2009; yt:0BA-Lka_QMI 3:38; creator-rule]
- Ice cubes placed in a hot covered Dutch oven melt and boil slowly, releasing steam over the first part of the bake. [breadcode-2010; yt:0BA-Lka_QMI 1:14; creator-rule]
- In a covered vessel in an oven without steam, a spray of water into the vessel before closing adds humidity; the lid is removed for the second, uncovered phase to build crust. — `{"covered_min": 25}` [breadcode-2015; yt:APstLJpWOiE 13:38; creator-rule]
- He bakes his loaves at 230 °C throughout and does not change the temperature between the steamed and unsteamed stages. — `{"oven_c": 230}` [breadcode-2017; yt:HgIKSVMkbFg 7:21; creator-rule]
- Without a Dutch oven, steam comes from a bowl of boiling water under a shield tray. Without enough steam the loaf bursts in random places. — `{"preheat_min": 30}` [breadcode-2022; yt:lJ38fY3ZEQk 6:13; creator-rule]
- If you bake without a preheated Dutch oven, use parchment paper; a loaf without parchment stuck so badly it was almost impossible to remove. [breadcode-2023; yt:lJ38fY3ZEQk 16:50; creator-rule]
- Baguette scoring: use 3, 5 or 7 overlapping, nearly straight cuts depending on length, with the blade held as horizontal as possible so the cut opens under the surface instead of spreading. — `{"cuts_count": [3, 5, 7]}` [breadcode-2035; yt:qRX3rulyXS8 8:58; creator-rule]
- Preheat the oven about 15 min at 230 °C; bake with a second tray on top to trap steam, switch off the fan, bake 20–30 min. — `{"oven_c": 230, "preheat_min": 15, "bake_min": [20, 30]}` [breadcode-2050; yt:OcOpIISBK6c [8:32]; creator-rule]
- In a side-by-side bake, the spritzed loaf got more vertical spring than the non-spritzed one. [breadcode-2066; yt:NrKHO2xZTEU [0:38]; [6:11]; measured]
- An ice cube in the oven gave similar steam to spritzing, with slightly more spring, possibly technique-dependent. [breadcode-2067; yt:NrKHO2xZTEU [1:16]; measured] *(low confidence)*
- Browning starts near 140 °C (280 °F) and stops the loaf expanding; steam condensing on the surface delays this. — `{"temp_c": 140}` [breadcode-2068; yt:NrKHO2xZTEU [1:51]; extrapolation]
- Crumb comparison of spritzed vs non-spritzed bakes showed no major difference. [breadcode-2071; yt:NrKHO2xZTEU [6:50]–[8:05]; measured] *(low confidence)*
- Ice-cube test: surface reached about 70–80 °C after about 5 min, about 100 °C with more steam, and browning began near 120 °C. — `{"surface_c": [70, 100], "browning_c": 120, "time_min": 5}` [breadcode-2073; yt:NrKHO2xZTEU [10:29]–[12:18]; measured]
- Oven temperature left at about 270 °C after loading gave flat loaves with large holes; the creator links this to steam. — `{"oven_c": 270}` [breadcode-2081; yt:MPWypFY0wyE [13:41]–[15:35]; anecdote]
- Preheat 30 min at 270 °C then bake at 230 °C; steam from a clay bowl of boiling water for 15 min. — `{"preheat_c": 270, "preheat_min": 30, "oven_c": 230, "steam_min": 15}` [breadcode-2091; yt:gOxCUh_kyMQ [15:37]–[16:51]; creator-rule]
- A covered bake in a glass bowl, removed after about 25 min, worked as an improvised oven. — `{"bake_min": 25}` [breadcode-2127; yt:6x9ic4YIni8 [13:02]–[13:45]; anecdote] *(low confidence)*
- Bread shipped half-baked; the customer finishes baking about 10 min at home. — `{"home_bake_min": 10}` [breadcode-2133; yt:zfxOxu3_4bE [1:09]; anecdote]
- After about 30 min of baking the core reaches about 95 C; at that point most microorganisms are dead and the bread no longer ferments. — `{"core_c": 95, "bake_min": 30}` [breadcode-2136; yt:zfxOxu3_4bE [8:01]–[8:36]; creator-rule]
- Slice only after about 10 min of cooling. — `{"cool_min": 10}` [breadcode-2138; yt:zfxOxu3_4bE [7:24]; creator-rule]
- In the first improvised attempt the dough was baked on a tray with a pot used as a lid for 45 minutes, then cooled for about one hour before cutting. — `{"bake_min": 45, "cool_h": 1}` [breadcode-2143; yt:6x9ic4YIni8 [7:24]; anecdote] *(low confidence)*
- Do not use the oven fan for this bake; use top and bottom heat only, with boiling water in a heat-resistant bowl inside the oven to create steam. [breadcode-2146; yt:gOxCUh_kyMQ [16:15]; creator-rule]
- Preheating only the bottom of the Dutch oven (lid not preheated) gave a loaf with less ear and fewer colour differences than the fully preheated bake, with a similar crumb. — `{"oven_c": 230}` [breadcode-2150; yt:NrKHO2xZTEU [6:11]; [7:29]; measured]
- Bake time depends on the crust colour wanted: about 30 min gives his colour, 20–40 min is the range. — `{"bake_min": [20, 40], "bake_min_typical": 30}` [breadcode-2151; yt:OcOpIISBK6c [9:11]; creator-rule]
- The first part of the bake (steam and oven spring) is the difficult part; the second part (crust colouring) can be reproduced in any home oven. [breadcode-2153; yt:zfxOxu3_4bE [8:36]; creator-rule]
- Baking several doughs at once on one stone reduced oven spring; the creator recommends baking only one or two loaves at a time. [breadcode-2165; yt:bJxpCVc2wYE 8:38; creator-rule]
- For steam, put a large heat-proof bowl on the bottom of the oven and pour in boiling water; steel or cast iron works, glass does not. A tray above the stone shields the top heat so the steam stays low. [breadcode-2172; yt:bJxpCVc2wYE 5:39; creator-rule]
- Preheat the stone about 30 minutes at maximum oven temperature, then reduce the oven to 230 °C before loading the bread. — `{"preheat_min": 30, "temp_c": 230}` [breadcode-2173; yt:bJxpCVc2wYE 6:50; creator-rule]
- A stone gives the dough extra bottom heat for more oven spring, and the crust forms later, which helps the rise. [breadcode-2174; yt:bJxpCVc2wYE 6:16; creator-rule]
- Sprinkle semolina on the stone so the dough does not stick; parchment paper is an alternative. [breadcode-2175; yt:bJxpCVc2wYE 7:25; creator-rule]
- Bake the barbecue bread on a barbecue at maximum heat, similar to a charcoal grill. [breadcode-2188; yt:EbO3blOpGks 12:42; creator-rule]
- Bake at about 230 °C and do not bake too hot. — `{"temp_c": 230}` [breadcode-2192; yt:I3h6heaKWC8 3:34; creator-rule]
- For the par-bake, bake about 30 minutes at about 230 °C; the crust has not formed yet. The core should be at least 95 °C before removing the loaf, which is then cooled. — `{"bake_min": 30, "core_c": 95}` [breadcode-2193; yt:I3h6heaKWC8 3:34; creator-rule]
- Cool a par-baked loaf at least 2 hours at room temperature before packing it. — `{"cool_h": 2}` [breadcode-2194; yt:I3h6heaKWC8 4:10; creator-rule]
- A friend finished a par-baked loaf with about 10 minutes in a home oven and reported that the flat smelled like fresh bread. — `{"finish_min": 10}` [breadcode-2196; yt:I3h6heaKWC8 0:34; anecdote] *(low confidence)*
- Bake at 230 °C with steam under a lid for 25 minutes, then 15 to 20 minutes uncovered until the colour is right. — `{"oven_c": 230, "covered_min": 25, "uncovered_min": [15, 20]}` [breadcode-2234; yt:iEfLO_Cl0uE 10:13; creator-rule]
- A loaf baked in a loaf pan tears and cracks without enough steam; cover it with a second pan or use a steam set-up. [breadcode-2248; yt:oAFjGTWMgUo 1:13; creator-rule]
- A very dark crust together with large pockets near the top points to too little steam during the bake. [breadcode-2249; yt:oAFjGTWMgUo 4:59; creator-rule]
- Cool a loaf 1 to 2 hours before slicing and slice without pressure; cutting it hot makes the crumb collapse. — `{"cool_h": [1, 2]}` [breadcode-2250; yt:oAFjGTWMgUo 2:24; creator-rule]
- Brush excess surface flour off a loaf; flour left on the crust gives an unpleasant taste. [breadcode-2258; yt:oAFjGTWMgUo 3:04; creator-rule] *(low confidence)*
- Hendrik cut a rye loaf while it was still hot, which he says is what always happens with rye bread. [breadcode-2266; yt:-LeUYBe7HlY 4:26; creator-rule] *(low confidence)*
- Par-baking a loaf to about 92 C core and finishing the crust later gave a lower pH (3.70) than a fully baked loaf (3.85) and tasted more sour. — `{"ph": 3.7}` [breadcode-2307; yt:0v1QhtyUic4 8:51; measured] *(low confidence)*
- Baking too hot gives large pockets near the top and a crust that seals early; too little steam gives the same signs. — `{"temp_c": 230}` [breadcode-2313; yt:sFQZIKb_iIA 6:07; creator-rule]
- A gummy crumb means the loaf was underbaked; the creator uses a core temperature of about 92 C as the finish point. — `{"core_c": 92}` [breadcode-2314; yt:sFQZIKb_iIA 9:13; creator-rule]
- Cast iron seasoning works best with an oil rich in unsaturated fats, heated to about 240 C; sunflower oil has a smoke point near 230 C. — `{"temp_c": 240}` [breadcode-2317; yt:vLdmMuAxB7w 6:23; creator-rule]
- Two seasoning rounds are usually enough, and rust and soap residue should be removed before seasoning. After cleaning, drying at about 50 C for about 15 min is used. — `{"temp_c": 50, "min": 15}` [breadcode-2318; yt:vLdmMuAxB7w 5:10; creator-rule]
- Polymerised oil makes cast iron somewhat nonstick. [breadcode-2319; yt:vLdmMuAxB7w 1:13; creator-rule]
- Crackers are baked at about 180 C fan, rotated after about 4 min, flipped at about 8 min, rotated again at about 12 min, and finished at about 16 min. — `{"temp_c": 180, "bake_total_min": 16, "turn_interval_min": 4}` [breadcode-2329; yt:urHYzIktD9M 6:56; creator-rule]
- Starting the bake hot (about 260 C) and then dropping to about 230 C gave more oven spring and more bottom crust than a constant 230 C. — `{"temp_c": [230, 260]}` [breadcode-2331; yt:33B6nXafahs 3:24; measured] *(low confidence)*
- The creator preferred the hot-start loaf for its crust crispiness and oven spring, found both crumbs good, and changed his own baking to a hot start. [breadcode-2332; yt:33B6nXafahs 11:34; creator-rule] *(low confidence)*
- Preheating the oven for about 30 min before baking is the creator's practice. — `{"min": 30}` [breadcode-2333; yt:33B6nXafahs 2:48; creator-rule]
- A Dutch oven at about 230 C is the creator's sweet spot; baking both too hot and too cold hurts oven spring. — `{"temp_c": 230}` [breadcode-2343; yt:HNQbwvZIAYc 4:20; creator-rule]
- The Maillard reaction runs from about 140 to 165 C. — `{"temp_c": [140, 165]}` [breadcode-2352; yt:NAjBY7S_u04 9:13; creator-rule]
- A loaf is finished at a core temperature of about 92 C; underbaked loaves are gummy. — `{"core_c": 92}` [breadcode-2360; yt:XkoIlgnIeaQ 4:53; creator-rule]
- Oven displays are unreliable, so the oven should be measured with a thermometer. [breadcode-2365; yt:BUtn4HKAiBs 7:25; creator-rule]
- In a second side-by-side test, starting at 250 °C gave less oven spring than starting at 230 °C. — `{"temp_c": [230, 250]}` [breadcode-2366; yt:BUtn4HKAiBs 6:47; 8:16; measured]
- A crust forms too quickly in an oven that is too hot, and hotter baking gives large pockets near the top of the crumb. [breadcode-2367; yt:BUtn4HKAiBs 10:08; creator-rule]
- A Dutch oven heated on a pizza stone heats differently when the stone is removed. [breadcode-2368; yt:BUtn4HKAiBs 3:35; anecdote] *(low confidence)*
- The creator proposes untested ideas for future bakes: an ice cube, or preheating only the bottom of the oven. [breadcode-2369; yt:BUtn4HKAiBs 10:43; extrapolation] *(low confidence)*
- Boiling points the creator uses for the bake: water 100 C, ethanol about 78 C, acetic acid about 118 C, lactic acid about 122 C. — `{"water_boil_c": 100, "ethanol_boil_c": 78, "acetic_boil_c": 118, "lactic_boil_c": 122}` [breadcode-2375; yt:0v1QhtyUic4 1:13; creator-rule]
- Standard bake: about 30 min at 230 C, then check the core; at about 92 C (about 200 F) the bread is done, and the steam is then removed for another 10 to 25 min to colour the crust. — `{"oven_c": 230, "bake_min": 30, "core_c": 92, "crust_colour_min": [10, 25]}` [breadcode-2377; yt:0v1QhtyUic4 3:40; creator-rule]
- For the par-bake test the creator baked about 20 min at 230 C with upper and bottom heat, under an inverted tray on the stone, which keeps the steam trapped under the dough. — `{"oven_c": 230, "bake_min": 20}` [breadcode-2378; yt:0v1QhtyUic4 6:47-7:30; creator-rule]
- The loaf is baked from cold at 230 °C with steam: 25 min with the lid on, then about 15 to 20 min with the lid off. — `{"oven_c": 230, "covered_min": 25, "uncovered_min": [15, 20]}` [breadcode-2395; yt:JoAG7R9QESA 8:25; creator-rule]
- A starter bake in a covered pan on medium heat is checked after about 3 minutes, then flipped once the top has set and cooked a few minutes more. — `{"check_min": 3}` [breadcode-2399; yt:sB81OLY25Jo 8:38; creator-rule]
- A loaf in a preheated Dutch oven is baked at 230 °C for 25 min with the lid on, then 20 min with the lid off. — `{"oven_c": 230, "covered_min": 25, "uncovered_min": 20}` [breadcode-2410; yt:vLdmMuAxB7w description; creator-rule]
- Spreading the spice mix thinly and baking it at 160 °C (fan/convection) for about one hour gave a dry disc that was then ground. — `{"temp_c": 160, "duration_min": 60}` [breadcode-2428; yt:bVrSZzpVUnY 4:37; anecdote]
- Finish the cast-iron pizza in a preheated oven with the broiler on, close to the element, and remove it when the rim is dark. [breadcode-2436; yt:HSEdU6_fscs 3:59; creator-rule]
- In a gas pizza oven at about 450 °C, naan charred in about two minutes on a hot stone or peel; semolina is optional with this dough. — `{"oven_c": 450, "bake_min": 2}` [breadcode-2447; yt:sH99_IGh0ic 7:24; creator-rule]
- Bake Schwarzbrot at 230 °C, covered for about 30 minutes, then uncover and bake until the centre reaches 95 °C; about one hour in total in his oven. — `{"oven_c": 230, "covered_min": 30, "core_c": 95, "total_min": 60}` [breadcode-2462; yt:cHwLBs9SRd4 6:11; creator-rule]
- He lined a cold, non-preheated Dutch oven with parchment because he expects dough to stick to an unpreheated vessel; the frozen loaf was placed this way. [breadcode-2482; yt:Typy5Uzp-EM 1:05; anecdote]
- Preheating the Dutch oven matters for higher-hydration dough: the initial high heat forms a surface gel that keeps the dough from spreading. [breadcode-2483; yt:Typy5Uzp-EM 1:05; creator-rule]
- The frozen loaf reached 94 °C at the centre after about 40 minutes at 230 °C (top and bottom, no fan); he treats 95 °C as done. — `{"core_c": [94, 95], "oven_c": 230, "bake_min": 40}` [breadcode-2484; yt:Typy5Uzp-EM 3:20; measured]
- Finishing the bake uncovered develops Maillard colour on the crust; he removed the Dutch oven lid for that. [breadcode-2485; yt:Typy5Uzp-EM 3:20; creator-rule]
- In frozen dough, the crust sets quickly while the centre is still frozen, so the loaf gets less oven spring and large air pockets form. [breadcode-2486; yt:Typy5Uzp-EM 3:55; anecdote]
- Baking the shaped dough for about 30 s on a hot stone before scoring dries the surface, which he found makes scoring easier and gives better opening. — `{"pre_bake_s": 30}` [breadcode-2502; yt:0h63OloQQd8 4:07; anecdote]
- Score at about 45 degrees to the surface; the scoring direction depends on where the blade is placed. — `{"score_angle_deg": 45}` [breadcode-2503; yt:0h63OloQQd8 4:44; creator-rule]
- Functional scoring is what makes the loaf open; decorative scoring only changes the look. [breadcode-2504; yt:0h63OloQQd8 5:20; creator-rule]
- Do not correct a scored line before baking: in his bake the re-cut area did not open. [breadcode-2505; yt:0h63OloQQd8 5:20; anecdote]
- Wetting the loaf before it goes in adds steam at the start of the bake, which helps the loaf open. [breadcode-2506; yt:0h63OloQQd8 5:20; creator-rule]
- In a home oven, an inverted tray over the loaves on a stone traps steam underneath, acting like a Dutch oven; boiling water is poured into a tray below. [breadcode-2507; yt:0h63OloQQd8 5:56; creator-rule]
- Steam for about 30 minutes, then remove the steam source and bake about 20 minutes more to the crust colour you like. — `{"steam_min": 30, "dry_min": 20}` [breadcode-2508; yt:0h63OloQQd8 6:35; creator-rule]
- A scored cut closes and the loaf stays flat if the oven is too hot or has too little steam. [breadcode-2509; yt:0h63OloQQd8 6:35; creator-rule]
- Batards baked on a stone with a water tray in the bottom and water sprayed once per minute for the first 10 minutes get good steam. — `{"spray_interval_min": 1, "spray_duration_min": 10}` [breadcode-2542; yt:2tOw0eyuEsU description; creator-rule]
- Bake the buns with steam at 200 °C for about 15 minutes, then about 15 minutes more or until the colour is right. — `{"oven_c": 200, "steam_min": 15, "dry_min": 15}` [breadcode-2555; yt:Dx7xM8IThCI 6:46; creator-rule]
- Steam at the start of the bake is essential: it keeps the crust from setting early so the buns can expand. [breadcode-2556; yt:Dx7xM8IThCI 6:46; creator-rule]
- Score rolls at 45 degrees with a swift cut about 0.5 to 1 cm deep. — `{"score_angle_deg": 45, "score_depth_cm": [0.5, 1]}` [breadcode-2683; yt:9dsgUixpOBM 5:49; creator-rule]
- Spraying the loaves with water before baking keeps the surface moist so they expand more in the oven. [breadcode-2684; yt:9dsgUixpOBM 6:28; creator-rule]
- Remove the steam source after half the bake so the crust can set; steaming the whole bake would prevent a crust. — `{"steam_until_bake_pct": 50}` [breadcode-2685; yt:9dsgUixpOBM 7:06; creator-rule]
- Bake at 230 °C in a home oven. — `{"oven_c": 230}` [breadcode-2691; yt:0oKA_iTytiw 3:15; creator-rule]
- Bake covered in a preheated Dutch oven at 230 °C for about 25 minutes, then remove the lid and bake until done. — `{"oven_c": 230, "lid_on_min": 25}` [breadcode-2705; yt:wz7tysfSDFM 5:46; creator-rule]
- Spray the loaf with extra water before covering it in the Dutch oven, so the covered phase has plenty of steam. [breadcode-2706; yt:wz7tysfSDFM 5:08; creator-rule]
- Score the loaf at about 45 degrees. — `{"score_angle_deg": 45}` [breadcode-2707; yt:wz7tysfSDFM 5:08; creator-rule]
- Rice flour makes the bottom nonstick; brushing off the excess keeps the crust from tasting floury. [breadcode-2708; yt:wz7tysfSDFM 4:29; creator-rule]
- Preheat the oven as hot as possible for at least 30 minutes for a pizza on a stone or steel. — `{"preheat_min": 30}` [breadcode-2720; yt:ziMMCphF054 4:28; creator-rule]
- Home setups (stone, steel or broiler) get to about 90 % of the result of a stone pizza oven, per the creator. [breadcode-2721; yt:ziMMCphF054 description; creator-rule] *(low confidence)*
- Without a Dutch oven, heat a stone or tray set over a heat-proof bowl at maximum temperature for about 30 minutes before baking. — `{"preheat_min": 30}` [breadcode-2726; yt:-iHm0XPAmbQ 0:00; creator-rule]
- Load the loaf onto the hot stone or tray, cover it with a second tray set higher, and pour boiling water into the bowl below to create steam. [breadcode-2727; yt:-iHm0XPAmbQ 0:39; creator-rule]
- This tray-and-bowl steam setup gave strong oven spring for an 86 % hydration loaf. — `{"hydration_pct": 86}` [breadcode-2728; yt:-iHm0XPAmbQ description; anecdote] *(low confidence)*
- Oil the loaf pan and line it with paper; without this wholegrain dough sticks. [breadcode-2733; yt:2NmD9KVyZCg 5:40; creator-rule]
- Preheat the oven 20 to 30 minutes, then bake at 230 °C for 45 minutes. — `{"oven_c": 230, "bake_min": 45, "preheat_min": [20, 30]}` [breadcode-2735; yt:2NmD9KVyZCg 7:23; creator-rule]
- Let the loaf cool for at least one hour before cutting. — `{"cool_min": 60}` [breadcode-2736; yt:2NmD9KVyZCg 8:01; creator-rule]
- Cook the flatbread in a medium-heated pan, flipping once after three to four minutes until it inflates. — `{"pan_flip_min": [3, 4]}` [breadcode-2752; yt:5DWcrVHK1WU 3:14; creator-rule]
- Wrap the baked flatbreads in a kitchen towel so they stay soft until eaten. [breadcode-2758; yt:5DWcrVHK1WU 3:46; creator-rule]
- A guest baker's explanation: crust bubble size depends on skin formation driven by fridge humidity and on fermentation, and a covered Dutch oven does not need extra steam to bake well. [breadcode-2763; yt:JltQXoU786k 0:35; extrapolation] *(low confidence)*
- Bake the seed loaf preheated to 230 °C for about 45–50 min. — `{"oven_c": 230, "bake_min": [45, 50]}` [breadcode-2773; yt:cN_skGXZ4Es 2:38; creator-rule]
- Steam keeps the crust soft early on, so the loaf can expand in the oven before a crust sets. [breadcode-2774; yt:cN_skGXZ4Es 3:18; creator-rule]
- Remove the water source about halfway through the bake, once oven spring is over, so a crust can form. — `{"steam_phase_pct": 50}` [breadcode-2775; yt:cN_skGXZ4Es 3:18; creator-rule]
- A covered Dutch oven traps the steam the loaf releases, so the crust forms later and the loaf springs more. [breadcode-2776; yt:v8VSfDF3Tqs 0:00; creator-rule]
- A tray of water in an ordinary oven does not make enough steam compared with a closed Dutch oven. [breadcode-2777; yt:v8VSfDF3Tqs 0:35; creator-rule]
- Preheat the Dutch oven for about 30 min at maximum oven temperature before loading the loaf. — `{"preheat_min": 30}` [breadcode-2778; yt:v8VSfDF3Tqs description; creator-rule]
- Bake the Dutch oven loaf covered for about 25 min, then uncovered for about 20 min (about 45 min total). — `{"covered_min": 25, "uncovered_min": 20}` [breadcode-2779; yt:v8VSfDF3Tqs 3:03; creator-rule]
- Invert the proofed loaf from the banneton straight onto the preheated base, score quickly and put the lid on right away, because the pot is very hot. [breadcode-2781; yt:v8VSfDF3Tqs 1:50; creator-rule]
- In a side-by-side test none of three buns stuck: two mini cocottes were greased (olive oil, butter) and one plain cocotte was preheated; earlier attempts without preheating had stuck. [breadcode-2808; yt:nIQEiSGGw2Q 1:18; measured]
- Mini cocottes steam the bun while the lid is on for the first half of the bake; the description says the top is removed afterwards. — `{"covered_phase_pct": 50}` [breadcode-2809; yt:nIQEiSGGw2Q description; creator-rule]
- Optionally, spray the loaf or the inside of the Dutch oven with water before baking to add more steam. [breadcode-2819; yt:v8VSfDF3Tqs description; creator-rule]
- A home oven usually fits only one Dutch oven, so only one loaf can be baked at a time; the creator says this suits home baking but not larger production. [breadcode-2820; yt:v8VSfDF3Tqs 1:13; creator-rule]
- Cool croissants for at least 1 hour before serving. — `{"cool_min_h": 1}` [breadcode-2835; yt:7wyikR8qwHQ description; creator-rule]
- Dust the peel with semolina so the loaf slides off onto the stone. [breadcode-2855; yt:pl4L3q0UMVk 0:38; creator-rule]
- Two German sourdough loaves on a stone at 230 °C for about 45 min: about 20 min with steam, then about 20 min without. — `{"oven_c": 230, "bake_min": 45, "steam_min": 20, "dry_min": 20}` [breadcode-2874; yt:PUpB5bENkqk 0:41; creator-rule]
- Bake from a non-preheated oven at 230 °C for about 40 min under a tray or second pan for steam, then remove the cover and bake about 5 min more. — `{"oven_c": 230, "covered_min": 40, "uncovered_min": 5}` [breadcode-2880; yt:l5FtMycJna4 description; creator-rule]
- Bake until the core temperature is at least 95 °C. — `{"core_c": 95}` [breadcode-2881; yt:l5FtMycJna4 description; creator-rule]

### baking.bake-loss

- Part of the dough's acidity evaporates during baking: a longer bake yields a less sour loaf, and a shorter bake keeps more acidity in the crumb. [breadcode-0533; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Smell and taste; creator-rule]

### baking.cooling

- Let the loaf cool for at least 1 h before slicing; a hot crumb tears when cut. — `{"cool_min": 60}` [breadcode-1954; yt:U9_GVHuI1Ro 18:58; creator-rule]
- Let the loaf cool for at least 2 h before slicing; he judges that the crust is best within about that time. — `{"cool_min": 120}` [breadcode-1956; yt:xmt3eXzOQLM 17:15; creator-rule]
- Cool at least an hour before cutting; cutting early makes the wet crumb stick to the knife. — `{"cool_min": 60}` [breadcode-2591; yt:nsyjVa5SSQI 6:00; creator-rule]

### baking.core

- The dough core does not exceed about 100 °C as long as it still contains water, so the crumb keeps most of its acids. — `{"core_c": 100}` [breadcode-0993; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; literature]

### baking.core-temp

- Loaf pan bread is done at about 92 °C (197 °F) core temperature; a thermometer is the best check. — `{"core_c": 92}` [breadcode-0307; SF Ch 6 Bread types; creator-rule]
- Rye loaf is done at about 92 °C (197 °F) internal temperature; check with a thermometer. — `{"core_c": 92}` [breadcode-0340; SF Ch 8 Non wheat sourdough; creator-rule]

### baking.core-temperature

- Bake the loaf with steam until the core reaches about 80 °C, then remove the steam; the loaf is then taken out of the pan and baked on until the core reaches at least 95 °C. A thermometer is the reliable endpoint. — `{"core_c": [80, 95]}` [breadcode-1898; yt:xmt3eXzOQLM 12:53; 16:00; creator-rule]
- The 3-minute loaf is baked in the lidded loaf pan from an unpreheated oven at 230 °C (upper and lower heat) for about 50–65 min, until the core reads about 80 °C; it is then unmoulded and baked uncovered for about 10 more minutes to above 95 °C. — `{"bake_c": 230, "core_c": [80, 95], "bake_lidded_min": [50, 65], "bake_uncovered_min": 10}` [breadcode-1905; yt:bi-zqG5y1I8 8:01; 9:09–11:28; creator-rule]
- Bake lidded at 230 °C until the core reaches about 80 °C, then uncover and bake until the core reaches 95 °C. — `{"oven_c": 230, "core_c_lidded": 80, "core_c_final": 95}` [breadcode-2601; yt:P0-EGk_1SYo 2:15; creator-rule]
- A loaf is done at a core temperature of about 92 °C (≈200 °F). — `{"core_c": 92}` [breadcode-2636; yt:BPQHSA2KoRY 4:15; creator-rule]
- A loaf-pan bread is done at a core temperature of about 95 °C; colour is not reliable (at 84 °C it was still underdone). — `{"core_c": 95}` [breadcode-2643; yt:P59OfgJ4Dps 5:14 (95 °C ready); 5:50 (84 °; creator-rule]

### baking.crumb-comparison

- In the side-by-side bake, both loaves were baked in the same Dutch oven and time. Einkorn gave slightly more oven spring than emmer, and the crumbs were similar, with einkorn a little more open. Judged by eye. [breadcode-1926; yt:PehhnV1lW2o 46:00-48:04; measured] *(low confidence)*

### baking.crust

- The Maillard reaction starts to thicken the crust once the surface has dried and reaches about 105-120 °C. — `{"crust_c": [105, 120]}` [breadcode-1065; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked too; literature]
- Slicing the loaf increases surface area and so gives more crust; scoring can turn the loaf into a crusty bread (Krustenbrot). [breadcode-2842; yt:di4QatuNB7Y description; creator-rule]

### baking.doneness

- The creator takes the loaves out when the thickest part of the crumb reaches about 90 °C core, using a probe thermometer. — `{"core_c": 90}` [breadcode-1927; yt:PehhnV1lW2o 41:56; creator-rule]

### baking.dutch-oven

- Dutch ovens come in enamelled cast iron, clay or with glass lids; all trap part of the steam, which lets the bread rise further, giving more oven spring and a fluffier crumb. [breadcode-0764; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens (figure caption); creator-rule]
- Dutch ovens are heavy, expensive, need care when hot and, if cast iron, need seasoning; because of cost the author advocates the inverted tray method for new bakers. [breadcode-0765; SF Ch 10 Baking §10.3 Building up steam > Dutch ovens; creator-rule]
- When using a Dutch oven, do not preheat it for too long; the creator also advises using the lid to trap steam. [breadcode-2393; yt:HNQbwvZIAYc 4:20; creator-rule]
- Bake covered at 230 °C for 25 min, then remove the lid so the crust forms. — `{"oven_c": 230, "covered_min": 25}` [breadcode-2590; yt:nsyjVa5SSQI 5:24; creator-rule]
- Bake in a preheated Dutch oven, covered for the first half of the bake. [breadcode-2613; yt:bjLCHRInmLY 4:11; creator-rule]
- Uncover for the last ~20 min so the crust colours and moisture escapes. — `{"uncovered_min": 20}` [breadcode-2614; yt:bjLCHRInmLY 4:47; creator-rule]

### baking.ear

- The ear is a characteristic that can be achieved on wheat sourdough with the right fermentation and scoring technique; it adds flavour and texture. [breadcode-0633; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Scoring at about 45° to the dough surface, slightly off the centre, makes the overhanging side rise more, producing the ear. — `{"angle_deg": 45}` [breadcode-0635; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- The ear is a thin crisp edge that is usually a little darker after baking and adds flavour and texture. [breadcode-0636; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- When scoring closer to one side of the dough, adjust the 45° angle so the ear still forms. — `{"angle_deg": 45}` [breadcode-0637; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]

### baking.flatbread-and-pan

- Small portions of the same bucket dough can be cooked as flatbread in hot olive oil in a pan (a few minutes, then flipped). They can also be cooked in a small cast-iron pan on a barbecue over medium heat until puffed; no timing is given for the barbecue version. [breadcode-1934; yt:MetItcI3Yks 6:52; anecdote] *(low confidence)*

### baking.flip-out

- Flour-side-down placement means the dough is flipped once for baking, which applies to baguette doughs too. [breadcode-0582; SF Ch 7 Wheat sourdough §7.12 Shaping > Prepare for proofing; creator-rule]

### baking.heat-transfer

- Creator's reasoning: a cast-iron Dutch oven releases heat to the dough faster than a stone, so the same 230 °C setting gives the stone base less heat. In the test the stone loaf's base was judged less done during the bake, but after finishing on a rack both bases were judged similar. [breadcode-1920; yt:O0REFqmB0rs 8:45; 12:26-13:08; 14:31; measured] *(low confidence)*

### baking.loaf-pan

- Loaf-pan bread: mix, place in the pan, wait until roughly doubled, then bake in a non-preheated oven for about 30–50 min. — `{"bake_min": [30, 50]}` [breadcode-0305; SF Ch 6 Bread types; creator-rule]
- Loaf pan dough takes under 5 min to work, and pans use oven space efficiently (several loaves per bake). — `{"work_min": 5}` [breadcode-0314; SF Ch 6 Bread types; creator-rule]

### baking.loaf-pan-unpreheated

- Bucket dough is baked from the fridge in a greased loaf pan: about 800 g of dough, put into an unheated oven, then 230 °C upper and lower heat, to 80 °C core, then unmoulded to 95 °C core, roughly 40 min. — `{"dough_g": 800, "bake_c": 230, "core_c": [80, 95], "bake_min": 40}` [breadcode-1932; yt:MetItcI3Yks 3:56; creator-rule]

### baking.no-preheat

- Baking in an unpreheated oven with a loaf pan gives the crust time to form later and lets the yeast become more active as the dough warms, which the creator says helps oven spring. [breadcode-1933; yt:MetItcI3Yks 4:35; creator-rule] *(low confidence)*

### baking.oven

- Loaf pan bread is baked around 200 °C (390 °F); free-standing around 230 °C (445 °F). — `{"oven_c": [200, 230]}` [breadcode-0308; SF Ch 6 Bread types; creator-rule]
- The loaf pan is baked at a lower temperature because edges heat slowly, so the top can overcook before the centre bakes. [breadcode-0309; SF Ch 6 Bread types; creator-rule]
- Once the dough has finished proofing, preheat the oven to about 230 °C (446 °F) before scoring. — `{"oven_c": 230}` [breadcode-0628; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- On a banneton proof, flip the dough onto the baking surface (oven rack, tray, stone, steel or Dutch oven) so that the side that was against the banneton now faces up. [breadcode-0634; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- He preheated the oven at maximum temperature for about 30 minutes with the stone or tray inside; without a special pizza oven, keep the stone or tray in the oven as long as possible and use the broiler. — `{"preheat_min": 30}` [breadcode-2303; yt:xLYxPneBx-M 9:32; creator-rule] *(low confidence)*
- The displayed oven setting can be far off the real temperature: 230 °C on the display might really be 250 or 210 °C, so measure the oven with a thermometer. — `{"set_temp_c": 230, "actual_range_c": [210, 250]}` [breadcode-2387; yt:BUtn4HKAiBs 10:08; creator-rule]
- If the bottom of the loaf is not browned on a tray or stone, flip the loaf upside down during the crust phase so it is exposed to the top heating element. [breadcode-2658; yt:BPQHSA2KoRY 5:24; creator-rule]
- Bake the loaf in a preheated Dutch oven with the heat set to about 230 °C. — `{"oven_c": 230}` [breadcode-2661; yt:lFzLCivvyes 1:13; creator-rule]
- Preheat the stone for about 30 min at maximum oven temperature. — `{"preheat_min": 30}` [breadcode-2871; yt:PUpB5bENkqk 0:00; creator-rule]

### baking.oven-spring

- Hot-start bake: the creator put the dough in a hot Dutch oven at about 260 C, and at the halfway point (25 min) took the lid off and baked about 20 min more. — `{"temp_c": 260, "min_first_phase": 25, "min_second_phase": 20}` [breadcode-2381; yt:33B6nXafahs 3:24-4:04; creator-rule]
- Oven spring depends on dough strength (autolyse, lamination, coil folds), proper fermentation, tight shaping and correct proofing. [breadcode-2968; yt:oPMJPhBLPxU description; creator-rule]
- The first 5 minutes of the bake show whether the dough will spring in the oven. — `{"first_bake_min": 5}` [breadcode-2969; yt:oPMJPhBLPxU description; creator-rule]
- Buns that looked very flat when loaded sprang strongly in the oven. [breadcode-2978; yt:ovzejNuq4Eo description; anecdote] *(low confidence)*

### baking.pan

- Cook flatbread in a pan on medium heat with a little oil for even heat; dough from the fridge goes straight into the pan without warming up. [breadcode-0295; SF Ch 6 Bread types; creator-rule]
- A lid over the pan cooks the top of the flatbread; the evaporating water circulates and heats the surface. [breadcode-0296; SF Ch 6 Bread types; creator-rule]
- Flip the flatbread after 2–4 minutes and cook the other side for the same time; a longer bake gives more char. — `{"flip_min": [2, 4]}` [breadcode-0298; SF Ch 6 Bread types; creator-rule]
- Flatbread: cook about 5 min per side, reduce time if too dark, extend if too pale. — `{"cook_min_per_side": 5}` [breadcode-0300; SF Ch 6 Bread types; creator-rule]
- Grease the loaf pan generously or use a nonstick spray; do not wash it with soap, use a towel, since patina improves release with every bake. [breadcode-0313; SF Ch 6 Bread types; creator-rule]
- In pancake or flatbread bakes, pockets form from evaporated water and CO2 made by microbes. [breadcode-0322; SF Ch 6 Bread types; creator-rule]
- Remove the rye loaf from its pan once it reaches core temperature, then finish unpanned with steam for an all-round crust; longer bakes give darker, crunchier crust. [breadcode-0341; SF Ch 8 Non wheat sourdough; creator-rule]
- The crust of rye bread bakes pale; crust colour is controlled by baking longer. [breadcode-0346; SF Ch 8 Non wheat sourdough; creator-rule]

### baking.par-bake

- For a next-morning finish, par-bake the loaf until the core reaches at least 95 °C, cover it in a towel and rest it about 1 hour, then cool, bag it airtight and refrigerate. — `{"core_c": 95, "rest_before_fridge_h": 1}` [breadcode-1675; yt:WOZLvsV1M8s 10:31–11:03; creator-rule]

### baking.partial-bake

- Half-bake with lots of steam so there is little crust, then bag the loaves airtight and refrigerate; finish baking later for fresh bread on demand. He says the loaves can be kept in the fridge for a few days. [breadcode-1921; yt:O0REFqmB0rs 10:38; creator-rule] *(low confidence)*

### baking.preheat

- Creator preheats the Dutch oven until its probe reads about 230 °C, then holds that temperature about 15 more minutes. In his recent tests 230 °C gave the best results. — `{"temp_c": 230, "hold_min": 15}` [breadcode-1945; yt:O0REFqmB0rs 5:06; creator-rule]

### baking.scoring

- Score the free-standing dough with a sharp blade before baking to control how it opens (the ear). [breadcode-0316; SF Ch 6 Bread types; creator-rule]
- Rye loaves are usually not scored; the crust cracks open on its own during baking. [breadcode-0344; SF Ch 8 Non wheat sourdough; creator-rule]
- Functional scoring is a small incision that lets the dough expand during baking. [breadcode-0629; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- An unscored dough tends to crack open at its weakest spots, which are usually where the edges were sealed after shaping. [breadcode-0630; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Decorative scoring creates artistic patterns on the loaf. [breadcode-0631; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Rub the surface with white rice flour before artistic scoring; it greatly raises the contrast of the incisions. [breadcode-0632; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- A razor blade gives a cleaner incision than a sharp knife because it is more flexible; it can be used directly or on a chopstick. [breadcode-0638; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- The scoring blade should be as sharp as possible so the dough is cut cleanly and not torn. [breadcode-0639; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Rubbing flour on the surface before proofing dries the outer layer of dough, which makes scoring easier and reduces tearing. [breadcode-0640; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Cold-proofed dough is easier to score than room-temperature proofed dough because its low viscosity lets the blade cut cleanly; room-temperature dough tears more easily. [breadcode-0641; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Practise scoring on a wet dough after developing its strength: the dough is sticky, so cut it, wait a few minutes, re-round it and repeat as often as needed. [breadcode-0643; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- After proofing there is only one chance to score the dough. [breadcode-0644; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Placing the dough in the freezer for 30 min just before baking, once it is nearly proofed, dries the surface further and lowers its viscosity, making scoring easier. — `{"freezer_min": 30}` [breadcode-0645; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]
- Baking for 30 seconds without steam dries the surface of the dough further and makes scoring easier; experiment with the timing. — `{"bake_no_steam_min": 0.5}` [breadcode-0646; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]

### baking.steam

- Use steam or cover the loaf pan with another equally sized pan to simulate a Dutch oven, so moisture stays in the dough. [breadcode-0310; SF Ch 6 Bread types; creator-rule]
- In the second stage of the bake, without steam, the crust builds thicker and darker. [breadcode-0766; SF Ch 10 Baking §10.2 The role of steam (Baking step 2 figure); creator-rule]
- Turn off the oven fan, as home ovens vent steam quickly; if it cannot be turned off, a Dutch oven helps. [breadcode-1069; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked too; creator-rule]
- Too little steam gives a thick crust all round and large alveoli towards the edges, because the crust forms too quickly. [breadcode-1070; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked wit; creator-rule]
- Steam during the first part of the bake keeps the crust surface near about 100 °C, preventing an early Maillard crust. — `{"crust_c": 100}` [breadcode-1072; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked wit; creator-rule]
- Steam can be produced by a Dutch oven, an inverted tray, a bowl of boiling water, or an oven's built-in steam function. [breadcode-1073; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked wit; creator-rule]
- A common method is an inverted tray over the dough with a bowl of boiling water low in the oven. [breadcode-1074; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked wit; creator-rule]
- Excess steam is hard to create; it occurred only with a Dutch oven combined with large ice cubes. [breadcode-1075; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; anecdote]
- Ice cubes of about 20 g are typical in Dutch-oven steaming; the author's cubes of about 80 g were four times heavier and gave excess steam. — `{"ice_cube_g": [20, 80]}` [breadcode-1076; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; anecdote] *(low confidence)*
- The ice-cube method made the dough surface jump close to 100 °C directly, because the steam carries more energy and heats the surface faster by convection. [breadcode-1077; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; creator-rule]
- Replicated with bread dough, the ice-cube method gave the author less oven spring. [breadcode-1078; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured] *(low confidence)*
- In the apple test, the ice-cube variant reached about 91 °C at the apple surface at about 12 min and about 99 °C by 20 min. — `{"time_min": 12, "surface_c": 91}` [breadcode-1079; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, a preheated Dutch oven (no ice) took the apple surface to about 85 °C by about 13 min and then held there. — `{"time_min": 13, "surface_c": 85}` [breadcode-1080; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, a non-preheated Dutch oven reached about 74 °C at the apple surface after 25 min and was still rising slowly. — `{"time_min": 25, "surface_c": 74}` [breadcode-1081; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, preheating only the bottom of the Dutch oven took the apple surface to about 100 °C at about 26 min. — `{"time_min": 26, "surface_c": 100}` [breadcode-1082; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, spritzing water into a preheated Dutch oven took the apple surface to about 96 °C by 17 min and about 100 °C by 28 min. — `{"time_min": 28, "surface_c": 100}` [breadcode-1083; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test the ice-cube variant heated the surface fastest of the five steaming variants tested. [breadcode-1084; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, the ambient temperature in the Dutch oven with an ice cube plateaued around 182 °C; with a preheated oven around 190 °C. — `{"ambient_c": [182, 190]}` [breadcode-1085; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, a non-preheated Dutch oven reached about 151 °C ambient after 25 min and was still rising. — `{"time_min": 25, "ambient_c": 151}` [breadcode-1086; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- In the apple test, preheating only the bottom gave an ambient temperature plateauing around 176 °C from about 22 min. — `{"time_min": 22, "ambient_c": 176}` [breadcode-1087; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Too much ; measured]
- Two stacked loaf pans can simulate a Dutch oven and trap steam during the first half of the bake. [breadcode-1377; yt:o_xtVmR4vlc 26:33; creator-rule]
- Choose a steam strategy by desired crust: steam the whole bake for soft crust, or remove the steam source midway for crust. [breadcode-1378; yt:o_xtVmR4vlc 23:38; creator-rule]
- Without a Dutch oven, use two trays with a heat-resistant bowl of water to trap steam during the first half of the bake. [breadcode-1394; yt:NMglhwp2lNs 34:52; creator-rule]
- Preheat the steam vessel (a heat-resistant bowl or tray) in the oven for about 30 minutes before the bake, so it releases heat to the dough quickly. — `{"preheat_min": 30}` [breadcode-1428; yt:o_xtVmR4vlc 23:38; creator-rule]
- Bake at 230 °C under a trapped-steam cover for about 20 min, then remove the steam source and the cover for the rest of the bake to finish the crust. — `{"bake_c": 230, "steam_min": 20}` [breadcode-1893; yt:U9_GVHuI1Ro 18:23; creator-rule]
- Covering the loaf pan with a lid or a second tray traps evaporating water, which keeps the crust from forming too quickly so the dough can expand more in the oven. [breadcode-1938; yt:bi-zqG5y1I8 7:25; creator-rule]
- Without a second loaf pan, make the steam chamber with a heat-resistant bowl of boiling water on the bottom and a tray over the loaf; bake at 230 °C top and bottom heat with no fan so the steam is not blown away. — `{"oven_c": 230}` [breadcode-1957; yt:xmt3eXzOQLM 14:04; creator-rule]
- Steam can come from a light spritz or a small ice cube of about 20 g; too much ice gives too much steam, which transfers heat too fast, and an ice cube heats the surface so fast it forms a gel too early, which hurts oven spring. — `{"ice_g": 20}` [breadcode-2344; yt:HNQbwvZIAYc 4:20; creator-rule]
- The Maillard reaction starts around 150 C, and steam keeps the surface below 100 C until then, which gives crust colour. — `{"temp_c": 150}` [breadcode-2345; yt:HNQbwvZIAYc 4:54; creator-rule]
- A stone with an inverted tray over it gives steam comparable to a Dutch oven. [breadcode-2346; yt:HNQbwvZIAYc 5:26; anecdote] *(low confidence)*
- In a side-by-side bake, the loaf with an ice cube in the Dutch oven showed more visible oven spring (it burst out through the score) than the loaf with water spritzes; the creator did not call an overall winner. [breadcode-2351; yt:NAjBY7S_u04 7:26; measured] *(low confidence)*
- The creator hypothesises that an ice cube keeps the crust below Maillard temperature longer, which would explain the extra spring. [breadcode-2353; yt:NAjBY7S_u04 9:48; extrapolation] *(low confidence)*
- Without a Dutch oven, the creator bakes on a preheated stone at 230 C with no fan (heat from top and bottom only), and an inverted tray on the upper shelf with a little boiling water acts as the steam source. — `{"oven_c": 230}` [breadcode-2413; yt:WOfTMskT44Y 3:46; creator-rule]
- Bake at about 230 C and use as much steam as possible in the first stage; a crust that seals too early under too little steam or too high heat produces large pockets that gather under the top crust. — `{"oven_c": 230}` [breadcode-2416; yt:XkoIlgnIeaQ 6:43; creator-rule]
- Without a Dutch oven, put water in the bottom of the oven for steam. [breadcode-2588; yt:nsyjVa5SSQI 4:17; creator-rule]
- A Dutch oven traps humidity, which works well for rye. [breadcode-2589; yt:nsyjVa5SSQI 4:17; creator-rule]
- Steam delays crust formation; without it the crust sets early and limits the bread's vertical expansion. [breadcode-2629; yt:BPQHSA2KoRY 0:38; creator-rule]
- Inverted-tray method: an upside-down tray above the loaves shields them from the top element and holds steam around the bread; a cheap alternative to a Dutch oven or steam oven. [breadcode-2630; yt:BPQHSA2KoRY 1:13; creator-rule]
- Preheat the oven with the baking stone or tray for 30 min at about 230 °C. — `{"oven_c": 230, "preheat_min": 30}` [breadcode-2631; yt:BPQHSA2KoRY 2:25; creator-rule]
- The inverted tray goes in unpreheated, above the loaves, with space for them to rise. [breadcode-2632; yt:BPQHSA2KoRY 3:40; creator-rule]
- Pour boiling water into a preheated metal tray at the base at loading; do not use glass, which can shatter. [breadcode-2633; yt:BPQHSA2KoRY 4:15; creator-rule]
- Bake about 30 min with steam before removing the steam sources. — `{"steam_min": 30}` [breadcode-2635; yt:BPQHSA2KoRY 4:15; creator-rule]
- After steam, remove the water tray and the inverted tray; the crust needs about 10–20 min more uncovered, depending on the colour you want. — `{"crust_min": [10, 20]}` [breadcode-2637; yt:BPQHSA2KoRY 5:24; creator-rule]
- Fan ovens eject steam, so the tray method does not work in them; use a Dutch oven there. [breadcode-2638; yt:BPQHSA2KoRY 5:24; creator-rule]
- A loaf pan covered with a lid or foil holds steam; remove the steam source about halfway through the bake. [breadcode-2644; yt:P59OfgJ4Dps 4:38; creator-rule]
- Bake at 200 °C until golden; put a bowl of boiling water in the oven for the first minutes to slow crust browning (Maillard). — `{"oven_c": 200}` [breadcode-2834; yt:7wyikR8qwHQ description; creator-rule]
- A home oven is dry, so a bowl of water is usually not enough; also spray the loaf and the oven walls before and during the bake. [breadcode-2851; yt:pl4L3q0UMVk 0:00; creator-rule]
- Spray the loaf every 2 minutes for the first 10 minutes of the bake (five sprays). — `{"spray_interval_min": 2, "spray_period_min": 10, "sprays": 5}` [breadcode-2852; yt:pl4L3q0UMVk 1:14; creator-rule]
- Remove the water bowl after half the bake time. — `{"remove_at_fraction": 0.5}` [breadcode-2853; yt:pl4L3q0UMVk description; creator-rule]
- Steam in the first part of the bake delays crust formation, because a crust blocks oven spring; lots of steam lets the dough expand. [breadcode-2856; yt:QbdwTFwhgWo description; creator-rule]
- Preheat a tray of rocks and a tray of kitchen towels for about 60 min; pour boiling water on the towels about 50 min in; pour more boiling water when the loaf goes in. — `{"preheat_min": 60, "boiling_water_at_min": 50}` [breadcode-2858; yt:QbdwTFwhgWo 0:00; 0:43; creator-rule]
- Remove both steam trays halfway through the bake so the crust can form. — `{"remove_at_fraction": 0.5}` [breadcode-2859; yt:QbdwTFwhgWo 0:43; creator-rule]
- Put a bowl of water in the oven and remove it after half the bake. — `{"remove_at_fraction": 0.5}` [breadcode-2872; yt:PUpB5bENkqk description; 0:41; creator-rule]
- Spray loaves with water every minute for the first 10 minutes of the bake (a bonus tip). — `{"spray_interval_min": 1, "spray_period_min": 10}` [breadcode-2873; yt:PUpB5bENkqk description; creator-rule] *(low confidence)*
- With no water source in the oven, spraying water into the oven every few minutes and opening the door every 2–3 min at the start kept a crust from forming early. — `{"door_open_interval_min": [2, 3]}` [breadcode-2979; yt:zkKMPxf2vBc description; anecdote]
- The crust should form in the later stage of the bake, not at the start. [breadcode-2980; yt:zkKMPxf2vBc description; creator-rule]
- The water source was removed during half of the bake of a 50 % wheat / 50 % whole wheat loaf, which still gave strong oven spring. — `{"water_source_removed_pct_of_bake": 50, "bake_min": 45}` [breadcode-2988; yt:zkKMPxf2vBc description; anecdote]

### baking.steam-duration

- Steam matters most in the first 20–25 min of the bake for oven spring; a Dutch oven makes it easier, but a home oven without one also works. — `{"steam_min": [20, 25]}` [breadcode-1913; yt:b7Pd7UcZxd0 16:29; creator-rule]

### baking.time

- The timelapse bake (50 % wheat, 50 % whole wheat loaf) took about 45 min overall. — `{"bake_min": 45}` [breadcode-2981; yt:zkKMPxf2vBc description; anecdote]

### baking.vessel

- Side-by-side test: a Dutch oven (230 °C upper and lower heat, 30 min, lid on) and a stone (230 °C with a second tray and a bowl of hot water, half the time with steam). Both were then finished together on a rack at 200 °C fan. Oven spring, crust and blisters were judged similar, with no clear winner. — `{"bake_c": 230, "dutch_oven_bake_min": 30, "stone_half_bake_min": 15, "finish_c": 200}` [breadcode-1919; yt:O0REFqmB0rs 6:53-14:31; measured] *(low confidence)*
- Grease the loaf pan well with a vegetable non-stick spray so the bread does not stick. [breadcode-2602; yt:P0-EGk_1SYo 0:31; creator-rule]
- A Dutch oven traps steam better than a stone; use the stone for loaves too large for a Dutch oven. [breadcode-2854; yt:pl4L3q0UMVk 1:14; creator-rule]
- A preheated stone gives more room for several loaves at once than a Dutch oven does. [breadcode-2870; yt:PUpB5bENkqk description; creator-rule]
