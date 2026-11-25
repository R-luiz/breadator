# troubleshooting — The Bread Code / The Sourdough Framework

Claims: 97 (book 41, videos 53, repos 3). Evidence: creator-rule 78, anecdote 16, measured 2, literature 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

Table 7.2 (`sf-table-7-2-fermentation-effects.csv`) is the core diagnostic: under-fermented → gummy bottom, "craters", pale taste, vertical spring from steam only; over-fermented → many tiny alveoli, gummy, very sour, flat "pancake"; perfect → even alveoli, balanced taste, upward spring. §12.4 maps crumb types to causes. Video _cb4vCQ2shk (2026) and HlgA2O3J0o0 add that rice flour in the banneton hides over-fermentation stickiness (an update to earlier rice-flour advice).

## Claims

### troubleshooting

- Unlike a yeast-only dough, sourdough cannot simply be punched down and left to continue fermenting; it can overferment into a sticky, unworkable mess. [breadcode-0015; SF Ch 1 The history of sourdough; creator-rule]
- If the first bread fails, the starter is a likely cause (wrong yeast/bacteria balance); keep feeding it once a day, and consider a stiff starter to boost the yeast share. [breadcode-0136; SF Ch 3 Making a sourdough starter; creator-rule]
- Converting a moldy starter sample to liquid is suggested as a remedy: the anaerobic environment blocks aerobic mold, and with each feeding the mold spores decline because they cannot reactivate. [breadcode-0206; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- No recipe can be followed blindly: adapt the formula to your local flour, tools and environment. [breadcode-0347; SF Ch 7 Wheat sourdough §7.1 The process; creator-rule]
- A fool's crumb, with several large holes instead of evenly distributed small ones, can indicate uneven fermentation or improper shaping. [breadcode-0844; SF Glossary > Fool’s Crumb; creator-rule]
- If mold reappears when restarting a starter, the flour batch may be very moldy; try a different flour. [breadcode-0927; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; anecdote]
- Mold also forms on the container's edges, where no active starter microorganisms reach. [breadcode-0929; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; anecdote]
- Remedy: take a mold-free portion of starter, feed it flour and water; after a few feedings the starter returns to normal. Only 1–2 g of starter is needed, since it contains millions of microorganisms. — `{"transfer_g": [1, 2]}` [breadcode-0930; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; creator-rule]
- In a single experiment, a dough made directly from a starter unfed for 30 days at room temperature tore completely after about 24 hours. — `{"unfed_days": 30, "tear_h": 24}` [breadcode-0966; SF Ch 12 Troubleshooting §12.2.4 My dough completely tears after a long f; anecdote]
- The crumb structure reveals how well fermentation went and flags technique errors. [breadcode-1018; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure; creator-rule]
- An open crumb with large alveoli requires touching the dough as little as possible, because each interaction degasses it and merges alveoli. [breadcode-1019; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Perfect f; creator-rule]
- A honeycomb crumb holds fillings such as jam better than an open crumb. [breadcode-1026; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Perfect f; creator-rule] *(low confidence)*
- To diagnose a baking problem, check the temperature first, then the steaming technique. [breadcode-1071; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Baked wit; creator-rule]
- If a sourdough does not rise after a few hours, the starter may be under-fed: feed it the evening before, use about 40 % sourdough to flour, and expect visible rise within about 3 h at a steady, moderate temperature. — `{"starter_pct_of_flour": 40, "expected_rise_h": 3}` [breadcode-1126; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Open crumb needs well-developed gluten and higher hydration; add stretch-and-folds up to shaping so the gas is held in the dough. [breadcode-1127; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Too much yeast or too long a fermentation over-ferments pizza dough. [breadcode-1208; repo:pizza-dough README.md; creator-rule]
- Rest the dough about 15 minutes before the window-pane test; 20 minutes was used earlier. [breadcode-1274; yt:MrOM_D3hnpc 34:26; creator-rule]
- A loaf with few gas pockets and little oven spring is usually under-fermented; lengthen bulk fermentation rather than shaping sooner. [breadcode-1317; yt:KRO9EcIyXSI 18:05; creator-rule]
- Over-fermented dough becomes sticky, tears when touched and cannot be shaped; the fix he uses is to put it straight into a loaf pan. [breadcode-1363; yt:o_xtVmR4vlc 3:51; creator-rule]
- A dough that spreads like a pancake was over-fermented or had too much water for its flour; he saves it by tossing it into a loaf pan. [breadcode-1373; yt:o_xtVmR4vlc 17:54; creator-rule]
- Scoring is easier on a slightly stiff, cold dough; cutting a warm, soft dough tears it and reduces oven spring. [breadcode-1396; yt:NMglhwp2lNs 32:26; creator-rule]
- If the fermentation sample does not rise as expected, check that the sample is at the same temperature as the dough and that the dough was fully homogenised. [breadcode-1429; yt:o_xtVmR4vlc 8:30; creator-rule]
- Adding more flour does not rescue an over-fermented sticky dough. Hendrik jokingly suggests a loaf pan as a last resort, but in his own test the dough would not hold shape, so he refrigerated it instead. [breadcode-1472; yt:qM9BXKaIr60 20:48; 29:13-29:48; anecdote]
- An over-fermented sticky dough can be put in a loaf pan and baked. [breadcode-1513; yt:HLPNdyGCSPk 12:51; creator-rule]
- If a scored loaf spreads flat like a pancake, it is likely over-fermented: the gluten has broken down and the dough can no longer hold its shape. [breadcode-1525; yt:HLPNdyGCSPk 18:40; creator-rule]
- Overproofed dough that starts to collapse gives an unpleasant, poor crumb. [breadcode-1590; yt:nQWlG1buMM8 1:44; creator-rule]
- If the dough does not hold together and tears when stretched during bench kneading or folding, it likely has too much water or the starter is too sour. [breadcode-1666; yt:wHL44ONu3so 3:09; 6:29; 22:45; 28:03; creator-rule]
- It takes about five attempts to make a decent sourdough loaf. [breadcode-1706; yt:9fWzWpL5g6w 20:58; anecdote] *(low confidence)*
- A loaf whose core stayed near 9 °C after fridge proofing came out flat with no blisters. — `{"core_c": 9}` [breadcode-1733; yt:9yI7QRpAgow 12:49; anecdote]
- Over-proofed dough loses its ability to hold gas: the loaf comes out flat with few blisters. [breadcode-1769; yt:9yI7QRpAgow 12:49; creator-rule]
- If the dough is sticky at shaping, it was probably fermented too long or made with too much water; ferment less next time. [breadcode-1783; yt:dl7crk2Nvj8 9:36; creator-rule]
- If flour is added during mixing, the no-knead method no longer works; knead for about 10 minutes instead. — `{"knead_min": 10}` [breadcode-1784; yt:dl7crk2Nvj8 7:21; creator-rule]
- Below about pH 4 the flour structure breaks down; the dough stops holding gas and collapses when shaped. [breadcode-1813; yt:dQEebR6pIng 13:02; anecdote] *(low confidence)*
- Poor oven spring can come from too much water, short or long fermentation, too hot or too cold a bake, insufficient steam, or bad scoring. [breadcode-2006; yt:qRX3rulyXS8 1:51; creator-rule]
- Retarding too long produces flat rolls. [breadcode-2048; yt:OcOpIISBK6c [5:28]; creator-rule]
- If a starter stays inactive, consider restarting with a different batch of flour; whole-grain flour (whole wheat or rye) is suggested as a faster start. [breadcode-2096; yt:_cb4vCQ2shk [3:25]–[3:39]; creator-rule]
- An overproofed loaf flattens into a pancake, gets sticky and tastes very sour. [breadcode-2102; yt:_cb4vCQ2shk [10:53]; creator-rule]
- A sticky dough has too much water or too much acid; add flour a tablespoon at a time until a windowpane forms. [breadcode-2111; yt:4VJQqOtwP_E [10:30]; creator-rule]
- The dough tore during proof after he had given it too much tension while shaping; he then scored the torn spots with a knife before baking. [breadcode-2124; yt:6x9ic4YIni8 [6:48]; anecdote] *(low confidence)*
- Ear problems come from under-proofing, under-bulk, or insufficient steam. [breadcode-2363; yt:_Rq512vOyqU description; creator-rule]
- A dense crumb has several possible causes: underfermentation, too little dough strength, flour type, baking problems or overfermentation; the crumb itself is the best clue for which one. [breadcode-2535; yt:RjTGiqefbAI 7:00; anecdote]
- A dough that is hard to shape points to over- or under-fermentation; hitting the sweet spot takes experience and many failures. [breadcode-2543; yt:2tOw0eyuEsU description; creator-rule]
- A dough that tears while being folded is over-fermented: acid has damaged the gluten. Ferment less, or use a less acidic starter. [breadcode-2567; yt:gMbZeUIVzZY 4:13; creator-rule]
- Over-pre-shaping tears the dough surface; rest it about 15 min and then continue. — `{"rest_min": 15}` [breadcode-2618; yt:HYCaVzqOveU 3:20; creator-rule]
- A dough that will not round up may have too much water for its flour; add a little flour, rest about 15 min, and try again. — `{"rest_min": 15}` [breadcode-2619; yt:HYCaVzqOveU 4:29; creator-rule]
- Over-fermented dough: lactic acid weakens the gluten network, making the dough sticky and unable to hold shape. Long bulk, warm conditions or a too-sour starter cause it. [breadcode-2639; yt:P59OfgJ4Dps 3:25; 6:26 (summary causes); creator-rule]
- Fixes for over-fermentation: shorten bulk, lower the temperature, or feed the starter at a higher ratio (1:2:2 or 1:5:5 instead of 1:1:1) to reduce its acidity. — `{"ratio": "1:2:2 or 1:5:5 (instead of 1:1:1)"}` [breadcode-2640; yt:P59OfgJ4Dps 3:25; creator-rule]
- An overnight bulk inside an oven with only the light on, estimated at about 40 °C, was too warm and over-fermented the dough. — `{"temp_c": 40}` [breadcode-2642; yt:P59OfgJ4Dps 2:51; anecdote]
- A sticky, over-fermented dough that can no longer be shaped can be rescued by baking it in a loaf pan, but it will expand less in the oven because the gluten network has broken down. [breadcode-2663; yt:P59OfgJ4Dps 1:22; creator-rule]
- A dough with too little strength bakes out as a flat pancake; common causes are too little water or over-fermentation. [breadcode-2673; yt:tWIn8NHJkFU description; creator-rule]
- Large air pockets towards the top of the loaf indicate too little steam during the bake. [breadcode-2857; yt:QbdwTFwhgWo description; creator-rule]
- A starter that has become too sour damages gluten and makes the bread flat. [breadcode-2909; yt:PR7OM4KCcHI description; creator-rule]
- A bulk fermentation that is too short shows as large air pockets in the crumb. [breadcode-2920; yt:GX301zZ24BM 0:00; creator-rule]
- A bulk fermentation that is too long gives a flat loaf with very tiny bubbles that spreads in the oven. [breadcode-2921; yt:GX301zZ24BM 0:00; creator-rule]
- Slice the loaf after baking and read the crumb to diagnose the bulk fermentation. [breadcode-2922; yt:GX301zZ24BM 0:00; creator-rule]
- A slight spread when the dough comes out of the banneton is normal and usually recovers in the oven. [breadcode-2931; yt:z8jcGuQ2d9Q 0:00; creator-rule]
- A weak or immature starter produces a dense, flat or gummy loaf. [breadcode-2947; yt:MsNy8be0m_Q description; creator-rule]
- Flat, dense or gummy loaves usually come from fermentation that ran too long or too short. [breadcode-2983; yt:G-CDMMzavKI 0:00; creator-rule]
- Three checks fix most of these problems: starter, bulk fermentation and proofing. [breadcode-2984; yt:G-CDMMzavKI 0:00; creator-rule]

### troubleshooting.acidity

- A pH meter check found the discard loaf less sour than other breads he baked the same day. The check is qualitative; no pH values are given. [breadcode-1901; yt:xmt3eXzOQLM 19:06; anecdote] *(low confidence)*

### troubleshooting.flat-bread

- A flat loaf usually means the gluten network has broken down fully during fermentation; the dough is then a fully fermented food that may taste too sour and be less fluffy. [breadcode-0970; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; creator-rule]
- Ciabatta dough tends to flatten somewhat after shaping. [breadcode-1012; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out during shapi; creator-rule]

### troubleshooting.over-fermentation

- At the end of the proof the dough measured about pH 4 and the creator says it is close to over-fermentation, so it should be baked promptly. [breadcode-1904; yt:bi-zqG5y1I8 6:19; measured]
- A dough whose pH had fallen below about 4 after more than 24 h in the fridge is in the creator's 'critical zone', where over-proofing risk is high. [breadcode-1922; yt:O0REFqmB0rs 5:06; anecdote] *(low confidence)*

### troubleshooting.over-kneading

- A dough kneaded for about 3 h in a stand mixer, with extra flour added along the way, would not come together and baked very white; the creator suspects the starch or gluten broke down but does not confirm the cause. [breadcode-1916; yt:G0nVQy-L3Bk 12:53-13:30; anecdote] *(low confidence)*

### troubleshooting.overferment

- If the dough does not hold its shape during shaping, it was likely fermented too far; bake it in a loaf pan instead of freestanding, which gives a less open texture. [breadcode-0575; SF Ch 7 Wheat sourdough §7.12 Shaping > Fold the dough together; creator-rule]

### troubleshooting.overfermentation

- Whole-wheat dough tended to overferment quickly; a white flour of similar gluten content performed well. Dropping the extended autolyse and reducing leavening improved the whole-wheat bread. [breadcode-0041; SF Ch 2 How sourdough works; anecdote]

### troubleshooting.overfermented

- If fermented too long, the dough is broken down, becoming sticky and flat. [breadcode-0977; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; creator-rule]
- If the dough flattens completely after removal from the banneton, it is over-fermented: bacteria have consumed much of the gluten, so the dough collapses in the oven and CO2 and water escape. [breadcode-1005; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out when removin; creator-rule]
- Dough sticking to the banneton is a symptom of over-fermentation; rice flour in the banneton only masks it and is not a real fix. [breadcode-1006; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out when removin; creator-rule]
- If dough sticks while being removed from a banneton, transfer it directly into a greased loaf pan; the pan limits flattening, giving a less fluffy but tangy loaf. [breadcode-1008; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out when removin; creator-rule]
- Dough that tears easily during shaping is over-fermented; dough that does not tear but flattens lacks dough strength. [breadcode-1010; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out during shapi; creator-rule]
- A dough that cannot be shaped can be rescued by baking it in a greased loaf pan, or a piece can be kept as starter for the next dough. [breadcode-1011; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out during shapi; creator-rule]
- Gluten rot: the gluten can no longer hold the CO2, which disperses outward, leaving smaller alveoli and a flat 'pancake' loaf whose oven spring is comparable to low-gluten einkorn dough. [breadcode-1028; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Over-fermented bread is very acidic and may taste too sour; in an informal tasting with 15-20 people this style was generally liked less. — `{"n_tasters": [15, 20]}` [breadcode-1029; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; anecdote] *(low confidence)*
- Over-fermented crumb can be gummy because bacteria have broken much of it down. [breadcode-1030; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Over-fermented dough becomes very sticky, is hard to shape, flattens when removed from a banneton and may stick to it. [breadcode-1032; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Stickiness from over-fermentation is a fermentation-control problem; controlling fermentation yields a dough that is not sticky. [breadcode-1033; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- If an over-fermented dough becomes sticky during stretch-and-fold or shaping, bake it in a loaf pan after it has risen a bit again. [breadcode-1034; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Over-fermented dough can be used to start a new starter, or poured into a heated oiled pan to make flatbread. [breadcode-1036; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule] *(low confidence)*
- Over-fermented dough (bulk too long) gets sticky and smells more sour, and bakes flat with a poor ear and a less open crumb. [breadcode-2836; yt:Cqn-VTknVO0 0:01; measured]
- If an over-fermented dough is sticky, use a little more flour when shaping, or place it in a loaf pan. [breadcode-2839; yt:Cqn-VTknVO0 description; creator-rule]
- Over-long bulk gives a sticky dough that is hard to shape, is overproofed, and gives less oven spring. [breadcode-2891; yt:6iAwFZm6nGs description; creator-rule]
- A badly over-fermented, warm, sticky pizza dough can be partly saved: gather it into one mass, cover, knead until smoother, then rest and stretch-and-fold every 30 min. — `{"fold_interval_min": 30}` [breadcode-2926; yt:x7uK-ZNXEjs 0:00; anecdote] *(low confidence)*
- Dough that sticks to the banneton and goes flat is over-fermented: bulk or proof ran too long. [breadcode-2930; yt:z8jcGuQ2d9Q 0:00; creator-rule]
- A very sour taste is another sign of over-fermentation. [breadcode-2932; yt:z8jcGuQ2d9Q 0:00; creator-rule]

### troubleshooting.overproof

- An overproofed dough becomes very sticky, tends to collapse in the oven and does not spring back. [breadcode-0606; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- It is better to end proofing too early than too late. [breadcode-0607; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]

### troubleshooting.scoring

- A ragged incision can prevent the dough from rising properly in the oven and makes the ear unlikely. [breadcode-0642; SF Ch 7 Wheat sourdough §7.14 Scoring; creator-rule]

### troubleshooting.tang

- More tang is obtained mainly by a longer fermentation, which requires a loaf pan or a flour that tolerates long fermentation (a strong flour). [breadcode-0981; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Baking the loaf twice (about 30 min, cool, then 20-30 min more) gives a more sour bread because less acid is lost during baking. — `{"bake_1_min": 30, "bake_2_min": [20, 30]}` [breadcode-0988; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; literature]
- For less sour bread, ferment for a shorter time. [breadcode-0997; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; creator-rule]

### troubleshooting.underfermented

- 'Underproofed' is a misleading term: it refers only to the final proof, and a perfectly timed bulk fermentation gives a good crumb even without a final proof. [breadcode-1045; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- An underfermented dough is usually caused by a problem during bulk fermentation or earlier, in the starter. [breadcode-1047; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- An underfermented dough has very large air pockets and is partially wet and gummy in some areas. [breadcode-1048; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- The 'fool's crumb' is an ear with several overly large alveoli caused by too short a fermentation. [breadcode-1050; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- Alveoli aid heat transfer in a dough; underfermented dough transfers heat poorly, leaving raw-looking gummy areas, and baking longer to fix them over-bakes other parts. [breadcode-1051; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
