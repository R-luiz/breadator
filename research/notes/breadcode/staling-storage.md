# staling-storage — The Bread Code / The Sourdough Framework

Claims: 65 (book 31, videos 30, repos 4). Evidence: creator-rule 54, anecdote 8, literature 2, extrapolation 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

Table 11.1: room temperature ≈1 day (crisp crust, dries fast); airtight container ≈1 week (toast to re-crisp, mould sooner); fridge weeks; freezer long term (thaw + toast). Very sour rye keeps ≈3 weeks (video). Par-bake to ≥95 °C core, cool, refrigerate/freeze, finish at 230 °C ≈15 min (videos KRO9EcIyXSI, WOZLvsV1M8s).

## Claims

### staling-storage

- Wrapping cooked flatbread in a towel retains moisture and softness. [breadcode-0302; SF Ch 6 Bread types; creator-rule]
- Store cooked flatbread or pancakes in an airtight bag in the fridge for weeks; if dry, spray with water and toast. [breadcode-0303; SF Ch 6 Bread types; creator-rule]
- Bread stored at room temperature (cut side down) is the most common option and is best eaten within about a day; the crust stays crisp when room humidity is not high. — `{"duration_days": [0, 1]}` [breadcode-0767; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- Higher room humidity makes the crust of room-temperature bread go soft faster. [breadcode-0768; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- Bread stored at room temperature keeps losing water from the crumb by evaporation, so it becomes hard and eventually inedible. [breadcode-0769; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- The more water a loaf contains, the longer it stays good at room temperature. [breadcode-0770; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- A low-hydration loaf dries out at room temperature after about 1–2 days. — `{"dry_out_days": [1, 2]}` [breadcode-0771; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- A high-hydration loaf needs about 3–4 days at room temperature before it dries out. — `{"dry_out_days": [3, 4]}` [breadcode-0772; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- Dried-out bread can be revived by holding it under tap water for about 10–15 seconds and baking it again in the oven. — `{"rinse_s": [10, 15]}` [breadcode-0773; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- Dried-out bread can be turned into breadcrumbs for later loaves or for dishes such as Knödel. [breadcode-0775; SF Ch 11 Storing bread §11.1 Room temperature; creator-rule]
- Paper bags and most bread boxes are not fully sealed, so some air diffuses out and the bread dries out slightly. [breadcode-0776; SF Ch 11 Storing bread §11.2 Room temperature in a container; creator-rule]
- A sealed plastic bag holds moisture in the bread so it stays good longer, but the crust loses crispness because water migrates from the bag into the crust. [breadcode-0777; SF Ch 11 Storing bread §11.2 Room temperature in a container; creator-rule]
- Toasting bread that was kept in a sealed bag restores its crust crispness. [breadcode-0778; SF Ch 11 Storing bread §11.2 Room temperature in a container; creator-rule]
- In the author's experience, bread stored in a sealed container in the fridge works well and does not dry out quickly. [breadcode-0782; SF Ch 11 Storing bread §11.3 Fridge; creator-rule]
- Some sources claim that bread dries out inside the fridge. [breadcode-0783; SF Ch 11 Storing bread §11.3 Fridge; literature] *(low confidence)*
- Fridge storage is said to draw liquid from the crumb to the surface of the bread. [breadcode-0784; SF Ch 11 Storing bread §11.3 Fridge; literature] *(low confidence)*
- At low fridge temperatures, bread in a sealable container can stay good for weeks, whereas the same sealed bread would mold at room temperature. [breadcode-0785; SF Ch 11 Storing bread §11.3 Fridge; creator-rule]
- Bread taken from the fridge loses crispness, so toasting is advised. [breadcode-0786; SF Ch 11 Storing bread §11.3 Fridge; creator-rule]
- Freezing is the option for very long-term storage: slice the whole loaf and portion the slices so that each container is eaten within a day. — `{"consume_within_days": 1}` [breadcode-0787; SF Ch 11 Storing bread §11.4 Freezing; creator-rule]
- Frozen bread slices should be thawed for a few hours to separate them, then toasted or baked until the desired crispness. [breadcode-0788; SF Ch 11 Storing bread §11.4 Freezing; creator-rule]
- Keeping a few frozen slices as an emergency reserve covers days when there is no time to bake. [breadcode-0789; SF Ch 11 Storing bread §11.4 Freezing; anecdote]
- Storing in an airtight container at room temperature keeps bread good for up to a week; it then needs toasting to re-crisp the crust and catches mold more quickly. — `{"duration_days": 7}` [breadcode-0791; SF Table 11.1; creator-rule]
- Storing bread in the fridge keeps it good for weeks; without an airtight container it can dry out a little, and it needs toasting. [breadcode-0792; SF Table 11.1; creator-rule]
- Storing bread in the freezer keeps it good for years; it needs thawing and then toasting. [breadcode-0793; SF Table 11.1; creator-rule]
- Storing bread at room temperature is the easiest option and best for bread eaten within a day; the bread dries out very quickly. — `{"duration_days": [0, 1]}` [breadcode-0794; SF Table 11.1; creator-rule]
- The author's sourdough stayed good for up to a week in a plastic bag, longer than a yeast-leavened dough that may mould after a few days. — `{"days_max": 7}` [breadcode-0994; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; anecdote] *(low confidence)*
- Acidity in sourdough helps prevent pathogens from entering the bread, which supports keeping quality. [breadcode-0995; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Crust typically becomes chewy after about a day, as residual moisture homogenises into the crumb and partly evaporates. — `{"days": 1}` [breadcode-1014; SF Ch 12 Troubleshooting §12.3 Bread > My crust becomes chewy; creator-rule]
- In humid tropical conditions the crust may stay crisp for only a few hours before turning chewy. — `{"crisp_h": "few"}` [breadcode-1015; SF Ch 12 Troubleshooting §12.3 Bread > My crust becomes chewy; anecdote] *(low confidence)*
- Storing bread in a container or plastic bag also makes the crust chewy. [breadcode-1016; SF Ch 12 Troubleshooting §12.3 Bread > My crust becomes chewy; creator-rule]
- Toasting a slice restores part of its crispness; the author stores bread in a plastic bag in the fridge to keep moisture in. [breadcode-1017; SF Ch 12 Troubleshooting §12.3 Bread > My crust becomes chewy; creator-rule]
- Reheat bread at about 150 °C in an oven with a tray of boiling water; the steam keeps the crust from thickening. — `{"reheat_c": 150}` [breadcode-1130; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- Mischbrot keeps about 2 weeks. — `{"shelf_life_weeks": 2}` [breadcode-1168; repo:the-bread-code recipes/sourdough/mischbrot-aka-graubrot.md; creator-rule] *(low confidence)*
- Dried sourdough pasta can be stored up to 1 year; the pasta is hung to dry before storage. — `{"storage_months": 12}` [breadcode-1177; repo:the-bread-code recipes/sourdough/sourdough-pasta.md; creator-rule]
- Frozen sourdough pizza dough: thaw in the fridge (about 1 day), never at room temperature. — `{"thaw_h": 24}` [breadcode-1184; repo:the-bread-code recipes/sourdough/sourdough-pizza.md; creator-rule]
- Rest bread about an hour before slicing; cool it on a rack so the bottom does not go soggy. [breadcode-1238; yt:MrOM_D3hnpc 1:52:58; creator-rule]
- Sliced bread frozen and then reheated in a toaster's defrost mode is said to taste close to fresh. [breadcode-1239; yt:MrOM_D3hnpc 2:08:31; anecdote] *(low confidence)*
- Keeping whole grain berries and milling just before use preserves the most flavour and nutrients and is the cheapest option. [breadcode-1285; yt:wdnX3baMB38 50:39; creator-rule]
- Paper bags let pests chew through; a sealed pail or jar is safer for grain storage. [breadcode-1286; yt:wdnX3baMB38 55:57; creator-rule]
- Sealing grain with an oxygen absorber in a container removes oxygen and deters pests during long-term storage. [breadcode-1287; yt:wdnX3baMB38 56:36; creator-rule]
- Freezing kills stored-grain insects; freeze bags for a few days, then let them sit on the counter a couple of days before moving to storage containers. — `{"counter_days": 2}` [breadcode-1288; yt:wdnX3baMB38 57:13; creator-rule]
- Stored grain held near -20 °F (about -29 °C) is cooled in farm bins, which also kills insects; the speaker does this every winter. — `{"cold_storage_temp_f": -20, "cold_storage_temp_c": -29}` [breadcode-1360; yt:wdnX3baMB38 57:13; anecdote] *(low confidence)*
- Store flour cool and dry, in its paper or sealed packaging, away from strong-smelling products such as cleaning agents. [breadcode-1406; yt:KooRWvS5c-A 43:58; creator-rule] *(low confidence)*
- Let the loaf rest at least 30 minutes before slicing. — `{"cool_min": 30}` [breadcode-1619; yt:0kFYyj3WKpE 25:29; creator-rule]
- A cooled loaf in an airtight bag in the fridge stays good for about a week; the sourdough's acidity slows mould, but the crumb dries out quickly. — `{"fridge_days": 7}` [breadcode-1646; yt:WOZLvsV1M8s 11:03; creator-rule]
- Sourdough's acidity slows mould growth, but a sourdough loaf dries out relatively quickly. [breadcode-1647; yt:WOZLvsV1M8s 11:03; creator-rule]
- Wait about 30 minutes before slicing a freshly baked loaf, or the crumb tears. — `{"cool_min": 30}` [breadcode-1649; yt:WOZLvsV1M8s 11:38–12:12; creator-rule]
- Wait at least an hour before cutting a loaf; a warm crumb tears. — `{"cool_min": 60}` [breadcode-1705; yt:9fWzWpL5g6w 19:42; creator-rule]
- Store the dough below 6 °C; it keeps up to 14 days; warm 3 hours before use. [breadcode-1848; yt:rcOxoJk7goM description; creator-rule]
- Par-baked rolls with core around 95 °C after 20 min can be kept at room temperature up to 7 days and in the fridge for months; he thinks they could be frozen for years ('I guess'); reheat about 10 min at 230 °C. — `{"core_c": 95, "bake_min": 20, "room_storage_d": 7, "reheat_min": 10}` [breadcode-2051; yt:OcOpIISBK6c [10:30]–[11:41]; creator-rule]
- High-hydration whole-wheat bread stores well and stays moist, a result the creator now relies on. [breadcode-2082; yt:MPWypFY0wyE [16:48]; creator-rule]
- If you plan to bake within two weeks, keep the refreshed starter in the fridge; otherwise freeze it; if there is no freezer, dry it. — `{"fridge_weeks": 2}` [breadcode-2121; yt:_P1B7PxAwk0 [9:22]; creator-rule]
- A 4-day express delivery arrived without mould. — `{"transit_d": 4}` [breadcode-2135; yt:zfxOxu3_4bE [3:42]–[4:17]; anecdote] *(low confidence)*
- An airtight bag keeps a sourdough loaf humid so it does not dry out; the creator says sourdough moulds more slowly than yeast bread, which he used to justify shipping it sealed. [breadcode-2195; yt:I3h6heaKWC8 4:51; creator-rule] *(low confidence)*
- Par-baking may extend shelf life because the higher acidity inhibits external pathogens. [breadcode-2308; yt:0v1QhtyUic4 10:47; extrapolation] *(low confidence)*
- Ground sourdough spice stored in a jar is said to stay good for about a month, possibly years. [breadcode-2431; yt:bVrSZzpVUnY 7:55; creator-rule] *(low confidence)*
- A shaped sourdough dough kept frozen for one week was baked straight from the freezer and produced a loaf. — `{"freeze_days": 7}` [breadcode-2481; yt:Typy5Uzp-EM 0:00; anecdote]
- Freezing shaped dough and baking from frozen is a practical option for home bakers, though the loaves looked less even than fresh ones. [breadcode-2488; yt:Typy5Uzp-EM 5:37; anecdote]
- A leftover stiff starter stored in the fridge keeps for several months. [breadcode-2521; yt:MqH3GVfjfBc 7:13; creator-rule]
- A very sour Bavarian rye bread keeps for about 3 weeks. — `{"shelf_life_h": 504}` [breadcode-2795; yt:VlD2YuNq1w0 1:16; creator-rule] *(low confidence)*
- The video description says sourdough dough degrades faster than poolish dough, but the spoken remark at 2:35 suggests poolish-based bread goes stale within a day or two; the two statements conflict. [breadcode-2796; yt:VlD2YuNq1w0 2:35; creator-rule] *(low confidence)*
- The creator says the more sour a bread is, the longer it keeps. [breadcode-2822; yt:VlD2YuNq1w0 0:37; creator-rule] *(low confidence)*
- To refresh a very sour rye bread that has dried out, put a bowl of water in the oven and reheat the bread next to it. [breadcode-2823; yt:VlD2YuNq1w0 1:16; creator-rule] *(low confidence)*
- To make bread taste fresh again, reheat it in the oven with steam; without a water source the crust gets crisper and the crumb drier. [breadcode-2924; yt:AR5kwlnxXMc 0:00; creator-rule]
- A rather sour sourdough keeps a long shelf life. [breadcode-2925; yt:AR5kwlnxXMc description; creator-rule] *(low confidence)*
