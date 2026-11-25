# bulk — The Bread Code / The Sourdough Framework

Claims: 279 (book 84, videos 193, repos 2). Evidence: creator-rule 247, anecdote 25, measured 6, extrapolation 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

- **Endpoint, not clock.** The book makes the aliquot jar the beginner's endpoint: take a sample into a straight-sided jar after building initial strength, mark it, and end bulk when it reaches a rise target set by flour protein — Table 7.3: 8–10 % protein → +25 %, 10–12 % → +50 %, 12–15 % → +100 %, >15 % → >100 % (`sf-table-7-3-aliquot.csv`). Begin at +25 % and work up to +100 % over successive bakes. Around 80 % of wheat protein is gluten.
- **Aliquot limits (book §7.9):** cylindrical container; water at room temperature (the small sample cools/warms faster than the mass — hot water makes it lag, cold water makes it lead); keep it next to the dough; unreliable with large ambient swings or doughs over ~10 kg flour (fermentation is exothermic). Video JZBtmQXKWA8 adds an estimate that a 2 °C dough–sample gap can shift timing by 3–4 h.
- **pH route:** example breakpoints 6.0 at mixing → 4.10 at divide → 4.05 at shaping → 3.80 after proof → 3.90 after baking (Table 7.4, one baker's process); calibrate your own values. Video dQEebR6pIng: when dividing several loaves stop at a slightly higher pH (~4.30 vs 4.20) because the dough keeps fermenting on the bench.
- **Target duration** 8–12 h of bulk, set via inoculation and temperature (see `starter.md`). Prefer erring long rather than short (flavour), but an over-fermented dough becomes sticky and spreads; rescue it in a greased loaf pan (≥30 min proof).
- **Signs:** surface bubbles, tacky-not-sticky feel, smell of a ripe starter just before collapse, pickled taste; over-fermented dough loses bubbles (leaky).
- **Videos vs book:** most videos use +50 % as the default endpoint and +75–100 % only for ~14–15 % protein flour; the November 2026 videos return to "start at +25 %" (_cb4vCQ2shk, CqlQU3ehU6w). Old repo tiers were lower (+10–20 / 20–40 / 40–70 / 70–110 %).

## Claims

### bulk

- Bulk fermentation is done on multiple loaves' worth of dough in one mass; the stop point is read from the dough's signs, not from a fixed time. [breadcode-0350; SF Ch 7 Wheat sourdough §7.1 The process; creator-rule]
- Scale any formula by recalculating each ingredient as a percentage of total flour (baker's math). [breadcode-0388; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- High-hydration dough can become too extensible during fermentation and lose strength; stretch-and-folds are used to compensate, which means more work. [breadcode-0397; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- More starter makes bulk fermentation faster; less starter makes it slower. [breadcode-0424; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Target a bulk fermentation of 8-12 h for most flours. — `{"bulk_h": [8, 12]}` [breadcode-0432; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- At a 20 % starter dose and about 25 °C, fermentation can finish in as little as 5 h; a longer fermentation makes the dough leaky. — `{"starter_pct_of_flour": 20, "temp_c": 25, "bulk_h_min": 5}` [breadcode-0445; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Targeting an 8 h bulk fermentation, the author uses a 10 % starter dose. — `{"bulk_h": 8, "starter_pct": 10}` [breadcode-0458; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- Targeting a 12 h bulk fermentation, the author uses about 5 % starter. — `{"bulk_h": 12, "starter_pct": 5}` [breadcode-0459; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- On a very warm day, 10 % starter makes the dough ready in about 5 h; one more hour breaks it down too far. — `{"starter_pct": 10, "bulk_h": 5}` [breadcode-0460; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- On a very warm day, use about 5 % starter to bring bulk back to the 8-12 h window. — `{"starter_pct": 5, "bulk_h": [8, 12]}` [breadcode-0461; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- The dose and timing figures are approximate and depend on flour and starter; adjust after several bakes. [breadcode-0463; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- A fermentation sample (covered later in the chapter) is more precise than relying on time alone. [breadcode-0464; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- Bulk fermentation is the first rise after mixing; a common target is about a 25 % increase in dough volume, which can then be raised to find the flour's best point. — `{"bulk_rise_pct": 25}` [breadcode-0824; SF Glossary > Bulk Fermentation; creator-rule]
- Because the gluten (protein) content of wheat flour sets how long dough tolerates bulk fermentation, a higher-protein flour can bulk ferment longer; a longer bulk improves flavour and texture. [breadcode-0825; SF Glossary > Bulk Fermentation; creator-rule]
- The best bulk rise is specific to each flour and has to be found flour by flour. [breadcode-0826; SF Glossary > Bulk Fermentation; creator-rule]
- Low-gluten flours such as rye need caution with long bulk fermentation, since the dough can become too sticky, collapse and lose its shape. [breadcode-0827; SF Glossary > Bulk Fermentation; creator-rule]
- Over-fermenting wheat or spelt dough breaks down most of its gluten, making it very sticky; the bread bakes flat with many tiny pockets, as trapped gas diffuses out during baking. [breadcode-0860; SF Glossary > Over Fermenting; creator-rule]
- If over-fermentation is noticed during bulk, place the dough in a loaf pan and bake it after a rest of 30 to 60 minutes. — `{"rest_min": [30, 60]}` [breadcode-0861; SF Glossary > Over Fermenting; creator-rule]
- Ferment one loaf at a time when aiming for an open crumb. [breadcode-1020; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Perfect f; creator-rule]
- For maximum openness, do not touch the dough for at least 1-2 hours before shaping. — `{"rest_before_shaping_h": [1, 2]}` [breadcode-1022; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Perfect f; creator-rule]
- To stop fermentation in time, judge it by the volume increase of a small fermentation sample taken from the dough. [breadcode-1037; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Start judging at about a 25 % volume increase of the main dough or sample. — `{"volume_increase_pct": 25}` [breadcode-1038; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- A strong flour with 14-15 % protein can safely be fermented to about a 100 % volume increase. — `{"protein_pct": [14, 15], "volume_increase_pct": 100}` [breadcode-1039; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Fix underfermentation by fermenting longer, within the limit set by flour breaking down in water; the author aims for a bulk of about 8-12 h. — `{"bulk_h": [8, 12]}` [breadcode-1052; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- At about 22 °C with a 20 % starter dose, bulk fermentation is ready after about 10 h. — `{"temp_c": 22, "starter_pct_of_flour": 20, "bulk_h": 10}` [breadcode-1090; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- At about 25 °C, reduce the starter to about 10 % of flour. — `{"temp_c": 25, "starter_pct_of_flour": 10}` [breadcode-1091; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- At about 25 °C with an unreduced starter dose, fermentation would finish in about 4-7 h and the dough becomes unstable. — `{"temp_c": 25, "bulk_h": [4, 7]}` [breadcode-1092; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- At rapid fermentation pace the usable timing window can shrink from about 1 h to about 20 min. — `{"window_min": [20, 60]}` [breadcode-1093; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- At about 30 °C, bulk fermentation can be complete in 2-4 h with a 10-20 % starter dose, and fridge proofing becomes almost impossible. — `{"temp_c": 30, "starter_pct_of_flour": [10, 20], "bulk_h": [2, 4]}` [breadcode-1094; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- In hot conditions, reduce the starter to about 1-5 % of flour to slow fermentation and widen the timing window. — `{"starter_pct_of_flour": [1, 5]}` [breadcode-1096; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- Aim for an overall bulk fermentation of at least 8-10 h in hot conditions, reducing the starter to achieve it. — `{"bulk_h": [8, 10]}` [breadcode-1097; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- An aliquot jar (small fermentation sample) shows progress reliably only when its temperature equals the ambient temperature; otherwise it heats or cools faster than the dough. [breadcode-1099; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- It is better to stop bulk fermentation slightly early than late. [breadcode-1100; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- Stretch-and-folds during bulk help develop a feel for the dough. [breadcode-1101; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- Mischbrot: ferment about 10 h at about 22 °C. — `{"bulk_h": 10, "temp_c": 22}` [breadcode-1165; repo:the-bread-code recipes/sourdough/mischbrot-aka-graubrot.md; creator-rule]
- Pizza bulk fermentation ends at about 50 % volume increase, after 8–12 h. — `{"rise_pct": 50, "bulk_h": [8, 12]}` [breadcode-1211; repo:pizza-dough README.md; creator-rule]
- Bubbles and open texture come from fermentation gas, not from kneading; a dough that is fermenting visibly bubbles. [breadcode-1217; yt:MrOM_D3hnpc 58:25; creator-rule]
- Dough that spreads fast in bulk needs extra stretch-and-folds or a lamination; a dough that holds its shape can go straight to bulk. [breadcode-1227; yt:MrOM_D3hnpc 1:21:48; creator-rule]
- Bulk readiness is judged by volume increase on a marked container, not by a fixed timer alone; an aliquot jar reads wrong when its temperature differs from the dough's. [breadcode-1228; yt:MrOM_D3hnpc 1:24:13; creator-rule]
- A sample in a small jar heats or cools faster than the main dough, so its reading can mislead; judge dough by look and feel as well. [breadcode-1229; yt:MrOM_D3hnpc 1:26:38; creator-rule]
- For a beginner, aim for a 20 to 30 % volume increase in bulk; a low-starter, cool-room bulk of at least 6 hours is typical. — `{"volume_increase_pct": [20, 30]}` [breadcode-1230; yt:MrOM_D3hnpc 1:31:52; creator-rule]
- At cool room temperatures the bulk takes longer; with a low starter ratio there is more margin for timing. [breadcode-1231; yt:MrOM_D3hnpc 1:31:17; anecdote]
- Aim for an 8 to 10 hour bulk by lowering starter or changing temperature; exact timing depends on flour and starter combination. [breadcode-1233; yt:MrOM_D3hnpc 1:37:46; creator-rule]
- Bulk end point is read from a small dough sample kept in a covered jar at dough temperature. For this flour he chose about a 50 % volume increase, and says the right target depends on the flour's protein. — `{"sample_increase_pct": 50}` [breadcode-1250; yt:ImVvQMvGZKE 24:33; creator-rule]
- Target about 10 h of bulk fermentation at a cool room temperature. [breadcode-1276; yt:MrOM_D3hnpc 43:30; creator-rule]
- Dividing and pre-shaping the dough before bulk fermentation leaves too little fermentation for gas pockets to form; bulk fermentation has to come first. [breadcode-1300; yt:KRO9EcIyXSI 18:05; creator-rule]
- Over-fermented dough turns sticky and bakes flat because the gluten breaks down. [breadcode-1302; yt:KRO9EcIyXSI 22:36; creator-rule]
- A 5 % starter dose lets bulk run overnight before proofing the next morning. — `{"starter_pct_of_flour": 5}` [breadcode-1304; yt:KRO9EcIyXSI 52:17; creator-rule]
- A reference table at about 24 °C gave roughly 4 h bulk followed by about 3 h proof. — `{"temp_c": 24, "bulk_h": 4, "proof_h": 3}` [breadcode-1305; yt:KRO9EcIyXSI 18:39; creator-rule] *(low confidence)*
- Mastering fermentation accounts for most of the quality of a sourdough loaf. [breadcode-1306; yt:KRO9EcIyXSI 32:51; creator-rule]
- Alternatively, bulk ends at about pH 4.5; a pH meter is costly but gives a repeatable endpoint. — `{"pH": 4.5}` [breadcode-1321; yt:-60DxtcJ9Fo 14:33; creator-rule]
- Shaping was done at about pH 4.2; the speaker said the dough is over-fermented below about 3.9 (he says 'five ish', possibly 3.5). — `{"pH": 4.2, "pH_overfermented_below": 3.9}` [breadcode-1352; yt:-60DxtcJ9Fo 23:17; creator-rule] *(low confidence)*
- A dough with 5 % starter, set in the evening, should be about 50 % risen by early morning, so it can be shaped and proofed for next-day bread. The starter dose sets the timing. — `{"starter_pct_of_flour": 5, "rise_pct": 50}` [breadcode-1357; yt:KRO9EcIyXSI 51:39; creator-rule] *(low confidence)*
- With his flour, dough can sit at room temperature for roughly 20 h before enzymes break it down completely; any shorter time is considered safe. — `{"room_temp_max_h": 20}` [breadcode-1358; yt:KRO9EcIyXSI 52:54; creator-rule] *(low confidence)*
- At about 21 °C he expected the sample to double in roughly 8–10 h. The actual sample took 12 h in winter and 6 h in summer, so the time depends on season and temperature. — `{"bulk_h": [8, 10]}` [breadcode-1371; yt:o_xtVmR4vlc 12:42; 15:00; anecdote]
- Use a clear, marked container for bulk fermentation so you can judge doubling by volume and avoid under- or over-fermenting. [breadcode-1397; yt:WxFE3bxrTSM 21:08; anecdote]
- The same dough can be a one-day or a three-day process: bulk ferment about 3-4 h at room temperature then proof about 2 h for a same-day bake, or bulk overnight, shape and retard in the fridge for a three-day schedule. — `{"bulk_h": [3, 4], "proof_h": 2, "total_ferment_days": [1, 3]}` [breadcode-1431; yt:WxFE3bxrTSM 15:26; creator-rule]
- Pizza dough should ferment long (a full day, not one hour) for flavour and a fluffier result. [breadcode-1443; yt:l0nAMbbYziI 1:50; creator-rule]
- A small dough sample in a marked jar shows when the first (bulk) fermentation is complete: when the sample doubles. — `{"rise_pct": 100}` [breadcode-1447; yt:l0nAMbbYziI 8:51; creator-rule]
- Bulk fermentation stopped too early gives a dense, less fluffy loaf; too long gives a very sticky dough because acid breaks down the gluten. [breadcode-1470; yt:qM9BXKaIr60 16:47; 17:22; creator-rule]
- If the dough is becoming sticky at the end of bulk, aim for about 75 % rise instead of 100 %; high-gluten flours tolerate a longer bulk. — `{"rise_pct": 75}` [breadcode-1471; yt:qM9BXKaIr60 19:40; creator-rule]
- The autolysed dough fermented noticeably faster and reached the target rise first, even though the non-autolysed dough started a little warmer. [breadcode-1478; yt:JZBtmQXKWA8 18:08; 19:21; 21:16; measured]
- At the sample check, both ciabatta doughs had risen about 50 %. — `{"rise_pct": 50}` [breadcode-1496; yt:HSdOUZq5i6U 14:06; anecdote]
- An overnight plan works when you mix in the evening and shape in the morning: about 12 hours of bulk. — `{"bulk_h": 12}` [breadcode-1503; yt:HLPNdyGCSPk 3:20; 12:17; description sched; creator-rule]
- Start a sample jar at about a 25 % rise; the target scales with flour protein, and a high-protein flour can go up to about 100 %. — `{"rise_pct": [25, 100]}` [breadcode-1508; yt:HLPNdyGCSPk 6:07; 6:40; creator-rule]
- A sample jar becomes unreliable when the dough is warmer than the jar, because the sample cools faster and ferments more slowly than the dough. [breadcode-1521; yt:LQPCXMpxO4c 9:48; creator-rule]
- In his overnight bulk the sample jar had almost doubled (about 90-100 % rise) before the morning stretch-and-fold, and he still gave one more fold. — `{"rise_pct": [90, 100], "bulk_h": 11}` [breadcode-1524; yt:HLPNdyGCSPk 11:11; anecdote] *(low confidence)*
- Start-to-finish time for the ciabatta (mixing to baking) is about 11 to 12 hours for both the sourdough and yeast versions. — `{"total_process_h": [11, 12]}` [breadcode-1528; yt:HSdOUZq5i6U 1:09; creator-rule]
- Bulk for this sourdough was run to a 50–75 % rise in a sample jar, which the author expected to take roughly 10–12 hours in his cool conditions. — `{"rise_pct": [50, 75], "bulk_h": [10, 12]}` [breadcode-1531; yt:JZBtmQXKWA8 15:34; 12:27; creator-rule]
- A 40 g fermentation sample was taken from each dough, and the rounded dough was rested 5–15 minutes before spreading the sample so it was easier to spread. — `{"sample_g": 40, "relax_min": [5, 15]}` [breadcode-1532; yt:JZBtmQXKWA8 14:52; 15:34; creator-rule]
- A coil fold during the overnight ferment, and a second one once the sample has doubled, builds strength without extra flour; the dough is not kneaded much at the start. — `{"coil_folds": 2}` [breadcode-1538; yt:l0nAMbbYziI 10:35; 12:18; creator-rule]
- Bulk fermentation is generally judged complete when the dough has roughly doubled in size (about 100 % rise); since this is hard to see in an opaque container, a small sample in a marked jar is used to check it. — `{"rise_pct": 100}` [breadcode-1544; yt:qM9BXKaIr60 16:47; creator-rule]
- A dough left in bulk about four hours longer than its counterpart became very sticky, its surface tore, and it needed much more flour to shape. — `{"extra_bulk_h": 4}` [breadcode-1546; yt:qM9BXKaIr60 28:35; anecdote]
- For this dough, bulk fermentation was estimated at about 6 hours at about 24 °C. — `{"temp_c": 24, "bulk_h": 6}` [breadcode-1550; yt:wHL44ONu3so 9:02; anecdote] *(low confidence)*
- Bulk fermentation was judged done at about 50 % rise, read from a small sample. — `{"rise_pct": 50}` [breadcode-1555; yt:36GdGT0XYwE 6:46; creator-rule]
- Bulk for this sourdough loaf took about 6–12 hours depending on conditions. — `{"bulk_h": [6, 12]}` [breadcode-1556; yt:36GdGT0XYwE 6:46; creator-rule]
- A starter-raised dough took about nine hours of bulk at room temperature, longer than the creator expected. — `{"bulk_h": 9}` [breadcode-1583; yt:yYkTrGHNW2w 23:06; anecdote] *(low confidence)*
- Bulk end-point cannot be fixed in time, because a starter's microbial balance shifts; judge by recording photos, heights and touch. [breadcode-1594; yt:nQWlG1buMM8 6:13; creator-rule]
- Finish the first (bulk) fermentation of sourdough at a dough pH of about 4.2–4.5; a lower pH gives a sourer loaf, a higher pH a fluffier one. — `{"bulk_end_ph": [4.2, 4.5]}` [breadcode-1641; yt:WOZLvsV1M8s 4:26; creator-rule]
- The same sourdough dough takes about twice as long to finish bulk at about 20 °C (~20 h) as at about 25 °C (~10 h); the starter dose can be raised to shorten time when it is colder. — `{"temp_c": [20, 25], "bulk_at_25c_h": 10, "bulk_at_20c_h": 20}` [breadcode-1653; yt:36GdGT0XYwE 2:26; creator-rule]
- If the starter has shifted toward lactic acid bacteria and lost its yeast, the bulk may need up to about three times longer, or may never reach the same bulk stage. — `{"bulk_time_multiplier_max": 3}` [breadcode-1661; yt:nQWlG1buMM8 7:59; anecdote] *(low confidence)*
- Track bulk fermentation by marking a small piece of dough in a separate container with a rubber band and waiting until it has doubled, rather than relying on clock time. [breadcode-1669; yt:wHL44ONu3so 9:42; 20:00; creator-rule]
- Coil-folded dough becomes less sticky as it inflates, because less of it touches the container and the hands; fold gently from the side that is less sticky. [breadcode-1670; yt:wHL44ONu3so 24:01; 27:25; creator-rule]
- Dough that is too short in bulk bakes flat; dough that runs too long becomes sticky. [breadcode-1679; yt:l_PLhPWE-8E 5:55; creator-rule]
- A well-mixed aliquot in a jar represents the main dough's composition, but it rises faster than the main dough because it is not degassed or strengthened by folds. [breadcode-1680; yt:l_PLhPWE-8E 6:30–7:07; anecdote]
- A bakery dough fridge is typically 8–10 °C; near 10 °C it shortens final fermentation. — `{"temp_c": [8, 10]}` [breadcode-1682; yt:l_PLhPWE-8E 8:19; creator-rule]
- For cold inoculation in a home fridge the guest uses 12–18 h, the lower end (10–12 h) for whole-grain doughs. — `{"ferment_h": [10, 18]}` [breadcode-1683; yt:l_PLhPWE-8E 8:52; anecdote]
- With high-gluten flour the bulk can run about 5 h instead of 3 h when the starter is reduced. — `{"bulk_h": [3, 5]}` [breadcode-1689; yt:l_PLhPWE-8E 18:34–19:09; anecdote] *(low confidence)*
- Aim for about a 25 % size increase at bulk end; stronger flour can go further. — `{"rise_pct": 25}` [breadcode-1699; yt:9fWzWpL5g6w 12:51; creator-rule]
- At about 15 % protein the dough is ready when the sample doubles. — `{"size_increase_pct": 100}` [breadcode-1713; yt:dl7crk2Nvj8 8:28; creator-rule]
- For about 14 % protein flour, sample size increase should reach about 75–100 %; start the watch at 25 %. — `{"size_increase_pct": [25, 100]}` [breadcode-1729; yt:9yI7QRpAgow 1:47; 2:24; creator-rule]
- Sample water must match the dough's room temperature. [breadcode-1730; yt:9yI7QRpAgow 2:24; creator-rule]
- The sample doubles in about 2 h in his kitchen. — `{"bulk_h": 2}` [breadcode-1739; yt:vyDTKFtfIb0 7:55; anecdote]
- Yeast-only dough can over-rise a bit without harm; that is not acceptable for sourdough. [breadcode-1740; yt:vyDTKFtfIb0 7:55–8:30; creator-rule]
- Bulk was run with hourly coil folds until a small marked jar doubled. The dough was bowl-fermented about 6 h, plus about 30 min more before the final coil fold. — `{"bulk_h": [6, 6.5], "coil_fold_interval_h": 1, "final_coil_fold_before_shape_min": 30}` [breadcode-1752; yt:2DFvfRIywC8 15:10 (about six hours, plus 3; anecdote]
- Bulk fermentation of this recipe ran about 8 hours, from mixing in the morning to shaping around 5 pm. — `{"bulk_h": 8}` [breadcode-1778; yt:dl7crk2Nvj8 9:36; creator-rule]
- Lower-protein flour inflates less and ferments faster, so the target size increase should be lower than for high-protein flour. [breadcode-1779; yt:dl7crk2Nvj8 8:28; creator-rule]
- For yeast dough a small sample jar is used; the creator waits for it to double. For sourdough he advises an earlier check, around 50% rise. [breadcode-1807; yt:TUkPI34Go7k 11:55; creator-rule]
- Use a sample jar: the bulk is ready when the sample doubles in size. [breadcode-1825; yt:B_2SEmyyvso 8:38; creator-rule]
- Bulk ends when the dough doubles; the creator marks a jar to watch the rise. — `{"bulk_h": 5}` [breadcode-1830; yt:3vr2_qqeAL8 8:39; creator-rule]
- Build strength before bulk: after autolyse, knead about 5 min, rest, then do a 15 min bench rest with bench kneading (no extra flour), then laminate. The dough should hold its shape afterwards. — `{"knead_min": 5, "bench_rest_min": 15}` [breadcode-1838; yt:BnvhhxQZQ14 5:04; creator-rule]
- Mark a sample jar and watch it double to know bulk is done. [breadcode-1839; yt:BnvhhxQZQ14 7:23; creator-rule]
- Ferment until the dough rises about 50%; a sample jar shows the endpoint. — `{"rise_pct": 50}` [breadcode-1847; yt:rcOxoJk7goM 4:46; creator-rule]
- The creator's sample of a 10 % starter dough took about 7 hours to double at his room temperature (sample taken about 10:00, doubled about 7 h later), which is the point where he measured pH. — `{"sample_double_h": 7, "starter_pct_of_flour": 10}` [breadcode-1873; yt:dQEebR6pIng 8:19-10:03; anecdote]
- Ferment the dough until it has roughly increased by 50 % in size before shaping. — `{"rise_pct": 50}` [breadcode-1950; yt:t3GkfPxQxik 14:18; creator-rule]
- For this 100 % whole wheat dough, bulk ends when a sample jar has risen about 50 %; he stopped at about 8 h, with the dough short of that rise, preferring to stop early over late. — `{"rise_pct": 50, "bulk_h": 8}` [breadcode-1958; yt:agCpWuDgAwg 4:27; creator-rule]
- The sample-jar rise target for bulk depends on the flour mix; he gives about 50–75 %. — `{"rise_pct": [50, 75]}` [breadcode-1963; yt:HgIKSVMkbFg 3:12; creator-rule]
- A roughly 2 h bulk was probably too short for this dough; he wants it fully gassed before shaping. — `{"bulk_h": 2}` [breadcode-1968; yt:HgIKSVMkbFg 13:20; creator-rule]
- Cold water slows fermentation in an overnight dough. [breadcode-1970; yt:APstLJpWOiE 2:28; creator-rule] *(low confidence)*
- Sample-based monitoring tells you fermentation progress at any temperature. [breadcode-1976; yt:PDeLAAloWNA 0:38; creator-rule] *(low confidence)*
- For ~15 % protein flour he chose to bulk until the dough doubles. — `{"rise_pct": 100}` [breadcode-1980; yt:lJ38fY3ZEQk description; creator-rule]
- A 5 °C difference (25 vs 30 °C) changed fermentation speed only modestly: the 30 °C dough doubled in about 6 h while the 25 °C dough lagged. — `{"temp_c": [25, 30], "bulk_h": 6}` [breadcode-1995; yt:vf5RbwlW8Xs 4:08; measured] *(low confidence)*
- Sample-jar size increase depends on flour protein: 25–100 %, with 100 % for about 15 % protein flour. — `{"rise_pct": [25, 100]}` [breadcode-2000; yt:02WlZa_N-vI 1:16; creator-rule]
- For yeast-raised dough (enriched, with milk, butter and sugar), aim for roughly a doubling in size during the first fermentation, whatever the flour protein; the dough may collapse slightly without harm. — `{"rise_pct": 100}` [breadcode-2016; yt:HgIKSVMkbFg 10:20; creator-rule]
- Roll dough is fermented to about a 50 % volume increase; with strong flour aim up to 100 %. — `{"rise_pct": [50, 100]}` [breadcode-2046; yt:OcOpIISBK6c [3:02]–[3:38]; creator-rule]
- Same-day overnight schedule: mix at 22:00, stretch-and-fold at 09:30, shape about 30 min later (~10:00), bake about 13:00. — `{"mix_time": "22:00", "fold_time": "09:30", "shape_time": "10:00", "bake_time": "13:00"}` [breadcode-2062; yt:0aylYlvA4K8 [7:16]; creator-rule]
- Bulk aims for a 50–75 % size increase, with coil folds about hourly. — `{"rise_pct": [50, 75], "fold_interval_h": 1}` [breadcode-2078; yt:MPWypFY0wyE [7:20]; creator-rule]
- Bulk to about a 50 % rise for flour under 14 % protein; higher-protein flour rises further (sample-jar method). — `{"rise_pct": [50, 100], "protein_under_pct": 14}` [breadcode-2087; yt:gOxCUh_kyMQ [5:45]–[6:22]; creator-rule]
- Overnight bulk takes about 14 h to double. — `{"bulk_h": 14}` [breadcode-2088; yt:gOxCUh_kyMQ [8:12]; creator-rule]
- Bulk rises about 25 % at first; expert flour can go to about 100 %. — `{"rise_pct": [25, 100]}` [breadcode-2097; yt:_cb4vCQ2shk [6:05]; creator-rule]
- Bulk takes about 8 h in summer and 12–14 h in winter. — `{"bulk_summer_h": 8, "bulk_winter_h": [12, 14]}` [breadcode-2098; yt:_cb4vCQ2shk [6:44]; creator-rule]
- Ferment checked with a marked probe jar: start checking at a 50 % size increase and work up to 100 % (doubled); the right target depends on the flour. — `{"rise_pct": [50, 100]}` [breadcode-2110; yt:4VJQqOtwP_E [8:40]-[9:14]; creator-rule]
- Pizza dough made in the morning was used in the evening after about 10 h of rest. — `{"rest_h": 10}` [breadcode-2128; yt:EI4RoemHe5M [6:26]; creator-rule]
- The pizzeria kneaded the Neapolitan dough for about 20 minutes and shaped dough balls straight away with no bulk fermentation, whereas at home he bulk-fermented for about 2 hours before shaping. — `{"knead_min": 20, "bulk_before_shaping_h": 2}` [breadcode-2144; yt:EI4RoemHe5M [7:38]; anecdote]
- A small fermentation sample is not a reliable check of bulk progress for this dough; the creator judged progress by the dough's size and fold behaviour instead. [breadcode-2148; yt:MPWypFY0wyE [6:44]; [7:56]; anecdote]
- A sample of dough can be taken in a small jar to judge fermentation; for a flour of about 15% protein, a 70 to 110% volume increase is aimed for. — `{"increase_pct": [70, 110], "protein_pct": 15}` [breadcode-2321; yt:JoAG7R9QESA 3:20; creator-rule]
- Bulk for the discard loaf is about 5 h, but it depends on the starter's activity; the creator calls it a ballpark figure. — `{"hours": 5}` [breadcode-2348; yt:7V3FyVzzVUI 6:23; creator-rule] *(low confidence)*
- A longer ferment gives a more open crumb, because the dough has more gas. [breadcode-2350; yt:7V3FyVzzVUI 6:58; anecdote] *(low confidence)*
- For sourdough, the rise to aim for during bulk depends on the flour's protein: more protein allows a longer ferment and more inflation, which gives a fluffier loaf. Doubling is the yeast-dough target, not the sourdough target. [breadcode-2391; yt:HNQbwvZIAYc 3:17; creator-rule]
- An overnight wholemeal-blend sourdough (12.5 % wholemeal flour, 80 g starter per 400 g flour) ferments about 12 h at about 22 °C and is shaped once a sample has grown about 50 %. — `{"temp_c": 22, "bulk_h": 12, "increase_pct": 50, "wholemeal_pct_of_flour": 12.5}` [breadcode-2409; yt:vLdmMuAxB7w description; creator-rule]
- Coil folds about three times during bulk fermentation; once or twice is also fine. — `{"coil_folds_n": [1, 3]}` [breadcode-2678; yt:9dsgUixpOBM 2:02; creator-rule]
- Each extra cut or fold degasses the dough, so cut and fold only as much as needed. [breadcode-2679; yt:9dsgUixpOBM 2:38; creator-rule]
- A small test piece set aside is used to judge bulk: with this stiff starter it may take 5, 10 or 15 hours to double; the time depends on many factors. — `{"bulk_h": [5, 15]}` [breadcode-2689; yt:0oKA_iTytiw 1:59; creator-rule]
- Einkorn ferments for about 6 h, the minimum he gives; a longer ferment makes the bread more sour. — `{"bulk_h": 6}` [breadcode-2703; yt:wz7tysfSDFM 1:15; creator-rule]
- With a yeast-leavened dough the doubling test piece is less critical, and a slightly longer bulk does no harm. [breadcode-2712; yt:lG9ERQFFn5I 1:51; creator-rule]
- Bulk about 7 h at roughly 19 °C until the dough has roughly doubled; colder takes longer, warmer takes less. — `{"bulk_h": 7, "temp_c": 19}` [breadcode-2731; yt:2NmD9KVyZCg 5:00; creator-rule]
- A small dough sample in a separate container is a fermentation meter; the bulk is ready when the sample has doubled. [breadcode-2748; yt:5DWcrVHK1WU 1:06; creator-rule]
- Stretch and fold once from each side when the dough flattens, around 3 to 6 h into the bulk. — `{"first_fold_h": [3, 6]}` [breadcode-2749; yt:5DWcrVHK1WU 1:38; creator-rule]
- Over-fermentation can prevent oven spring; a third-party test raised a dough by 25, 50, 75 and 100 % volume to compare. — `{"bulk_rise_pct": [25, 100]}` [breadcode-2754; yt:t_-FTKXgLMU 3:18; anecdote] *(low confidence)*
- Let the seed mix ferment about 10 h before shaping; up to 24 h gives a sourer loaf. — `{"bulk_h": [10, 24]}` [breadcode-2768; yt:cN_skGXZ4Es 1:23; creator-rule]
- Let the pasta dough rise about 50 % in volume in the bowl before using or chilling it. — `{"rise_pct": 50}` [breadcode-2785; yt:A5blYerC8VE 0:34; creator-rule]
- With 10 % starter overnight, bulk took about 8 h to double. — `{"starter_pct_of_flour": 10, "bulk_h": 8}` [breadcode-2898; yt:MyGMF5sLg1U description; creator-rule]
- The dough degassed slightly by morning after the first coil fold. [breadcode-2899; yt:MyGMF5sLg1U description; anecdote] *(low confidence)*
- Bulk fermentation time varies with the season: about 8 h in summer and 12–14 h in winter, so a time in a recipe is only an estimate. — `{"bulk_h": [8, 14]}` [breadcode-2948; yt:CqlQU3ehU6w 0:00; creator-rule]
- Sourdough buns at 67 % hydration were bulk-fermented for 7 h at room temperature. — `{"hydration_pct": 67, "bulk_h": 7}` [breadcode-2976; yt:ovzejNuq4Eo description; anecdote]
- Bulk is hard to read by look and feel; the creator says about 10 loaves are needed to learn it. [breadcode-2982; yt:apKpvGqxfV0 0:00; creator-rule]

### bulk.aliquot-jar

- An aliquot jar is a small sample of dough taken after initial dough strength is built; its size increase tracks the fermentation of the main dough, and when the sample reaches the target size the main dough is ready to shape and proof. [breadcode-0496; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar; creator-rule]
- The target size increase depends on the flour: a higher-gluten flour tolerates a longer fermentation, so it is aimed at a higher increase. [breadcode-0497; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar; creator-rule]
- For a first attempt, aim for a size increase of about +25 % in the aliquot jar, test higher targets (up to +100 %) in later bakes, and keep the value you like best. — `{"aliquot_increase_pct": [25, 100]}` [breadcode-0499; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar; creator-rule]
- Reference table: for flour protein content 8–10 %, aim for an aliquot jar size increase of about +25 %. — `{"flour_protein_pct": [8, 10], "aliquot_increase_pct": 25}` [breadcode-0500; SF Table 7.3; creator-rule]
- Reference table: for flour protein content 10–12 %, aim for an aliquot jar size increase of about +50 %. — `{"flour_protein_pct": [10, 12], "aliquot_increase_pct": 50}` [breadcode-0501; SF Table 7.3; creator-rule]
- Reference table: for flour protein content 12–15 %, aim for an aliquot jar size increase of about +100 %. — `{"flour_protein_pct": [12, 15], "aliquot_increase_pct": 100}` [breadcode-0502; SF Table 7.3; creator-rule]
- Reference table: for flour protein above 15 %, aim for an aliquot jar size increase above +100 %. — `{"flour_protein_pct": [15, null], "aliquot_increase_pct": [100, null]}` [breadcode-0503; SF Table 7.3; creator-rule]
- Because the aliquot is judged by size increase rather than clock time, the bulk duration varies with temperature (e.g. about 8 h in summer and about 12 h in winter in the author's bakes), while the endpoint stays the same. — `{"bulk_h": [8, 12]}` [breadcode-0504; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar; creator-rule]
- Example: one aliquot bake reached a +50 % size increase after about 10 h of bulk. — `{"bulk_h": 10, "aliquot_increase_pct": 50}` [breadcode-0505; SF Figure 'aliquot-before-after'; measured]
- An aliquot jar must be cylindrical so its size increase can be judged properly. [breadcode-0506; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar limitations; creator-rule]
- Use room-temperature water: hot water makes the small aliquot cool faster and ferment more slowly than the main dough; cold water makes the aliquot warm faster and run ahead, causing an early stop. [breadcode-0507; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar limitations; creator-rule]
- Keep the aliquot next to the main dough, ideally in the same container, so both share the same ambient temperature. [breadcode-0508; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar limitations; creator-rule]
- Large daily temperature swings make the aliquot adapt faster than the main dough, so its readings are slightly off. [breadcode-0509; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar limitations; creator-rule]
- The aliquot is less reliable for doughs of more than about 10 kg flour, because fermentation is exothermic and heats the large dough mass. — `{"flour_g_min": 10000}` [breadcode-0510; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar limitations; creator-rule]
- An aliquot jar is a small dough sample taken after initial dough strength is developed; it is used to monitor fermentation progress. [breadcode-0796; SF Glossary > Aliquot jar; creator-rule]
- The aliquot's water temperature should match the room temperature, so the aliquot readings represent the main dough. [breadcode-0797; SF Glossary > Aliquot jar; creator-rule]
- A small aliquot heats or cools faster than the main dough, so under large kitchen temperature swings it can misrepresent the dough's fermentation. [breadcode-0798; SF Glossary > Aliquot jar; creator-rule]
- A cylindrical aliquot container is needed to judge the dough's size increase properly. [breadcode-0799; SF Glossary > Aliquot jar; creator-rule]
- If the dough rises but the aliquot does not, the two are fermenting at different speeds, often because the kitchen temperature changed; the smaller aliquot heats or cools faster than the main dough. [breadcode-0956; SF Ch 12 Troubleshooting §12.2.2 My dough sample (aliquot) doesn't rise. ; creator-rule]
- Use room-temperature water for the dough so the aliquot and dough are at the same temperature and ferment at the same rate. [breadcode-0957; SF Ch 12 Troubleshooting §12.2.2 My dough sample (aliquot) doesn't rise. ; creator-rule]
- If room temperature swings significantly during the day, use a see-through container and mark it to measure the dough's size increase. [breadcode-0958; SF Ch 12 Troubleshooting §12.2.2 My dough sample (aliquot) doesn't rise. ; creator-rule]
- An alternative to an aliquot is a pH meter, which measures acidity build-up; it is more expensive. [breadcode-0959; SF Ch 12 Troubleshooting §12.2.2 My dough sample (aliquot) doesn't rise. ; creator-rule]
- Take a sample into a cylindrical jar after the final kneading and let it rise about 50 %; he says a strong flour supports this and he would go lower than doubling. — `{"rise_pct": 50}` [breadcode-1301; yt:KRO9EcIyXSI 21:12; creator-rule]
- Take the sample after the last kneading stage so the starter is evenly distributed through the dough. [breadcode-1303; yt:KRO9EcIyXSI 23:51; creator-rule]
- Pizza dough is ready for cold fermentation when a sample jar reaches about +50 % rise; this works for strong flour. — `{"rise_pct": 50}` [breadcode-1320; yt:-60DxtcJ9Fo 12:17; 14:00; creator-rule]
- Start checking a sample at about +25 % rise; strong flour can go up to about +100 %. — `{"rise_pct": [25, 100]}` [breadcode-1336; yt:013Qw3BojM4 5:21; creator-rule]
- For pizza he stopped the bulk at about +50 % before retarding; he says the same flour could go to about +100 %, and a bread dough would be taken that far, keeping leavening power for later. — `{"rise_pct": [50, 100]}` [breadcode-1347; yt:-60DxtcJ9Fo 16:53; 17:27; creator-rule]
- Use room-temperature water so the sample jar and the main dough warm at the same rate; otherwise the sample can misstate the dough's progress. [breadcode-1348; yt:-60DxtcJ9Fo 14:00; creator-rule]
- Ferment a small dough sample in a cylindrical jar and mark its level; for a high-protein dough he waits until the sample doubles. — `{"rise_pct": 100}` [breadcode-1366; yt:o_xtVmR4vlc 10:23; creator-rule]
- Lower-protein flour needs a smaller target rise than high-protein flour, so the endpoint must be adjusted to the flour. [breadcode-1367; yt:o_xtVmR4vlc 10:58; creator-rule]
- Track bulk fermentation in a small jar marked with a rubber band; bulk is done when the sample doubles. — `{"rise_pct": 100}` [breadcode-1581; yt:yYkTrGHNW2w 19:29; creator-rule]
- For about 14 % protein flour, stop bulk at a doubling of the sample; with lower-protein flour stop at about +75 % instead. — `{"protein_pct": 14, "rise_pct": 100, "rise_pct_lower_protein": 75}` [breadcode-1582; yt:yYkTrGHNW2w 20:38; creator-rule]
- Judge bulk from a small sample jar marked with a rubber band; when in doubt, finish bulk early rather than late. [breadcode-1613; yt:0kFYyj3WKpE 12:19; creator-rule]
- The rise a sourdough can take from a sample jar depends on the flour's protein: with more protein you can ferment and inflate the dough longer before it turns into a sticky mess. [breadcode-1640; yt:WOZLvsV1M8s 3:22–3:55; creator-rule]
- Bulk fermentation is judged by a small dough sample kept in a cylindrical jar with a marked level; bulk is done when the sample has doubled, rather than by a fixed timer. [breadcode-1795; yt:vyDTKFtfIb0 5:30–6:40; creator-rule]
- Mark a sample jar with a rubber band after mixing in the starter; the bulk is complete when the sample has doubled. — `{"rise_pct": 100}` [breadcode-2241; yt:nIOPCeLPqrM 6:55; creator-rule]
- Use a sample jar: finish the bulk once the sample has doubled. — `{"rise_pct": 100}` [breadcode-2253; yt:oAFjGTWMgUo 8:11; creator-rule]
- For new bakers with very large holes, aim for about 50 % rise in the sample jar rather than a doubling. — `{"rise_pct": 50}` [breadcode-2261; yt:-LeUYBe7HlY 6:52; creator-rule]
- Judge bulk fermentation by a small dough sample in a small jar: mark its starting level and ferment until it has risen to a set size. [breadcode-2390; yt:HNQbwvZIAYc 2:42; creator-rule]
- An aliquot jar (a portion of dough kept in a marked jar) lets you follow bulk progress by its size increase. [breadcode-2950; yt:CqlQU3ehU6w 0:00; creator-rule]
- Bulk is done when the aliquot jar has risen about 25 %. — `{"rise_pct": 25}` [breadcode-2951; yt:CqlQU3ehU6w description; creator-rule]

### bulk.container

- Cover the dough to stop drying and fruit flies; a transparent container makes readiness easier to see. [breadcode-0287; SF Ch 6 Bread types; creator-rule]

### bulk.endpoint

- Timing is unreliable; judge readiness by smell: dairy, fruity or alcoholic notes versus vinegary (acetic) notes, which depend on the starter's microbes and temperature. [breadcode-0286; SF Ch 6 Bread types; creator-rule]
- Tasting raw dough shows its acidity; it will taste quite sour, and the author uses this to judge readiness. [breadcode-0290; SF Ch 6 Bread types; creator-rule]
- Rye bulk endpoint: wait until the dough has risen about 50 %. — `{"rise_pct": 50}` [breadcode-0334; SF Ch 8 Non wheat sourdough; creator-rule]
- Bulk fermentation ends when the dough is divided and pre-shaped, or directly shaped into the final loaf or loaves. [breadcode-0485; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Definition; creator-rule]
- Controlling bulk fermentation is the decisive factor for good home-baked sourdough; a well-judged bulk can compensate for weaker shaping or baking. [breadcode-0486; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Definition; creator-rule]
- A too-short bulk produces a gummy crumb with large air pockets called craters. [breadcode-0487; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Under- and over-fermenta; creator-rule]
- A too-long bulk breaks the dough down: it sticks to the banneton and spreads into a flat, pancake-like loaf in the oven. [breadcode-0488; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Under- and over-fermenta; creator-rule]
- When in doubt, lean toward a longer bulk rather than a shorter one: a slightly over-fermented dough tastes better than a pale, under-fermented one. [breadcode-0489; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Under- and over-fermenta; creator-rule]
- Table of fermentation stages: a too-short bulk gives unbaked gummy areas near the loaf bottom and overly large craters; a too-long bulk gives a gummy-feeling crumb (most gluten broken down), many tiny evenly spread alveoli, and a flat pancake-like oven spring. [breadcode-0490; SF Table 7.2; creator-rule]
- A perfect bulk gives an evenly baked, moist but not gummy crumb, evenly distributed alveoli without craters, balanced acidity (vinegary or lactic depending on the starter) and vertical oven spring. [breadcode-0491; SF Table 7.2; creator-rule]
- A too-short bulk gives a pale, neutral taste; a too-long bulk gives a strong acidic profile that overweighs when tasting. [breadcode-0492; SF Table 7.2; creator-rule]
- A recipe's stated bulk time is unreliable: differences in flour, starter activity and fermentation temperature shift the timing, and a 1–2 h difference can leave the dough under-fermented or over-fermented. — `{"timing_error_h": [1, 2]}` [breadcode-0493; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Timing from recipes; creator-rule]
- Commercial bakeries favour yeast-only doughs because removing lactic and acetic bacteria makes fermentation timing more predictable and widens the error margin. [breadcode-0494; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Timing from recipes; creator-rule]
- Judging bulk by look and feel only works after many loaves; beginners should use a measured method such as an aliquot jar, pH meter or size-increase marks. [breadcode-0495; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Methods to judge bulk; creator-rule]
- Visual check: mark the starting level on the container (a transparent rectangular container helps) and watch for the size increase that suits your dough. [breadcode-0529; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Visual signs; creator-rule]
- Surface bubbles indicate the dough is inflated with gas; more bubbles appear the further bulk is pushed, and an over-fermented dough becomes leaky and its bubbles disappear. [breadcode-0530; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Visual signs; creator-rule]
- Smell: a ripe dough at the end of bulk smells like a starter just before it collapses. [breadcode-0531; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Smell and taste; creator-rule]
- Taste: a ready dough tastes pickled, and the amount of acidity indicates how far fermentation has progressed. [breadcode-0532; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Smell and taste; creator-rule]
- At the right endpoint the dough feels tacky but is less sticky than in earlier stages; an overly sticky dough means fermentation has gone too far. [breadcode-0534; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Touch; creator-rule]
- An over-fermented dough can no longer be baked as a freestanding loaf; it can be moved to a greased loaf pan or used as part of the next starter. [breadcode-0535; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Over-fermented dough; creator-rule]
- Very large crumb cavities can be reduced with more stretch and folds, but they can also come from under-fermentation (Fool's Crumb); check fermentation before adding folds. [breadcode-0541; SF Ch 7 Wheat sourdough §7.10 Stretch and folds > Homogenization; creator-rule]
- If the gluten does not heal after tearing, the fermentation has probably run too long and the gluten has largely broken down. [breadcode-0545; SF Ch 7 Wheat sourdough §7.10 Stretch and folds > Frequency; creator-rule]
- After one overnight stretch and fold, the author allows about 2 more hours of bulk before dividing or shaping. — `{"bulk_after_fold_h": 2}` [breadcode-0548; SF Ch 7 Wheat sourdough §7.10 Stretch and folds > When to fold; creator-rule]
- A flowchart check cycle: check dough with size increase, pH or smell; if ready, divide and pre-shape; if flattened, stretch and fold; otherwise wait about 60 min and check again. — `{"check_interval_min": 60}` [breadcode-0560; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation; creator-rule]
- Bulk fermentation should finish when the dough has roughly doubled; over-bulking gives a sticky dough, while under-bulking loses the open crumb. — `{"size_increase_pct": 100}` [breadcode-1891; yt:U9_GVHuI1Ro 9:18; creator-rule]
- Bulk endpoint: the description says shape once the sample has risen 50%; in the video he says the sweet spot for his flour is 75-100% (more gluten allows longer bulk). The sample jar marks the endpoint, then shape and proof (room temperature until the finger-poke test passes, or 12-24 h in the fridge). — `{"size_increase_pct": [50, 100], "fermentation_h": 12, "fermentation_temp_c": 22}` [breadcode-1918; yt:O0REFqmB0rs description; 2:59 (75-100% swe; creator-rule]
- The rye dough reached about double in size after roughly five hours in his kitchen, the endpoint he used before panning. — `{"bulk_h": 5}` [breadcode-2460; yt:cHwLBs9SRd4 3:56; anecdote]
- Take a small fermentation sample when kneading ends; with a stiff starter he aims for the sample to double, and a more sour starter does not behave as predictably. — `{"rise_pct": 100}` [breadcode-2472; yt:v85zeRckFt0 1:13; creator-rule]
- Fixed hour-count instructions do not work for sourdough; read the dough, using a sample that tells you when fermentation is complete. [breadcode-2473; yt:v85zeRckFt0 1:13; creator-rule]
- Shape the dough when a small sample has risen about 50 %, then proof until a finger poke almost passes back, then freeze. — `{"sample_rise_pct": 50}` [breadcode-2480; yt:Typy5Uzp-EM description; creator-rule]
- Judge bulk by a small dough sample in a jar: ferment until it has risen about 50 %, not by clock time. — `{"sample_rise_pct": 50}` [breadcode-2499; yt:0h63OloQQd8 1:45; creator-rule]
- Knead until the dough releases the bowl, then ferment until it has roughly doubled before shaping. — `{"rise_pct": 100}` [breadcode-2523; yt:MqH3GVfjfBc 6:34; creator-rule]
- Any starter type can reach the same acidity if fermented for longer, so the choice of starter changes the time needed, not the endpoint. [breadcode-2532; yt:nPSsWa4nGgU 6:05; extrapolation]
- Do a coil fold with wet hands once the dough has nearly doubled; big bubbles then show good fermentation. Timing depends on kitchen temperature (he saw about an hour). — `{"rise_pct": 100}` [breadcode-2549; yt:Dx7xM8IThCI 1:37; creator-rule]
- Wait until the dough has doubled rather than a fixed time, because kitchen temperature changes the timing. — `{"rise_pct": 100}` [breadcode-2550; yt:Dx7xM8IThCI 2:10; creator-rule]
- Bulk is finished at doubled volume; finish slightly before that, because longer bulk makes the dough more sour and weakens its structure. — `{"rise_pct": 100}` [breadcode-2837; yt:Cqn-VTknVO0 0:34; creator-rule]
- Bulk time depends on starter amount, starter activity and kitchen warmth, so the doubling point matters more than a fixed time. [breadcode-2838; yt:Cqn-VTknVO0 0:01; creator-rule]
- Jar test: put a dough sample in a glass jar, mark the level after adding starter, and judge bulk by its rise; he saw about 30 % with a 10 % starter dose. — `{"rise_pct": 30, "starter_pct_of_flour": 10}` [breadcode-2888; yt:6iAwFZm6nGs 0:00; creator-rule]
- Bulk is done when the marked jar sample has doubled. — `{"rise_pct": 100}` [breadcode-2889; yt:6iAwFZm6nGs description; creator-rule]
- A rubber band on the jar marks the dough level more easily than a pencil mark. [breadcode-2890; yt:6iAwFZm6nGs description; creator-rule]

### bulk.endpoints

- In the same bake, the liquid-starter dough, the least fully fermented of the three, showed the least oven spring; the creator attributes this to under-fermentation and extra water. — `{"starter_g": 20}` [breadcode-2164; yt:bJxpCVc2wYE 8:01; measured]
- Liquid-starter doughs ferment faster and can over-ferment quickly, so check them at the mid-point of the bulk rather than trusting a timing. [breadcode-2168; yt:bJxpCVc2wYE 3:03; creator-rule]
- The creator aimed for a doubling for the liquid and yeast doughs, and stopped the stiff-starter dough near 50 %, which he thought could have gone further. — `{"rise_pct": [50, 100]}` [breadcode-2169; yt:bJxpCVc2wYE 3:03; creator-rule]
- At about 50 % rise he gave the dough one stretch and fold, then left it 15 minutes at room temperature. — `{"rise_pct": 50, "rest_min": 15}` [breadcode-2170; yt:bJxpCVc2wYE 3:45; creator-rule]
- Bulk is complete when a small sample in a glass has doubled; this takes roughly one to two hours at kitchen temperature, depending on conditions. — `{"rise_pct": 100, "bulk_h": [1, 2]}` [breadcode-2184; yt:EbO3blOpGks 8:00; creator-rule]
- During bulk, stretch and fold every 2 hours; shape once a separate sample has doubled. — `{"stretch_fold_interval_h": 2, "rise_pct": 100}` [breadcode-2190; yt:I3h6heaKWC8 2:59; creator-rule]
- Judge bulk fermentation by a doubled sample rather than by time; he says a doubling might take 10, 12 or 14 hours. — `{"rise_pct": 100, "bulk_h": [10, 14]}` [breadcode-2204; yt:xEwk5yHf_Gc 6:52; creator-rule]
- With a small, unfed starter in 1000 g flour, bulk can take 14 to 16 hours, against the usual 6 to 12 hours. — `{"bulk_h": [14, 16], "usual_bulk_h": [6, 12], "flour_g": 1000, "starter_pct_of_flour": 5}` [breadcode-2223; yt:ngcISK7ZDjc 4:08; creator-rule]
- Judge bulk with a cylindrical sample: stop at about +50 %; with strong flour you can go to +100 %; in a non-cylindrical glass use about +25 %. — `{"rise_pct": 50, "rise_pct_strong_flour_max": 100, "rise_pct_noncylindrical_glass": 25}` [breadcode-2224; yt:ngcISK7ZDjc 9:58; creator-rule]
- Bulk time cannot be planned from a table: the same dough can take 8 to 20 hours, so decide by the sample rather than the clock. — `{"bulk_h": [8, 20]}` [breadcode-2225; yt:ngcISK7ZDjc 9:19-9:58; creator-rule]
- Bulk until a sample reaches the size increase set by the flour's protein; stronger flour can go further. [breadcode-2229; yt:iEfLO_Cl0uE 0:33; creator-rule]
- From one dough split in two, a 6-hour bulk gave excellent oven spring, while the 8-hour bulk became too flat and sticky. — `{"bulk_h": [6, 8]}` [breadcode-2239; yt:nIOPCeLPqrM 1:49; measured]
- Large irregular pockets and a missing ear are signs of under-fermentation; ferment the dough longer. [breadcode-2252; yt:oAFjGTWMgUo 7:32; creator-rule]
- From his fermentation table, a dough at 30 °C with 20 % starter needs about 5 hours of bulk. — `{"temp_c": 30, "starter_pct_of_flour": 20, "bulk_h": 5}` [breadcode-2255; yt:oAFjGTWMgUo 10:06; creator-rule]
- Giant holes indicate under-fermentation: the dough was not inflated enough. [breadcode-2262; yt:-LeUYBe7HlY 7:27; creator-rule]
- Gummy areas in a crumb show that little or no fermentation happened there, so the dough was not inflated. [breadcode-2267; yt:-LeUYBe7HlY 5:02; creator-rule]
- Bulk fermentation for these sourdough doughs can take 6 to 12 hours; use a size-increase sample rather than the clock. — `{"bulk_h": [6, 12]}` [breadcode-2268; yt:-LeUYBe7HlY 6:52; creator-rule]
- The dough is ready about 30 minutes after rounding, when it has risen about 50 to 100 %. — `{"rise_min": 30, "rise_pct": [50, 100]}` [breadcode-2279; yt:-rwuxy5KS6Q 9:55; 10:30; creator-rule]
- Judge the end of bulk with a marked sample in a jar: stop when the sample has doubled. — `{"sample_rise_pct": 100}` [breadcode-2568; yt:gMbZeUIVzZY 4:13; creator-rule]
- Rye sourdough with warm water takes about 6 h at room temperature to double. — `{"bulk_h": 6}` [breadcode-2585; yt:nsyjVa5SSQI 1:49; creator-rule]
- Bake when a small extracted sample has doubled (about 5–10 h), not at a fixed time. — `{"sample_rise_pct": 100, "ferment_h": [5, 10]}` [breadcode-2600; yt:P0-EGk_1SYo 1:09; creator-rule]
- Bulk 6–12 h until doubled, with a stretch and fold whenever it flattens; knead, rest 15 min, knead again, then shape and fridge-proof. — `{"bulk_h": [6, 12], "rest_min": 15}` [breadcode-2604; yt:rzakvUe7eIE description; creator-rule]
- Cover the dough and let it rise until doubled, about 6 h; time depends on temperature. — `{"bulk_h": 6}` [breadcode-2610; yt:bjLCHRInmLY 1:52; creator-rule]
- Put a marked sample in a separate jar to see exactly when bulk is done. [breadcode-2641; yt:P59OfgJ4Dps 4:01; creator-rule]
- Stop bulk when the dough has risen about 50 %; he does this in a lidded pot. — `{"bulk_rise_pct": 50}` [breadcode-2654; yt:nlqVo9sxTlQ 2:38; creator-rule]
- Judge the end of bulk by the dough, not the clock: it inflates, feels slightly tacky and shows bubbles. [breadcode-2949; yt:CqlQU3ehU6w 0:00; creator-rule]
- Pizza dough is ready when a sample has grown about 50 % in size. — `{"rise_pct": 50}` [breadcode-2972; yt:lz0DC9JaqIk 0:00; creator-rule]

### bulk.one-hour

- After kneading, the one-hour dough rested about 30 minutes and roughly doubled in size; he said a little more yeast or a longer fermentation would have helped. — `{"rest_min": 30}` [breadcode-2301; yt:xLYxPneBx-M 1:56; anecdote] *(low confidence)*
- The one-hour dough was divided into three pieces and then rested about 50 minutes before shaping, so the gluten could relax. — `{"rest_min": 50, "pieces": 3}` [breadcode-2302; yt:xLYxPneBx-M 2:30 and 3:44; creator-rule] *(low confidence)*

### bulk.open-crumb

- An open crumb can be achieved at lower hydration (creator cites 60% Neapolitan-style dough) by timing fermentation well, since longer fermentation makes the dough more extensible; pushing hydration is the riskier route. — `{"hydration_pct": 60}` [breadcode-1888; yt:t3GkfPxQxik 6:12; creator-rule]

### bulk.overfermentation

- Overfermented dough turns flat in the oven and becomes overly sticky to handle; the creator attributes this mainly to acid in the starter, which also inhibits yeast. [breadcode-2370; yt:-6dlWqFlFSo 5:39; creator-rule]
- Over time, yeast and bacteria degrade the gluten network, so an over-fermented dough loses its shape and becomes a sticky mess. [breadcode-2389; yt:HNQbwvZIAYc 2:09; creator-rule]

### bulk.ph-check

- A pH meter measures dough acidity, which tracks bulk progress, and is a reliable alternative to visual checks; a good one costs about USD 200. — `{"cost_usd": 200}` [breadcode-2392; yt:HNQbwvZIAYc 3:49; creator-rule]

### bulk.rise

- In the loaf pan, the low-gluten rye-blend dough took about 10 hours to roughly double in size before going to the fridge. — `{"bulk_h": 10}` [breadcode-1354; yt:013Qw3BojM4 11:21-12:01; measured]

### bulk.rise-target

- For a flour of about 15% protein the creator bulk-fermented the dough to a doubling (about +100% rise). — `{"flour_protein_pct": 15, "rise_pct": 100}` [breadcode-2380; yt:33B6nXafahs description; creator-rule]

### bulk.sample-jar

- A sample jar of dough kept at the same temperature as the main dough shows its size increase; read bubbles at the glass edge and the smell. This works only if room temperature stays consistent. [breadcode-1758; yt:9fWzWpL5g6w 13:33; creator-rule]

### bulk.sample-test

- The creator judges the end of bulk fermentation with a small extracted dough sample. He stops when the sample has grown about 75-100 % for his flour, and says a weaker flour needs less fermentation. The description advises starting at 25 % and adjusting to the flour. — `{"sample_increase_pct": [25, 100]}` [breadcode-2397; yt:NAjBY7S_u04 description; 1:13; creator-rule]
- For this weak cake-flour dough the creator used a sample to judge bulk: bulk was ready at about a 50 % increase, which took about 10 h in his test; he says this varies with flour and temperature. — `{"sample_increase_pct": 50, "bulk_h": 10}` [breadcode-2414; yt:WOfTMskT44Y 1:52; creator-rule] *(low confidence)*

### bulk.stretch-and-fold

- Coil folds are done about once an hour during bulk fermentation (fewer if the baker is away) to build strength in the high-hydration dough. — `{"interval_min": 60}` [breadcode-1896; yt:U9_GVHuI1Ro 8:09; creator-rule]

### bulk.time

- With a starter dose of 20% of flour, fed the evening before with a 1:5:5 ratio, the dough doubled in roughly 12 hours of overnight bulk. — `{"starter_pct_of_flour": 20, "bulk_h": 12, "ratio": "1:5:5"}` [breadcode-2379; yt:33B6nXafahs 1:13; creator-rule]

### bulk.time-model

- Flatbread dough is typically ready after 6–12 h; more starter shortens and less starter lengthens the time. — `{"bulk_h": [6, 12]}` [breadcode-0284; SF Ch 6 Bread types; creator-rule]
- For flavour aim for about 8–12 h of fermentation; too early tastes less developed, too late tastes more sour. — `{"bulk_h": [8, 12]}` [breadcode-0285; SF Ch 6 Bread types; creator-rule]
- Rye bulk fermentation can last from a few hours up to weeks; longer fermentation raises acidity. — `{"bulk_h_min": 3}` [breadcode-0332; SF Ch 8 Non wheat sourdough; creator-rule]
- Bulk fermentation to a +50 % sample-jar rise took roughly 6 to 10 hours; the speaker expected 6 to 8 h and saw about 10 h in one run. Warmer rooms make it faster. — `{"bulk_h": [6, 10]}` [breadcode-1345; yt:-60DxtcJ9Fo 15:06; 15:44; anecdote]

### bulk.time-models

- The whole yeast-plus-sourdough loaf takes about 4 hours from mixing to bread (description); in the video it took longer than planned. — `{"total_time_h": 4}` [breadcode-1650; yt:0kFYyj3WKpE description; 26:52; creator-rule]
- Recipe timings are unreliable: starter state, room temperature and dough temperature change how fast fermentation runs, so judge by dough signs instead of the recipe's clock. [breadcode-1756; yt:9fWzWpL5g6w 5:28; creator-rule]
- In the creator's overnight recipe at about 75 % hydration, bulk fermentation ran about 10 hours before shaping. — `{"bulk_h": 10, "hydration_pct": 75}` [breadcode-2398; yt:NAjBY7S_u04 1:49; creator-rule]
- At about 22 °C with about 20 % starter the dough doubled in roughly 6 h; the time depends on temperature and starter activity. — `{"temp_c": 22, "starter_pct_of_flour": 20, "bulk_h": 6}` [breadcode-2594; yt:l9g4AI_z3uY 5:38; creator-rule]
- A higher starter dose speeds up fermentation; he picks the dose by how fast you want it. — `{"starter_pct_of_flour": [5, 20]}` [breadcode-2599; yt:P0-EGk_1SYo description; creator-rule]
- Use about 10 % sourdough (100 g per kg flour) and aim for a main fermentation of 6–12 h after adding water. — `{"starter_pct_of_flour": 10, "main_ferment_h": [6, 12]}` [breadcode-2625; yt:ep0mua88Sa8 1:48; creator-rule]
- Little volume change happens early in bulk; most of the rise comes in the last minutes. — `{"bulk_h": 1.5}` [breadcode-2649; yt:Hbxw4vOVzFc 3:15; anecdote]
