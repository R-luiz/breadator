# yeast — The Bread Code / The Sourdough Framework

Claims: 58 (book 26, videos 28, repos 4). Evidence: creator-rule 41, literature 8, anecdote 5, extrapolation 3, measured 1.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

Fresh : dry yeast = 3 : 1 by weight throughout. Fast doughs ≈1 % dry (3 % fresh); slow/overnight 0.05–0.25 % dry; pizza calculator 0.05 % dry at ≈22 °C, double below 20 °C. Use less yeast rather than an autolyse. History: commercial yeast shortened leavening from ≈10 h to ≈1 h (Ch 1).

## Claims

### yeast

- In 1857 Louis Pasteur showed that yeast microorganisms, not chemical catalysts, cause alcoholic fermentation, and he was the first to isolate and grow pure yeast strains. [breadcode-0003; SF Ch 1 The history of sourdough; creator-rule]
- In 1868 the Fleischmann brothers patented pure yeast strains for bread making, isolated from sourdough batches; by 1879 machinery multiplied yeast in large centrifuges. [breadcode-0004; SF Ch 1 The history of sourdough; literature]
- Pure commercial yeast cut leavening time: a dough that took about 10 hours to leaven could be leavened within about 1 hour. — `{"bulk_before_h": 10, "bulk_after_h": 1}` [breadcode-0005; SF Ch 1 The history of sourdough; literature]
- After World War II packaged dry yeast was developed; with pure yeast, fermentation is reproducible if temperature, flour and yeast strain stay the same. [breadcode-0006; SF Ch 1 The history of sourdough; creator-rule]
- Brewer's yeast used alone in doughs (e.g. Kaiser Semmeln) gives fluffy bread without the acidity of sourdough. [breadcode-0008; SF Ch 1 The history of sourdough; creator-rule]
- Yeasts are single-celled fungi that reproduce by budding or by forming spores; the spores are very resistant to external factors. [breadcode-0047; SF Ch 2 How sourdough works; creator-rule]
- About 1500 yeast species have been identified. [breadcode-0048; SF Ch 2 How sourdough works; literature]
- Unlike mould, yeasts do not ordinarily form a mycelium network. [breadcode-0049; SF Ch 2 How sourdough works; literature]
- Yeast is saprotrophic: it cannot make its own food and instead breaks down external carbohydrates into carbon dioxide and alcohol, which is fermentation. [breadcode-0050; SF Ch 2 How sourdough works; creator-rule]
- With oxygen present, yeast produces almost exclusively carbon dioxide and water; without oxygen its metabolism shifts to producing alcoholic compounds. [breadcode-0051; SF Ch 2 How sourdough works; literature]
- Yeast temperature range depends on the species: Leucosporidium frigidum grows from about -2 °C to 20 °C, while other species prefer higher temperatures. — `{"temp_c": [-2, 20]}` [breadcode-0052; SF Ch 2 How sourdough works; creator-rule]
- In general, the warmer the environment, the faster the yeast's metabolism. [breadcode-0053; SF Ch 2 How sourdough works; creator-rule]
- A starter yeast works best within the temperature range where its grain was grown and harvested; a yeast from a cold-climate rye likely prefers colder conditions. [breadcode-0054; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- A beneficial yeast from cold caves near Pilsen (Czech Republic) produces good beers at lower temperatures and gave rise to strains now used for lagers. [breadcode-0055; SF Ch 2 How sourdough works; creator-rule]
- Yeasts are common on cereal grains, fruit, plants, soil and in the gut; baking yeasts are cultivated on plant leaves, and their ecology is poorly known. [breadcode-0056; SF Ch 2 How sourdough works; creator-rule]
- Baking yeasts are epiphytes: they live on the plant surface, feeding on rain water, air, insects or aphid honeydew, without penetrating the cell wall. [breadcode-0058; SF Ch 2 How sourdough works; creator-rule]
- Plants release sugars, organic and amino acids, methanol and salts onto the leaf surface, which attracts epiphytes; some research supports this. [breadcode-0059; SF Ch 2 How sourdough works; literature]
- Epiphytes protect their host plant against mould and other pathogens, so it is in their interest to keep the plant alive. [breadcode-0060; SF Ch 2 How sourdough works; literature]
- Yeasts are being researched as food-safe biocontrol agents to shield plants from mould, which could help vineyards with mildew. [breadcode-0061; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- Ergot fungus contaminates rye; lawmakers have reduced the allowed ergot in rye flour because of its toxicity to the liver, and the author suggests yeasts might help mitigate it. [breadcode-0062; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- In a grape experiment, wounds inoculated with mould and then with some of 150 wild yeast strains isolated from leaves sustained no significant damage, while mould-only wounds did. [breadcode-0063; SF Ch 2 How sourdough works; literature]
- Brewer's yeast can act as an aggressive pathogen on grapevines after heavy vine damage, producing hyphae that penetrate the tissue. [breadcode-0064; SF Ch 2 How sourdough works; creator-rule]
- Fermentation is the metabolism by which yeast and bacteria convert sugars into alcohol or acids; in bread it produces carbon dioxide that raises the dough. [breadcode-0836; SF Glossary > Fermentation; creator-rule]
- Yeast ferments dough sugars into alcohol, carbon dioxide and heat, which makes the dough rise. [breadcode-0885; SF Glossary > Yeast; creator-rule]
- Yeast viability test: dissolve a spare portion of yeast in warm sugared water; if it is not bubbly after about 2 h, the yeast is likely dead. — `{"test_time_h": 2}` [breadcode-1109; repo:the-bread-code basics/common-problems-faq.md; creator-rule]
- With melted chocolate, use cold water so the hot chocolate does not harm the yeast. [breadcode-1201; repo:the-bread-code recipes/sweet/full-milk-chocolate-bread.md; creator-rule]
- Yeast converts sugar to gas, which makes a chocolate loaf fluffy. [breadcode-1202; repo:the-bread-code recipes/sweet/full-milk-chocolate-bread.md; creator-rule]
- Below about 20 °C, the README recommends doubling the yeast. — `{"temp_threshold_c": 20, "yeast_multiplier": 2}` [breadcode-1209; repo:pizza-dough README.md; creator-rule]
- With instant yeast, about 1 tsp per 1,000 g of flour is enough to slow a dough down so it can ferment over up to three days; a packet or a tablespoon per the same amount ferments far too fast. — `{"instant_yeast_tsp_per_kg_flour": 1, "ferment_days": [1, 3]}` [breadcode-1430; yt:WxFE3bxrTSM 16:04; also 0:00; creator-rule]
- Fresh and dry yeast are the same organism; he cites a ratio of one to three. He does not say which side is which, so the usual convention (fresh yeast about three times the weight of dry) is an inference. — `{"fresh_to_dry_ratio": 3}` [breadcode-1444; yt:l0nAMbbYziI 2:24; creator-rule]
- For a long ferment use only a very small amount of yeast; if your scale cannot weigh under 1 g, use 1 g. — `{"yeast_g_min": 1}` [breadcode-1445; yt:l0nAMbbYziI 4:38; creator-rule]
- The yeast-based ciabatta used 0.1 % dry yeast by flour weight (0.4 g per 400 g flour) for a slow ferment. — `{"dry_yeast_pct": 0.1, "dry_yeast_g": 0.4}` [breadcode-1491; yt:HSdOUZq5i6U 5:25; 10:25; creator-rule]
- Fresh yeast is used at three times the weight of dry yeast. — `{"fresh_to_dry_ratio": 3}` [breadcode-1492; yt:HSdOUZq5i6U 5:25; 11:00; creator-rule]
- Too much yeast ferments the dough faster than the flour can be broken down; keep yeast minimal for a slow ferment and better flavour. [breadcode-1493; yt:HSdOUZq5i6U 10:25; 11:00; creator-rule]
- Use 1 % dry yeast or 3 % fresh yeast of flour weight; fresh yeast is about three times the dry weight. — `{"dry_yeast_pct": 1, "fresh_yeast_pct": 3}` [breadcode-1609; yt:0kFYyj3WKpE 5:55; creator-rule]
- A yeast-only bulk took about an hour, while the sourdough bulk for the same croissant dough took around 5 hours. — `{"bulk_yeast_h": 1, "bulk_sourdough_h": 5}` [breadcode-1635; yt:dY9GeoZX8DY 13:56; anecdote] *(low confidence)*
- Dry yeast in a poolish is about 1 % of the poolish weight (0.8 g for 80 g, rounded up to 1 g) because the poolish multiplies overnight, so little yeast is needed. — `{"poolish_g": 80, "dry_yeast_pct_of_poolish": 1, "dry_yeast_g": 1}` [breadcode-1823; yt:B_2SEmyyvso 3:22; creator-rule]
- Fresh yeast can replace dry yeast at three times the weight; both are the same organism. — `{"ratio": 3}` [breadcode-1854; yt:1Dbbfq5SKi0 2:24; creator-rule]
- The amount of dry yeast sets how fast the dough ferments, so the dose can be changed to speed up or slow down the schedule; this bread used about 1 % of flour in dry yeast. — `{"yeast_pct_of_flour": 1, "yeast_g": 10}` [breadcode-1855; yt:1Dbbfq5SKi0 2:24; creator-rule] *(low confidence)*
- Dry yeast is about 1% of flour weight in this video; fresh yeast is replaced by dry at about one third of the fresh weight (creator's factor of three). Overnight pizza doughs may use as little as 0.1 g. — `{"dry_yeast_pct_flour": 1, "fresh_to_dry_factor": 3, "overnight_pizza_yeast_g": 0.1}` [breadcode-1882; yt:TUkPI34Go7k 10:10; creator-rule]
- A very small amount of fresh yeast is used. [breadcode-2129; yt:EI4RoemHe5M [5:44]; creator-rule] *(low confidence)*
- A yeast-only dough with about 0.5 g dry yeast per roughly 400 g flour had a fluffy crumb and good browning but a neutral, non-acidic taste. — `{"dry_yeast_g": 0.5, "flour_g": 400}` [breadcode-2162; yt:bJxpCVc2wYE 10:34; measured]
- A very small yeast dose (about 0.5 g dry yeast in roughly 400 g flour) gives a deliberately slow fermentation, which the creator calls the key to a good yeast-only dough. — `{"dry_yeast_g": 0.5, "flour_g": 400}` [breadcode-2163; yt:bJxpCVc2wYE 2:25; creator-rule]
- Fresh and dry yeast can be swapped at 1 g dry for 3 g fresh, since they are the same organism with different water content. — `{"fresh_per_dry_ratio": 3}` [breadcode-2269; yt:-rwuxy5KS6Q 2:25; creator-rule]
- For a fast pizza dough, use about 1 % dry yeast (or 3 % fresh) of the flour weight; very small yeast amounts ferment very slowly. — `{"dry_yeast_pct": 1, "fresh_yeast_pct": 3}` [breadcode-2270; yt:-rwuxy5KS6Q 1:50; creator-rule]
- Water at about room temperature is recommended; warming it to about 40 °C would make the dough ferment faster. — `{"water_temp_c": 40}` [breadcode-2276; yt:-rwuxy5KS6Q 3:00; creator-rule]
- A dough in which all water was replaced by about 300 g of champagne (about 36 g of alcohol) did not visibly rise during the first wait; its sample only rose after about two days. — `{"liquid_g": 300, "alcohol_g": 36, "days_to_sample_rise": 2}` [breadcode-2474; yt:v85zeRckFt0 1:49; anecdote]
- A rum dough with the same ~36 g of alcohol (96 g rum + 204 g water, no champagne) also did not rise, which points to the alcohol rather than sulfites. — `{"rum_g": 96, "water_g": 204, "alcohol_g": 36, "rum_abv_pct": 37.5}` [breadcode-2475; yt:v85zeRckFt0 3:44; anecdote]
- A dough made with beer (about 5 % alcohol) fermented more slowly but rose well and gave an open, lightly spring crumb. — `{"beer_abv_pct": 5}` [breadcode-2476; yt:v85zeRckFt0 4:19; anecdote]
- Keeping total liquid alcohol near 4.2 % of the liquid worked: 112 g champagne plus 208 g water (320 g liquid, 13.4 g alcohol) on 400 g flour rose, and the dough was retarded about 12 hours before baking. — `{"liquid_g": 320, "alcohol_pct_of_liquid": 4.2, "alcohol_g": 13.44, "champagne_g": 112, "water_g": 208, "flour_g": 400, "retard_h": 12}` [breadcode-2477; yt:v85zeRckFt0 5:31; anecdote]
- Yeast dosing should be very low: about 1 g dry or 3 g fresh yeast per kg flour, for a slow fermentation. — `{"dry_yeast_g_per_kg_flour": 1, "fresh_yeast_g_per_kg_flour": 3}` [breadcode-2626; yt:ep0mua88Sa8 2:23; creator-rule]
- Adding extra yeast to a poolish to shorten its time works, but is not recommended; the poolish alone takes longer. [breadcode-2802; yt:oJs8FrlZQB0 1:49; creator-rule]

### yeast.dosing

- Very little yeast combined with a fermentation of at least 8 h (Neapolitan-style) gives a much fluffier, more open crumb. — `{"fermentation_min_h": 8}` [breadcode-0413; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- The author's default pizza dough uses about 150 mg dry yeast per kg of flour (0.015 % of flour weight, computed). — `{"yeast_mg_per_kg_flour": 150}` [breadcode-0414; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- He states that EU rules allow about 0.2 % dry yeast in bread labelled sourdough, and that his own bread used more yeast. — `{"dry_yeast_pct_of_flour": 0.2}` [breadcode-2713; yt:lG9ERQFFn5I 4:25; creator-rule] *(low confidence)*
- For a long overnight ferment, very little dry yeast (about 0.25 % of flour) is enough to double the dough. — `{"yeast_pct": 0.25}` [breadcode-2716; yt:ziMMCphF054 0:40; creator-rule]
- Dry yeast is used at about one third of the fresh-yeast amount (speaker's conversion). — `{"ratio": "1:3 dry:fresh"}` [breadcode-2717; yt:ziMMCphF054 1:20; creator-rule] *(low confidence)*

### yeast.dosing-time

- Adding a little more yeast can shorten the full process by about an hour; a little sugar also speeds it up, which he does not like. — `{"time_saved_h": 1}` [breadcode-1651; yt:0kFYyj3WKpE 26:52; creator-rule] *(low confidence)*
