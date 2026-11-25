# mixing — The Bread Code / The Sourdough Framework

Claims: 190 (book 30, videos 155, repos 5). Evidence: creator-rule 170, anecdote 12, measured 7, extrapolation 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

The book drops autolysis in favour of **fermentolysis**: a lower inoculation (5–10 % instead of ≈20 %) lengthens bulk so enzymes do the autolysis work during fermentation (2 h autolyse + 5 h bulk ≈ 7 h bulk). Salt timing made no difference in his test. Yeasted doughs: reduce yeast instead of autolysing. Many 2020–21 videos still use autolyse (1–2 h, sometimes overnight) — an update between video era and book. Strength: kneading/slap-and-fold, stretch and folds and coil folds during early bulk; windowpane test.

## Claims

### mixing

- Rye flour is very sticky and hard to mix by hand; a spatula helps. [breadcode-0330; SF Ch 8 Non wheat sourdough; creator-rule]
- For non-wheat dough the mixing goal is only to homogenise and incorporate the starter; no dough strength is developed. [breadcode-0331; SF Ch 8 Non wheat sourdough; creator-rule]
- Mix the starter thoroughly into the dough so fermentation is even across the whole batch. [breadcode-0348; SF Ch 7 Wheat sourdough §7.1 The process; creator-rule]
- High-hydration dough needs longer kneading to build a proper gluten network. [breadcode-0396; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Chop chunks smaller, soak dry ingredients in water or oil first, or squeeze out excess moisture to reduce melting and charring. [breadcode-0723; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Mix-ins can be added during kneading; check hydration 30 min after the first kneading and add flour if needed. — `{"check_hydration_min": 30}` [breadcode-0740; SF Ch 9 Mix-ins §9.3 Techniques; creator-rule]
- Swirled colours: separate the base dough, add colour to one part, bulk ferment separately, then laminate and stack the coloured sheets before the last fold so the colours stay in layers. [breadcode-0746; SF Ch 9 Mix-ins §9.3 Techniques > Swirled colors; creator-rule]
- Sun-dried tomatoes: squeeze out the oil if they are pickled; soak dried ones in water before use. [breadcode-0754; SF Ch 9 Mix-ins §9.2 Examples > Highlights; creator-rule]
- Autolyse is resting flour and water before the other ingredients are added; the rest activates amylase and protease, which shortens bulk fermentation and improves the final loaf's browning and crumb. [breadcode-0805; SF Glossary > Autolyse; creator-rule]
- Autolyse is recommended when the starter inoculation exceeds about 20 % of flour weight. — `{"starter_pct_of_flour": [20, null]}` [breadcode-0806; SF Glossary > Autolyse; creator-rule]
- In bassinage, water is added in stages: the dough is first mixed at lower hydration so gluten bonds form, then more water is worked in during kneading, which increases extensibility and suits lower-gluten flours. [breadcode-0818; SF Glossary > Bassinage method; creator-rule]
- The coil fold is a gentle stretch-and-fold technique suited to use throughout bulk fermentation; it improves dough strength while minimizing damage to the dough structure. [breadcode-0828; SF Glossary > Coil fold; creator-rule]
- Fermentolyse combines autolyse and fermentation: a small amount of starter, typically about 10 % of flour weight, is mixed with the flour and water at the start. — `{"starter_pct_of_flour": 10}` [breadcode-0837; SF Glossary > Fermentolyse; creator-rule]
- Adding starter early in a fermentolyse makes the dough more extensible and easier to handle. [breadcode-0838; SF Glossary > Fermentolyse; creator-rule]
- For einkorn and rye, where gluten alignment is not the goal, kneading homogenizes the dough into a consistent mixture. [breadcode-0847; SF Glossary > Homogenizing; creator-rule]
- The author considers autolysis unnecessary in about 95 % of cases and recommends fermentolysis instead. — `{"unnecessary_pct": 95}` [breadcode-0953; SF Ch 12 Troubleshooting §12.2.1 Should I autolyse my dough?; creator-rule]
- Fermentolysis gives the benefits of autolysis without needing to knead the dough multiple times. [breadcode-0954; SF Ch 12 Troubleshooting §12.2.1 Should I autolyse my dough?; creator-rule]
- Autolysis only makes sense for a fast-fermenting yeast dough with a high yeast rate; even then, lowering the yeast is the better fix than autolysis. [breadcode-0955; SF Ch 12 Troubleshooting §12.2.1 Should I autolyse my dough?; creator-rule]
- For a same-day yeast bake, extra sugar and heavy kneading speed things up: 5 % sugar, about 15 min hand or 10 min machine kneading, then 30 min rest in the bowl before baking. — `{"sugar_pct": 5, "knead_hand_min": 15, "knead_machine_min": 10, "rest_before_bake_min": 30}` [breadcode-1121; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Kneading only homogenises the ingredients and develops gluten; gluten also develops without kneading over time, so kneading is only needed when time is short. [breadcode-1122; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Basic bread: autolyse the flour and water for about 1 h before adding salt and yeast. — `{"autolyse_h": 1}` [breadcode-1151; repo:the-bread-code recipes/savory/basic-bread.md; creator-rule]
- Burger buns: knead about 5 min, rest 15–30 min, then stretch-and-fold every 30 min until doubled. — `{"knead_min": 5, "rest_min": [15, 30], "fold_interval_min": 30}` [breadcode-1156; repo:the-bread-code recipes/savory/burger-buns.md; creator-rule]
- Pizza dough: mix about 5 min, then knead about 5 min until a window pane forms. — `{"mix_min": 5, "knead_min": 5}` [breadcode-1210; repo:pizza-dough README.md; creator-rule]
- During bulk, do 1–3 coil folds only when the dough visibly flattens out; he advises against a fixed hourly fold schedule. — `{"coil_folds_count": [1, 3]}` [breadcode-1261; yt:ImVvQMvGZKE 10:19; creator-rule]
- In an overnight autolyse test, the batch without autolyse, shaped well, showed the most oven spring; the autolyse batches with weaker shaping had less. [breadcode-1264; yt:ImVvQMvGZKE 3:27; measured] *(low confidence)*
- Mix the dough by hand for about 1.5 minutes. [breadcode-1273; yt:MrOM_D3hnpc 52:24; creator-rule]
- Kneading aligns gluten in wheat; low-gluten flours such as rye, spelt, einkorn or emmer only need to be mixed. [breadcode-1334; yt:013Qw3BojM4 1:46; creator-rule]
- Mix dough by hand for about 2 minutes until the starter is evenly spread, then rest 10 minutes so gluten develops without kneading. — `{"mix_min": 2, "rest_min": 10}` [breadcode-1386; yt:msqU-ylXWUs 10:02; creator-rule]
- If the dough does not come together after the 10-minute rest, add a little flour, homogenise, and rest another 10 minutes. — `{"rest_min": 10}` [breadcode-1418; yt:msqU-ylXWUs 11:15; creator-rule]
- An overnight rest of flour and water (autolyse) lets flour hydrate and begin breaking down, making the dough easier to work with later. [breadcode-1466; yt:qM9BXKaIr60 3:24; 6:46; creator-rule]
- After adding starter, about 5 minutes of hand mixing and stretch-and-fold builds strength; a dough with less strength work spreads faster and holds shape less. [breadcode-1468; yt:qM9BXKaIr60 6:46; 11:21; 14:21; measured]
- After an overnight autolyse, the dough was more elastic and spread less during a 15-minute bench rest than the same dough mixed right away. [breadcode-1477; yt:JZBtmQXKWA8 11:10; 11:48; measured]
- In a stand mixer, knead in two sets of about 5 minutes so both doughs get the same strength before comparing them. — `{"knead_min_per_set": 5, "rest_min": 15}` [breadcode-1484; yt:JZBtmQXKWA8 4:32; 5:11; creator-rule]
- High-hydration dough in a stand mixer needs a faster speed so it slaps and folds against the bowl. [breadcode-1485; yt:JZBtmQXKWA8 9:18; 9:55; creator-rule]
- At least one stretch-and-fold during bulk builds strength; coil folds are gentle on the crumb. [breadcode-1488; yt:JZBtmQXKWA8 16:54; 19:21; creator-rule]
- Autolyse flour and water for about 2 hours before adding starter, salt and yeast; it makes the dough more extensible. — `{"autolyse_h": 2}` [breadcode-1494; yt:HSdOUZq5i6U 6:40; 7:15; creator-rule]
- Skip autolyse for an overnight dough: mix all ingredients together at once to save a step. [breadcode-1505; yt:HLPNdyGCSPk 2:14; creator-rule]
- Replace night folds with one stretch-and-fold about an hour before shaping to build strength for a better oven spring. [breadcode-1506; yt:HLPNdyGCSPk 10:38; 11:11; anecdote] *(low confidence)*
- When kneading by hand, knead in sets with a short rest between them (about 5 minutes) so the gluten aligns without overworking. [breadcode-1517; yt:LQPCXMpxO4c 2:26; creator-rule]
- Friction from a stand mixer heats dough; the heat can temporarily weaken gluten, which may recover on rest. Cool water and alternating kneading and rest limit heating. [breadcode-1519; yt:LQPCXMpxO4c 23:18; 23:55; anecdote] *(low confidence)*
- During sourdough bulk, give a stretch and fold every two or three hours; if you cannot, do at least one before shaping. — `{"fold_interval_h": [2, 3]}` [breadcode-1522; yt:LQPCXMpxO4c 10:24; creator-rule]
- The author says an autolyse builds dough strength without kneading; common practice he cites is 1 hour, 2 hours or overnight. [breadcode-1535; yt:JZBtmQXKWA8 0:00; 0:37; creator-rule]
- The dough mixed straight away, without autolyse, needed about 10 minutes of kneading in the first set before it would stretch; the planned 5 minutes was not enough. — `{"knead_min_planned": 5, "knead_min_actual": 10}` [breadcode-1536; yt:JZBtmQXKWA8 8:06; anecdote] *(low confidence)*
- After the first mix of flour, water, salt and dry yeast, let the dough rest about 15 minutes before adding strength; then a little kneading and folding builds gluten. — `{"rest_min": 15}` [breadcode-1537; yt:l0nAMbbYziI 5:12; creator-rule]
- With a stand mixer, the author kneads until the dough pulls away from the bowl, which usually takes about 15 minutes; the test doubled the kneading time, so the dough was kneaded for much longer. — `{"knead_to_release_min": 15}` [breadcode-1540; yt:LQPCXMpxO4c 3:36; 4:13; creator-rule]
- After about 5 minutes of initial mixing on low speed, let the dough rest 10 to 15 minutes so the flour absorbs the water before further kneading; kneading before that is wasted. — `{"rest_min": [10, 15], "initial_mix_min": 5}` [breadcode-1541; yt:LQPCXMpxO4c 4:13; 4:48; creator-rule]
- Let flour and water autolyse for at least 30 minutes before adding salt or starter. — `{"autolyse_min": 30}` [breadcode-1548; yt:wHL44ONu3so 1:53; creator-rule]
- One optional stretch-and-fold can be given about 7 hours into bulk. — `{"fold_at_h": 7}` [breadcode-1558; yt:36GdGT0XYwE 10:27; creator-rule] *(low confidence)*
- Autolyse overnight at room temperature with 500 g flour and 350 g water (75 % hydration), then add salt and starter in the morning. — `{"flour_g": 500, "water_g": 350, "hydration_pct": 75}` [breadcode-1577; yt:yYkTrGHNW2w 16:30; creator-rule]
- Incorporate the starter by hand for about two minutes, until no starter chunks remain, without developing strength. — `{"mix_min": 2}` [breadcode-1578; yt:yYkTrGHNW2w 18:56; creator-rule]
- Stir salt into the dough for about one minute only to homogenise it, not to build strength. — `{"mix_min": 1}` [breadcode-1579; yt:yYkTrGHNW2w 17:04; creator-rule]
- Once a dough stops spreading in its container, switch from full stretch-and-folds to gentle coil folds to add structure without pulling hard. [breadcode-1603; yt:nQWlG1buMM8 19:46; anecdote]
- After mixing, rest the dough about 15 minutes; gluten then forms with only brief stirring and no kneading. — `{"rest_min": 15}` [breadcode-1611; yt:0kFYyj3WKpE 6:37; creator-rule]
- Wet hands reduce stickiness; bench kneading without added flour builds strength in the dough. [breadcode-1612; yt:0kFYyj3WKpE 8:27; 9:04–9:43; creator-rule]
- Use gentle coil folds with wet hands, lifting and turning the dough without degassing it. [breadcode-1614; yt:0kFYyj3WKpE 14:16; creator-rule]
- After mixing, let the dough rest about 15 minutes so the gluten forms without kneading. — `{"rest_min": 15}` [breadcode-1624; yt:dY9GeoZX8DY 5:45; creator-rule]
- Stir flour and water for about a minute until no dry chunks remain, then leave the dough covered for about 15 minutes (autolyse) before kneading further. — `{"rest_min": 15}` [breadcode-1636; yt:WOZLvsV1M8s 0:33–1:06; creator-rule]
- Autolysis improves handling, flavour and browning of the bread; these are biochemical reactions in the flour and water. [breadcode-1637; yt:WOZLvsV1M8s 0:00; creator-rule]
- Mix the stiff starter into the dough by hand for about 5 minutes until no large chunks remain; a stand mixer on medium speed takes about 5 minutes, a 15-minute rest, then 5 more minutes. — `{"mix_min": 5, "rest_min": 15, "second_knead_min": 5}` [breadcode-1655; yt:36GdGT0XYwE 4:55; creator-rule]
- The guest's routine in the last one or two weeks of bulk: about two large stretch-and-folds, then one to three small coil folds. — `{"large_folds": 2, "small_folds": [1, 3]}` [breadcode-1662; yt:nQWlG1buMM8 19:46; anecdote]
- Rest the dough about 15 minutes between strength steps (before bench kneading and before lamination). — `{"rest_min": 15}` [breadcode-1667; yt:wHL44ONu3so 8:23; 14:10; creator-rule]
- Coil folds do not have to be done every hour; in the demonstration a gap of about 1.5 hours was acceptable. — `{"coil_fold_gap_h": 1.5}` [breadcode-1668; yt:wHL44ONu3so 23:25; anecdote]
- Shape about 30 minutes after the final coil fold. — `{"shape_after_final_fold_min": 30}` [breadcode-1671; yt:wHL44ONu3so 28:52; creator-rule]
- In a stand mixer, add the liquid before the flour so no dry flour sits at the bottom; after a first mix, put the levain under the dough and then the salt on the bottom, and the mixer picks them up. [breadcode-1678; yt:l_PLhPWE-8E 1:12–1:48; creator-rule]
- Mix, rest 15 min, knead 2–3 min, then repeat: strength builds with far less kneading. — `{"rest_min": 15, "knead_min": [2, 3]}` [breadcode-1696; yt:9fWzWpL5g6w 9:46; creator-rule]
- For fermentolyse, use about 10 % starter in summer and up to about 20 % in winter. — `{"starter_pct_of_flour": [10, 20]}` [breadcode-1697; yt:9fWzWpL5g6w 11:01; creator-rule]
- Autolyse softens the gluten network so the starter inflates it more easily; using only 5-10 % starter (of flour weight) slows the whole fermentation and gives a similar effect. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-1721; yt:lNCHzD8HxBA 6:32; 7:08; creator-rule]
- Overnight dough at 75 % hydration is rested about 10–13 h without stretch-and-folds, so strength must be built during mixing. — `{"hydration_pct": 75, "rest_h": [10, 13]}` [breadcode-1728; yt:9yI7QRpAgow 1:13; creator-rule]
- Stir about 1 min, then rest 15 min; the rest builds a window pane without kneading. — `{"mix_min": 1, "rest_min": 15}` [breadcode-1738; yt:vyDTKFtfIb0 4:14; creator-rule]
- Mixing sequence: flour, water and salt rest 60 min before a window-pane check. Starter is then mixed in for 3-4 min, rests 15 min, is bench-kneaded 3-4 min, rests 15 min, and is laminated. — `{"autolyse_min": 60, "starter_mix_min": [3, 4], "rest_min": 15, "bench_knead_min": [3, 4]}` [breadcode-1751; yt:2DFvfRIywC8 description (instructions step; creator-rule]
- The less strength built during mixing, the more stretch-and-folds are needed to build it; a shorter knead can be offset by more folds or less water. [breadcode-1765; yt:9yI7QRpAgow 6:03; creator-rule]
- Autolyse is flour and water only, rested overnight (Thursday evening to Friday morning) before the starter and salt go in; it develops dough strength and starts breaking down the flour. [breadcode-1776; yt:dl7crk2Nvj8 3:59; creator-rule]
- After autolyse, the flour-water mix should show a windowpane; if it does not, there is probably too much water and a little flour can be added. [breadcode-1785; yt:dl7crk2Nvj8 6:14; creator-rule]
- Mix the dough by hand until there are no large dry flour chunks (about 3 min in the video), and prefer hands over a machine to learn the dough feel. — `{"mix_min": 3}` [breadcode-1860; yt:3vr2_qqeAL8 2:50-4:03; creator-rule]
- Paddle-type mixing and fast mixing create friction that heats the dough; the creator says a few degrees' change in dough temperature changes how fast it ferments, so a steady dough temperature is wanted. [breadcode-1939; yt:G0nVQy-L3Bk 1:50-2:25; creator-rule]
- Slap-and-fold (tossing dough against the bowl) makes a mess and adds friction; regular stretch-and-folds build the same dough strength. [breadcode-1940; yt:G0nVQy-L3Bk 8:43; creator-rule]
- The creator's arm mixer kneaded a single batch of about 2 kg flour with about 1.5 kg water (about 75 % hydration) into one dough; he states the machine is rated for roughly 3 to 4 kg of dough. — `{"flour_g": 2000, "water_g": 1500, "hydration_pct": 75, "dough_capacity_kg": [3, 4]}` [breadcode-1942; yt:G0nVQy-L3Bk 14:04; anecdote]
- Stretch and folds are only needed when the dough visibly flattens. [breadcode-1964; yt:HgIKSVMkbFg 6:11; creator-rule]
- For an overnight dough with no stretch-and-fold schedule, mix flour, water and starter together, rest about 15 min, coil-fold, and coil-fold again after about 50 min before the long ferment. — `{"rest_min": 15, "second_fold_after_min": 50}` [breadcode-2013; yt:APstLJpWOiE 2:28; creator-rule]
- In the roll method there is no autolyse: flour, water, starter and salt are mixed together and kneaded straight away. [breadcode-2045; yt:OcOpIISBK6c [2:26]; creator-rule]
- Flour and water are mixed and rested overnight (autolyse); the next morning the starter and salt are added, then 3 min of kneading and a 15 min rest. — `{"mix_min": 3, "rest_min": 15}` [breadcode-2107; yt:4VJQqOtwP_E [1:06]-[1:40]; creator-rule]
- A sticky dough gets a 5 min rest, then a 45° push-pull and lamination. — `{"rest_min": 5, "angle_deg": 45}` [breadcode-2108; yt:4VJQqOtwP_E [2:18]-[6:56]; creator-rule]
- Four-way folds build strength. [breadcode-2109; yt:4VJQqOtwP_E [8:06]; creator-rule]
- Coil folds are done only once the dough has spread; too-frequent coil folds damage the crumb. [breadcode-2112; yt:4VJQqOtwP_E [11:02]; creator-rule]
- Autolyse for about 4 h in an improvised bake without scales. — `{"autolyse_h": 4}` [breadcode-2122; yt:6x9ic4YIni8 [2:18]; anecdote] *(low confidence)*
- Mix ~1 min, knead ~10 min, then bench-knead 2-3 min without added flour so the dough sticks to the bench and gains strength. — `{"knead_min": 10, "bench_knead_min": [2, 3], "mix_min": 1}` [breadcode-2145; yt:gOxCUh_kyMQ [4:30]; creator-rule]
- Comparing little work (about 5 minutes hands-on) with much work (autolyse, benching, lamination and stretch-and-folds, about an hour) gave only a marginal difference in the bake. — `{"hands_on_min": [5, 60]}` [breadcode-2240; yt:nIOPCeLPqrM 0:36; measured]
- Hand-knead the dough until it lets go of the bowl edges, which happens after about 2 minutes; add water if it looks too dry, since flours absorb differently. — `{"release_min": 2}` [breadcode-2277; yt:-rwuxy5KS6Q 7:14; creator-rule]
- Smooth the dough on a dry counter by tapping and sliding it with the thumbs held still, so the dough drags and tightens. [breadcode-2278; yt:-rwuxy5KS6Q 9:15; creator-rule]
- Stretch and fold the dough one more time before shaping so it sticks less and releases from the bowl more easily. [breadcode-2283; yt:5--bR1mPiZE description; creator-rule]
- Before the first stretch-and-fold, the stiff-starter dough got about 15 minutes of total hand working: a 15 min rest after mixing, a 5 min bench knead, then a 15 min rest, and a lamination for final strength. — `{"rest_min": 15, "knead_min": 5, "work_min_total": 15}` [breadcode-2298; yt:I3h6heaKWC8 2:24; creator-rule]
- Autolysing flour and water overnight is a simple way to start a dough. [breadcode-2322; yt:JoAG7R9QESA 1:08; creator-rule]
- Building strength uses an autolyse, then adding starter, bench kneading, and sometimes lamination. [breadcode-2340; yt:-6dlWqFlFSo 9:09; creator-rule]
- For a dough lacking strength, knead for about 5 min, rest 5 min, and repeat, or reduce the water. — `{"min": [5, 5]}` [breadcode-2359; yt:XkoIlgnIeaQ 3:36; creator-rule]
- Naan dough needs only about 1–2 minutes of hand kneading and then a rest of about 30 minutes; it is not kneaded to a strong gluten network. — `{"knead_min": [1, 2], "rest_min": 30}` [breadcode-2443; yt:sH99_IGh0ic 2:28; creator-rule]
- Schwarzbrot is made by stirring only, with no kneading, to a dough of roughly one part flour to one part water. — `{"flour_to_water_ratio": 1}` [breadcode-2456; yt:cHwLBs9SRd4 0:32; creator-rule]
- Bench-knead the dough without adding flour to the bench; he calls this very important, because flipping the dough builds strength. [breadcode-2547; yt:Dx7xM8IThCI 0:32; creator-rule]
- Bench-knead for about two minutes (speeded up in the video), flipping the dough over each time, and stop when it sticks less to the bench. — `{"knead_min": 2}` [breadcode-2548; yt:Dx7xM8IThCI 1:37; creator-rule]
- The whole bun process took about six hours from mixing to baked buns in his kitchen. — `{"total_h": 6}` [breadcode-2558; yt:Dx7xM8IThCI 0:00; anecdote] *(low confidence)*
- After autolysis, starter and salt are added and the dough is kneaded by stretching for about three minutes. — `{"knead_min": 3}` [breadcode-2667; yt:tWIn8NHJkFU 0:33; creator-rule]
- Kneading in a machine on high speed until the dough pulls away from the bowl took about ten minutes. — `{"knead_min": 10}` [breadcode-2688; yt:0oKA_iTytiw 1:21; creator-rule]
- Stirring the wholegrain dough at very high hydration is enough; no kneading is needed, and the dough is easy to handle. — `{"hydration_pct": 95}` [breadcode-2730; yt:2NmD9KVyZCg 3:11; creator-rule]
- After autolyse, starter, salt and any poolish are added and kneaded by hand for about seven minutes. — `{"knead_min": 7}` [breadcode-2739; yt:Yki-0UoPIn4 0:35; creator-rule]
- Hand kneading gives a better feel for the dough; a kitchen machine can over-knead it. [breadcode-2740; yt:Yki-0UoPIn4 2:28; creator-rule]
- After adding the ingredients, mix until no dry flour remains; the dough stops sticking to the hands after about five minutes. — `{"mix_min": 5}` [breadcode-2747; yt:5DWcrVHK1WU 0:33; creator-rule]
- After mixing the starter dough, rest it for about 15 minutes before continuing. — `{"rest_min": 15}` [breadcode-2756; yt:5DWcrVHK1WU 0:33; creator-rule]
- Einkorn dough is mixed by hand: stir for about 3 minutes until combined; no kneading is needed. — `{"mix_min": 3}` [breadcode-2761; yt:wz7tysfSDFM 1:15; creator-rule]
- Pasta dough is homogenised by mixing rather than kneaded, then left in a bowl. [breadcode-2784; yt:A5blYerC8VE 0:34; creator-rule]
- Non-wheat breads need no kneading, only stirring, so baking with them is typically a little faster. [breadcode-2850; yt:FMP2-Qa8vXk description; creator-rule]
- Bench knead on an unfloured surface: pull the dough and fold it onto itself; each fold adds strength, and no extra flour is used. [breadcode-2916; yt:U59OdOmemWM description; creator-rule]
- Bench kneading is also practice for shaping sticky dough. [breadcode-2917; yt:U59OdOmemWM description; creator-rule] *(low confidence)*
- In his routine, lamination comes after a 15-minute rest that follows bench kneading, and before bulk fermentation until doubled. — `{"rest_before_lamination_min": 15}` [breadcode-2933; yt:7Nztji3srjQ description; creator-rule]
- After kneading in the starter for 5 min, rest 15 min, bench knead, rest 15 min more, then fold the dough over itself (lamination) before bulk. — `{"knead_min": 5, "rest_min": 15, "rest_2_min": 15}` [breadcode-2937; yt:MyGMF5sLg1U description; creator-rule]
- In his sourdough routine, the autolyse starts 5 hours after feeding the starter (1:1:1), lasts 1 hour, then the starter is added and the dough is kneaded 5 minutes until incorporated. — `{"feed_ratio": "1:1:1", "autolyse_delay_h": 5, "autolyse_h": 1, "knead_min": 5}` [breadcode-2941; yt:U59OdOmemWM description; creator-rule]
- In his routine, the dough rests 15 minutes after the initial knead, is bench-kneaded, then rests another 15 minutes before lamination. — `{"rest_min": [15, 15]}` [breadcode-2942; yt:U59OdOmemWM description; creator-rule]
- A tongue-in-cheek 'trick' in the video says a dough stops being sticky with a sequence of mixing 1 min, kneading 2 min, rests of 10 min, then 1 more minute of kneading, rests of 5 min, and a final 5-minute rest after rounding. Presented as a joke; no control was made. — `{"mix_min": 1, "knead_min": 2, "rest_min": [5, 10]}` [breadcode-2975; yt:UyaoezmQjGw 0:00; anecdote] *(low confidence)*

### mixing.ancient-grain-gluten

- Einkorn and emmer doughs developed little gluten and were sticky. Emmer stuck less than einkorn. He compared the handling with rye, but the transcript is unclear on that comparison. [breadcode-1925; yt:PehhnV1lW2o 16:40; 19:28; 20:58; measured] *(low confidence)*

### mixing.arm-mixer-kneading

- In side-by-side tests on the same flour, the arm mixer's dough looked well developed by about 5 min at 60 % hydration, while the Bosch stand mixer's dough was not together after 10 min on low speed; at 80 % hydration the arm mixer's dough came together after about 20 min, while the Bosch dough still stuck to the bowl. — `{"hydration_pct": [60, 80], "mix_min": [5, 20]}` [breadcode-1914; yt:G0nVQy-L3Bk 6:52-12:16; measured] *(low confidence)*

### mixing.autolyse

- An extended autolyse (flour and water rested before adding leaven) gives the dough an enzymatic head start and is generally recommended. [breadcode-0042; SF Ch 2 How sourdough works; creator-rule]
- Reducing the amount of leavening (starter) is an equally effective alternative to extended autolyse: the same biochemical reactions run at roughly the same rate, without mixing the dough several times. [breadcode-0043; SF Ch 2 How sourdough works; creator-rule]
- Autolysis is used to shorten the main fermentation of wet dough, which would otherwise break down quickly. [breadcode-0402; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- With a low starter dose, enzymes have enough time to break down the flour, so autolysis can be skipped. [breadcode-0436; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Autolysis is mixing only flour and water and resting them about 30 min up to several hours before adding starter and salt. — `{"rest_min": 30}` [breadcode-0437; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Autolysis extends flour-water contact and enables enzyme reactions that improve taste and dough characteristics. [breadcode-0439; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Autolysis starts enzyme reactions before microbial fermentation; example: 2 h autolysis plus 5 h fermentation gives dough ready for proofing. — `{"autolyse_h": 2, "ferment_h": 5}` [breadcode-0448; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Adding salt and starter to an autolysed dough is laborious and amounts to kneading the dough again. [breadcode-0449; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Autolyse of flour and water overnight at room temperature works well, but the mixture should not stay at room temperature longer than 24 h. — `{"max_h": 24}` [breadcode-1368; yt:o_xtVmR4vlc 6:11; creator-rule]
- Autolyse flour and water for about 1 h without developing strength, then add starter and salt. — `{"autolyse_h": 1}` [breadcode-1390; yt:NMglhwp2lNs 3:40-4:26; 6:14; creator-rule]
- Autolyse (flour and water rested before adding starter) is typically 1-2 hours, sometimes overnight, and softens the gluten network. — `{"autolyse_h": [1, 2]}` [breadcode-1790; yt:lNCHzD8HxBA 6:32; creator-rule]
- For yeast-only doughs a long autolyse is recommended, because without lactic acid bacteria the gluten is not softened and the dough inflates less. [breadcode-1791; yt:lNCHzD8HxBA 14:56; creator-rule]
- Because the dough is kneaded very little, the initial autolyse rest is extended to about 1.5 h so the wheat gluten absorbs the water and the dough comes together. — `{"autolyse_h": 1.5}` [breadcode-1858; yt:3vr2_qqeAL8 description; 4:38; creator-rule]
- After mixing, let the dough rest about 15 minutes; the gluten network forms on its own, so there is no need to knead much. — `{"rest_min": 15}` [breadcode-1949; yt:t3GkfPxQxik 4:19; creator-rule]
- An overnight autolyse (flour and water only, room temperature) builds dough strength by itself and starts breaking down the flour. [breadcode-2018; yt:lJ38fY3ZEQk 1:08; creator-rule]
- An autolyse (flour and water rested before salt and yeast) builds gluten and breaks down the flour, so the yeast can digest it more easily and the bread becomes fluffier. — `{"autolyse_h": 7}` [breadcode-2180; yt:EbO3blOpGks 1:58; creator-rule]
- The flour-and-water mixture for the three-dough test rests about 2 hours at room temperature before the starter or yeast is added; the creator says this gives a fluffier bread and better browning. — `{"autolyse_h": 2}` [breadcode-2292; yt:bJxpCVc2wYE 1:11; creator-rule]
- Flour and water can be autolysed overnight and combined with the starter in the morning. [breadcode-2373; yt:-6dlWqFlFSo 7:56; creator-rule]
- Mix everything, rest 5–15 min so the flour hydrates and gluten starts to form, then knead about 5 min in short bouts. — `{"rest_min": [5, 15], "knead_min": 5}` [breadcode-2627; yt:ep0mua88Sa8 2:23; creator-rule]
- Autolysing flour and water for about an hour before adding starter and salt builds dough strength and reduces the kneading needed. — `{"autolyse_h": 1}` [breadcode-2666; yt:tWIn8NHJkFU 0:00; creator-rule]
- Autolyse (flour and water only) for at least 15 minutes, up to about 4 h, saves kneading time. — `{"autolyse_min": [15, 240]}` [breadcode-2738; yt:Yki-0UoPIn4 0:00; creator-rule]
- Autolyse: stir flour and water only until no dry flour is left, without kneading (he says a little kneading is fine), then cover airtight. [breadcode-2862; yt:XZyQVXJN4pM 0:00; 0:40; creator-rule]
- Autolyse rest is at least 30 minutes and typically no more than 2 hours; with high-protein flour he rested the dough up to about 3 hours. — `{"rest_min_min": 30, "rest_max_h": 2, "rest_max_highprotein_h": 3}` [breadcode-2863; yt:XZyQVXJN4pM description; 0:40; creator-rule]
- A two-hour autolyse without kneading developed a windowpane. — `{"autolyse_h": 2}` [breadcode-2864; yt:XZyQVXJN4pM 1:18; anecdote]
- Add the starter after autolyse and work it in. [breadcode-2865; yt:XZyQVXJN4pM 1:18; creator-rule]
- Autolyse: mix flour, water and salt, then rest at least 15 min (he often rests about 2 h, sometimes overnight in the fridge). — `{"rest_min_min": 15, "rest_typical_h": 2}` [breadcode-2892; yt:MJIeTnjI0Qg 0:00; creator-rule]
- Autolysis strengthens the gluten network without kneading, which improves oven spring. [breadcode-2893; yt:MJIeTnjI0Qg 0:00; creator-rule]
- During autolyse, stir for only about 2 min, with no stretch-and-folds; cover the dough so it does not dry out. — `{"mix_min": 2}` [breadcode-2894; yt:MJIeTnjI0Qg 0:40; creator-rule]
- Salt in the autolyse did not noticeably hinder gluten development in his tests. [breadcode-2895; yt:MJIeTnjI0Qg description; anecdote] *(low confidence)*
- Add the starter or yeast after the autolyse rest. [breadcode-2896; yt:MJIeTnjI0Qg 0:40; creator-rule]
- Autolyse 500 g flour (half bread, half whole wheat) with water for 1 h, then add starter and knead 5 min. — `{"autolyse_min": 60, "flour_g": 500}` [breadcode-2901; yt:MyGMF5sLg1U description; creator-rule]
- Description recommends resting the autolyse at least 60 minutes to test how much difference it makes. — `{"rest_min": 60}` [breadcode-2935; yt:MJIeTnjI0Qg description; creator-rule]
- During the autolyse rest gluten develops on its own, so the dough gains strength without kneading. [breadcode-2944; yt:XZyQVXJN4pM 0:40; description; creator-rule]

### mixing.autolysis

- Resting flour and water for about 30 minutes before adding starter and salt (autolysis) adds dough strength. — `{"autolyse_min": 30}` [breadcode-2454; yt:s1gM_jziXcI 3:53; creator-rule]

### mixing.bassinage

- Raise hydration gradually: start at 60 %, knead until the water is absorbed, add more and repeat (bassinage). — `{"hydration_start_pct": 60}` [breadcode-0404; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Add water slowly, not all at once; this is the bassinage method. [breadcode-2623; yt:ep0mua88Sa8 0:36; creator-rule]
- Once gluten has formed, the network traps extra water, so very high hydration becomes workable. [breadcode-2624; yt:ep0mua88Sa8 1:13; creator-rule]

### mixing.bench-kneading

- For high-hydration dough, bench kneading and lamination on a slightly tacky surface (without added flour) build gluten strength quickly; the test is whether the dough holds its shape and does not tear apart. [breadcode-1895; yt:U9_GVHuI1Ro 4:02; creator-rule]
- Bench kneading (slap-and-fold on the counter, no flour) develops about as much strength as roughly five minutes of machine kneading. — `{"equivalent_machine_min": 5}` [breadcode-2182; yt:EbO3blOpGks 4:28; extrapolation] *(low confidence)*
- Adding flour during bench kneading makes the dough harder to handle and works against gluten development. [breadcode-2183; yt:EbO3blOpGks 5:02; creator-rule]
- Bench kneading is done in two sets with about 15 minutes rest between them, using a 45-degree push-and-pull on the counter without flour. — `{"rest_min": 15}` [breadcode-2295; yt:EbO3blOpGks 6:10; creator-rule]
- Bench kneading, shaping the dough like a final loaf on the bench during mixing, adds gluten strength and is good shaping practice. [breadcode-2668; yt:tWIn8NHJkFU 1:06; creator-rule]

### mixing.coil-fold

- Coil fold: with wet hands, lift the dough from each side and fold it over itself; this is the gentlest bulk stretch-and-fold. [breadcode-2803; yt:6JQm2n4aVZc 0:00; creator-rule]
- Do a coil fold about once an hour during bulk; the more often it is done, the better the dough holds together. — `{"interval_h": 1}` [breadcode-2804; yt:6JQm2n4aVZc 0:00; creator-rule]
- Each coil fold adds strength but evens out the crumb a little, so apply one when the dough has visibly flattened. [breadcode-2805; yt:6JQm2n4aVZc description; creator-rule]

### mixing.fermentolyse

- A fermentolyse (longer bulk ferment instead of a separate autolyse) gives similar benefits with one mix. [breadcode-1381; yt:msqU-ylXWUs 6:34; creator-rule]

### mixing.fermentolysis

- The author does not recommend autolysis and prefers fermentolysis. [breadcode-0440; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Fermentolysis gives the same dough properties as autolysis without mixing twice: one fermentation of about 7 h replaces 2 h autolysis plus 5 h bulk. — `{"fermentation_total_h": 7, "autolyse_h": 2, "bulk_h": 5}` [breadcode-0450; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- Even for yeasted doughs the author no longer uses autolysis; he reduces the yeast instead. [breadcode-0465; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]

### mixing.folds

- Stretch and fold about every 2–3 h, when the dough has flattened out. — `{"fold_interval_h": [2, 3]}` [breadcode-1387; yt:msqU-ylXWUs 2:14; creator-rule]
- Coil folds are needed when the dough has flattened out; there is no fixed number. [breadcode-1391; yt:NMglhwp2lNs description; creator-rule]
- Wet hands and stretch-and-fold passes, repeated at least about four times, build gluten strength; he calls it almost like kneading. — `{"folds_min": 4}` [breadcode-1861; yt:3vr2_qqeAL8 6:55; creator-rule]
- Do stretch-and-folds only when the dough visibly flattens out; wetter dough needs more frequent folds, and each fold is harder with wet hands on sticky dough. [breadcode-1951; yt:t3GkfPxQxik 12:29; 13:05; creator-rule]
- For an overnight-style dough, one stretch and fold shortly before doubling was enough in his method. — `{"folds": 1}` [breadcode-2039; yt:vf5RbwlW8Xs 2:57; creator-rule]
- Use a coil fold with wet hands to gently re-gather the dough when it cannot be turned over in the container. [breadcode-2230; yt:iEfLO_Cl0uE 1:09; creator-rule]
- The coil fold (lift an edge, fold it over, press it to glue) is a gentler stretch-and-fold than a regular stretch and fold; the same idea works for yeast doughs and pizza. [breadcode-2563; yt:gMbZeUIVzZY 1:50; creator-rule]
- Do the next fold when the dough visibly flattens out, not on a fixed clock. [breadcode-2564; yt:gMbZeUIVzZY 1:13; creator-rule]
- More folds even out and reduce the large gas pockets, so fewer folds favour a more open crumb. [breadcode-2566; yt:gMbZeUIVzZY 3:36; creator-rule]
- Wet your hands before stretching and folding; the water keeps the dough from sticking. [breadcode-2659; yt:gMbZeUIVzZY 1:13; creator-rule]
- Coil folds done about every hour during bulk fermentation add strength; he says they do not degas the dough and calls them the best way to do stretch-and-folds. — `{"fold_interval_h": 1}` [breadcode-2670; yt:tWIn8NHJkFU 1:41; creator-rule]
- The first stretch-and-folds come about 30 minutes after kneading, when the dough has flattened. — `{"first_fold_min": 30}` [breadcode-2741; yt:Yki-0UoPIn4 2:28; creator-rule]
- Repeat stretch-and-folds once every hour during bulk fermentation. — `{"fold_interval_h": 1}` [breadcode-2742; yt:Yki-0UoPIn4 3:04; creator-rule]
- A wet-hand fold lifts the dough off the container sides without degassing it; he calls it the best method. [breadcode-2743; yt:Yki-0UoPIn4 3:04; creator-rule]

### mixing.kneading

- After mixing, einkorn and emmer doughs were kneaded for about 5-10 minutes in a stand mixer. He says they have less gluten and do not stick together as much as wheat. — `{"knead_min": [5, 10]}` [breadcode-1946; yt:PehhnV1lW2o 9:45; 16:00; creator-rule]

### mixing.lamination

- A second lamination (bench-kneading style) is needed only for high-hydration doughs; with less water it can be skipped. [breadcode-2020; yt:lJ38fY3ZEQk 3:34; creator-rule]
- Lamination (laying the wetted dough flat and folding it over itself so the sticky side bonds) adds strength. [breadcode-2669; yt:tWIn8NHJkFU 1:41; creator-rule]
- Laminate on a slightly wetted surface: spread the dough across it, then fold layers over each other; more contact area gives a stronger bond and more dough strength. [breadcode-2923; yt:7Nztji3srjQ description; creator-rule]

### mixing.no-knead

- No-knead dough: 1000 g bread flour, 700 g water (70 %, depending on the flour), 200 g active starter (20 %) and 20 g salt (2 %). — `{"hydration_pct": 70, "starter_pct_of_flour": 20, "salt_pct": 2}` [breadcode-2226; yt:iEfLO_Cl0uE description; creator-rule]
- Mix flour and water without salt or starter on Thursday evening, then add salt and starter on Friday morning before bulk. [breadcode-2228; yt:iEfLO_Cl0uE description; creator-rule]

### mixing.rest

- After adding starter and salt, rest the dough about 15 min before bench kneading so the flour absorbs the added water. — `{"rest_min": 15}` [breadcode-1421; yt:NMglhwp2lNs 12:24-13:00; creator-rule]

### mixing.temperature

- At this small batch size, the two mixers' doughs read the same temperature after the first 10 min of the 80 % test, although the arm mixer's outside was slightly warmer than the Bosch; the creator says the test is too small to settle whether the motors differ in heat. [breadcode-1915; yt:G0nVQy-L3Bk 11:40; measured] *(low confidence)*

### mixing.windowpane

- A sticky dough is normal for this formula; if a window-pane test fails, too much water is the likely cause. [breadcode-2181; yt:EbO3blOpGks 3:54; creator-rule]
