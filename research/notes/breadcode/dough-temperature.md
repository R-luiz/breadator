# dough-temperature — The Bread Code / The Sourdough Framework

Claims: 15 (book 3, videos 12, repos 0). Evidence: creator-rule 13, anecdote 1, measured 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

The book has no desired-dough-temperature formula. Its rules: warmer = faster; use water at ambient temperature (e.g. 30 °C water in a 30 °C room) so the aliquot tracks the dough; adjust inoculation by room temperature (Table 7.1). Videos: warmer water speeds fermentation, ≈30 °C water is fine (l9g4AI_z3uY); a 2 °C dough–sample mismatch can shift timing 3–4 h.

## Claims

### dough-temperature

- Warmer dough ferments faster and colder dough ferments slower. [breadcode-0425; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Retarding (cold proofing): as dough chills, fermentation slows; starting at a warmer temperature makes the deceleration take longer. [breadcode-1042; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Use water at about the ambient temperature; if the room reaches about 30 °C, start the dough with 30 °C water. — `{"water_c": 30, "ambient_c": 30}` [breadcode-1098; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- Match dough water temperature to the fermentation chamber temperature so the dough starts at chamber temperature. [breadcode-1234; yt:MrOM_D3hnpc 1:38:20; creator-rule]
- Use water at roughly the ambient (room) temperature so the dough ferments at the same rate as its environment; he checks dough temperature with the sample. [breadcode-1260; yt:ImVvQMvGZKE 11:30; creator-rule]
- Warm ambient conditions (around 30 °C) make dough ferment much faster than in winter, so judge timing from your own dough rather than the clock. — `{"ambient_c": 30}` [breadcode-1446; yt:l0nAMbbYziI 5:12; creator-rule]
- The main dough was about 2 °C above room temperature, so its small sample cooled faster and lagged; he estimates a 2 °C difference could change fermentation time by three or four hours. [breadcode-1483; yt:JZBtmQXKWA8 13:01; 13:40; 17:29; creator-rule] *(low confidence)*
- Keep the water and the flour for an overnight autolyse at the same room temperature as the main dough, so there is no temperature difference between the parts. [breadcode-1677; yt:yYkTrGHNW2w 21:15; creator-rule]
- A home fridge runs about 3.5 °C; above 4 °C is the food-safety danger zone. — `{"temp_c": [3.5, 4]}` [breadcode-1681; yt:l_PLhPWE-8E 8:19; creator-rule]
- pH changes with temperature. Creator used a temperature-correction calculator; finger poke test does not work on cold dough. — `{"ph_reading": 4.2, "dough_temp_c_estimated": 9, "fridge_set_c": 4}` [breadcode-1814; yt:dQEebR6pIng 14:41; creator-rule]
- Pizza dough kept in the fridge stayed usable after 7 days. [breadcode-1846; yt:rcOxoJk7goM 7:46; anecdote] *(low confidence)*
- At 11:55 he says that when dividing and pre-shaping several loaves, you should stop bulk at a slightly higher pH (about 4.30 instead of 4.20) so the dough keeps fermenting on the bench. — `{"ph_bulk_end_single_loaf": 4.2, "ph_bulk_end_multi_loaf": 4.3}` [breadcode-1872; yt:dQEebR6pIng 11:19; creator-rule]
- A 30 °C ferment and a 25 °C ferment reached doubling at a similar pH (about 4.1–4.3); the warmer dough was not clearly more acidic at that point. — `{"temp_c": [25, 30], "ph": [4.1, 4.3], "rise_pct": 100}` [breadcode-1994; yt:vf5RbwlW8Xs 4:08; measured] *(low confidence)*
- Warmer water speeds up fermentation. He says it should not be super hot and that about 30 C is fine. — `{"water_temp_c": 30}` [breadcode-2595; yt:l9g4AI_z3uY 2:50; creator-rule]
- Warm water is used to make a sourdough dough ferment faster. [breadcode-2677; yt:9dsgUixpOBM 0:45; creator-rule]
