# proof — The Bread Code / The Sourdough Framework

Claims: 227 (book 51, videos 162, repos 14). Evidence: creator-rule 200, anecdote 19, measured 6, extrapolation 2.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

- **Room temperature** (book §7.13): 30 min–3 h (author: never under 1 h); finger poke every 15 min, press 0.5–1 cm; ready when the dent is still visible after 1 min. End early rather than late. Poke test fails on cold dough.
- **Cold retard:** a scheduling tool (author detects no flavour gain). Fermentation halts below ≈4 °C (strain-dependent); up to 24 h, sometimes 48 h; bakeable 6–24 h after going in. Calibrate iteratively 8 → 10 → 12 … 24 h. A warmer dough entering the fridge cools slowly and keeps fermenting: shorten retard in summer (book §12.5: a 30 °C dough takes much longer to reach 4–6 °C than a 20–25 °C dough).
- **Videos** fill in numbers: 30–60 min at room temperature then fridge; ≥12 h (ImVvQMvGZKE), 16 h at 4–6 °C (qM9BXKaIr60, KRO9EcIyXSI), "16 h at 6 °C or 24 h at 4 °C" (yYkTrGHNW2w), 18–24 h (b7Pd7UcZxd0), 8–48 h (9fWzWpL5g6w 2025); keep the fridge below ≈6 °C (-60DxtcJ9Fo); bake straight from cold; 30–45 min in the freezer before scoring.

## Claims

### proof

- Proofing completes the fermentation and can be done at room temperature or in the fridge, depending on available time. [breadcode-0352; SF Ch 7 Wheat sourdough §7.1 The process; creator-rule]
- Extensible, high-hydration dough inflates more in the oven, giving a bigger loaf with an airier, more open crumb. [breadcode-0394; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- High-hydration dough can stick to the banneton during proofing. [breadcode-0399; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Over-proofing removes the dough strength needed to rise upward, so the loaf stays flat. [breadcode-0400; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- In a loaf pan, an over-fermented dough should proof at least 30 min before baking, so transfer cavities even out. — `{"pan_proof_min": 30}` [breadcode-0536; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Over-fermented dough; creator-rule]
- The author notes that pan proofing of an over-fermented dough can be extended to 24 h or even 72 h, which gives a very tangy bread. — `{"pan_proof_h": [24, 72]}` [breadcode-0537; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Over-fermented dough; creator-rule]
- A banneton supports dough during the final proof and is typically rattan or wood pulp; a bowl lined with a kitchen towel is an alternative. [breadcode-0816; SF Glossary > Banneton; creator-rule]
- While resting in a banneton, the dough surface dries out and becomes easier to score before baking. [breadcode-0817; SF Glossary > Banneton; creator-rule]
- After the final rise, press a lightly floured finger about half an inch (12.7 mm, converted) into the dough: a slow spring-back leaving a slight indentation means ready; a quick spring-back means more time; collapse or no spring-back means over-fermented. — `{"press_depth_mm": 12.7}` [breadcode-0839; SF Glossary > Finger poke test; creator-rule]
- Retarding slows the proof by placing dough in a colder environment, usually a refrigerator, which lets bakers schedule when bread is ready. [breadcode-0873; SF Glossary > Retarding; creator-rule]
- Some bakers say retarding improves flavour, in addition to scheduling. [breadcode-0874; SF Glossary > Retarding; creator-rule] *(low confidence)*
- A honeycomb crumb needs precise fermentation: too long lets gas escape during baking, too short leaves too little gas to inflate the structures. [breadcode-1025; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Perfect f; creator-rule]
- Example: a dough ideal after 8 h of retarding might be ready in about 4 h if it started warmer. — `{"retard_ideal_h": 8, "retard_warm_start_h": 4}` [breadcode-1043; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- A dough that starts cold stops fermenting quickly in the fridge; a short room-temperature proof before refrigerating can help. [breadcode-1044; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Proofing makes the dough more extensible and lets it inflate more. [breadcode-1046; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- Cooling a 30 °C dough to 4-6 °C in a fridge takes much longer than cooling a dough that starts at 20-25 °C, and the dough stays more active; overnight retarding can overproof it. — `{"start_temp_c": [20, 30], "fridge_c": [4, 6]}` [breadcode-1095; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- After shaping, rest at least about 1 h; the dough should rise a further 20–30 %. Dent with a finger: the dent should slowly spring back, and if it collapses the dough has waited too long. — `{"rest_min_h": 1, "rise_pct": [20, 30]}` [breadcode-1125; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Cover resting dough with anything that stops drying while keeping some air exchange, e.g. plastic wrap, towel or foil. [breadcode-1129; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Sourdough proofed 3 h at room temperature and baked the same evening rose less and had a tighter crumb than sourdough retarded about 10 h in the fridge; taste was similar. — `{"rt_proof_h": 3, "fridge_proof_h": 10}` [breadcode-1139; repo:the-bread-code experiments/proofing-sourdough-3-hours-room-temperature-vs-10-hours-fridge.md; measured]
- Basic bread: proof about 2 h at room temperature or about 24 h in the fridge. — `{"proof_rt_h": 2, "proof_fridge_h": 24}` [breadcode-1152; repo:the-bread-code recipes/savory/basic-bread.md; creator-rule]
- Basic sourdough proof: 2–4 h at room temperature, or 1 h at room temperature then 6–24 h in the fridge. — `{"proof_rt_h": [2, 4], "proof_fridge_h": [6, 24]}` [breadcode-1161; repo:the-bread-code recipes/sourdough/basic-sour-dough.md; creator-rule]
- Mischbrot: after shaping in a rice-floured banneton, retard 24 h at about 5 °C. — `{"retard_h": 24, "fridge_c": 5}` [breadcode-1166; repo:the-bread-code recipes/sourdough/mischbrot-aka-graubrot.md; creator-rule]
- Broa: ferment about 10 h at about 22 °C, shape in a banneton, retard 24 h at about 5 °C, then bake 25 min covered + 20 min uncovered at 230 °C and rest 30 min. — `{"bulk_h": 10, "retard_h": 24, "bake_covered_min": 25, "bake_uncovered_min": 20, "bake_c": 230, "rest_min": 30}` [breadcode-1172; repo:the-bread-code recipes/sourdough/portuguese-broa-corn-bread.md; creator-rule]
- Sourdough pizza: rest 1 h, then divide into two balls of about 260 g; rest 1 h at room temperature (2 h cold), then refrigerate at least 24 h, for up to 2–3 days. — `{"rest_h": 1, "ball_g": 260, "balls": 2, "fridge_min_h": 24, "fridge_max_days": [2, 3]}` [breadcode-1182; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- Sourdough pizza fridge storage: keep at 4–5 °C at most, or the dough overproofs. — `{"fridge_max_c": 5, "fridge_min_c": 4}` [breadcode-1183; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- Sourdough pizza dough: take out of the fridge at least 1 h before stretching, and no longer. — `{"out_before_stretch_min": 60}` [breadcode-1185; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- Sourdough sandwich loaf: leaven the night before, incorporate after about 8 h, autolyse 30 min, then knead 15–20 min with salt added after the autolyse. — `{"incorporate_after_h": 8, "autolyse_min": 30, "knead_min": [15, 20]}` [breadcode-1188; repo:the-bread-code recipes/sourdough/sourdough-sandwich.md; creator-rule]
- Pan sourdough loaves rise less than free-standing shapes; the pan rise takes about 8–12 h. — `{"pan_rise_h": [8, 12]}` [breadcode-1189; repo:the-bread-code recipes/sourdough/sourdough-sandwich.md; creator-rule]
- Pizza dough: if eating within 3 h, ball it; otherwise refrigerate and take it out about 3 h before use. — `{"eat_within_h": 3, "remove_from_fridge_h": 3}` [breadcode-1212; repo:pizza-dough README.md; creator-rule]
- Pizza dough: after dividing, preshape and rest about 3 h. — `{"preshape_rest_h": 3}` [breadcode-1213; repo:pizza-dough README.md; creator-rule] *(low confidence)*
- Covering dough during rest prevents a dry skin; dry patches stay visible in the final crumb. [breadcode-1226; yt:MrOM_D3hnpc 1:15:49; creator-rule]
- Flour dusting of the banneton is unnecessary; a rice- or flour-dusted crust becomes burnt and needs scraping. [breadcode-1243; yt:MrOM_D3hnpc 2:04:44; creator-rule]
- Fridge proof guideline: start with about 30 min at room temperature, then at least 12 h in the fridge. For this loaf he gave 1 h at room temperature before refrigerating overnight. — `{"room_temp_min": [30, 60], "fridge_h_min": 12}` [breadcode-1251; yt:ImVvQMvGZKE 30:16–30:52; creator-rule]
- In his test, cold proofing gave slightly more oven spring than room-temperature proofing but not a clearly better crumb. [breadcode-1263; yt:ImVvQMvGZKE 2:54; measured]
- The finger-poke test works at room temperature but not in the fridge, because dough viscosity changes when cold. [breadcode-1307; yt:KRO9EcIyXSI 36:31; creator-rule]
- Cold-retarded pizza dough keeps up to 7 days; take it out about 3 h before shaping. — `{"retard_max_days": 7, "warm_before_shape_h": 3}` [breadcode-1323; yt:-60DxtcJ9Fo description; creator-rule]
- Covered shaped pizza balls on a wet towel start being good about 3 hours later and stay good for another 3 to 6 hours. — `{"rest_h": 3, "good_additional_h": [3, 6]}` [breadcode-1324; yt:-60DxtcJ9Fo description; creator-rule]
- Final proof: about 60 min at room temperature, then 10–12 h in the fridge for a next-morning bake. — `{"room_temp_min": 60, "fridge_h": [10, 12]}` [breadcode-1374; yt:o_xtVmR4vlc 21:59; creator-rule]
- Put the shaped dough in the freezer for about 45 min before scoring (same-day bake trick); it is then colder and easier to score. — `{"freezer_min": 45, "dough_temp_before_c": 22}` [breadcode-1392; yt:NMglhwp2lNs 31:17-31:52; creator-rule]
- Check proof with the finger-poke test every 15 min; the dough is ready when it springs back slowly. — `{"check_interval_min": 15}` [breadcode-1395; yt:NMglhwp2lNs 30:38; creator-rule]
- A cold retard in the fridge (about 24 h at about 4 °C, lower shelf) can replace about 4 h of room-temperature proof for a bake on another day. — `{"fridge_h": 24, "fridge_temp_c": 4}` [breadcode-1422; yt:NMglhwp2lNs 30:03; creator-rule]
- A fridge slows fermentation, so a dough can be held cold until a planned bake time. [breadcode-1448; yt:l0nAMbbYziI 12:54; 13:29; creator-rule]
- In the finger-poke test, a dent that recovers quickly means wait longer; slow recovery means ready; no recovery means bake now, with the oven hot. [breadcode-1473; yt:qM9BXKaIr60 30:21; 30:54; creator-rule]
- Room-temperature proofing typically takes 1.5–3 hours. — `{"proof_h": [1.5, 3]}` [breadcode-1474; yt:qM9BXKaIr60 30:54; creator-rule]
- Retard in a fridge at about 4–6 °C for at least 16 hours; going much longer risks over-proofing. — `{"temp_c": [4, 6], "retard_h": [16, 24]}` [breadcode-1475; yt:qM9BXKaIr60 31:29; 32:03; creator-rule]
- Finger-poke test: if the dent is still visible about one minute later, ciabatta is ready to bake. [breadcode-1499; yt:HSdOUZq5i6U 19:15; 19:52; creator-rule]
- Shaped ciabatta can be refrigerated overnight and baked the next day if timing requires. [breadcode-1500; yt:HSdOUZq5i6U 19:52; creator-rule]
- After shaping, rest 15 minutes, then refrigerate at about 4 °C overnight until the bake. — `{"temp_c": 4, "rest_min": 15}` [breadcode-1510; yt:HLPNdyGCSPk 15:16; 15:49; creator-rule]
- For a high-hydration dough, line the banneton with linen; a stiffer dough can go without. [breadcode-1527; yt:HLPNdyGCSPk 15:16; 15:49; creator-rule]
- The yeast-based ciabattas needed roughly 30 more minutes than the sourdough ones before the finger-poke test showed they were ready. — `{"extra_proof_min": [30]}` [breadcode-1530; yt:HSdOUZq5i6U 17:51; anecdote] *(low confidence)*
- After shaping, the loaves rested 30 minutes at room temperature, then went into the fridge overnight for about 16 hours and were baked straight from cold. — `{"room_rest_min": 30, "fridge_h": 16}` [breadcode-1533; yt:JZBtmQXKWA8 22:40; 23:36; creator-rule]
- Move the fermenting dough to the fridge and hold it cold until about two hours before the planned bake, then divide and shape. — `{"fridge_hold_before_bake_h": 2}` [breadcode-1539; yt:l0nAMbbYziI 12:54; 13:29; creator-rule]
- Dough can be refrigerated overnight for about 8 to 10 hours after shaping and baked the next morning; the author did this for his test loaves. — `{"fridge_h": [8, 10]}` [breadcode-1543; yt:LQPCXMpxO4c 16:48; 28:30; creator-rule]
- Final proof is judged with a finger-poke test: the loaf is ready when the dent stays visible after about 1–2 minutes; for this dough the proof took about 1–2 hours. — `{"proof_h": [1, 2]}` [breadcode-1560; yt:36GdGT0XYwE 19:41; creator-rule]
- As a general rule the creator advises about 16 hours cold proof at 6 °C or 24 hours at 4 °C; for a morning bake he used one hour at room temperature followed by the fridge instead of a 24-hour fridge proof. — `{"temp_c": 6, "proof_h": 16, "alt_temp_c": 4, "alt_proof_h": 24}` [breadcode-1584; yt:yYkTrGHNW2w 25:37; creator-rule]
- A one-hour room-temperature step before the fridge helps finish proofing when the bake is scheduled for morning. — `{"room_proof_h": 1}` [breadcode-1585; yt:yYkTrGHNW2w 25:37; creator-rule]
- At about 4 °C fermentation almost stops, while at about 5 °C dough keeps fermenting slowly overnight. — `{"temp_c": [4, 5]}` [breadcode-1586; yt:yYkTrGHNW2w 26:48; anecdote]
- Proof is ready when a floured finger poke leaves a dent that recovers only slowly; poke about every 15 minutes near the end. — `{"poke_interval_min": 15}` [breadcode-1615; yt:0kFYyj3WKpE 19:06; creator-rule]
- After an overnight fridge rest, the sourdough croissants took about five hours at room temperature to double in size. — `{"proof_room_temp_h": 5}` [breadcode-1631; yt:dY9GeoZX8DY 22:14; anecdote] *(low confidence)*
- Overnight fridge proofing barely expanded the cold croissants; they needed room-temperature proofing afterwards. [breadcode-1632; yt:dY9GeoZX8DY 21:40; anecdote]
- Proof at a thermostat-controlled, steady temperature: a few degrees of change can speed or slow sourdough fermentation a lot. [breadcode-1639; yt:WOZLvsV1M8s 2:15; creator-rule]
- A floured finger poke is the proof test: a dent that recovers very slowly means the dough is ready; sourdough usually takes 1–3 hours, yeast dough 30 minutes to 2 hours. — `{"proof_h_sourdough": [1, 3], "proof_h_yeast": [0.5, 2]}` [breadcode-1642; yt:WOZLvsV1M8s 5:34–6:05; creator-rule]
- Two hours of room-temperature proofing did not double the shaped sourdough croissants; they needed about five hours. — `{"proof_h": 2}` [breadcode-1658; yt:dY9GeoZX8DY 21:07; anecdote] *(low confidence)*
- Proofing shaped laminated sourdough croissants for about eight hours risks over-proofing, so he moved them to the fridge overnight. — `{"proof_h": 8}` [breadcode-1659; yt:dY9GeoZX8DY 21:40; creator-rule]
- Cold laminated dough proofs much more slowly than warm dough, so the final proof takes much longer than expected. [breadcode-1660; yt:dY9GeoZX8DY 17:36; creator-rule]
- A shaped loaf can be retarded in the fridge for 8 to 48 h. — `{"bake_after_h": [8, 48]}` [breadcode-1703; yt:9fWzWpL5g6w 18:28; creator-rule]
- A whole-wheat loaf at 110 % hydration was proofed about 2 h at room temperature until the finger-poke dent recovered slowly, then retarded in the fridge. He planned 8-9 h there; the loaf actually spent about 11 h before baking. — `{"room_h": 2, "fridge_planned_h": [8, 9], "fridge_actual_h": 11}` [breadcode-1711; yt:2DFvfRIywC8 25:39 (room temperature ~2 h);; anecdote]
- Yeast starts working above about 6 °C; a fridge proof of about 24 h at about 4 °C is typical. — `{"yeast_active_above_c": 6, "temp_c": 4, "fridge_h": 24}` [breadcode-1714; yt:dl7crk2Nvj8 15:38; creator-rule]
- A hybrid proof (30 min at room temperature, then fridge) works well. — `{"room_temp_min": 30}` [breadcode-1715; yt:dl7crk2Nvj8 16:12; creator-rule]
- Retarding the shaped dough in the fridge gives more oven spring and makes it easier to score, because cold dough is firmer. [breadcode-1754; yt:2DFvfRIywC8 25:06; creator-rule]
- He attributes the modest ear and crumb partly to fermenting too long in the fridge: about 11 h instead of the planned 8 h. He calls fermentation timing the key variable for this dough. — `{"fridge_actual_h": 11, "fridge_planned_h": [8, 9]}` [breadcode-1755; yt:2DFvfRIywC8 31:37; 27:30 (about 11 h in fr; anecdote] *(low confidence)*
- After shaping, the creator proofs 60 minutes at room temperature, then retards 12–16 hours at 4 °C in the fridge. — `{"room_proof_min": 60, "fridge_c": 4, "fridge_h": [12, 16]}` [breadcode-1764; yt:9yI7QRpAgow description; creator-rule]
- Freezing shaped dough briefly before baking is presented as a more reliable alternative to long fridge proofing; proof at room temperature until the finger-poke test passes. [breadcode-1768; yt:9yI7QRpAgow 13:29; creator-rule]
- Final proof is done when a floured finger poke springs back very slowly; proof took 30 min to 2 h, depending on yeast amount and water temperature. — `{"proof_min": [30, 120]}` [breadcode-1796; yt:vyDTKFtfIb0 14:44–15:19; creator-rule]
- If the dough is overproofed the finger dent does not recover at all and the dough feels sticky and fluffy. [breadcode-1797; yt:vyDTKFtfIb0 15:19; creator-rule]
- Fridge proofing: after a 30 min room-temperature rest, proof in the fridge for about 8 h and up to 24 h. A warmer fridge proofed the dough much more, so the finger poke test is the safer check. — `{"room_rest_min": 30, "fridge_proof_h": [8, 24], "fridge_temp_c": 8}` [breadcode-1826; yt:B_2SEmyyvso 15:35; creator-rule]
- Proof 2 h at room temperature, then 2 h in the fridge (he moved them to the fridge because guests were coming). He recommends a 24 h cold proof as a reproducible option. — `{"proof_rt_h": 2, "fridge_h": 2, "cold_proof_recommended_h": 24}` [breadcode-1831; yt:3vr2_qqeAL8 16:20; creator-rule]
- Cold proof gave better oven spring, the creator thinks because cooled gases hold better. [breadcode-1832; yt:3vr2_qqeAL8 18:07; anecdote] *(low confidence)*
- Rice flour in the banneton keeps sticky dough from sticking. [breadcode-1841; yt:BnvhhxQZQ14 9:37; creator-rule]
- Proof about 2 hours at room temperature, then about 8 hours in the fridge; use the finger poke test to judge. [breadcode-1842; yt:BnvhhxQZQ14 12:31; creator-rule]
- Optional freezer hack: freeze the shaped dough for about 30 min before scoring; it makes scoring easier. — `{"freeze_min": 30}` [breadcode-1870; yt:B_2SEmyyvso description; creator-rule]
- Take the cold-retarded pizza dough out of the fridge about 3 hours before baking, divide and shape it into balls, and cover them with a wet towel while they rest. — `{"rest_h": 3}` [breadcode-1874; yt:rcOxoJk7goM 11:00; creator-rule]
- Shaped whole wheat dough in a loaf pan can be retarded in the fridge at about 4 °C for at least 12 h. — `{"temp_c": 4, "retard_h": 12}` [breadcode-1959; yt:agCpWuDgAwg 7:22; creator-rule]
- For a viewer's 10 h fridge proof, Dr. Dough judged it probably too short; his own routine is about 1 h at room temperature then 16–24 h in the fridge. — `{"room_h": 1, "retard_h": [16, 24]}` [breadcode-1965; yt:HgIKSVMkbFg 7:21; creator-rule]
- A 1.5 h final proof sounds too short to him; he typically proofs roughly 2.5–3 h, probably including freezer time. — `{"proof_h": [2.5, 3]}` [breadcode-1966; yt:HgIKSVMkbFg 9:45; creator-rule]
- Final proof: about 1 h at room temperature, then about 8 h in the fridge. — `{"proof_h": 1, "retard_h": 8}` [breadcode-1972; yt:APstLJpWOiE 13:04; creator-rule]
- He typically recommends a 24 h fridge retard for a consistent environment; for this bake he used 1 h at room temperature followed by a fridge retard because guests were coming. — `{"retard_h": 24}` [breadcode-1974; yt:APstLJpWOiE 12:26; creator-rule]
- A proofing box with on/off thermostat control holds a set temperature, so proof timing becomes repeatable; his setpoint was 25 °C. — `{"temp_c": 25}` [breadcode-1975; yt:PDeLAAloWNA 6:13; anecdote]
- Proof about 1 h at room temperature, then about 16 h in the fridge. — `{"proof_h": 1, "retard_h": 16}` [breadcode-1981; yt:lJ38fY3ZEQk 5:36; creator-rule]
- Dough came out of the fridge at about 7.5 °C. — `{"dough_temp_c": 7.5}` [breadcode-1982; yt:lJ38fY3ZEQk 7:30; measured] *(low confidence)*
- A cold proof makes the finger-poke test unreliable. [breadcode-1993; yt:0BA-Lka_QMI 6:34; creator-rule]
- Below about 6 °C fermentation practically stops; the first ~6 h in the fridge still ferment. — `{"temp_c": 6}` [breadcode-1996; yt:vf5RbwlW8Xs 8:41; creator-rule] *(low confidence)*
- After the finger-poke test passes, 30 min in the freezer before baking lets you bake on time. — `{"freezer_min": 30}` [breadcode-1997; yt:vf5RbwlW8Xs 9:15; creator-rule]
- Dough proofed only in the fridge (some people use ~24 h) becomes too stiff to pass the finger-poke test; adding about 1 h at room temperature before a shorter fridge stage (~8 h) is one schedule he suggests, to be tuned by experiment because fridge temperature matters. — `{"rt_h": 1, "retard_h": 8, "fridge_only_h": 24}` [breadcode-2001; yt:02WlZa_N-vI 5:10; creator-rule]
- In his one-off comparison, the loaf given 30 min at room temperature then 16 h in the fridge had more vertical oven spring; he judged its crumb about equal to, or slightly less open than, the room-temperature loaf. — `{"rt_h": 0.5, "retard_h": 16}` [breadcode-2002; yt:02WlZa_N-vI 14:36; 15:50–16:24; measured] *(low confidence)*
- Finger-poke test for room-temperature proofing: the dough is ready when a poke springs back slowly and the dent stays for a while. [breadcode-2007; yt:02WlZa_N-vI 7:04; creator-rule]
- In a loaf pan, a cold-retarded whole wheat sandwich dough stays usable in the fridge for about 12 to 36 h; he says a banneton supports the dough less well over that span. — `{"retard_h": [12, 36]}` [breadcode-2012; yt:agCpWuDgAwg 7:58; creator-rule]
- A dough that is warmer when it goes into the fridge keeps fermenting longer there, so it proofs more overnight. [breadcode-2040; yt:vf5RbwlW8Xs 12:27; creator-rule]
- Roll dough is retarded in a very cold fridge, below about 6 °C, for at least 8 h and up to 24–48 h. — `{"temp_c": 6, "retard_h": [8, 48]}` [breadcode-2047; yt:OcOpIISBK6c [4:52]; creator-rule]
- A towel-lined bowl can replace a banneton for proofing the rolls. [breadcode-2049; yt:OcOpIISBK6c [4:13]; creator-rule] *(low confidence)*
- Fridge proof then bake the next morning is the alternative schedule. [breadcode-2063; yt:0aylYlvA4K8 [7:48]; creator-rule]
- For room-temperature proofing he recommends freezing the shaped loaf for about 30 min before baking, which makes scoring easier. — `{"freeze_min": 30}` [breadcode-2064; yt:0aylYlvA4K8 [7:16]; [8:55]–[9:28]; creator-rule]
- Fridge proofing is hard to time: fridge temperature can drift, and the finger-poke test does not work in the fridge; he recommends room-temperature proofing for beginners. [breadcode-2065; yt:0aylYlvA4K8 [9:28]; creator-rule]
- A fridge reading of about 9 °C was measured; dough keeps proofing at that temperature in a fridge. — `{"temp_c": 9}` [breadcode-2069; yt:NrKHO2xZTEU [2:29]; measured] *(low confidence)*
- Proof 15 min at room temperature, then retard in the fridge overnight. — `{"proof_min": 15}` [breadcode-2080; yt:MPWypFY0wyE [11:51]–[12:26]; creator-rule]
- Proof time is judged by a finger poke, typically 1–3 h. — `{"proof_h": [1, 3]}` [breadcode-2090; yt:gOxCUh_kyMQ [13:35]; creator-rule]
- Room-temperature proof is checked with a poke test within 2 h; a fridge proof needs at least 12 h, since 4 °C halts activity. — `{"proof_h": 2, "fridge_min_h": 12, "fridge_c": 4}` [breadcode-2101; yt:_cb4vCQ2shk [9:06]–[9:41]; creator-rule]
- Proof at room temperature until the dough nearly passes the finger-poke test, then move it to the freezer to make scoring easier. [breadcode-2113; yt:4VJQqOtwP_E [13:52]; creator-rule]
- A finger-poke dent that recovers slowly means the dough is ready to bake. [breadcode-2126; yt:6x9ic4YIni8 [11:51]; creator-rule]
- The rested dough is covered with wet towels so it does not dry out; this matters more in warm environments. [breadcode-2130; yt:EI4RoemHe5M [7:03]; creator-rule]
- Cold-proofed dough is stiffer and easier to score than room-temperature dough. [breadcode-2140; yt:0aylYlvA4K8 [8:55]; creator-rule]
- In the finger poke test, a dent that is still visible about 1 minute after poking means the dough is ready to bake. — `{"poke_recovery_min": 1}` [breadcode-2155; yt:_cb4vCQ2shk [9:06]; creator-rule]
- Proofing time has no fixed value; it depends on temperature, the amount of starter and its activity, so judge it by the poke test. [breadcode-2242; yt:nIOPCeLPqrM 7:31; creator-rule]
- Finger-poke test: a dent that springs back at once means under-proofed; one that springs back slowly means ready; one that stays put means over-proofed. [breadcode-2243; yt:nIOPCeLPqrM 8:07; creator-rule]
- Dough sticking to the banneton collapses in the oven into a flat pancake; flour the banneton's edges well and shake off the excess flour after baking. [breadcode-2286; yt:5--bR1mPiZE 4:03; creator-rule]
- Cover the banneton loosely with foil or a towel, without tension, so that the cover does not restrict the rise. [breadcode-2287; yt:5--bR1mPiZE 4:39; creator-rule]
- A slightly overfermented dough gives a flatter loaf with small pockets. [breadcode-2309; yt:sFQZIKb_iIA 0:10; creator-rule]
- An underfermented dough gives large pockets in the crumb and is sometimes called fool's crumb. [breadcode-2311; yt:sFQZIKb_iIA 3:42; creator-rule]
- A finger poke test can be used to judge when a dough is done proofing, but it does not work with fridge-proofed doughs. [breadcode-2323; yt:JoAG7R9QESA 6:41; creator-rule]
- The discard loaf is proofed in a bag for about 3 h, then baked about 45 min, half with steam and half without. — `{"hours": 3, "min": 45}` [breadcode-2349; yt:7V3FyVzzVUI 7:34; creator-rule]
- An underfermented dough gives pockets and a fool's crumb. [breadcode-2358; yt:XkoIlgnIeaQ 1:46; creator-rule]
- Severe overfermentation makes the dough flat and sticky, because the gluten has broken down. [breadcode-2361; yt:XkoIlgnIeaQ 8:33; creator-rule]
- During the bag proof the discard loaf rises about 2 cm. — `{"rise_cm": 2}` [breadcode-2386; yt:7V3FyVzzVUI 6:58; creator-rule] *(low confidence)*
- After shaping, the dough proofs at room temperature for about an hour before going into the fridge overnight. — `{"proof_rt_min": [30, 60]}` [breadcode-2396; yt:JoAG7R9QESA 7:16; creator-rule]
- Degassed starter dough rested 30 to 60 minutes before baking so it puffs up again. — `{"rest_min": [30, 60]}` [breadcode-2400; yt:sB81OLY25Jo 7:20; anecdote] *(low confidence)*
- An underfermented dough should be fermented about 30 minutes to 2 hours longer, and it should show some acidity when tasted. — `{"extra_ferment_min": [30, 120]}` [breadcode-2401; yt:sFQZIKb_iIA 4:19; creator-rule]
- After shaping, the loaf proofs at room temperature until the finger poke test passes, or is refrigerated for 12 to 24 h before baking. — `{"fridge_h": [12, 24]}` [breadcode-2411; yt:vLdmMuAxB7w description; creator-rule]
- After preshaping naan into balls, rest them at least about 30 minutes before the final shaping; shaping sooner makes the dough pull back. — `{"rest_min": 30}` [breadcode-2444; yt:sH99_IGh0ic 6:15; creator-rule]
- Proof the panned Schwarzbrot about two hours at room temperature, then about eight hours in the fridge before baking. — `{"proof_room_h": 2, "retard_fridge_h": 8}` [breadcode-2461; yt:cHwLBs9SRd4 description; creator-rule]
- Refrigerate the shaped dough overnight and bake it the next day. [breadcode-2501; yt:0h63OloQQd8 3:31; creator-rule]
- Shaped loaves can proof in the fridge for about 24 hours, covered with a plastic bag or a liner so the surface does not dry out. — `{"retard_h": 24}` [breadcode-2539; yt:2tOw0eyuEsU 5:49; creator-rule]
- Buns are ready to bake when a fingertip dent springs back very slowly. [breadcode-2553; yt:Dx7xM8IThCI 5:04; creator-rule]
- Covered rolls proof for two to three hours until doubled in size. — `{"proof_h": [2, 3]}` [breadcode-2682; yt:9dsgUixpOBM 5:14; creator-rule]
- Experimental proof schedule for both arms: 2 h at room temperature, then 8 h in the fridge, baked the next morning. — `{"proof_room_h": 2, "proof_cold_h": 8}` [breadcode-2697; yt:bBSuFq346YM 3:22; measured]
- Dough balls rest covered for about 2 h before shaping. — `{"rest_h": 2}` [breadcode-2719; yt:ziMMCphF054 3:52; creator-rule]
- A 24 h fridge final proof in the loaf pan gives slightly better oven spring than a room-temperature proof (about 3 h); the creator states this without a test. — `{"proof_cold_h": 24, "proof_room_h": 3}` [breadcode-2732; yt:2NmD9KVyZCg 6:48; creator-rule]
- Shaped pieces rest on a floured towel under a cover for one to two hours until puffy. — `{"proof_h": [1, 2]}` [breadcode-2751; yt:5DWcrVHK1WU 3:14; creator-rule]
- The creator wonders whether proofing a loaf in the fridge under a kitchen towel, instead of inside a plastic bag, could explain much larger crust blisters. [breadcode-2762; yt:JltQXoU786k 1:10; anecdote] *(low confidence)*
- At room temperature, a seed loaf takes about 2 to 3 h to rise about 30 %; warmer conditions shorten this and colder ones lengthen it. — `{"duration_h": [2, 3], "rise_pct": 30}` [breadcode-2771; yt:cN_skGXZ4Es 2:02; creator-rule]
- A very wet loaf proofed 24 h in the fridge may not rise as much as intended, because the extra water evaporates in the oven and still gives oven spring. — `{"cold_proof_h": 24}` [breadcode-2772; yt:cN_skGXZ4Es 2:38; extrapolation] *(low confidence)*
- Cold-fermenting the shaped dough for 24 h in the fridge gives a consistent environment and a well-risen loaf before baking. — `{"cold_ferment_h": 24}` [breadcode-2780; yt:v8VSfDF3Tqs 1:13; creator-rule]
- Proof shaped buns until they pass the finger-poke test before baking. [breadcode-2807; yt:nIQEiSGGw2Q description; creator-rule]
- After shaping, the buns rest about 1 hour on lining before baking. — `{"rest_h": 1}` [breadcode-2817; yt:nIQEiSGGw2Q 0:43; creator-rule] *(low confidence)*
- Proof croissants until doubled in size before egg wash and baking. — `{"rise_pct": 100}` [breadcode-2832; yt:7wyikR8qwHQ description; creator-rule]
- Shaped loaves proof 24 h in the fridge before baking. — `{"proof_h": 24}` [breadcode-2900; yt:MyGMF5sLg1U description; creator-rule]
- In his routine, after preshaping the dough rests 10 minutes on the bench, is shaped, then proofs 24 hours in the fridge before baking in a Dutch oven or with lots of steam. — `{"bench_rest_min": 10, "proof_fridge_h": 24}` [breadcode-2943; yt:U59OdOmemWM description; creator-rule]
- Rice flour in the banneton stops dough sticking, so it can hide poor fermentation. [breadcode-2952; yt:HlgA2O3J0o0 0:00; creator-rule]

### proof.cold-retard

- Some bakers say cold proofing improves flavour; in the author's retarded loaves he could not tell a flavour difference. [breadcode-0589; SF Ch 7 Wheat sourdough §7.13 Proofing; anecdote] *(low confidence)*
- Cold proofing mainly helps manage the schedule: a dough shaped at 22:00 can be retarded overnight and baked the next day instead of needing about 2 h proof plus 1 h bake that evening. — `{"proof_wait_h": 2, "bake_wait_h": 1}` [breadcode-0591; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- After shaping, the dough can go straight into the fridge, proof overnight and be baked at any time the next day. [breadcode-0592; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- Large bakeries proof dough a day ahead in the fridge, bake directly out of the fridge early morning, and have the first bread ready within about 2 h. — `{"ready_within_h": 2}` [breadcode-0593; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- Retarded dough can be baked anywhere from about 6 h to 24 h after it goes into the fridge. — `{"bake_window_h": [6, 24]}` [breadcode-0594; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- Use cold proofing (fridge) when you do not plan to bake within about the next 3 h. — `{"bake_after_h": 3}` [breadcode-0608; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- A dough can rest in the fridge for up to about 24 h; in some experiments it was still good at 48 h. — `{"fridge_max_h": 24}` [breadcode-0611; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- Find the best fridge proofing time iteratively: try 8 h on the first dough, 10 h on the second, 12 h on the third, and so on up to 24 h. — `{"fridge_h": [8, 24]}` [breadcode-0614; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- A fridge is typically at a constant temperature, so timings that work once can be relied on for later doughs. [breadcode-0615; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- Shaped dough can go into the fridge overnight and stays usable for about 8 to 24 hours; it is baked straight from the fridge without warming up first. — `{"fridge_h": [8, 24]}` [breadcode-1408; yt:msqU-ylXWUs 19:14; creator-rule]
- Finger poke test for a same-day bake: after 30 minutes poke the dough; if the dent is still visible a minute later, freeze it 30 minutes while preheating the oven; if not, wait another 30 minutes and repeat. — `{"first_wait_min": 30, "freezer_min": 30, "repeat_wait_min": 30}` [breadcode-1409; yt:msqU-ylXWUs 19:14; creator-rule]
- Proof the shaped dough at room temperature for about 2 h until it almost passes the finger-poke test, then refrigerate overnight (about 8 h). The fridge slows fermentation; the creator says the cold gas keeps the dough from bursting up when it meets the hot oven. — `{"proof_room_h": 2, "fridge_h": 8}` [breadcode-1892; yt:U9_GVHuI1Ro 14:58; creator-rule]
- Cold-proofing shaped dough in the fridge takes about 18–24 h; the creator suggests starting at 18 h and going to 24 h if oven spring is weak. Fermentation is still active early on and nearly stops later, so the bake time is flexible by an hour or two; fridge temperature and shelf position change the time needed. — `{"fridge_proof_h": [18, 24]}` [breadcode-1908; yt:b7Pd7UcZxd0 7:16; 7:50; creator-rule]
- The grains were bulk fermented about 6 h at room temperature until they had risen to the same height. They were then shaped and refrigerated overnight; the cold rest firms the very sticky dough so it does not spread in the oven. — `{"bulk_room_h": 6}` [breadcode-1929; yt:PehhnV1lW2o 21:41; 27:59-28:40; creator-rule] *(low confidence)*
- Shaped dough held in the fridge becomes insensitive to timing because fermentation is still active early in the cold stage and almost stops later, so it can be baked an hour or two either way. [breadcode-1936; yt:b7Pd7UcZxd0 7:50; creator-rule]
- After the fold, each dough rests 15 minutes at room temperature, goes into the fridge overnight, and is baked straight from the fridge the next morning. — `{"rest_min": 15}` [breadcode-2171; yt:bJxpCVc2wYE 5:02; creator-rule]
- After shaping, rest the loaf 15 minutes at room temperature, then refrigerate overnight and bake the next morning. — `{"rest_min": 15}` [breadcode-2191; yt:I3h6heaKWC8 2:59; creator-rule]
- Proof 30 to 60 minutes at room temperature after shaping, then refrigerate overnight and bake the next morning straight from the fridge. — `{"proof_min": [30, 60]}` [breadcode-2235; yt:iEfLO_Cl0uE description; creator-rule]
- After the poke test, refrigerate the dough for about 8 hours; cold dough is easier to score and gives more oven spring. — `{"retard_h": 8}` [breadcode-2244; yt:nIOPCeLPqrM 9:23; creator-rule]
- Refrigeration cools the gas in the dough, which then expands more during baking and gives extra spring. [breadcode-2245; yt:nIOPCeLPqrM 9:23; extrapolation] *(low confidence)*
- Either prove until the dough almost passes the poke test and then refrigerate about 8 hours, or refrigerate right after shaping for about 24 hours. — `{"retard_h": [8, 24]}` [breadcode-2256; yt:oAFjGTWMgUo 10:44; creator-rule]
- For the overnight loaf, the dough is retarded in the fridge at 4 °C for 8 to 24 hours after a 1-hour room-temperature proof. — `{"proof_room_h": 1, "retard_temp_c": 4, "retard_h": [8, 24]}` [breadcode-2388; yt:BUtn4HKAiBs description; creator-rule]
- Cold-proof the shaped seed loaf in the fridge at about 5–7 °C for about 24 h. — `{"temp_c": [5, 7], "duration_h": 24}` [breadcode-2770; yt:cN_skGXZ4Es 2:02; creator-rule]
- Refrigerate the pasta dough at about 4 °C for up to a month to deepen its flavour. — `{"temp_c": 4, "duration_h": 720}` [breadcode-2786; yt:A5blYerC8VE 0:34; creator-rule]
- Temperature differs between fridge shelves; at about 4 °C almost all microbial activity stops. — `{"temp_c": 4}` [breadcode-2962; yt:v8O8NkIy2tM 0:00; creator-rule]
- As a rule of thumb, move dough into the fridge for at least 12 h. — `{"retard_h": [12, null]}` [breadcode-2963; yt:v8O8NkIy2tM 0:00; creator-rule]
- Sourdough pizza dough can be refrigerated for up to 7 days; take it out about 3 h before shaping. — `{"retard_max_h": 168, "warm_up_h": 3}` [breadcode-2973; yt:lz0DC9JaqIk 0:00; creator-rule]

### proof.decision

- If an overnight dough is ready in the morning and you want to bake the same day, proof at room temperature rather than adding about 6 h in the fridge. — `{"fridge_proof_h": 6}` [breadcode-0595; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- Choose room-temperature or fridge proofing according to your schedule and availability. [breadcode-0596; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- Preheat the oven once the dough is close to perfect proofing, so the dough is not overproofed while the oven heats. [breadcode-0605; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- In warm conditions choose a shorter fridge proofing time; in cold conditions choose a longer one. [breadcode-0617; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- Decision flowchart for proofing: room-temperature route: poke, wait 15 min, repeat until the dent is still visible after one minute, then score and bake. Fridge route: if baking within about 10 h, wait 15 min then proof in the fridge at 4 °C (40 °F); otherwise go straight to the fridge; bake from the fridge. — `{"wait_min": 15, "dent_check_min": 1, "bake_within_h": 10, "temp_c": 4}` [breadcode-0620; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]

### proof.dusting

- Rice flour is used to dust the banneton because, according to the creator, it absorbs a lot of the dough's hydration. [breadcode-1894; yt:U9_GVHuI1Ro 9:54; creator-rule] *(low confidence)*

### proof.endpoint

- The 3-minute loaf is baked when the dough has roughly doubled (about a 100% size increase), judged by eye in the pan or by a marked sample jar. — `{"size_increase_pct": 100}` [breadcode-1903; yt:bi-zqG5y1I8 3:30; creator-rule]
- Bake the no-knead loaf once the dough has doubled, before the starter collapses, because the dough still has enough structure to give oven spring. — `{"size_increase_pct": 100}` [breadcode-1937; yt:bi-zqG5y1I8 1:09–1:41; creator-rule]

### proof.final

- For loaf pan bread proof until the dough has roughly doubled in size. — `{"rise_pct": 100}` [breadcode-0306; SF Ch 6 Bread types; creator-rule]
- Rye proof about 60 min; a longer proof does not make it fluffier, though a short proof gives a more uniform crumb. — `{"proof_min": 60}` [breadcode-0338; SF Ch 8 Non wheat sourdough; creator-rule]
- The buns were then proofed for 2 h on the baking tray. — `{"proof_h": 2}` [breadcode-2977; yt:ovzejNuq4Eo description; anecdote]
- Poke test for a room-temperature final proof: press the dough; if the dent is still visible about 1 minute later, the dough is ready to bake. — `{"poke_recovery_min": 1}` [breadcode-2986; yt:9pPWWxZ9tNM 0:00; creator-rule]
- The finger poke test does not work for dough that was proofed in the fridge; do not use it to judge fridge-proofed dough. [breadcode-2987; yt:9pPWWxZ9tNM 0:00; creator-rule]

### proof.finger-poke

- Finger-poke test at room temperature: a dent that recovers very slowly means proofed; one that springs back fast means under-proofed; one that stays means over-proofed. [breadcode-1907; yt:b7Pd7UcZxd0 6:08; creator-rule]

### proof.flour-rub

- Before proofing, rub the surface with flour again: it dries the surface, reduces sticking in the banneton and simplifies later scoring. [breadcode-0578; SF Ch 7 Wheat sourdough §7.12 Shaping > Prepare for proofing; creator-rule]

### proof.freezer-trick

- Placing the proofed dough in a freezer for about 30 min cools the surface by roughly 10 °C, which makes scoring easier and allows baking sooner; the creator notes the flavour may be less developed than with a fridge proof. — `{"freezer_min": 30, "surface_temp_drop_c": 10}` [breadcode-1911; yt:b7Pd7UcZxd0 10:40; 14:39–15:14; anecdote] *(low confidence)*

### proof.fridge

- Cold proof in a fridge needs about 16 to 24 h; fridge temperature swings can speed up fermentation and overproof the dough. — `{"fridge_h": [16, 24]}` [breadcode-1308; yt:KRO9EcIyXSI 37:18; creator-rule]
- Keep the fridge below 6 °C for retarding; above that, fermentation continues and the dough becomes sticky. — `{"fridge_max_c": 6}` [breadcode-1322; yt:-60DxtcJ9Fo description; creator-rule]
- After the rise, retard the loaf in the fridge for 12 to 24 h; the dough keeps rising slightly. — `{"fridge_h": [12, 24]}` [breadcode-1338; yt:013Qw3BojM4 12:37; creator-rule]
- At about 4 °C the cold-retarded dough almost stops fermenting, so it can wait days before shaping. — `{"fridge_c": 4}` [breadcode-1346; yt:-60DxtcJ9Fo 16:21; creator-rule]
- His own fridge practice is about 16 h at 4 °C; repeated door opening and temperature swings sped fermentation enough to over-proof a loaf. — `{"fridge_h": 16, "fridge_temp_c": 4}` [breadcode-1356; yt:KRO9EcIyXSI 38:36; anecdote]

### proof.fridge-cooling

- Chilling halts fermentation: dough that is ready early can go in the fridge and stay good for several days, getting more sour the longer it waits. [breadcode-0288; SF Ch 6 Bread types; creator-rule]
- Fermentation is slower the colder it gets; a few cultures are adapted to cold, but a fridge preserves dough state. [breadcode-0289; SF Ch 6 Bread types; creator-rule]
- Rye dough can be proofed in the fridge, where it stays good for weeks. [breadcode-0339; SF Ch 8 Non wheat sourdough; creator-rule]
- Fermentation stops when the dough drops below about 4 °C (40 °F); the exact threshold depends on the yeast and bacteria in the starter. — `{"temp_c": 4}` [breadcode-0610; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- A warmer dough going into the fridge takes longer to cool, so its fridge time has to account for the starting core temperature. [breadcode-0616; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- Retard the shaped rye loaf for 24 h in the fridge in a covered basket; the consistent cold makes the timing easy. — `{"retard_h": 24}` [breadcode-2587; yt:nsyjVa5SSQI 3:39; creator-rule]
- Rest the shaped loaf about 1 h at room temperature, then 2–3 h in the fridge; the cold stage lets you choose the bake time and, he says, gives a more open crumb. — `{"rest_h": 1, "fridge_h": [2, 3]}` [breadcode-2597; yt:l9g4AI_z3uY 9:52; creator-rule]
- Retard the shaped loaves 24 h in the fridge before baking. — `{"retard_h": 24}` [breadcode-2611; yt:bjLCHRInmLY 3:37; creator-rule]
- A cold-stored, over-fermented dough was still stiff enough to hold its shape and got some oven spring because the crust set quickly. [breadcode-2647; yt:lFzLCivvyes 2:37; anecdote] *(low confidence)*
- Once the dough is cooled, fermentation stops, so it can be kept for a week or more and baked later. — `{"storage_d": 7}` [breadcode-2655; yt:nlqVo9sxTlQ 3:15; creator-rule]

### proof.fridge-slow

- When dough has risen too much before it is ready to shape, putting it in the fridge slows fermentation; pre-shaped pieces can also be held cold until their turn to be shaped and baked. [breadcode-1856; yt:1Dbbfq5SKi0 4:54; creator-rule]

### proof.general

- Proofing is the final rise after shaping; the chemical and biological processes are the same as in bulk fermentation. [breadcode-0585; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- Shaping releases much of the gas built during bulk fermentation; proofing re-inflates the dough. [breadcode-0586; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- A dough that is not proofed has a noticeably less fluffy and softer crumb than a properly proofed dough. [breadcode-0587; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- There are two proofing options: room-temperature proofing, or cold proofing in the fridge (called retarding). [breadcode-0588; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]

### proof.hybrid

- Hybrid proof: proof at room temperature until near the finger-poke endpoint, then refrigerate about 8 h, which the creator uses for overnight bakes; the time can be over-run and over-ferment the dough. — `{"fridge_h": 8}` [breadcode-1909; yt:b7Pd7UcZxd0 9:01; creator-rule]

### proof.loaf-pan

- In the loaf pan the dough was proofed at room temperature for about 5 h, until a finger dent recovers very slowly. — `{"proof_h": 5}` [breadcode-1900; yt:xmt3eXzOQLM 11:38; anecdote] *(low confidence)*

### proof.oven-heat

- A freshly baked, still hot loaf in the oven next to unbaked dough made that dough proof noticeably faster. [breadcode-1857; yt:1Dbbfq5SKi0 12:04; anecdote] *(low confidence)*

### proof.poke-test

- For the finger poke test, flour the thumb and press about 0.5 to 1 cm deep into the dough. — `{"depth_cm": [0.5, 1]}` [breadcode-0599; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- Right after shaping, the poke dent recovers quickly and is gone after about one minute. — `{"dent_recovery_min": 1}` [breadcode-0600; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- As the dough fills with gas it also becomes more extensible; the poke dent then disappears more slowly. [breadcode-0601; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- The dough is ready to score and bake when the poke dent is still visible after one minute. — `{"dent_visible_min": 1}` [breadcode-0602; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- Perform the finger poke test about every 15 minutes during proofing. — `{"poke_interval_min": 15}` [breadcode-0603; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- The finger poke test does not work on cold dough: low temperatures change its elasticity, so the dent never recovers. [breadcode-0613; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]

### proof.retard

- After a 15-minute room-temperature rest and shaping, the banneton went into the fridge overnight before baking. — `{"rest_min": 15}` [breadcode-2690; yt:0oKA_iTytiw 3:15; creator-rule]
- Shaped einkorn dough chilled for 24 h in the fridge gains flavour and closes up into a consistent shape. — `{"retard_h": 24}` [breadcode-2704; yt:wz7tysfSDFM 3:49; creator-rule]
- To delay the bake, he moved the dough to a cold place (fridge, or outdoors in winter) for about 3 h, then brought it back in to shape. — `{"delay_h": 3}` [breadcode-2718; yt:ziMMCphF054 2:39; creator-rule] *(low confidence)*
- Overnight refrigeration is optional; the dough can also be shaped the same day. [breadcode-2750; yt:5DWcrVHK1WU 1:38; creator-rule]
- After shaping, he lets the dough sit briefly at room temperature, then refrigerates it overnight before baking. [breadcode-2760; yt:lG9ERQFFn5I 3:05; creator-rule]

### proof.room-temp

- Room-temperature proofing is the easiest and most reliable method and works well for an overnight dough proofed the next morning. [breadcode-0597; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]

### proof.time

- Room-temperature proofing typically takes 30 minutes to 3 hours; most bakers judge readiness with the finger poke test rather than timing. — `{"proof_min": [30, 180]}` [breadcode-0598; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; creator-rule]
- In the author's experience proofing takes at least 1 h and sometimes up to 3 h, and was never faster than 1 h even at warmer temperatures. — `{"proof_min": [60, 180]}` [breadcode-0604; SF Ch 7 Wheat sourdough §7.13 Proofing > Room temperature-proofing; anecdote] *(low confidence)*
