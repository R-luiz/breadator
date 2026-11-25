# starter — The Bread Code / The Sourdough Framework

Claims: 484 (book 217, videos 257, repos 10). Evidence: creator-rule 429, anecdote 35, measured 14, literature 3, extrapolation 3.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

- **Types (Table 4.1):** regular 100 % hydration (balanced), liquid ≈500 % (minimal yeast, high bacterial activity, dairy/yoghurt notes), stiff 50–60 % (high yeast, low bacteria, milder, more leavening). Videos give stiff as 50 % (1 : 2 water:flour, e.g. 10 g starter + 50 g flour + 25 g water) in 2021–22 and 60 % in Nov 2026 (_cb4vCQ2shk); one 2022 course says 40–50 % (msqU-ylXWUs).
- **Readiness (book §7.2 / figure):** fed within 3 days → feed 1:5:5 (seed:flour:water) 6–12 h before mixing; last fed >3 days ago → feed 1:10:10 twice (≈48 h and 6–12 h before). Check size increase and smell.
- **Dose (Table 7.1, starter mass as % of flour):** recently fed 5 / 10 / 15 % at 30 / 25 / 20 °C; starving 2.5 / 5 / 10 %; aim 8–12 h bulk. The same chapter's prose says ≈5 % at ≈25 °C in summer and 10–20 % at ≈20 °C in winter, 10 % for an 8 h bulk and 5 % for 12 h, and as low as 1 % when very hot — an internal inconsistency (prose ≈ half of the table at 25 °C). Book §7.5: most recipes use ≈20 %; he recommends 5–10 %. Very high (≈90 %) leaves no room to rebalance; ≈1 % lets protease degrade the dough before the culture catches up.
- **Videos:** 10 % baseline, 2.5–5 % when hot, up to 20 % when cold (36GdGT0XYwE 2022, _P1B7PxAwk0 2022, m7J6CBZcq8o 2022); "never more than 20 %" (_cb4vCQ2shk 2026) — while 2020–21 videos use 20 % routinely (o_xtVmR4vlc, dl7crk2Nvj8, MqH3GVfjfBc). Liquid starter ≈10 % ferments like ≈20 % regular (ES7x28d10pg, low confidence).
- **Maintenance:** fridge storage for weeks, drying and freezing as backups (book §3.4); revive dried/frozen starter with daily 1:5:5 for ≈3 days. Starting a starter takes 4–20 days depending on flour microbes; switch flour if it fails after ≈10 days.

## Claims

### starter

- Repeated feeding of a starter selects microorganisms better adapted to fermenting the flour in use, so more mature starters tend to leaven dough faster. [breadcode-0002; SF Ch 1 The history of sourdough; literature]
- Kicking off a sourdough starter at a lower temperature is predicted to select cold-tolerant microorganisms and could change the flavour of the bread; the author has not tested this. [breadcode-0087; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- A starter can be started from roughly 50 g flour and 50 g water, mixed at 100 % hydration; eyeballing the amounts is acceptable and the values are only a reference. — `{"flour_g": 50, "water_g": 50, "hydration_pct": 100}` [breadcode-0096; SF Ch 3 Making a sourdough starter; creator-rule]
- Use non-chlorinated water for a starter: chlorine is a disinfectant that stops the starter's microbes from growing. Bottled water is ideal; tap water is fine in some regions. [breadcode-0097; SF Ch 3 Making a sourdough starter; creator-rule]
- Stir until all flour is hydrated; this step revives the dormant microbial spores in the flour mixture. [breadcode-0098; SF Ch 3 Making a sourdough starter; creator-rule]
- Cover the starter loosely (not airtight, e.g. a glass with an inverted glass on top) so gas exchange is possible. [breadcode-0099; SF Ch 3 Making a sourdough starter; creator-rule]
- In one author-reported case a new starter became lively within a few days, possibly because flour microbes already in the kitchen contributed. [breadcode-0107; SF Ch 3 Making a sourdough starter; anecdote] *(low confidence)*
- Around 24 h after mixing, early fermentation signs (smell, bubbles, slight rise) normally appear; check them to judge the first 24 h. — `{"duration_h": 24}` [breadcode-0108; SF Ch 3 Making a sourdough starter; creator-rule]
- At ~24 h, keep 10 g of the mixture, discard the rest, and feed it with 50 g flour and 50 g water (1:5:5). — `{"starter_g": 10, "flour_g": 50, "water_g": 50, "ratio": "1:5:5", "interval_h": 24}` [breadcode-0109; SF Ch 3 Making a sourdough starter; creator-rule]
- The feeding ratio 1:5:5 (old starter : flour : water) is the author's default both for starters and for doughs. — `{"ratio": "1:5:5"}` [breadcode-0110; SF Ch 3 Making a sourdough starter; creator-rule]
- A dough is essentially a large starter; the author uses about 100–200 g of starter per 1000 g flour (10–20 %) for dough. — `{"starter_g": [100, 200], "flour_g": 1000, "starter_pct_of_flour": [10, 20]}` [breadcode-0111; SF Ch 3 Making a sourdough starter; creator-rule]
- After feeding, homogenise the mixture and cover it again with a glass or lid. [breadcode-0112; SF Ch 3 Making a sourdough starter; creator-rule]
- If the top of the starter dries out a lot, use a different cover; the dried parts are composted by mould and other microbes. [breadcode-0113; SF Ch 3 Making a sourdough starter; creator-rule]
- Discard the day-one leftovers: they contain activated microbes that may be harmful. [breadcode-0115; SF Ch 3 Making a sourdough starter; creator-rule]
- Start keeping discard after your first successful loaf; it is long-fermented flour usable in crackers, pancakes or sandwich bread, or dried as a pizza-dusting flour. [breadcode-0116; SF Ch 3 Making a sourdough starter; creator-rule]
- A fully depleted starter can be revived from a small piece of dough, and discard (rich in spores) mixed with a little flour and water can also restart a starter. [breadcode-0117; SF Ch 3 Making a sourdough starter; creator-rule]
- Store all discard in one fridge jar, adding new discard on top and using it as needed. [breadcode-0118; SF Ch 3 Making a sourdough starter; creator-rule]
- Between days 2 and 3 the visible signs can weaken, because a few selected microbes take over; activity continues at a microscopic level. [breadcode-0119; SF Ch 3 Making a sourdough starter; creator-rule]
- Repeat the 24-hour discard-and-feed routine until the starter shows clear activity. [breadcode-0120; SF Ch 3 Making a sourdough starter; creator-rule]
- Setting up a starter takes about 4 days for some people, 7 for others, and up to 20 for some; it depends on how well the wild microbes ferment flour. — `{"duration_h": [96, 480]}` [breadcode-0121; SF Ch 3 Making a sourdough starter; creator-rule]
- Each feeding makes the starter better adapted to flour fermentation; a mature starter received from another baker may initially be stronger, and a new starter catches up over time. [breadcode-0122; SF Ch 3 Making a sourdough starter; creator-rule]
- Judge starter readiness at least 6–12 h after feeding. — `{"wait_h": [6, 12]}` [breadcode-0123; SF Ch 3 Making a sourdough starter; creator-rule]
- Assess starter readiness with three criteria together: size increase, airy texture (bubbles and pockets), and smell. [breadcode-0124; SF Ch 3 Making a sourdough starter; creator-rule]
- The amount of size increase is not a fixed criterion; some bakers claim doubling, tripling or quadrupling, and the increase depends on the microbes and on the flour used. [breadcode-0126; SF Ch 3 Making a sourdough starter; creator-rule]
- Combining size increase, pockets and smell is the best readiness test; the flowchart calls a starter ready when it has grown, is bubbly and smells vinegary or yogurty. [breadcode-0129; SF Ch 3 Making a sourdough starter; creator-rule]
- Readiness decision for a starter last fed within 3 days: feed 1:5:5 6–12 h before making dough, then check size and smell. — `{"last_fed_within_h": 72, "ratio": "1:5:5", "wait_h": [6, 12]}` [breadcode-0130; SF Ch 3 Making a sourdough starter; creator-rule]
- Readiness decision for a starter last fed more than 3 days ago: feed twice, once 48 h before and once 6–12 h before dough, using a 1:10:10 ratio (10 g starter, 100 g flour, 100 g water) for the first feeding. — `{"last_fed_over_h": 72, "first_feed_before_h": 48, "ratio": "1:10:10", "starter_g": 10, "flour_g": 100, "water_g": 100, "wait_h": [6, 12]}` [breadcode-0131; SF Ch 3 Making a sourdough starter; creator-rule]
- If the starter is still not ready after more than 10 feedings, discard it all and start over. — `{"max_feedings": 10}` [breadcode-0132; SF Ch 3 Making a sourdough starter; creator-rule]
- The float test (a piece of starter floats in water when full of gas) is an alternative readiness check; the author does not recommend it, since rye starter often does not float even when active. [breadcode-0133; SF Ch 3 Making a sourdough starter; creator-rule]
- Once the starter is ready, give it one last feeding, then make the dough the same evening or the next day. [breadcode-0135; SF Ch 3 Making a sourdough starter; creator-rule]
- Maintenance depends on the next bake date; the flowchart assumes a 100 % hydration starter and water must be adjusted for stiff starters. — `{"hydration_pct": 100}` [breadcode-0137; SF Ch 3 Making a sourdough starter; creator-rule]
- Maintenance option: if you bake again the next day, take 10 g of your bread dough (if all starter was used) or all but at most 10 g of starter, then feed 1:5:5. — `{"starter_max_g": 10, "ratio": "1:5:5"}` [breadcode-0138; SF Ch 3 Making a sourdough starter; creator-rule]
- A piece of bread dough can serve as the next starter; to make a specific amount, prepare that amount of starter in advance. [breadcode-0139; SF Ch 3 Making a sourdough starter; creator-rule]
- Starter microbes are resilient and block pathogens; a purchased starter keeps its original microbes, which are expected to remain dominant. [breadcode-0140; SF Ch 3 Making a sourdough starter; creator-rule]
- As long as the starter has food, it outcompetes other microbes; when food runs out, the microbes form spores that can wait for new food. [breadcode-0143; SF Ch 3 Making a sourdough starter; creator-rule]
- A starter is almost impossible to kill; the author has seen very few starters die. [breadcode-0146; SF Ch 3 Making a sourdough starter; anecdote]
- After long hibernation the yeast/bacteria balance shifts toward bacteria, and a long dormancy may leave only spores; a few feedings are needed to restore the balance. [breadcode-0147; SF Ch 3 Making a sourdough starter; creator-rule]
- Baking next day: feed leftover starter at 1:5:5; if it is hot, or the bake is ~24 h after the last feeding, use 1:10:10 instead. — `{"ratio": "1:5:5", "ratio_hot_or_24h": "1:10:10", "interval_h": 24}` [breadcode-0148; SF Ch 3 Making a sourdough starter; creator-rule]
- If little starter is left, a 1:50:50 or 1:100:100 ratio can be used; the more new flour, the longer the starter takes to be ready again. — `{"ratio": "1:50:50 or 1:100:100"}` [breadcode-0149; SF Ch 3 Making a sourdough starter; creator-rule]
- After a next-day feeding at 1:5:5, make the bread dough again after 8–12 h. — `{"wait_h": [8, 12], "ratio": "1:5:5"}` [breadcode-0150; SF Ch 3 Making a sourdough starter; creator-rule]
- If you are baking within about 2 weeks, store the leftover starter in the fridge at 4 °C (40 °F). — `{"temp_c": 4, "bake_within_weeks": 2}` [breadcode-0151; SF Ch 3 Making a sourdough starter; creator-rule]
- Before the next bake, feed the fridge starter with 1:5:5 about 8–12 h before making the dough. — `{"ratio": "1:5:5", "wait_h": [8, 12]}` [breadcode-0152; SF Ch 3 Making a sourdough starter; creator-rule]
- Store fridge starter under a little water to protect it from drying; drained liquid can be used or stirred into dough. [breadcode-0153; SF Ch 3 Making a sourdough starter; creator-rule]
- Colder storage preserves a good yeast/bacteria balance longer; warmer storage speeds fermentation and colder storage slows it. — `{"temp_c": 4}` [breadcode-0155; SF Ch 3 Making a sourdough starter; creator-rule]
- Below 4 °C starter fermentation almost completely stops; the speed at low temperature depends on the wild yeast and bacteria strains. — `{"temp_c": 4}` [breadcode-0156; SF Ch 3 Making a sourdough starter; creator-rule]
- To dry a starter for months: mix it with flour, crumble it, and keep adding flour until no moisture is left; then dry in a dry place. [breadcode-0157; SF Ch 3 Making a sourdough starter; creator-rule]
- A dehydrator at about 30 °C for 12–20 h can speed up drying of the starter-flour mix. — `{"temp_c": 30, "duration_h": [12, 20]}` [breadcode-0158; SF Ch 3 Making a sourdough starter; creator-rule]
- Drying continues over about 2 more days, adding flour each day, until no moisture remains; residual moisture can lead to mould. — `{"duration_h": 48}` [breadcode-0159; SF Ch 3 Making a sourdough starter; creator-rule]
- Store the fully dried starter in an airtight container or freeze it; silica bags can be added to absorb moisture. [breadcode-0160; SF Ch 3 Making a sourdough starter; creator-rule]
- Dried starter stays viable for years. [breadcode-0161; SF Ch 3 Making a sourdough starter; creator-rule]
- If you have a freezer, freeze the starter; to reactivate a dried or frozen starter, feed it daily at 1:5:5 for 3 days. — `{"ratio": "1:5:5", "reactivation_h": 72}` [breadcode-0162; SF Ch 3 Making a sourdough starter; creator-rule]
- If you are not baking within about 2 weeks and have no freezer, dry the starter. [breadcode-0163; SF Ch 3 Making a sourdough starter; creator-rule]
- After drying, the author's starter initially took about 3 days to become active again, but later sometimes needed only a single feeding as the surviving microbes adapt. — `{"reactivation_h": 72}` [breadcode-0164; SF Ch 3 Making a sourdough starter; anecdote] *(low confidence)*
- Daily feeding is not needed for a mature starter; feed daily only while the starter is still immature, when each feeding improves its flour fermentation. [breadcode-0165; SF Ch 3 Making a sourdough starter; creator-rule]
- The starter is the most important input: its yeast-to-bacteria balance is passed to the main dough, so an off starter makes the whole dough ferment badly. [breadcode-0354; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A starter last fed within 3 days should be fed 6-12 h before making dough, using the 1:5:5 ratio. — `{"last_fed_max_days": 3, "feed_before_h": [6, 12], "ratio": "1:5:5"}` [breadcode-0357; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A starter not fed within 3 days is fed twice: once 48 h before and once 6-12 h before making dough, using the 1:10:10 ratio. — `{"feed_1_before_h": 48, "feed_2_before_h": [6, 12], "ratio": "1:10:10"}` [breadcode-0358; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Feeding ratio for a recently fed starter in the chart: 10 g starter : 50 g flour : 50 g water (1:5:5). — `{"ratio": "1:5:5", "starter_g": 10, "flour_g": 50, "water_g": 50}` [breadcode-0359; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Feeding ratio for an older starter in the chart: 10 g starter : 100 g flour : 100 g water (1:10:10). — `{"ratio": "1:10:10", "starter_g": 10, "flour_g": 100, "water_g": 100}` [breadcode-0360; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A starter is ready for dough when it is bubbly or has risen in size and smells vinegary or yogurty; otherwise feed it again. [breadcode-0361; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- The older a starter gets, the more acid-resistant its yeast becomes. [breadcode-0363; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A young starter may need 2-3 feedings to rebalance; a mature starter may need only one. — `{"feedings": [1, 3]}` [breadcode-0364; SF Ch 7 Wheat sourdough §7.2 Readying your starter; anecdote]
- A 1:1:1 refresh (old starter : flour : water) is not recommended, because a starter is a miniature dough and dough is never mixed at 1:1:1. — `{"ratio": "1:1:1"}` [breadcode-0365; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- For a stiff starter, the author's refresh ratio is 1:5:2.5 (starter : flour : water), i.e. water at half the flour weight. — `{"ratio": "1:5:2.5"}` [breadcode-0367; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- In warm conditions, refresh a starter at 1:10:5 or 1:20:10 to slow its ripening. — `{"ratio": ["1:10:5", "1:20:10"]}` [breadcode-0368; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Increasing flour and water in the feed delays starter readiness, which helps fit a feeding schedule. [breadcode-0369; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Making dough directly from dry starter was too slow to ferment in the author's attempts. [breadcode-0376; SF Ch 7 Wheat sourdough §7.2 Readying your starter; anecdote]
- Feeding the starter (option 1) is recommended over reducing the starter dose (option 2) for reliably better bread. [breadcode-0378; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- The float test checks starter readiness: a starter that floats on water has reached its peak and is ready to leaven dough, because carbon dioxide gives it buoyancy. [breadcode-0840; SF Glossary > Float test; creator-rule]
- A starter that sinks in the float test is not yet ready, because fermentation has not progressed enough for good leavening power. [breadcode-0841; SF Glossary > Float test; creator-rule]
- The float test is reliable for wheat starters but not for non-wheat starters, because gas escapes more readily from non-wheat doughs and they float less. [breadcode-0842; SF Glossary > Float test; creator-rule]
- For non-wheat starters, judge readiness by bubbles in the jar and aroma; a mature starter smells mildly sour, not overly pungent. [breadcode-0843; SF Glossary > Float test; creator-rule]
- Hooch, a liquid layer on a sourdough starter, indicates the starter is hungry and needs feeding; it acts as a barrier against mold and can be stirred back in or used for hot sauces. [breadcode-0848; SF Glossary > Hooch; creator-rule]
- A young starter that is not yet good at fermenting flour is a common cause of flat bread; starter performance improves with repeated feeding. [breadcode-0973; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; creator-rule]
- Safe fermentation length depends on the starter's yeast/bacteria balance: more bacterial activity breaks down dough structure faster. [breadcode-1040; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- Building a starter takes about 7 days with about 2 min of work per day. — `{"build_days": 7, "min_per_day": 2}` [breadcode-1142; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- Day 1 of a starter: 100 g flour and 100 g water (100 % hydration), water between 20 and 30 °C. — `{"flour_g": 100, "water_g": 100, "hydration_pct": 100, "water_temp_c": [20, 30]}` [breadcode-1143; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- Daily feeding of a starter: 50 g starter, 50 g flour, 50 g water (1:1:1 by weight). — `{"starter_g": 50, "flour_g": 50, "water_g": 50}` [breadcode-1144; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- A starter is ready when it doubles in about 6 h at about 22 °C. — `{"doubling_h": 6, "temp_c": 22}` [breadcode-1145; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- A starter kept in the fridge should be taken out about 1 day before use and fed twice to reactivate. — `{"out_of_fridge_days": 1, "reactivation_feeds": 2}` [breadcode-1146; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- Reset a neglected starter: 1 g starter + 50 g flour + 50 g water, then wait about 48 h. — `{"starter_g": 1, "flour_g": 50, "water_g": 50, "time_h": 48}` [breadcode-1147; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- A vinegary smell means the starter is hungry; it is fixed by feeding. [breadcode-1148; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- Use about 20 % starter for a shorter ferment and about 10 % for an overnight ferment. — `{"starter_pct_short": 20, "starter_pct_overnight": 10}` [breadcode-1149; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- Basic sourdough uses 100 g starter (20 % of flour) and 10 g salt (2 %) per 500 g flour. — `{"starter_g": 100, "starter_pct": 20, "salt_pct": 2}` [breadcode-1160; repo:the-bread-code recipes/sourdough/basic-sour-dough.md; creator-rule]
- Mischbrot intro says up to 40 % starter (20 % active plus 20 % discard) can be used. — `{"starter_pct_max": 40, "starter_active_pct": 20, "starter_discard_pct": 20}` [breadcode-1169; repo:the-bread-code recipes/sourdough/mischbrot-aka-graubrot.md; creator-rule] *(low confidence)*
- Starter quantity is typically about 10 % of flour; in cold weather a higher amount is sometimes used to speed up, but starter must be checked for readiness before use. [breadcode-1232; yt:MrOM_D3hnpc 1:37:07; anecdote]
- A stiff starter (about 50 % hydration, stored in the fridge and activated the night before) gives better rise. [breadcode-1241; yt:MrOM_D3hnpc 2:02:56; creator-rule]
- A starter should be kept active with regular feedings for a few days before baking with it. [breadcode-1249; yt:ImVvQMvGZKE 13:17; creator-rule]
- Before baking, feed the starter daily for 2–3 days at room temperature at a 1:5:5 ratio to make it very active; the evening-before 1:5:5 feed also keeps starting acidity low. — `{"ratio": "1:5:5", "days_before_bake": [2, 3]}` [breadcode-1259; yt:ImVvQMvGZKE 13:17; creator-rule]
- A stiffer starter boosts yeast activity and counters the effect of low-protein flour. [breadcode-1275; yt:MrOM_D3hnpc 24:35; creator-rule]
- Starter hydration types: stiff (lievito madre), regular at about 100 %, liquid at about 500 %; a more hydrated starter is more active. — `{"regular_hydration_pct": 100, "liquid_hydration_pct": 500}` [breadcode-1330; yt:-60DxtcJ9Fo 8:55; creator-rule]
- Whole rye is his preferred flour for a new starter; whole wheat or whole spelt also work. [breadcode-1402; yt:ZTHHI0iuamA 1:04; creator-rule]
- Mix 50 g flour with 50 g water to start; each later day feed equal parts flour and water. — `{"flour_g": 50, "water_g": 50}` [breadcode-1403; yt:ZTHHI0iuamA description; creator-rule]
- Discard starter can be stored in the fridge and used later. [breadcode-1404; yt:ZTHHI0iuamA description; creator-rule]
- A healthy, active starter shows steady bubbling and roughly doubles or triples in volume after feeding; this activity predicts the bake better than the starter's age. — `{"rise_ratio": [2, 3]}` [breadcode-1433; yt:WxFE3bxrTSM 1:16; creator-rule]
- A starter moved to a new place adapts to the local microbes within about a day, so where it is kept matters more than how old it is. — `{"adaptation_time_h": 24}` [breadcode-1434; yt:WxFE3bxrTSM 3:46; anecdote] *(low confidence)*
- For a new starter, he recommends one more day of feeding than seems needed, because a healthier, more active starter pays off long term. [breadcode-1437; yt:ZTHHI0iuamA 4:29; creator-rule]
- A 1:5:5 feed (10 g starter, 50 g water, 50 g flour) is the ratio he uses to keep starter acidity moderate. — `{"ratio": "1:5:5", "starter_g": 10, "water_g": 50, "flour_g": 50}` [breadcode-1461; yt:qM9BXKaIr60 1:44; creator-rule]
- For an overnight bulk, use 5–10 % starter by flour weight. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-1467; yt:qM9BXKaIr60 6:12; creator-rule]
- A stiff starter is built from 10 g of regular starter, 50 g flour and 25 g water (50 % hydration), refreshed daily for 2–4 days; it has a milder aroma and more leavening. — `{"starter_hydration_pct": 50, "build_flour_g": 50, "build_water_g": 25, "build_days": [2, 4]}` [breadcode-1489; yt:HSdOUZq5i6U 3:35; 4:13; creator-rule]
- A new starter can take about 7 days (sometimes up to 15) to mature; it is ready when it doubles within about 5–10 h of feeding at room temperature. — `{"days": [7, 15], "peak_h": [5, 10]}` [breadcode-1490; yt:HSdOUZq5i6U 2:21; 3:35; creator-rule]
- For an overnight bulk a small starter dose is enough: about 5 % of flour weight when warm, up to 10 % when cold. — `{"starter_pct_of_flour": [5, 10], "bulk_h": 12}` [breadcode-1504; yt:HLPNdyGCSPk 3:52; 4:26; creator-rule]
- If the starter is struggling to ferment, do a couple of 1:5:5 feeds (1 part starter, 5 parts flour, 5 parts water) before use. — `{"ratio": "1:5:5"}` [breadcode-1515; yt:HLPNdyGCSPk 3:20; creator-rule]
- A starter fed 1:5:5 the previous evening was almost ready about 8 hours later. — `{"ratio": "1:5:5", "ready_h": 8}` [breadcode-1547; yt:wHL44ONu3so 2:30; creator-rule]
- A 10 % starter dose is the baseline; use about 2.5–5 % in warm conditions and up to 20 % in cold conditions. — `{"starter_pct_of_flour": [2.5, 20], "baseline_pct": 10}` [breadcode-1551; yt:36GdGT0XYwE 3:03; creator-rule]
- Feed a starter with whole-grain flour (rye, spelt, wheat, emmer or einkorn) rather than bread or all-purpose flour, because the nutrients and microbes it needs sit mostly in the grain hull. [breadcode-1564; yt:yYkTrGHNW2w 6:23; creator-rule]
- Switching between whole-grain flours from feed to feed (for example rye, then wheat, then spelt) is acceptable and adds microbial variety. [breadcode-1565; yt:yYkTrGHNW2w 6:58; creator-rule] *(low confidence)*
- At about 4 °C fermentation comes completely to a halt, so a starter can be stored in the fridge for about a month without issues before it needs reactivating. — `{"fridge_c": 4, "fridge_storage_weeks": 4}` [breadcode-1566; yt:yYkTrGHNW2w 8:05; creator-rule]
- Reactivate a fridge starter with a 1:10:10 feed (starter : flour : water) in a clean, marked jar. — `{"ratio": "1:10:10"}` [breadcode-1567; yt:yYkTrGHNW2w 9:12; creator-rule]
- Bringing a fridge starter back to normal activity can take about two days of feeding. — `{"recovery_days": 2}` [breadcode-1568; yt:yYkTrGHNW2w 10:19; creator-rule]
- A maintenance feed of 1:5:5 keeps the starter's acidity low. — `{"ratio": "1:5:5"}` [breadcode-1569; yt:yYkTrGHNW2w 10:51; creator-rule]
- A 1:5:5 fed starter fermented overnight for about 10 hours and showed bubbles and a dome, and was not very sour when tasted. — `{"ratio": "1:5:5", "ferment_h": 10}` [breadcode-1570; yt:yYkTrGHNW2w 13:11; creator-rule]
- In warm conditions, or when the starter is too acidic, feed at 1:10:10 or 1:20:20 instead of 1:5:5. — `{"ratio": "1:10:10 or 1:20:20"}` [breadcode-1571; yt:yYkTrGHNW2w 13:45; creator-rule]
- Regularly empty the starter jar completely, rinsing the edges, so little acid remains; the starter regrows and behaves like a 1:50 or 1:100:100 feed. — `{"ratio": "1:50:50 to 1:100:100"}` [breadcode-1573; yt:yYkTrGHNW2w 14:52; creator-rule]
- A liquid starter moves starch and enzymes faster, so it acidifies faster and favours bacteria. [breadcode-1599; yt:nQWlG1buMM8 15:51; creator-rule]
- The host built a very high-hydration starter (his estimate about 500 %) without refreshing and had a usable starter in 3–4 days; Karl says the flour determines the outcome. — `{"starter_hydration_pct": 500, "build_days": [3, 4]}` [breadcode-1600; yt:nQWlG1buMM8 16:24; anecdote] *(low confidence)*
- After a long fridge stay, feed a stiff starter three times, 4–6 hours apart, and keep each fermentation under about 8 hours to favour yeast. — `{"feeds": 3, "feed_interval_h": [4, 6], "max_ferment_h": 8}` [breadcode-1601; yt:nQWlG1buMM8 17:32; creator-rule]
- A starter can be stored submerged in water to dilute acid, but should not go longer than about one week without feeding. — `{"max_days_without_feed": 7}` [breadcode-1602; yt:nQWlG1buMM8 18:39; creator-rule] *(low confidence)*
- For a beginner, one feeding per day keeps a sourdough starter active; the older and more fed a starter is, the better it ferments flour. — `{"feedings_per_day": 1}` [breadcode-1654; yt:36GdGT0XYwE 0:36; creator-rule]
- A starter that has become too sour (too much acid) weakens the gluten network and makes strong dough hard to build; adjust its feeding schedule so it is less acidic. [breadcode-1664; yt:wHL44ONu3so 0:35; 2:30; creator-rule]
- A standard 1:5:5 maintenance feed is 10 g starter, 50 g water and 50 g whole-grain flour. — `{"ratio": "1:5:5", "starter_g": 10, "water_g": 50, "flour_g": 50}` [breadcode-1676; yt:yYkTrGHNW2w 10:51; creator-rule]
- A fridge slows a starter but does not stop it; a starter left cold eventually collapses. [breadcode-1685; yt:l_PLhPWE-8E 11:52; creator-rule]
- A colleague baking 100 % whole-wheat breads reported a more open crumb from a stiff levain than from a liquid levain; no pH data. [breadcode-1687; yt:l_PLhPWE-8E 15:35–16:10; anecdote] *(low confidence)*
- A starter made from whole-grain flour starts faster than one made from white flour. [breadcode-1691; yt:9fWzWpL5g6w 0:36; creator-rule]
- An older, established starter ferments flour faster and more reliably than a young one. [breadcode-1692; yt:9fWzWpL5g6w 1:11; creator-rule]
- Before using a starter from the fridge, give it at least one extra feeding; an active starter shows bubbles at the container edge and smells good. [breadcode-1693; yt:9fWzWpL5g6w 2:26; creator-rule]
- A starter at about 60 % hydration (matching the dough) is stiffer and yeast-favouring; a liquid starter is risky because it over-ferments. — `{"hydration_pct": 60}` [breadcode-1695; yt:9fWzWpL5g6w 6:05; creator-rule]
- Keep about 50 g of dough as the next starter; refrigerated it stays usable about a week. — `{"starter_g": 50, "fridge_days": 7}` [breadcode-1700; yt:9fWzWpL5g6w 14:46; creator-rule]
- Salt in a backup starter is recommended. [breadcode-1701; yt:9fWzWpL5g6w 15:25; creator-rule] *(low confidence)*
- A starter fed 1:5:5 that doubles in about 5 h is active enough for the dough. — `{"ratio": "1:5:5", "starter_double_h": 5}` [breadcode-1709; yt:2DFvfRIywC8 4:55; creator-rule]
- A liquid starter (about 500 % hydration) is cut off from oxygen, which favours lactic acid bacteria over acetic acid, giving milder dairy notes. — `{"hydration_pct": 500}` [breadcode-1718; yt:lNCHzD8HxBA 2:19; creator-rule]
- A liquid starter can be built with 10 g starter, 50 g flour and 250 g water, repeated daily for 4–5 days. — `{"starter_g": 10, "flour_g": 50, "water_g": 250, "days": [4, 5]}` [breadcode-1719; yt:lNCHzD8HxBA 3:31; creator-rule]
- A dough made with stiff-starter inflated more than one made with a regular starter; starter properties carry over to the dough. [breadcode-1723; yt:lNCHzD8HxBA 10:43; measured]
- A starter past peak can still be used at a smaller inoculum of about 5–10 % of flour, since the balance re-establishes in the dough. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-1724; yt:lNCHzD8HxBA 12:32-13:06; creator-rule]
- A stiff sourdough starter could replace dry yeast in yeast recipes such as ciabatta or baguette. [breadcode-1726; yt:lNCHzD8HxBA 17:29; extrapolation] *(low confidence)*
- Choose starter by bread style: stiff for ciabatta or croissant, regular 100 % for rye. [breadcode-1727; yt:lNCHzD8HxBA 19:16; creator-rule]
- A too-sour starter gives flat bread, sticky dough, makes pre-shaping hard, and an overly sour loaf; reset it at least weekly. — `{"reset_interval_h": 168}` [breadcode-1745; yt:AKJxiROZasU description; creator-rule]
- Reset: feed 50 g starter with 25 g flour and 25 g water; expect about 50 g of starter after about 24 h, and about 6 h to peak at 1:1:1 (50 g starter + 50 g flour + 50 g water). — `{"ratio": "1:1:1", "peak_h": 6, "recover_h": 24}` [breadcode-1747; yt:AKJxiROZasU 2:29; creator-rule]
- Feeding the starter 1:5:5 for the dough means 10 g starter, 50 g whole-wheat flour and 50 g water. — `{"ratio": "1:5:5", "starter_g": 10, "flour_g": 50, "water_g": 50}` [breadcode-1749; yt:2DFvfRIywC8 description (instructions step; creator-rule]
- The starter was added at 20 % of the base flour (100 g on 500 g), with salt at 2 % (10 g). — `{"starter_pct_of_flour": 20, "starter_g": 100, "salt_pct_of_flour": 2, "salt_g": 10}` [breadcode-1750; yt:2DFvfRIywC8 description (recipe list); tra; creator-rule]
- A starter taken from the fridge should get one more feeding before use; a beginner should feed it, while an experienced baker may use it straight from the fridge. [breadcode-1757; yt:9fWzWpL5g6w 15:25; creator-rule]
- A small amount of starter is enough to re-establish a starter in a new environment; about half a teaspoon is given as sufficient. — `{"starter_tsp": 0.5}` [breadcode-1770; yt:AKJxiROZasU description; 3:45; creator-rule]
- For wheat breads that need strong oven spring and an open crumb, the starter must be active; a slightly sour, less active starter is acceptable only for other breads such as rye. [breadcode-1771; yt:AKJxiROZasU 1:14; 1:50; creator-rule]
- Feed the starter 1:5:5 by weight (10 g starter, 50 g flour, 50 g water) to prepare it for the main dough. — `{"starter_g": 10, "flour_g": 50, "water_g": 50, "ratio": "1:5:5"}` [breadcode-1772; yt:dl7crk2Nvj8 2:52; creator-rule]
- Starter feeding schedule before the main dough: 1:5:5 feedings on Wednesday morning and evening and Thursday morning and evening, plus one overnight feeding Thursday night; main dough is mixed Friday morning. — `{"feedings_count": 5, "feeding_ratio": "1:5:5"}` [breadcode-1773; yt:dl7crk2Nvj8 3:24; creator-rule]
- Main dough uses 20 % starter by flour weight (100 g starter for 500 g flour). — `{"starter_pct_of_flour": 20, "starter_g": 100, "flour_g": 500}` [breadcode-1775; yt:dl7crk2Nvj8 6:47; creator-rule]
- A starter that is too acidic leads to over-fermentation, flat bread and a loaf that collapses; a healthy starter should be active but not too sour before use. [breadcode-1788; yt:dl7crk2Nvj8 2:19; creator-rule]
- A starter with pH below about 4 suggests less starter in the dough; the creator aimed for about 10% starter. [breadcode-1811; yt:dQEebR6pIng 4:26; creator-rule]
- A liquid starter at 1 part flour to 2 parts water matured faster than the conventional 1:1 starter in the creator's test. — `{"ratio_flour_to_water_liquid": "1:2", "ratio_flour_to_water_conventional": "1:1"}` [breadcode-1816; yt:rOu-phxyQbQ 1:45; measured] *(low confidence)*
- In the creator's test, the liquid starter reached pH 3.85 on the morning of about day 2 from start, with no feeding. — `{"ph": 3.85, "days_from_start": 2}` [breadcode-1817; yt:rOu-phxyQbQ 10:23; anecdote] *(low confidence)*
- Stirring a starter adds oxygen, which lactic bacteria need. [breadcode-1818; yt:rOu-phxyQbQ 4:38; creator-rule]
- Smell (like apple vinegar) and bubbles are used alongside pH to judge starter activity; the starter is judged ready at about pH 3.7 for his method. [breadcode-1819; yt:rOu-phxyQbQ 6:58; creator-rule] *(low confidence)*
- Feed the starter at least once before baking. Refrigerated starters need at least one feeding after removal. [breadcode-1820; yt:rOu-phxyQbQ 15:31; creator-rule]
- The creator's feeding mix was about 300% hydration with a stirred mix of bread flour and whole wheat. [breadcode-1821; yt:rOu-phxyQbQ 16:03; creator-rule] *(low confidence)*
- Starter acidity can damage high-hydration dough; check the starter before use. [breadcode-1837; yt:BnvhhxQZQ14 4:30; creator-rule]
- A liquid starter that has already fermented a long time has broken down a lot, so use less of it in the dough; using too much can harm the dough. [breadcode-1877; yt:rOu-phxyQbQ 12:40-13:12; creator-rule]
- A starter fed 1:5:5 is ready in the morning; when warm, feed 1:10:10. — `{"ratio": "1:5:5"}` [breadcode-1979; yt:lJ38fY3ZEQk 2:21; creator-rule]
- A starter is ready when it shows bubbles, has risen a little, and smells slightly dairy or vinegary. [breadcode-1983; yt:m7J6CBZcq8o 1:05; creator-rule]
- A 500 % hydration (liquid) starter gives a milder, more dairy-like flavour, while a stiff starter favours yeast activity and fluffier bread. — `{"hydration_pct": 500}` [breadcode-1985; yt:m7J6CBZcq8o 3:18; anecdote] *(low confidence)*
- A stiff starter can make a weaker flour work and gives fluffier bread, after a few days of adaptation. [breadcode-1986; yt:m7J6CBZcq8o 7:11; creator-rule]
- For a main fermentation of about 10 h, use about 20 % starter in winter and about 10 % in summer; in very hot places 5 % or 2.5 %. — `{"starter_pct_of_flour": [2.5, 20], "bulk_h": 10}` [breadcode-1987; yt:m7J6CBZcq8o 9:23; creator-rule]
- Discard stored in the fridge keeps a long time and can be used as needed. [breadcode-1988; yt:m7J6CBZcq8o 9:58; anecdote] *(low confidence)*
- A dehydrated starter reactivates with a few feedings. [breadcode-1989; yt:m7J6CBZcq8o 8:17; creator-rule] *(low confidence)*
- Kefir- or kombucha-started cultures may not ferment flour well; a flour-made starter is usually better at that job, though its microbes adapt over time. [breadcode-2025; yt:m7J6CBZcq8o 2:13; anecdote] *(low confidence)*
- A starter that has been in the fridge for a long time may have an off bacteria-to-yeast ratio, so it can take longer to reactivate; an extra feeding before use helps. [breadcode-2026; yt:m7J6CBZcq8o 1:05; creator-rule]
- In winter, feeding with warmer water speeds up a starter's readiness, though the water cools quickly. [breadcode-2027; yt:m7J6CBZcq8o 2:45; creator-rule]
- A starter in summer typically works about twice as fast as in winter, so winter baking uses a larger starter dose. — `{"speed_ratio_summer_vs_winter": 2}` [breadcode-2028; yt:m7J6CBZcq8o 3:18; creator-rule]
- If a new starter does not rise after being added to the dough, keep feeding it for several more days until it shows activity; a cold climate slows it further. [breadcode-2029; yt:m7J6CBZcq8o 4:58; creator-rule]
- A whole-meal flour is a good choice when starting a starter, since the bran carries more wild yeast and bacteria. [breadcode-2031; yt:m7J6CBZcq8o 6:39; creator-rule]
- The float test is unreliable: a low-gluten flour may not let a ripe starter float, so it is a rough tool only. [breadcode-2032; yt:m7J6CBZcq8o 6:39; creator-rule]
- Roll dough uses 10 % starter and 2 % salt on flour weight (40 g starter and 8 g salt per 400 g flour). — `{"starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-2042; yt:OcOpIISBK6c description; [2:26]; creator-rule]
- Regular, liquid and lievito madre starters all work for overnight schedules; regular is more sour, while liquid and lievito madre give more dairy-like notes. [breadcode-2053; yt:0aylYlvA4K8 [1:05]–[1:39]; creator-rule]
- Overnight recipes use 5–10 % starter (20–40 g per 400 g flour) versus 20 % (80 g) for same-day recipes. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-2054; yt:0aylYlvA4K8 [2:13]; creator-rule]
- Above about 25 °C (77 °F), use about 1–5 % starter (he says as low as 1 % when very hot). — `{"starter_pct_of_flour": [1, 5], "temp_above_c": 25}` [breadcode-2056; yt:0aylYlvA4K8 [2:47]; [3:22]; creator-rule]
- Below 25 °C (77 °F), a regular starter is dosed at about 10 % of flour weight; a liquid starter needs less because it adds water. — `{"starter_pct_of_flour": [10], "temp_below_c": 25}` [breadcode-2057; yt:0aylYlvA4K8 [3:22]; creator-rule]
- A starter kept unfed in the fridge is dosed at about 5 % (regular) or 2.5–5 % (liquid); in hot conditions 1–2.5 %. — `{"starter_pct_of_flour": [2.5, 5]}` [breadcode-2058; yt:0aylYlvA4K8 [4:28]–[5:03]; creator-rule]
- If oven spring is lacking, feed the starter a couple of times before use to rebalance yeast and bacteria. [breadcode-2059; yt:0aylYlvA4K8 [5:03]; creator-rule]
- The float test is unreliable; judge ripeness by smell — one day old is fine, two days is very ripe. — `{"age_d": [1, 2]}` [breadcode-2060; yt:0aylYlvA4K8 [5:35]; creator-rule]
- The starter is added to the dough directly, without a separate refresh first. — `{"starter_g": 20}` [breadcode-2077; yt:MPWypFY0wyE [1:11]; creator-rule]
- Starter at about 20 % of flour is used for this sourdough (ASR-level number). — `{"starter_pct_of_flour": 20}` [breadcode-2083; yt:gOxCUh_kyMQ [1:30]–[3:56]; creator-rule] *(low confidence)*
- A stiff starter is kept at about 60 % hydration (60 g water per 100 g flour). — `{"hydration_pct": 60}` [breadcode-2092; yt:_cb4vCQ2shk [1:12]–[1:53]; creator-rule]
- A fridge starter needs feeding once or twice before use. — `{"feeds": [1, 2]}` [breadcode-2093; yt:_cb4vCQ2shk [2:20]; creator-rule]
- A ready starter smells of vinegar or yogurt and rises. [breadcode-2094; yt:_cb4vCQ2shk [1:12]; creator-rule]
- A starter dose of about 10 % of flour (10 g per 100 g flour) is used. — `{"starter_pct_of_flour": 10}` [breadcode-2095; yt:_cb4vCQ2shk [3:02]–[4:14]; creator-rule]
- In very hot conditions use 2.5–5 % starter; ideal fermentation is 8–12 h. — `{"starter_pct_of_flour": [2.5, 5], "bulk_h": [8, 12]}` [breadcode-2099; yt:_cb4vCQ2shk [8:28]; creator-rule]
- Never use more than 20 % starter. — `{"starter_pct_of_flour": 20}` [breadcode-2100; yt:_cb4vCQ2shk [8:28]; creator-rule]
- A rye starter is fed 1:5:5 (starter:flour:water). — `{"feed_ratio": "1:5:5"}` [breadcode-2106; yt:4VJQqOtwP_E [1:06]–[1:40]; creator-rule]
- A starter is built from 50 g flour and 50 g water daily and is ready after 5–10 days. — `{"flour_g": 50, "water_g": 50, "ready_d": [5, 10]}` [breadcode-2114; yt:_P1B7PxAwk0 [3:17]-[4:27]; creator-rule]
- About 12 h after feeding, check the starter for bubbles and rise; it is ready when it smells mild and yogurt- or vinegar-like, not strong. — `{"check_after_feed_h": 12}` [breadcode-2115; yt:_P1B7PxAwk0 [4:27]-[5:24]; creator-rule]
- A stiff starter is 50 g flour and 25 g water (50 % hydration), refreshed daily with 10 g starter for about 3 days. — `{"hydration_pct": 50, "flour_g": 50, "water_g": 25, "days": 3}` [breadcode-2116; yt:_P1B7PxAwk0 [6:37]; creator-rule]
- Starter dose: 10 % above 22 °C, 20 % in a cold kitchen, 5 % when very hot. — `{"starter_pct_of_flour": [5, 20], "temp_above_c": 22}` [breadcode-2119; yt:_P1B7PxAwk0 [8:18]; creator-rule]
- Maintenance: take at most 10 g and feed it 50 g flour and 25 g water. — `{"feed_flour_g": 50, "feed_water_g": 25, "take_max_g": 10}` [breadcode-2120; yt:_P1B7PxAwk0 [8:50]; creator-rule]
- An unfed or fridge starter can be used directly; a very small dose of ripe starter is enough because its microorganisms multiply in the dough. [breadcode-2139; yt:0aylYlvA4K8 [3:56]–[4:28]; creator-rule]
- A starter past peak can still be used for a dough: a small dose (20 g) was mixed directly, because a larger dose would make the bread too sour. — `{"starter_g": 20}` [breadcode-2147; yt:MPWypFY0wyE [1:11]; creator-rule]
- A liquid starter adds extra water to the dough, so the water amount must be watched when it is used. [breadcode-2152; yt:OcOpIISBK6c [2:26]; creator-rule]
- A new starter takes about 7 days or more to mature; feed it daily while it is young. — `{"maturity_days": 7}` [breadcode-2154; yt:_cb4vCQ2shk [0:35]; creator-rule]
- Whole-grain flour (whole wheat or whole rye) gives a faster starter start than refined flour. [breadcode-2156; yt:_cb4vCQ2shk [3:39]; creator-rule] *(low confidence)*
- If the next bread is baked the following day, make the dough 8-12 h after feeding the maintained starter. — `{"feed_to_mix_h": [8, 12]}` [breadcode-2157; yt:_P1B7PxAwk0 [8:50]-[9:22]; creator-rule]
- If a new starter is not bubbly or has not risen after 12 h, wait another 12 h and feed again; if it smells too strong, keep feeding every 12 h. — `{"refeed_interval_h": 12}` [breadcode-2158; yt:_P1B7PxAwk0 [4:27]-[5:24]; creator-rule]
- A stiff starter (about 50 % hydration, 1 part flour to 0.5 part water) favours yeast over bacteria. — `{"starter_hydration_pct": 50}` [breadcode-2316; yt:sFQZIKb_iIA 1:15; creator-rule]
- A starter doubling in about 8 h at room temperature is the creator's benchmark for being active enough. — `{"hours": 8}` [breadcode-2336; yt:-6dlWqFlFSo 3:57; creator-rule]
- A starter taken from the fridge needs about two days of feeding at room temperature, one or two feeds a day, before it is active again. — `{"days": 2, "feeds_per_day": [1, 2]}` [breadcode-2337; yt:-6dlWqFlFSo 5:39; creator-rule]
- A 1:1:1 feed leaves more pre-fermented flour and acid in the starter; a 1:5:5 feed (starter : water : flour) gives less acid. — `{"ratio": "1:5:5"}` [breadcode-2338; yt:-6dlWqFlFSo 6:45; creator-rule]
- A stiffer starter (about 50% water) gave a higher rise than a liquid starter: 10 cm against 6 cm. — `{"hydration_pct": 50, "rise_cm": [6, 10]}` [breadcode-2354; yt:WOfTMskT44Y 6:52; measured] *(low confidence)*
- Feeding the starter once a day gives a better balance of yeast and bacteria, and a stiff starter boosts the yeast. — `{"feeds_per_day": 1}` [breadcode-2362; yt:XkoIlgnIeaQ 1:11; creator-rule]
- The starter is fed 1:5:5 (10 g starter, 50 g flour, 50 g water) the evening before mixing so it is active the next morning. — `{"ratio": "1:5:5", "starter_g": 10, "flour_g": 50, "water_g": 50}` [breadcode-2394; yt:JoAG7R9QESA 0:34; creator-rule]
- Discard starter works about as well as an active starter in this rye bread. [breadcode-2458; yt:cHwLBs9SRd4 1:39; creator-rule]
- A regular sourdough starter is typically about 100 % hydration: equal parts flour and water. — `{"hydration_pct": 100}` [breadcode-2511; yt:MqH3GVfjfBc 1:12; creator-rule]
- A liquid starter at 500 % hydration (1 part flour to 5 parts water) removes the vinegary notes and favours dairy notes, giving a mild, yogurt-like sourness that is less sharp than a 100 % starter. — `{"hydration_pct": 500}` [breadcode-2512; yt:MqH3GVfjfBc 1:48; creator-rule]
- Readiness signs for a starter include pockets of air and a mild smell. [breadcode-2536; yt:RjTGiqefbAI 3:01; anecdote] *(low confidence)*
- To dry a starter for storage, mix 10 g of healthy starter into 100 g wholegrain flour until fully incorporated, then keep the mix in a closed container in a dark cupboard. — `{"starter_g": 10, "flour_g": 100}` [breadcode-2693; yt:EEqjqaTKV24 1:04; creator-rule]
- To revive the dried mix, add 100 g water for a 1:1 flour-to-water ratio and stir until no dry lumps remain. — `{"water_g": 100, "ratio": "1:1"}` [breadcode-2694; yt:EEqjqaTKV24 2:14; creator-rule]
- Reviving a dried starter: feed daily at 1:5:5 (10 g starter, 50 g flour, 50 g water) until it doubles within 5 to 10 h, then bake. — `{"ratio": "1:5:5", "doubling_h": [5, 10]}` [breadcode-2695; yt:EEqjqaTKV24 2:47; creator-rule]
- A liquid (very hydrated) starter is easier to carry on a flight; he uses 100 ml containers (EU airline limit as he states it). — `{"container_ml": 100}` [breadcode-2722; yt:T4YvXrlp4eQ 1:52; creator-rule]
- To make a starter liquid for travel, dilute about 1 g of starter in 50 g of water. — `{"starter_g": 1, "water_g": 50}` [breadcode-2723; yt:T4YvXrlp4eQ description; creator-rule]
- Reviving a weak, low-volume starter: he fed it 50 g flour and 40 g water (slightly less than 1:1) and baked the next day. — `{"flour_g": 50, "water_g": 40, "hydration_pct": 80}` [breadcode-2724; yt:T4YvXrlp4eQ 3:09; creator-rule]
- A wild-yeast starter from whole-wheat flour and water is fed daily for about seven days, discarding the excess, until bubbly and sour-smelling. — `{"feed_days": 7}` [breadcode-2746; yt:5DWcrVHK1WU 0:00; creator-rule]
- Use the leftover dough sample as the starter for the next dough and repeat. [breadcode-2759; yt:5DWcrVHK1WU 3:46; creator-rule]
- A new sourdough starter may be ready after about 7 days, but sometimes takes longer, so judge readiness by its signs rather than the day count. — `{"maturation_h": [168, null]}` [breadcode-2945; yt:MsNy8be0m_Q 0:00; creator-rule]
- A starter is ready to use when it visibly rises in size and smells vinegary or yogurt-like. [breadcode-2946; yt:MsNy8be0m_Q 0:00; creator-rule]
- A fed starter is ready when it doubles in about 6–8 h; if slower, feed again until it reaches that activity. — `{"doubling_h": [6, 8], "temp_c": 20}` [breadcode-2953; yt:HPM6FgAt8DA description; creator-rule]
- A 1:1:1 feed by weight (old starter : flour : water, 80 g each) with freshly milled rye gave a very active starter within 6 h. — `{"ratio": "1:1:1 starter:flour:water", "feed_each_g": 80, "time_h": 6}` [breadcode-2954; yt:HPM6FgAt8DA description; anecdote]
- The creator places a rubber band on the jar to check whether the starter is active enough. [breadcode-2955; yt:HPM6FgAt8DA description; creator-rule]
- On day 1, mix 50 g whole wheat flour with 50 g water to start a sourdough starter. — `{"flour_g": 50, "water_g": 50, "ratio": "1:1 flour:water"}` [breadcode-2956; yt:w6Jzfk68BHk 0:00; creator-rule]
- Repeat the daily 1:1:1 feed (50 g old starter, 50 g flour, 50 g water) until the starter doubles within 10 h. — `{"doubling_h": [null, 10], "flour_g": 50, "water_g": 50}` [breadcode-2957; yt:w6Jzfk68BHk 0:00; creator-rule]
- An active starter with bubbles that doubles or triples in size is a strong indicator of how the bread will turn out. [breadcode-2960; yt:St46bcVwFGw 0:00; creator-rule]
- After weeks in the fridge, a starter's live microbes become dormant spores that must reactivate before use. [breadcode-2965; yt:Cxlvohn9oDU 0:00; creator-rule]
- Before using a refrigerated starter for bread, feed it once or twice to reactivate it. — `{"feeds_before_bake": [1, 2]}` [breadcode-2966; yt:Cxlvohn9oDU 0:00; creator-rule]

### starter.3-minute-recipe

- A no-knead 3-minute loaf uses 900 g bread flour, 900 g room-temperature water, 18 g salt (description; the spoken recipe says 20 g) and 45–180 g starter (5–20% of flour); the dough is mixed, rested in an oiled loaf pan and baked without shaping or folds. — `{"flour_g": 900, "water_g": 900, "starter_g": [45, 180], "salt_g": 18, "hydration_pct": 100}` [breadcode-1902; yt:bi-zqG5y1I8 description; 2:13; creator-rule]

### starter.acetic

- The author's rye starters tend to contain many acetic acid bacteria, which suits very hearty, strong-tasting rye bread. [breadcode-0950; SF Ch 12 Troubleshooting §12.1.10 Why does my starter smell like vinegar ; anecdote]

### starter.ancient-grain-recipe

- Einkorn or emmer sourdough: 160 g starter (40% of flour), 400 g flour plus 80 g from the starter (480 g total), 260 g water (65% on flour, about 70% total), 9.6 g salt (2%), rested overnight, baked 25 min covered then 20 min uncovered at 230 °C. — `{"flour_g": 480, "flour_from_starter_g": 80, "starter_g": 160, "water_g": 260, "salt_g": 9.6, "hydration_pct": 65, "total_hydration_pct": 70}` [breadcode-1924; yt:PehhnV1lW2o 0:00; creator-rule]

### starter.bucket-recipe

- Bucket dough: 1000 g bread flour, 1000 g water, 20 g salt and 45–180 g starter (5–20% of flour), homogenised and left to roughly double at room temperature, then used over days from the same container. — `{"flour_g": 1000, "water_g": 1000, "salt_g": 20, "starter_g": [45, 180], "hydration_pct": 100}` [breadcode-1930; yt:MetItcI3Yks 0:35; creator-rule]

### starter.creation

- Whole-wheat or whole-rye flour is a good first flour for a starter because the grain hull carries natural yeast. [breadcode-2824; yt:8Djw4hm5FHQ description; creator-rule]
- Day 1 of a from-scratch starter: mix 50 g whole-wheat flour with 50 g water (equal weights, 100 % hydration) and stir. — `{"flour_g": 50, "water_g": 50, "hydration_pct": 100}` [breadcode-2825; yt:8Djw4hm5FHQ 0:00; creator-rule]
- From day 2, keep 50 g of the mix, add 50 g whole-wheat flour and 50 g water, and repeat the feeding until the starter doubles within about 8 hours. — `{"kept_g": 50, "flour_g": 50, "water_g": 50, "rise_pct": 100, "rise_max_h": 8}` [breadcode-2826; yt:8Djw4hm5FHQ 0:00; creator-rule]
- Whole-grain flour (whole wheat or whole rye) gives a starter a head start and makes it faster to become active. [breadcode-2918; yt:PRJ5OzQnb4I 0:00; creator-rule]
- If a starter stays inactive, a bad batch of flour may be the cause; switch to a different batch. [breadcode-2919; yt:PRJ5OzQnb4I 0:00; anecdote] *(low confidence)*

### starter.direct-use

- Skipping the starter build: about 1 % of older starter (flour basis) mixed directly into the dough makes it ready in about 24 h, depending on temperature. — `{"starter_pct_of_flour": 1, "ferment_h": 24}` [breadcode-0291; SF Ch 6 Bread types; creator-rule]

### starter.discard

- To reactivate discard before baking, add fresh flour and water equal to half the discard weight. — `{"flour_pct_of_discard": 50, "water_pct_of_discard": 50}` [breadcode-2382; yt:7V3FyVzzVUI 2:57; creator-rule]
- Warm water speeds up the fermentation of a discard dough. [breadcode-2383; yt:7V3FyVzzVUI 2:57; creator-rule]

### starter.discard-recipe

- Slow-fermented discard starter can be used as the base for a loaf: about 1000 g discard, 200 g flour, 100 g seeds, 15 g salt and 10 g turmeric, mixed by hand, with no bulk fermentation and a loaf-pan proof. — `{"sourdough_g": 1000, "flour_g": 200, "seeds_g": 100, "salt_g": 15, "turmeric_g": 10, "hydration_pct": 70}` [breadcode-1897; yt:xmt3eXzOQLM description (Specs for the dou; creator-rule]

### starter.dose

- Use at most about 20 % starter (by flour weight) when making dough. — `{"starter_pct_of_flour": [0, 20]}` [breadcode-0366; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A regular starter is typically dosed at 10-20 % of flour weight. — `{"starter_pct_of_flour": [10, 20]}` [breadcode-0370; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A starter dose as low as about 1 % of flour weight is possible. — `{"starter_pct_of_flour": [1, 1]}` [breadcode-0371; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A low inoculation (about 1 %) gives microbes more room to rebalance during dough fermentation. — `{"starter_pct_of_flour": [1, 1]}` [breadcode-0372; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- If the starter has not been fed for about 1 day, use about 5 % starter in the dough. — `{"starter_pct_of_flour": [5, 5], "unfed_days": 1}` [breadcode-0373; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- If the starter has gone 2 days without feeding, reduce the dough starter further, to about 1 %. — `{"starter_pct_of_flour": [1, 1], "unfed_days": 2}` [breadcode-0374; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Reducing the dough starter dose can be used when a starter was fed but the dough is delayed; using less very ripe starter avoids re-feeding. [breadcode-0379; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Most bakers use about 20 % starter (by flour weight); the author recommends going lower, about 5-10 %. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-0423; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- A 90 % starter dose leaves very little room for microbes to adjust in the main dough. — `{"starter_pct_of_flour": 90}` [breadcode-0428; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- A 1 % starter dose lets microbes regrow into a balanced state in the dough. — `{"starter_pct_of_flour": 1}` [breadcode-0429; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- A high starter dose adds more pre-fermented flour, so enzymes break down more of the dough; too long a fermentation gives very sticky dough. [breadcode-0430; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Summer dosing: about 5 % starter at about 25 °C ambient (77 °F). — `{"starter_pct_of_flour": 5, "temp_c": 25}` [breadcode-0433; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Winter dosing: about 10-20 % starter at about 20 °C ambient (68 °F). — `{"starter_pct_of_flour": [10, 20], "temp_c": 20}` [breadcode-0434; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- A low inoculation lets the starter regrow inside the dough into a balanced state, so a non-ideal starter can still be used. [breadcode-0435; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- For fermentolysis, reduce the starter from about 20 % (the conventional autolysis recipe) to about 5-10 %, or ferment in a colder place. — `{"starter_pct_of_flour": [5, 10]}` [breadcode-0451; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- Table 7.1, recently fed starter: 5 % at 30 °C (86 °F). — `{"temp_c": 30, "starter_pct": 5}` [breadcode-0452; SF Table 7.1; creator-rule]
- Table 7.1, recently fed starter: 10 % at 25 °C (77 °F). — `{"temp_c": 25, "starter_pct": 10}` [breadcode-0453; SF Table 7.1; creator-rule]
- Table 7.1, recently fed starter: 15 % at 20 °C (68 °F). — `{"temp_c": 20, "starter_pct": 15}` [breadcode-0454; SF Table 7.1; creator-rule]
- Table 7.1, starving starter: 2.5 % at 30 °C (86 °F). — `{"temp_c": 30, "starter_pct": 2.5}` [breadcode-0455; SF Table 7.1; creator-rule]
- Table 7.1, starving starter: 5 % at 25 °C (77 °F). — `{"temp_c": 25, "starter_pct": 5}` [breadcode-0456; SF Table 7.1; creator-rule]
- Table 7.1, starving starter: 10 % at 20 °C (68 °F). — `{"temp_c": 20, "starter_pct": 10}` [breadcode-0457; SF Table 7.1; creator-rule]
- On a very hot day, the dose can be as low as about 1 % starter. — `{"starter_pct": 1}` [breadcode-0462; SF Ch 7 Wheat sourdough §7.7 Fermentolysis; creator-rule]
- Use 20 % starter by flour weight (80 g on 400 g flour) for this dough. — `{"starter_pct_of_flour": 20}` [breadcode-1369; yt:o_xtVmR4vlc 7:55; creator-rule]
- Use about 10 % starter by flour weight in summer and about 20 % in winter, aiming for a bulk of 8–12 h. — `{"starter_pct_of_flour": [10, 20], "bulk_h": [8, 12]}` [breadcode-1380; yt:msqU-ylXWUs 6:01; creator-rule]
- For a same-day bake use about 20 % rye starter (baker's %); for an overnight bake use about 5-10 %. — `{"starter_pct_of_flour": [5, 20]}` [breadcode-1389; yt:NMglhwp2lNs 6:54-8:41; creator-rule]

### starter.dosing

- Most recipes call for about 20 % starter based on flour weight; for a starter past peak or straight from the fridge he uses 5-10 % instead (about 25-50 g for 500 g flour), relying on the dough to re-balance the starter. — `{"typical_recipe_starter_pct_of_flour": 20, "starter_pct_of_flour": [5, 10], "example_flour_g": 500, "example_starter_g": [25, 50]}` [breadcode-1789; yt:lNCHzD8HxBA 12:32; creator-rule]

### starter.flavour-conversion

- To move a 100 % starter toward a mild, yoghurt-like flavour, take about 15 g of starter, feed it about 30 g of whole-wheat flour, then stir in about 150 g of water (roughly five times the flour weight) in a clean jar; cover, and repeat the feeding about 3–4 times over roughly a week. — `{"starter_g": 15, "flour_g": 30, "water_g": 150, "water_to_flour_ratio": 5, "repeats": [3, 4], "duration_days": 7}` [breadcode-2417; yt:HgcGcG7rdZM 0:00-0:33; creator-rule]
- After a starter has been pushed to lactic-only, the sharp acetic note was hard to regain; he attributes this to each feeding selecting for lactic-producing strains. [breadcode-2421; yt:HgcGcG7rdZM 2:46; anecdote]
- Before converting a starter to a lactic profile, keep part of the original acidic starter (dried, frozen or refrigerated) so the acidic profile can be restored later. [breadcode-2422; yt:HgcGcG7rdZM 3:20; creator-rule]
- Resetting a converted starter to 100 % hydration (equal weights of flour and water) and feeding it for a week removed the acidic smell in his single starter. — `{"hydration_pct": 100, "feeding_days": 7}` [breadcode-2425; yt:HgcGcG7rdZM 4:59; anecdote]

### starter.fridge-storage

- After the dough doubles, it goes into the fridge; the creator says fermentation nearly stops below about 6 °C. After about 48 h it had collapsed a little, and it was then baked. — `{"fridge_c_max": 6, "fridge_storage_h": 48}` [breadcode-1931; yt:MetItcI3Yks 2:12; anecdote] *(low confidence)*

### starter.hydration

- Strong acetic notes are favoured by a starter kept at about 100 % hydration. — `{"hydration_pct": 100}` [breadcode-0985; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; anecdote] *(low confidence)*
- The discard flour amounts assume the discard is at 100% hydration. If the discard is wetter or stiffer, adjust the flour until the dough is stiff and holds together. — `{"discard_hydration_pct": 100}` [breadcode-1955; yt:xmt3eXzOQLM 3:02; creator-rule]
- A normal sourdough starter is kept at 100 % hydration: equal weights of flour and water. — `{"starter_hydration_pct": 100}` [breadcode-2288; yt:-LeUYBe7HlY 8:03; creator-rule]
- A regular sourdough starter is typically 100 % hydration, equal parts flour and water. — `{"hydration_pct": 100}` [breadcode-2297; yt:ES7x28d10pg 0:00; creator-rule]
- A 100% hydration discard is brought to about 50% hydration by adding dry flour; the description's mix (400 g discard = 200 g flour + 200 g water, plus 200 g flour) gives 50%. — `{"discard_g": 400, "discard_hydration_pct": 100, "added_flour_g": 200, "dough_hydration_pct": 50}` [breadcode-2403; yt:urHYzIktD9M description; 2:53-3:27; creator-rule]

### starter.lievito-madre

- Lievito madre and pasta madre belong to the same category as the stiff starter, and the two terms are used interchangeably. [breadcode-0233; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; creator-rule]
- Lievito madre can be made from dried or fresh fruit or from garden leaves; wild yeast and bacteria consume the plant glucose. [breadcode-0234; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; creator-rule]

### starter.liquid

- Liquid sourdough starter is fed at a water-to-flour ratio of 5:1 (500 % hydration). — `{"hydration_pct": 500, "ratio": "5:1 water:flour"}` [breadcode-0168; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- A liquid starter suits a very sour bread made with a very strong wheat flour. [breadcode-0172; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- A liquid starter is kept at about 500 % hydration: much more water than flour, and the water layer on top changes the microbiome. — `{"hydration_pct": 500}` [breadcode-0192; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- Returning a liquid-converted starter to regular or stiff hydration keeps the liquid flavour profile, while yeast activity is enhanced again. [breadcode-0194; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- Liquid conversion is treated as irreversible: the microbe subset it selects remains after the starter goes back to regular or stiff hydration. [breadcode-0195; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- Keep a backup of the original starter before the liquid conversion, because the liquid environment selects a different microbe subset that persists. [breadcode-0196; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- Liquid conversion starts by mixing 1 g of existing starter with 5 g of flour and 25 g of water (ratio 1:5:25); the flour settles and the process is repeated over a few days. — `{"starter_g": 1, "flour_g": 5, "water_g": 25, "ratio": "1:5:25", "hydration_pct": 500}` [breadcode-0197; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- A liquid starter is ready when tiny CO2 bubbles move in the liquid when gently shaken and it smells creamy and dairy. [breadcode-0198; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- The liquid conversion is done with a 24 h wait between feedings, repeated three times; the last feed is used for dough 6–12 h later. — `{"wait_h": 24, "feedings": 3, "dough_after_last_feed_h": [6, 12]}` [breadcode-0199; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- The liquid conversion takes around 3 days in total. — `{"duration_h": 72}` [breadcode-0200; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- The liquid conversion ready signs in the flowchart are a sour yogurt-like smell and bubbles visible on the flour; if not, feed again at 1:5:25. — `{"ratio": "1:5:25"}` [breadcode-0201; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- A liquid starter works best with a very strong flour that can withstand a long fermentation; a weak wheat flour will not work with it. [breadcode-0202; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- A liquid starter can be used with a loaf pan when a free-standing loaf is not required. [breadcode-0203; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- For a hearty pancake-style dough, use about 5 % liquid starter by flour weight (about 50 g per 1000 g flour), and subtract that starter's weight from the water in the recipe (e.g. 600 g water with 50 g starter becomes 550 g water). — `{"starter_pct_of_flour": 5, "flour_g": 1000, "starter_g": 50, "water_g": 550, "water_g_recipe": 600}` [breadcode-0204; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- The liquid on top of a liquid starter can be drained and used for acidity, for example in lacto-fermented hot sauces. [breadcode-0207; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; anecdote]
- Liquid starter: very strong wheat flour, minimal yeast activity, high bacterial activity. [breadcode-0247; SF Table 4.1; creator-rule]
- A liquid starter boosts anaerobic bacterial fermentation, so it tends to produce more lactic acid than acetic acid. [breadcode-0904; SF Ch 12 Troubleshooting §12.1.4 What's the benefit of using a liquid sou; creator-rule]
- Converting a starter to liquid permanently changes its microbiome: once acetic acid-producing bacteria are eliminated, the starter cannot go back, so keep a backup of the original. [breadcode-0906; SF Ch 12 Troubleshooting §12.1.4 What's the benefit of using a liquid sou; creator-rule]
- To pickle a starter, take 5 g of existing starter and feed it 20 g of flour and 100 g of water, a hydration of about 500 %. — `{"starter_g": 5, "flour_g": 20, "water_g": 100, "hydration_pct": 500}` [breadcode-0934; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; creator-rule]
- The pickling feed is repeated daily: take 5 g of the previous mixture and feed it 20 g of flour and 100 g of water; after 2–3 additional feedings the starter should be adapted. — `{"starter_g": 5, "flour_g": 20, "water_g": 100, "feedings": [2, 3]}` [breadcode-0936; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; creator-rule]
- Converting a starter to lactic-dominant prevents it from returning to acetic acid production. [breadcode-0949; SF Ch 12 Troubleshooting §12.1.10 Why does my starter smell like vinegar ; creator-rule]
- In a side-by-side test of three roughly 400 g flour doughs, the dough built with liquid sourdough starter (about 20 g, 5x water to flour) gave the most complex and clearly sour flavour. — `{"starter_g": 20, "starter_hydration_pct": 500, "flour_g": 400}` [breadcode-2159; yt:bJxpCVc2wYE 11:46; measured]
- A mouldy fruit-fly culture was taken from below the mould layer and regrown as a liquid starter (about 5 parts water to 1 part flour). After one more feed the mould was gone and the culture was active. — `{"hydration_pct": 500}` [breadcode-2199; yt:xEwk5yHf_Gc 3:10; anecdote] *(low confidence)*
- Bread formula: 400 g flour, 320 g water (80 %), 40 g liquid starter (10 % of flour) and 8 g salt (2 %). — `{"hydration_pct": 80, "starter_pct_of_flour": 10, "salt_pct": 2}` [breadcode-2201; yt:xEwk5yHf_Gc 5:38; creator-rule]
- To make a liquid starter, feed about 15 g of regular starter with about 30 g whole wheat flour and about 150 g water (roughly 5x the flour weight) and stir it in a clean jar. — `{"starter_g": 15, "flour_g": 30, "water_g": 150, "hydration_pct": 500}` [breadcode-2205; yt:ES7x28d10pg 0:32; creator-rule]
- When replacing a regular starter with liquid starter, count the liquid's water: in his example, 50 g liquid starter replaces 50 g of the 400 g water in a formula that had 500 g flour, 400 g water and 100 g regular starter. — `{"liquid_starter_g": 50, "water_reduction_g": 50, "regular_formula_hydration_pct": 82}` [breadcode-2208; yt:ES7x28d10pg 4:26; creator-rule]
- The creator recommends 5 to 10 % liquid starter by flour weight; in his experience about 10 % liquid starter gives roughly the same fermentation speed as 20 % regular starter. — `{"liquid_starter_pct_of_flour": [5, 10], "regular_equivalent_pct": 20}` [breadcode-2209; yt:ES7x28d10pg 3:51; creator-rule] *(low confidence)*
- If a liquid starter was fed less than 24 hours earlier, use about 30 g in a dough; if it has been unfed longer, use less, since the cultures regrow in the dough. — `{"starter_g": 30}` [breadcode-2211; yt:ES7x28d10pg 7:45; creator-rule]
- Liquid starters are harder to use because their extra water changes the dough hydration, so beginners should start with a regular starter. [breadcode-2212; yt:ES7x28d10pg 4:59; creator-rule]
- To build a very liquid starter, the creator takes 100 g of regular starter and adds about 500 g water (roughly 1:5 starter to water). — `{"starter_g": 100, "water_g": 500, "ratio": "1:5"}` [breadcode-2296; yt:ES7x28d10pg 1:06-1:39; creator-rule]
- Balloon test: the liquid starter inflated a balloon less than a South-African starter inflated a condom; the test was unfair because the flour amounts differed. [breadcode-2903; yt:FKlQFryXYmY description; anecdote] *(low confidence)*

### starter.maintenance

- A starter last fed about 48 h earlier has very active bacteria while yeast may be dormant; feeding can be skipped and a very small inoculum used. — `{"hours_since_feed": 48}` [breadcode-0176; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- For a starter fed ~48 h earlier, an inoculum of about 1 % of flour weight is the author's practice (10 g per 1 kg flour). — `{"starter_pct_of_flour": 1, "starter_g_per_kg_flour": 10}` [breadcode-0177; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- A very young starter fed only 6–8 h earlier may need up to about 20 % of flour weight as inoculum. — `{"starter_pct_of_flour": [1, 20], "hours_since_feed": [6, 8]}` [breadcode-0178; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Keep some unbaked dough as starter; for later use add water and flour and keep it in the fridge, where it stays good for months. [breadcode-0304; SF Ch 6 Bread types; creator-rule]
- Rye dough tolerates starter that is active or inactive, from a week at room temperature or straight from the fridge. — `{"room_temp_unfed_days": 7}` [breadcode-0327; SF Ch 8 Non wheat sourdough; creator-rule]
- Some bakers require the starter to double in size before use, because the idea is to use it at peak performance for balanced fermentation. [breadcode-0888; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- Doubling is an imperfect metric: rye flour retains little fermentation gas, so a healthy rye starter may not rise much. [breadcode-0889; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- A wheat flour with less gluten also makes a starter rise less, because the weaker gluten network lets more gas escape. [breadcode-0890; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- Build your own volume-increase metric: use the starter at the point before it peaks and collapses, which may be around 50 %, 100 % or 200 % rise. — `{"rise_pct": [50, 200]}` [breadcode-0891; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- Using a starter a little early is safer than using it late. [breadcode-0892; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- If the starter is used late, reduce its quantity; for example, a recipe calling for 20 % starter should use 10 % instead, because the starter regrows in the main dough. — `{"recipe_starter_pct": 20, "adjusted_starter_pct": 10}` [breadcode-0893; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- A stronger starter smell indicates that the starter has fermented further, so use less of it in the dough. [breadcode-0894; SF Ch 12 Troubleshooting §12.1.1 My starter does not double in size; creator-rule]
- Treat the starter as a dough: use the same ratios as for bread dough, and train it in the same environment where it will later ferment dough. [breadcode-0899; SF Ch 12 Troubleshooting §12.1.2 What's the best starter feeding ratio?; creator-rule]
- Use unchlorinated water, because chlorine added to tap water kills microorganisms. [breadcode-0909; SF Ch 12 Troubleshooting §12.1.5 My new starter doesn't rise at all; creator-rule]
- To avoid chlorine, use bottled spring water or water filtered through activated charcoal. [breadcode-0910; SF Ch 12 Troubleshooting §12.1.5 My new starter doesn't rise at all; creator-rule]
- Tap water drawn into a loosely covered container and left to stand should dissipate the chlorine within 12–24 hours, and it reaches room temperature. — `{"stand_h": [12, 24]}` [breadcode-0911; SF Ch 12 Troubleshooting §12.1.5 My new starter doesn't rise at all; creator-rule]
- Use whole grain flour (e.g. whole wheat or whole rye) for a starter, because it carries more natural wild yeast and bacteria; a starter made only from white flour sometimes fails. [breadcode-0912; SF Ch 12 Troubleshooting §12.1.5 My new starter doesn't rise at all; creator-rule]
- A starter that rises early and then falls is normal: different microbes activate as it matures, and early bad microbes such as mold can cause a large rise. [breadcode-0914; SF Ch 12 Troubleshooting §12.1.6 I made a starter, it rose on day 3 and n; creator-rule]
- With each feeding, the starter selects the microbes best at fermenting flour, so the leftover starter from the first days should be discarded. [breadcode-0915; SF Ch 12 Troubleshooting §12.1.6 I made a starter, it rose on day 3 and n; creator-rule]
- Unneeded starter later on should not be thrown away; it can be used for discard bread. [breadcode-0916; SF Ch 12 Troubleshooting §12.1.6 I made a starter, it rose on day 3 and n; creator-rule]
- A starter typically takes around 7 days to grow. — `{"days": 7}` [breadcode-0917; SF Ch 12 Troubleshooting §12.1.6 I made a starter, it rose on day 3 and n; anecdote]
- Each feeding makes the starter better at fermenting flour. [breadcode-0918; SF Ch 12 Troubleshooting §12.1.6 I made a starter, it rose on day 3 and n; creator-rule]
- Liquid (hooch) that forms on an inactive starter results from the heavier flour settling to the bottom while the water rises to the top. [breadcode-0919; SF Ch 12 Troubleshooting §12.1.7 Liquid on top of my starter; creator-rule]
- The hooch darkens because fine flour particles float up into it, and dead microorganisms also float in it. [breadcode-0920; SF Ch 12 Troubleshooting §12.1.7 Liquid on top of my starter; creator-rule]
- Stirring the hooch back into the starter removes it. [breadcode-0922; SF Ch 12 Troubleshooting §12.1.7 Liquid on top of my starter; creator-rule]
- Hooch usually means the starter has fermented for a long time and is very sour; it is suited to discard crackers or discard bread. [breadcode-0923; SF Ch 12 Troubleshooting §12.1.7 Liquid on top of my starter; creator-rule]
- Every starter becomes more sour the longer it waits; check smell and taste to judge sourness. [breadcode-0939; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- If a starter becomes sour too fast, feed it daily and reduce the amount of old starter used for feeding. [breadcode-0940; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- The float test may not reliably show readiness for non-wheat doughs: fermentation gas escapes from non-wheat doughs, so the starter tends to sink. [breadcode-0951; SF Ch 12 Troubleshooting §12.1.11 Why does my starter not float after usi; creator-rule]
- For non-wheat starters, watch for bubbles at the container edge and judge the aroma: a mature starter smells mildly sour without being overly pungent. [breadcode-0952; SF Ch 12 Troubleshooting §12.1.11 Why does my starter not float after usi; creator-rule]
- After a long time without feeding, microbes sporulate and hibernate; adding food reactivates them, but activation may be too slow to ferment the dough before protease breaks down the gluten. [breadcode-0967; SF Ch 12 Troubleshooting §12.2.4 My dough completely tears after a long f; creator-rule]
- To avoid tearing, feed the starter a couple more times until it is lively before making the dough, so the dough is ready to shape before the gluten breaks down. [breadcode-0969; SF Ch 12 Troubleshooting §12.2.4 My dough completely tears after a long f; creator-rule]
- Keep the starter at room temperature and feed it daily at a 1:5:5 ratio (old starter : flour : water) to balance yeast and bacteria. — `{"ratio": "1:5:5 (starter:flour:water)"}` [breadcode-0974; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; creator-rule]
- Feeding the starter once a day at room temperature shifts it toward yeast fermentation. — `{"feeds_per_day": 1}` [breadcode-1000; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; literature]
- Frequent feeding and a stiff starter improve yeast activity, giving fluffier dough with less bacterial activity and less acidity in the bread. [breadcode-1041; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; creator-rule]
- A more active starter is built by daily feeding over several days at the dough's ratio; for a 20 % starter dose the author uses a 1:5:5 feed. — `{"starter_pct_of_flour": 20, "ratio": "1:5:5 (starter:flour:water)"}` [breadcode-1053; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- Example feed: 10 g starter with 50 g flour and 50 g water. — `{"starter_g": 10, "flour_g": 50, "water_g": 50}` [breadcode-1054; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- A starter can be fed with freshly milled grain and water in equal volumes (1:1 by volume) with no storage of milled flour. — `{"grain_to_water_ratio_by_volume": 1}` [breadcode-1290; yt:wdnX3baMB38 54:10; creator-rule]
- Feed the starter 1:5:5 (starter:flour:water) in the evening before a morning bake, after keeping it active at room temperature for at least two days; the low-acid feed protects gluten. — `{"ratio": "1:5:5 starter:flour:water", "starter_g": 10, "flour_g": 50, "water_g": 50, "active_days_room_temp": 2}` [breadcode-1425; yt:NMglhwp2lNs 5:02-5:38; creator-rule]
- A fruit-fly trap: 50 g whole wheat flour, 25 g vinegar and 25 g water left out so fruit flies land on it and carry wild yeast into the mix. — `{"whole_wheat_flour_g": 50, "vinegar_g": 25, "water_g": 25}` [breadcode-2197; yt:xEwk5yHf_Gc 1:53; anecdote] *(low confidence)*
- At 8:04 a 50 % hydration feed (1 part flour to 0.5 part water) was given to boost yeast activity; the dough then doubled and was shaped. — `{"feed_hydration_pct": 50}` [breadcode-2203; yt:xEwk5yHf_Gc 8:04; anecdote] *(low confidence)*
- Daily liquid-starter maintenance: keep about 50 g, add about 50 g flour and about 250 g water (about 5x), and stir; a sample jar lets you judge readiness by eye. — `{"feed_flour_g": 50, "feed_water_g": 250, "hydration_pct": 500}` [breadcode-2210; yt:ES7x28d10pg 5:31; creator-rule]
- The older an unfed starter, the lower the inoculation: a starter unfed for about a week needs only about 1 % of flour. — `{"starter_pct_of_flour": 1, "unfed_days": 7}` [breadcode-2213; yt:ngcISK7ZDjc description; creator-rule]
- A very old starter may have sporulated and needs several feedings before it is active again. [breadcode-2214; yt:ngcISK7ZDjc description; creator-rule]
- Dried starter powder took up to about 3 days to reactivate in the creator's experience. — `{"reactivation_days": 3}` [breadcode-2215; yt:ngcISK7ZDjc 0:34; anecdote] *(low confidence)*
- An unfed starter kept at room temperature is good for about 5 to 7 days at most. — `{"room_temp_unfed_max_days": [5, 7]}` [breadcode-2216; yt:ngcISK7ZDjc 1:45; creator-rule] *(low confidence)*
- Some bakers report good bread from a starter left unfed in the fridge for a year; the fridge temperature decides how long it keeps. — `{"fridge_unfed_reported_months": 12}` [breadcode-2217; yt:ngcISK7ZDjc 1:45; anecdote] *(low confidence)*
- For an old unfed starter, use a small inoculum in the dough: about 5 % of flour (against about 20 % for a fresh starter in his standard recipe) and about 1 % for a starter unfed for a week. — `{"starter_pct_of_flour": [1, 5], "usual_starter_pct": 20}` [breadcode-2219; yt:ngcISK7ZDjc 2:57; creator-rule]
- Starter prep for this bake (1:5:5 feeds): 10 g starter with 50 g flour and 50 g water on Wednesday morning, Wednesday evening and Thursday morning; then 20 g starter with 100 g flour and 100 g water on Thursday evening. — `{"ratio": "1:5:5"}` [breadcode-2227; yt:iEfLO_Cl0uE description; creator-rule]
- A healthy starter doubles within about 5 hours after a 1:1:1 feed; if it does not, feed twice a day until it does, using whole wheat or rye flour. — `{"ratio": "1:1:1", "doubling_h": 5}` [breadcode-2236; yt:nIOPCeLPqrM 2:27; creator-rule]
- For a weak starter, feed 1:5:5 twice a day for about 48 hours using whole wheat or rye flour. — `{"ratio": "1:5:5", "feeds_per_day": 2, "days": 2}` [breadcode-2254; yt:oAFjGTWMgUo 10:44; creator-rule]
- To boost yeast in a new starter, feed it 1:5:5 daily for one to two weeks. — `{"ratio": "1:5:5", "days": [7, 14]}` [breadcode-2263; yt:-LeUYBe7HlY 8:03; creator-rule]
- A starter kept in the fridge without feeding can last a couple of months, because fermentation nearly stops; after removal it needs several days of daily feeding to regain its earlier activity. — `{"months": 2}` [breadcode-2371; yt:-6dlWqFlFSo 5:06; creator-rule]
- He feeds the mother dough (starter) with equal amounts of flour and water, a 100 % hydration feed. — `{"ratio": 1, "feed_hydration_pct": 100}` [breadcode-2818; yt:nIQEiSGGw2Q 0:00; creator-rule]
- Feed mother dough with equal weights of water and flour (1:1 water:flour), which makes a 100 % hydration starter. — `{"flour_water_ratio": 1, "hydration_pct": 100}` [breadcode-2866; yt:9BxRgTMftWk description; creator-rule]
- To get 160 g starter for a bake: take 80 g mother dough, add 85 g flour and 85 g water (250 g total), wait about 3 h, use 160 g and return the rest to the mother dough. — `{"mother_g": 80, "flour_g": 85, "water_g": 85, "total_g": 250, "used_g": 160, "wait_h": 3}` [breadcode-2867; yt:9BxRgTMftWk description; creator-rule]
- Make about 5 % more starter than the recipe needs, since the culture consumes around 5 % of it. — `{"consumed_pct": 5}` [breadcode-2868; yt:9BxRgTMftWk description; creator-rule] *(low confidence)*
- With cold water and equal weights of flour and water, the starter was ready after about 3 hours; warm water would speed it up. — `{"ready_h": 3}` [breadcode-2869; yt:9BxRgTMftWk 0:40; 1:32; creator-rule]
- To revive a neglected starter, take about 1 g of starter and feed it with 50 g water and 50 g of a second ingredient (the description prints 'water' twice; flour is inferred from the 1:1:1 follow-up feed); wait about 24 h. — `{"starter_g": 1, "water_g": 50, "flour_g": 50, "wait_h": 24}` [breadcode-2906; yt:PR7OM4KCcHI description; creator-rule] *(low confidence)*
- Repeat the feed 24 h later at 1:1:1; the starter is ready when it rises in about 5-6 h; if not, feed twice a day until active. — `{"ratio": "1:1:1", "rise_h": [5, 6], "feeds_per_day_if_slow": 2}` [breadcode-2907; yt:PR7OM4KCcHI description; creator-rule]
- The liquid on top of a neglected starter is lactic acid with alcohol; pour it off and feed the starter normally. [breadcode-2908; yt:PR7OM4KCcHI 0:00; creator-rule]
- A starter stored in the fridge for five weeks could be revived (his anecdote). [breadcode-2910; yt:PR7OM4KCcHI 0:00; anecdote] *(low confidence)*
- Use about 10 g starter per 100 g flour (10 %) so the starter can regrow in the dough. — `{"starter_pct_of_flour": 10}` [breadcode-2927; yt:n-37Z43aRpg 0:00; creator-rule]
- In very hot kitchens (about 30 °C / 86 °F), reduce starter to 2.5-5 % of flour. — `{"starter_pct_of_flour": [2.5, 5], "temp_c": 30}` [breadcode-2928; yt:n-37Z43aRpg 0:00; creator-rule]
- Never use more than 20 % starter. — `{"starter_pct_of_flour_max": 20}` [breadcode-2929; yt:n-37Z43aRpg 0:00; creator-rule]
- A typical feed in the video is about 130 g mother dough with 130 g flour and 130 g water; for a smaller amount, 50 g flour and 50 g water, always equal weights. — `{"mother_g": 130, "flour_g": 130, "water_g": 130, "alt_flour_g": 50, "alt_water_g": 50, "flour_water_ratio": 1}` [breadcode-2934; yt:9BxRgTMftWk 0:00; 0:40; creator-rule]
- Feed the starter 1:1:1 and start the autolyse about 5 hours after feeding. — `{"ratio": "1:1:1", "autolyse_delay_h": 5}` [breadcode-2936; yt:MyGMF5sLg1U description; creator-rule]
- A smaller starter dose leaves the main dough to be driven by the dough itself, so the starter can regrow inside the dough during fermentation rather than being used up up front. [breadcode-2939; yt:n-37Z43aRpg 0:00; creator-rule]

### starter.overnight-recipe

- Overnight dough per the description: 350 g bread flour and 50 g whole wheat (400 g flour), 300 g water, 80 g starter and 8 g salt (75% hydration). All mixed at once with strong early kneading. The description says no stretch-and-folds; in the video he does fold when the dough flattens. — `{"flour_g": 400, "water_g": 300, "hydration_pct": 75, "starter_g": 80, "salt_g": 8}` [breadcode-1917; yt:O0REFqmB0rs 2:59 (recipe); 3:35 (folds); creator-rule]

### starter.peak-timing

- A 100 % hydration starter fed 1:1:1 (starter, flour, water) at about 25 °C shows its first volume peak after about 4 h; at about 23 °C the whole process takes roughly twice as long. — `{"temp_c": 25, "peak_h": 4, "ratio": "1:1:1"}` [breadcode-1355; yt:KRO9EcIyXSI 19:57; creator-rule]
- A rye starter is ready when it domes; the float test does not work for rye because it has too little gluten to hold a bubble structure. [breadcode-1370; yt:o_xtVmR4vlc 7:19; creator-rule]
- A starter is ready when it doubles within about 5 hours after feeding; he says the process takes about nine days but can take 5–14 days depending on the environment. — `{"peak_h": 5, "days": 9}` [breadcode-1401; yt:ZTHHI0iuamA description; creator-rule]
- Ready the starter about 8 to 12 hours before mixing the dough (he does this in the evening for a morning mix). — `{"starter_ready_before_mix_h": [8, 12]}` [breadcode-1417; yt:msqU-ylXWUs 2:14; creator-rule]

### starter.ratio

- A 10 % starter dose is a practical default for pizza dough; more starter speeds up fermentation. — `{"starter_pct_of_flour": 10}` [breadcode-1319; yt:-60DxtcJ9Fo description; creator-rule]

### starter.ratios

- The commonly used starter feeding ratios are 1:5:5 and 1:10:10 (starter : flour : water by weight). — `{"ratio": "1:5:5 or 1:10:10"}` [breadcode-0895; SF Ch 12 Troubleshooting §12.1.2 What's the best starter feeding ratio?; creator-rule]
- For a stiff starter, halve the water in the 1:5:5 ratio, giving 1:5:2.5. — `{"ratio": "1:5:2.5", "starter_hydration_pct": [50, 60]}` [breadcode-0896; SF Ch 12 Troubleshooting §12.1.2 What's the best starter feeding ratio?; creator-rule]
- A 1:10:10 feeding suits a starter that is old or has not been fed recently. — `{"ratio": "1:10:10"}` [breadcode-0897; SF Ch 12 Troubleshooting §12.1.2 What's the best starter feeding ratio?; creator-rule]
- A lower inoculation ratio gives the microbes a cleaner environment in which to regrow into a balanced culture. [breadcode-0898; SF Ch 12 Troubleshooting §12.1.2 What's the best starter feeding ratio?; creator-rule]
- The exception is the initial set-up of a new starter: a 1:1:1 ratio speeds up the early days while microbe numbers are still low. — `{"ratio": "1:1:1"}` [breadcode-0900; SF Ch 12 Troubleshooting §12.1.2 What's the best starter feeding ratio?; creator-rule]
- A 1:5:5 or 1:10:10 feeding ratio helps; example: 10 g of starter with 50 g of flour and 50 g of water. — `{"ratio": "1:5:5 or 1:10:10", "starter_g": 10, "flour_g": 50, "water_g": 50}` [breadcode-0941; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- Refresh the starter at a 1:5:5 ratio (starter:flour:water) so it is ready the next morning. — `{"ratio": "1:5:5"}` [breadcode-1361; yt:o_xtVmR4vlc 2:45; creator-rule]
- A low-concentration feed (1:5:5) keeps the starter's base acidity lower, which slows over-fermentation later. — `{"ratio": "1:5:5"}` [breadcode-1426; yt:o_xtVmR4vlc 2:45; creator-rule]
- Feed the starter 1:2:2 (about 30–35 g starter, 70 g whole wheat flour, 70 g water). Use it when it has doubled within about 5 h. — `{"starter_g": [30, 35], "flour_g": 70, "water_g": 70, "ratio": "1:2:2", "ready_h": 5}` [breadcode-1952; yt:U9_GVHuI1Ro 0:34; 1:09; creator-rule]

### starter.refresh

- Feeding 1 g of starter with 5 g flour and 5 g water in the morning gives about 11 g of ready starter by evening (1:5:5). — `{"ratio": "1:5:5", "ready_starter_g": 11}` [breadcode-0282; SF Ch 6 Bread types; creator-rule]
- A warmer kitchen makes the starter ready sooner; in cold conditions use warm water. [breadcode-0283; SF Ch 6 Bread types; creator-rule]

### starter.regular

- Starter types are defined mainly by hydration; the choice trades off acidity, volume increase and the gluten strength of the flour. [breadcode-0166; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Regular sourdough starter is fed at 100 % hydration (equal weights flour and water, 1:1). — `{"hydration_pct": 100}` [breadcode-0167; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Changing starter type is done only by changing the feeding ratio of flour and water; the microbes adapt over a few days of feeding at the same ratio. [breadcode-0173; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- The regular starter is kept at about 100 % hydration: equal parts flour and water; it is the most common and universal starter, with a good balance of yeast and bacteria. — `{"hydration_pct": 100, "flour_water_ratio": "1:1"}` [breadcode-0184; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- After a feeding the regular starter's volume increases greatly, then it reaches a peak and collapses. [breadcode-0185; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- Readiness of a regular starter is judged by air pockets on the edges of the container and by its smell. [breadcode-0186; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- If a regular starter does not perform well, the yeast and bacteria ratio is probably off; frequent daily feedings at 1:5:5 (starter:flour:water) help. — `{"ratio": "1:5:5 starter:flour:water", "feedings_per_day": "frequent daily"}` [breadcode-0187; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- Use about 1–20 % regular starter by flour weight in the dough. — `{"starter_pct_of_flour": [1, 20]}` [breadcode-0190; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- Starter types differ only in the water used when feeding: regular 100 %, liquid 500 %, stiff 50–60 %. — `{"hydration_pct": {"regular": 100, "liquid": 500, "stiff": [50, 60]}}` [breadcode-0245; SF Table 4.1; creator-rule]
- Regular starter: strong wheat flour, balanced yeast and balanced bacterial activity. [breadcode-0246; SF Table 4.1; creator-rule]

### starter.stiff

- Stiff sourdough starter is fed with half as much water as flour (about 50 % hydration). — `{"hydration_pct": 50}` [breadcode-0169; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- In a climate with weaker wheat flours, a stiff starter is advised because it increases yeast activity and reduces bacterial activity. [breadcode-0171; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Changes in starter type are usually visible after one feeding; 2 to 3 feedings, one per day, give a stronger effect. — `{"feedings": [2, 3], "feeding_interval_h": 24}` [breadcode-0174; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- The stiff starter is the driest starter, at about 50–60 % hydration: 100 g of flour takes about 50–60 g of water. — `{"hydration_pct": [50, 60], "flour_g": 100, "water_g": [50, 60]}` [breadcode-0208; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- A correctly hydrated stiff starter has a consistency comparable to pasta or pizza dough, with no dry flour chunks left after mixing. [breadcode-0210; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- A stiff starter that is hydrated enough slightly sticks to the counter when lifted from it. [breadcode-0211; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- If the stiff starter is too dry, fermentation slows greatly and the starter seems inactive; increase the water quantity when the flour cannot be mixed. [breadcode-0212; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- The stiff starter should be much drier than a regular starter but must not be a brick of dough; it should feel like pasta dough. [breadcode-0213; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- Stiff starter conversion from a regular or liquid starter: mix 10 g of existing starter with 50 g flour and 25 g water (ratio 1:5:2.5, 50 % hydration). — `{"starter_g": 10, "flour_g": 50, "water_g": 25, "ratio": "1:5:2.5", "hydration_pct": 50}` [breadcode-0214; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- In the stiff conversion, if the starter is too dry and hard to mix, add more water before the 24 h wait. [breadcode-0215; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- The stiff conversion waits 24 h between feedings, repeats the process three times, and the last feed is used for dough 6–12 h later. — `{"wait_h": 24, "feedings": 3, "dough_after_last_feed_h": [6, 12], "ratio": "1:5:2.5"}` [breadcode-0216; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- The stiff conversion is ready when the starter increases in size and has a sour smell; then one last feed is given. [breadcode-0217; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- The stiff conversion takes around 3 days; hydration starts at 50 % and may be raised to 60 % if the dough is too stiff. — `{"duration_h": 72, "hydration_pct": [50, 60]}` [breadcode-0218; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- The stiff starter boosts yeast activity; in the stiffer environment yeast thrives more, producing more CO2 and less acid. [breadcode-0219; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- Balloon test (author's own, same flour and starter amount, only hydration varied): the stiff starter inflated a balloon most, the regular starter second, and the liquid starter third with far less CO2. — `{"hydration_pct": {"stiff": 50, "regular": 100, "liquid": 500}}` [breadcode-0221; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; measured]
- Using a stiff starter with a low-gluten flour that normally causes problems: reduce hydration for the low-gluten flour and substitute the stiff starter for the yeast; the dough withstood a much longer fermentation and had good oven spring. [breadcode-0222; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; anecdote] *(low confidence)*
- The author could not find a scientific explanation for the stronger yeast activity in the stiff starter; a possible reason is that the bacteria are inhibited by the lack of water. [breadcode-0224; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; extrapolation] *(low confidence)*
- A stiff starter's dough rises until the gluten structure breaks down from fermentation; then the starter falls into the jar. [breadcode-0225; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- Assess a stiff starter by a dome on top, air pockets at the sides of the container, and a mild smell; it smells more alcoholic than the other starters. [breadcode-0226; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- Use about 1–20 % stiff starter by flour weight; the right amount depends on how ripe the starter is. — `{"starter_pct_of_flour": [1, 20]}` [breadcode-0227; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- Use about 1–10 % stiff starter when the starter is ripe in warm conditions (author's practice), and about 20 % in cold conditions. — `{"starter_pct_of_flour": {"warm": [1, 10], "cold": 20}}` [breadcode-0228; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- In very hot conditions, lower the stiff starter amount to about 1–5 % of flour weight to slow fermentation. — `{"starter_pct_of_flour": [1, 5]}` [breadcode-0229; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- In very cold conditions, increase the stiff starter amount up to about 30 % of flour weight. — `{"starter_pct_of_flour": [20, 30]}` [breadcode-0230; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- The stiff starter's amount controls fermentation speed, so it is an adjustment lever. [breadcode-0231; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- Stiff starter hardly homogenises with the dough; dissolving it in the water used for the dough makes mixing much easier. [breadcode-0232; SF Ch 4 Sourdough starter types, §4.4 Stiff starter; creator-rule]
- If only one starter type is used, the stiff starter gives consistent results with little effort; any yeast-based dough can use the stiff starter in place of the yeast. [breadcode-0240; SF Ch 4 Sourdough starter types, §4.6 Conclusion; creator-rule]
- For the same volume increase, the stiff starter produces the least acidity but much more CO2; over a long enough time it reaches the same acidity as the other starters. — `{"volume_increase_pct": 100}` [breadcode-0243; SF Ch 4 Sourdough starter types, §4.6 Conclusion; creator-rule]
- Stiff starter: any wheat flour, high yeast activity, low bacterial activity. [breadcode-0248; SF Table 4.1; creator-rule]
- A stiff starter is easier to mix if first dissolved in the dough's water. [breadcode-0329; SF Ch 8 Non wheat sourdough; creator-rule]
- A stiff starter (hydration 50-60 %) uses the readiness-chart ratios with the water halved: 100 g water on the chart becomes 50-60 g. — `{"starter_hydration_pct": [50, 60]}` [breadcode-0355; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Stiff starters have enhanced yeast activity compared with liquid starters. [breadcode-0356; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- A regular starter has equal parts flour and water, i.e. 100 % hydration. — `{"hydration_pct": 100}` [breadcode-0901; SF Ch 12 Troubleshooting §12.1.3 What's the benefit of using a stiff sour; creator-rule]
- A stiff starter has a hydration of 50–60 %. — `{"hydration_pct": [50, 60]}` [breadcode-0902; SF Ch 12 Troubleshooting §12.1.3 What's the benefit of using a stiff sour; creator-rule]
- A stiff starter favours the yeast part of the starter, so the gluten degrades more slowly and the dough can ferment for longer; this helps with lower-gluten flours. [breadcode-0903; SF Ch 12 Troubleshooting §12.1.3 What's the benefit of using a stiff sour; creator-rule]
- Converting the starter to a stiff starter is another approach if it is too sour. [breadcode-0944; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- A stiff starter boosts the yeast share, giving less bacterial fermentation and a stronger gluten network toward the end of fermentation. [breadcode-0975; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; literature]
- A stiff starter at about 50 % hydration favours yeast activity, giving fluffier and less sour dough for a given volume increase. — `{"hydration_pct": 50}` [breadcode-1001; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; creator-rule]
- In an informal balloon test with the same flour amount, a stiff starter produced more CO2 than a regular or a liquid starter. [breadcode-1002; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; measured]
- A stiff starter lets the dough start fermenting with stronger yeast activity and less bacterial activity. [breadcode-1056; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]
- A stiff starter is about 40–50 % hydration, versus equal parts flour and water (100 %) for a regular starter. — `{"hydration_pct": [40, 50]}` [breadcode-1382; yt:msqU-ylXWUs 7:38; creator-rule]
- To make a stiff starter, feed the regular starter with about half the usual water over a few days. — `{"hydration_pct": [40, 50]}` [breadcode-1385; yt:msqU-ylXWUs 8:45; creator-rule]
- The starter used is stiff, at about 50–60 % hydration. — `{"starter_hydration_pct": [50, 60]}` [breadcode-1552; yt:36GdGT0XYwE 3:42; creator-rule]
- A stiff starter (lower water content) slows acid production and favours yeast over lactic acid bacteria. [breadcode-1598; yt:nQWlG1buMM8 14:43; creator-rule]
- For more dough strength, use a stiffer starter at about 65 % hydration; the guest says this for whole-wheat dough. — `{"starter_hydration_pct": 65}` [breadcode-1792; yt:l_PLhPWE-8E 15:35; creator-rule]
- In the same test, the stiff-starter dough (about 40 g, roughly 50 % hydration) was mild, with only a slight tang at the end. — `{"starter_g": 40, "starter_hydration_pct": 50, "flour_g": 400}` [breadcode-2160; yt:bJxpCVc2wYE 11:11; measured]
- A stiff starter (about half its weight in water) puts the emphasis on yeast, so most of the tang comes from bacteria and a stiff starter gives a milder bread. — `{"starter_hydration_pct": 50}` [breadcode-2161; yt:bJxpCVc2wYE 1:46; creator-rule]
- A loaf of 400 g flour at 80 % hydration, 2 % salt and a stiff starter at 20 % of flour (10 % in summer) uses 80 g starter, or 40 g in summer. — `{"hydration_pct": 80, "salt_pct": 2, "starter_pct_of_flour": [10, 20]}` [breadcode-2189; yt:I3h6heaKWC8 1:45; creator-rule]
- A stiff starter (about 50 % hydration) is suggested as a way to raise yeast activity when other fixes fail. — `{"starter_hydration_pct": 50}` [breadcode-2264; yt:-LeUYBe7HlY 8:38; creator-rule] *(low confidence)*
- For a neglected starter, a stiff build (1 part flour to 0.5 part water, about 50 % hydration) boosts yeast activity more than a regular 1:1 starter, so the creator avoids high hydration. — `{"hydration_pct": 50, "regular_hydration_pct": 100}` [breadcode-2299; yt:ngcISK7ZDjc 7:44-8:20; creator-rule]
- To have a starter ready for a morning bake, the creator feeds 10 g starter with 50 g water and 50 g flour (1:5:5) the night before. — `{"starter_g": 10, "water_g": 50, "flour_g": 50, "ratio": "1:5:5"}` [breadcode-2372; yt:-6dlWqFlFSo 7:21; creator-rule]
- A stiff starter is built from about 100 g flour and 50-60 g water, i.e. about 50-60 % hydration, and the creator recommends it to boost yeast activity. — `{"flour_g": 100, "water_g": [50, 60], "hydration_pct": [50, 60]}` [breadcode-2415; yt:XkoIlgnIeaQ 1:11; creator-rule]
- Switching one starter from 500 % to 50 % hydration (stiff) made extra-large adult balloons inflate much more, which he reads as more yeast activity and less bacterial activity. — `{"hydration_pct": [500, 50]}` [breadcode-2513; yt:MqH3GVfjfBc 2:25; measured]
- A stiffer starter gives less bacterial breakdown of gluten during long ferments; beginners get less sticky dough, more oven spring and a fluffier crumb. [breadcode-2514; yt:MqH3GVfjfBc 3:03; creator-rule]
- Stiff starter maintenance: 10 g starter, 10 g water and 20 g flour (1 : 1 : 2, 50 % hydration), half whole wheat and half white flour. — `{"starter_g": 10, "water_g": 10, "flour_g": 20, "hydration_pct": 50}` [breadcode-2515; yt:MqH3GVfjfBc 3:39; creator-rule]
- To convert a liquid starter into a stiff one, knead it into a dough, keep it in a jar that is not airtight, and feed it once a day for three days. — `{"feed_days": 3}` [breadcode-2516; yt:MqH3GVfjfBc 4:48; creator-rule]
- A stiff starter is ready when the dough shows pockets of air at the bottom of the jar. [breadcode-2517; yt:MqH3GVfjfBc 5:24; creator-rule]
- Use about 20 % starter by flour weight (80 g on 400 g flour) in winter and about 10 % (40 g) in summer, with 300 g water and 8 g salt. — `{"starter_pct_of_flour": [10, 20], "starter_g": [40, 80], "flour_g": 400, "water_g": 300, "salt_g": 8}` [breadcode-2518; yt:MqH3GVfjfBc 5:59; creator-rule]
- Before baking from refrigerated stiff starter, feed 10 g starter with 20 g water and 40 g flour (1 : 2 : 4, 50 % hydration). — `{"starter_g": 10, "water_g": 20, "flour_g": 40, "hydration_pct": 50}` [breadcode-2522; yt:MqH3GVfjfBc 7:13; creator-rule]
- A liquid starter (1 part flour to 5 parts water) makes no acetic acid, since acetic acid needs oxygen, and so keeps a dairy profile. — `{"hydration_pct": 500}` [breadcode-2526; yt:nPSsWa4nGgU 1:52; creator-rule]
- At the same volume increase, a stiff-starter dough (50 % hydration) has the highest pH and is the least acidic of the three. — `{"hydration_pct": 50}` [breadcode-2528; yt:nPSsWa4nGgU 3:04; measured]
- A stiff starter is more forgiving with lower-gluten flour and gives a more open, fluffier crumb. [breadcode-2531; yt:nPSsWa4nGgU 4:54; creator-rule]
- Converting a liquid starter to a regular or stiff starter keeps its mild dairy profile. [breadcode-2534; yt:nPSsWa4nGgU 7:17; anecdote]
- A stiff sourdough starter uses much less water than a regular starter, which lets the dough ferment for a much longer time and gives more flavour (his view). [breadcode-2559; yt:0h63OloQQd8 0:35; creator-rule]
- A stiff starter has 50 % hydration: two parts flour to one part water by weight. — `{"hydration_pct": 50}` [breadcode-2561; yt:MqH3GVfjfBc 2:25; creator-rule]
- In a single test, soaking a stiff starter for 20 min in plain water or in water with about 2 % sugar did not measurably change its pH (about 4.3 before, 4.26–4.27 after). — `{"soak_min": 20, "ph_before": 4.3, "ph_after": [4.26, 4.27], "sugar_pct_of_water": 2}` [breadcode-2571; yt:NCHTg1NHv3E 4:26; measured]
- The 'water bath makes starter milder' idea was not supported by this test; the creator found no clear winner between plain water and sugar water. [breadcode-2572; yt:NCHTg1NHv3E 5:01; measured]
- The soak water itself drifted to about pH 6.8 (plain) and 6.5 (sugar); the creator suspects an osmotic effect of sugar but calls the difference possibly random. — `{"soak_min": 20}` [breadcode-2573; yt:NCHTg1NHv3E 3:50; measured] *(low confidence)*
- A stiffer starter favours yeast over bacteria, so it can replace the yeast in yeast-based recipes, at the cost of less tang in the dough. [breadcode-2574; yt:NCHTg1NHv3E 1:51; creator-rule]
- A stiff starter about 12 h after feeding measured pH 4.3 in this test. — `{"since_feed_h": 12, "ph": 4.3}` [breadcode-2575; yt:NCHTg1NHv3E 1:16; anecdote] *(low confidence)*
- Before soaking, the plain water used in the stiff-starter soak test measured about pH 8.1; after 20 min it had dropped to about pH 6.5-6.8. — `{"water_ph_before": 8.1, "soak_min": 20, "water_ph_after": [6.5, 6.8]}` [breadcode-2662; yt:NCHTg1NHv3E 1:16; measured]
- Stiff, recently fed starter at 20 % of flour weight (80 g on 400 g flour) was used. — `{"starter_pct_of_flour": 20}` [breadcode-2687; yt:0oKA_iTytiw 1:21; creator-rule]
- A stiff sourdough starter (40 g per 200 g flour, about 20 % of flour) in a pasta dough gives a better balance of yeast and bacteria, so the pasta is only mildly sour. — `{"starter_pct_of_flour": 20, "starter_g": 40, "flour_g": 200}` [breadcode-2782; yt:A5blYerC8VE 0:34; creator-rule]
- A stiff starter favours yeast over lactic bacteria, giving a milder bread with more leavening power. [breadcode-2882; yt:l5FtMycJna4 description; creator-rule]
- Converting regular starter to liquid, then to stiff, removed vinegary notes and left dairy-fruity notes (his tasting). [breadcode-2883; yt:l5FtMycJna4 description; anecdote] *(low confidence)*
- Three starter hydrations from the same liquid starter (10 g) with 150 g flour: stiff 50 % (75 g water), regular 100 % (150 g water), liquid 300 % (450 g water). — `{"stiff_hydration_pct": 50, "regular_hydration_pct": 100, "liquid_hydration_pct": 300, "flour_g": 150, "starter_g": 10}` [breadcode-2911; yt:UYBJZ-xmRFM description; creator-rule]
- In a balloon test, the stiff starter inflated a balloon fastest, the regular starter second and the liquid starter last. [breadcode-2912; yt:UYBJZ-xmRFM description; measured]
- The stiff starter tasted milder than the regular or liquid starter (his tasting). [breadcode-2913; yt:UYBJZ-xmRFM description; anecdote] *(low confidence)*
- Lower starter hydration favours yeast; higher hydration favours lactic acid bacteria. Stiff starter gives fluffier bread; high-hydration starter gives more tang. [breadcode-2915; yt:UYBJZ-xmRFM description; creator-rule]
- For a stiff starter use about 60 g water per 100 g flour (about 60 % hydration) instead of 100 g. — `{"hydration_pct": 60, "flour_g": 100, "water_g": 60}` [breadcode-2958; yt:40s3HVk462Y 0:00; creator-rule]
