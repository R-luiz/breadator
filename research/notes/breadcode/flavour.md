# flavour — The Bread Code / The Sourdough Framework

Claims: 55 (book 21, videos 31, repos 3). Evidence: creator-rule 39, anecdote 10, extrapolation 3, measured 3.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Claims

### flavour

- Adding sourdough discard to rye dough can be done in any amount and gives a more complex flavour. [breadcode-0345; SF Ch 8 Non wheat sourdough; creator-rule]
- Long fermentation (8 h or more) of yeast dough improves flavour and gives a crisper crust. — `{"fermentation_min_h": 8}` [breadcode-0415; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Too little steam keeps scoring incisions from opening, because the crust cannot expand. [breadcode-0671; SF Ch 10 Baking §10.2 The role of steam; creator-rule]
- Flavour mix-ins (for example rye, corn, spices, sugar) significantly change the taste. [breadcode-0715; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Dark stout can replace water in sourdough and gives a hearty malty taste. [breadcode-0732; SF Ch 9 Mix-ins §9.2 Examples > Liquids; creator-rule]
- Blueberries can be added as skins pressed through a sieve to remove the juice, or as raw blueberries. [breadcode-0751; SF Ch 9 Mix-ins §9.2 Examples > Spices and flavor mix-ins; creator-rule]
- Highlight mix-ins are mostly chunks that add strong contrast; use only one, or at most two, per loaf and combine with a flavour or flour mix-in if desired. — `{"highlights_max_per_loaf": 2}` [breadcode-0752; SF Ch 9 Mix-ins §9.2 Examples > Highlights; creator-rule]
- Highlight chunks listed: chocolate chunks or drops, black garlic, cheese (cheddar, feta), cornflakes, dried fruits (cranberries, dates, raisins), olives, pickled pepperoni and sun-dried tomatoes. [breadcode-0753; SF Ch 9 Mix-ins §9.2 Examples > Highlights; creator-rule]
- Spices and flavour mix-ins listed: browned onions, candied fruits (lemon, orange, pineapple), cinnamon, grated hard cheese, Mediterranean herbs, miso, molasses, sugar, spices (anise, fennel, coriander, cumin), and zests (lime, lemon, orange). [breadcode-0758; SF Ch 9 Mix-ins §9.2 Examples > Spices and flavor mix-ins; creator-rule]
- Combinations that complement each other: butter and milk with cinnamon and brown sugar added before shaping; cheddar with pepperoni or jalapeño; cocoa with cacao nibs and whole hazelnuts; cranberry with walnuts; semolina with Mediterranean herbs, olives and sun-dried tomatoes. [breadcode-0759; SF Ch 9 Mix-ins §9.2 Examples > Combinations; creator-rule]
- Acetic acid tastes pungent, so bread made with a very acetic starter tastes strongly acidic. [breadcode-0946; SF Ch 12 Troubleshooting §12.1.10 Why does my starter smell like vinegar ; creator-rule]
- The acetic smell is not harmful; to change the bread's flavour toward dairy, convert the starter to a liquid starter so lactic acid producers dominate. [breadcode-0948; SF Ch 12 Troubleshooting §12.1.10 Why does my starter smell like vinegar ; creator-rule]
- Overly sour bread can be made more pleasant by toasting it, which boils off additional acidity. [breadcode-0996; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Fermentation speed depends on temperature: warmer dough ferments faster, colder dough slower. [breadcode-1088; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- Dates darken the crumb. [breadcode-1197; repo:the-bread-code recipes/sweet/date-walnut-bread.md; creator-rule]
- Dates lose sweetness in the bake because yeast converts their sugar. [breadcode-1198; repo:the-bread-code recipes/sweet/date-walnut-bread.md; creator-rule] *(low confidence)*
- Crush walnuts before adding them. [breadcode-1199; repo:the-bread-code recipes/sweet/date-walnut-bread.md; creator-rule] *(low confidence)*
- The autolysed loaf tasted slightly tangier than the non-autolysed loaf. [breadcode-1480; yt:JZBtmQXKWA8 28:29; 29:08; measured] *(low confidence)*
- The author found both versions tasted great with no clear difference. An external tester (his parents' dog) went for the sourdough version first, then ate the yeast version, and was judged to like both equally. [breadcode-1502; yt:HSdOUZq5i6U 26:47; 27:21; 27:57; anecdote] *(low confidence)*
- As spoken: yeast activity stops below about −2 °C; lactic acid bacteria become much less active below about 20 °C; colder, longer fermentation tends to develop more flavour compounds, but this depends on the yeast present. — `{"yeast_stop_c": -2, "lab_slow_below_c": 20}` [breadcode-1604; yt:nQWlG1buMM8 20:55; creator-rule] *(low confidence)*
- A slightly under-fermented levain or dough (reported second-hand from a Tartine baker) favours sweet notes; longer, more heavily inoculated builds push the sourness up. [breadcode-1794; yt:l_PLhPWE-8E 16:45–17:20; anecdote] *(low confidence)*
- In the side-by-side yeast bake, the autolysed loaf came out larger and felt fluffier to the touch, and tasted better to the creator; in the bulk sample test the autolysed dough had risen more slowly. [breadcode-1808; yt:TUkPI34Go7k 19:38; measured] *(low confidence)*
- A liquid starter gave dairy and banana notes to the pizza. [breadcode-1850; yt:rcOxoJk7goM 2:21; anecdote] *(low confidence)*
- A longer ferment adds tang; in his tasting 25 °C and 30 °C ferments were not clearly different otherwise. [breadcode-1998; yt:vf5RbwlW8Xs 14:23; anecdote] *(low confidence)*
- Longer fermentation makes the bread more sour, but a longer, wetter dough is harder to shape and a loaf pan helps; a strong flour is needed because long fermentation breaks down gluten. [breadcode-2030; yt:m7J6CBZcq8o 4:26; creator-rule]
- A liquid starter gives a stronger dairy or yogurt note in bread than a regular starter. [breadcode-2207; yt:ES7x28d10pg description; creator-rule]
- The Mediterranean seasoning is 0.5% each of rosemary, basil, oregano, marjoram and thyme, i.e. 2.5% of dough weight in total. — `{"seasoning_total_pct": 2.5}` [breadcode-2405; yt:urHYzIktD9M description, seasonings; 4:40; creator-rule]
- The paprika seasoning is 3% paprika, 1% smoked paprika, 1% oregano, 1% thyme and 1% rosemary, i.e. 7% of dough weight in total. — `{"seasoning_total_pct": 7}` [breadcode-2406; yt:urHYzIktD9M description, seasonings; 4:40; creator-rule]
- The sweet and spicy chili seasoning is 2% paprika, 2% cayenne pepper and 4% brown sugar, i.e. 8% of dough weight in total. — `{"seasoning_total_pct": 8}` [breadcode-2407; yt:urHYzIktD9M description, seasonings; 4:40; creator-rule]
- The turmeric masala seasoning is 2% garam masala and 2% turmeric, i.e. 4% of dough weight in total. — `{"seasoning_total_pct": 4}` [breadcode-2408; yt:urHYzIktD9M description, seasonings; 4:40; creator-rule]
- An acidic starter suits dark rye bread; a mild, lactic starter suits a fluffy wheat bread (his stated preference). [breadcode-2423; yt:HgcGcG7rdZM 3:53; creator-rule]
- Some commercial bread contains inactivated sourdough spice, which adds a sourdough aroma without live fermentation. [breadcode-2426; yt:bVrSZzpVUnY 1:54; creator-rule]
- Homemade sourdough spice is made by collecting dried leftover starter over about a month, combining it, drying it flat on parchment and grinding it. — `{"collection_period_days": 30}` [breadcode-2427; yt:bVrSZzpVUnY 3:08; creator-rule]
- Drying the spice at 120 °C instead of 160 °C gave a spice that tasted clearly more sour. — `{"temp_c": 120}` [breadcode-2429; yt:bVrSZzpVUnY 5:11; measured]
- Hotter drying evaporates more of the acid, so the bake temperature sets how sour the finished spice is; a lower temperature for longer keeps more acid. — `{"temp_c": [120, 160]}` [breadcode-2430; yt:bVrSZzpVUnY 5:45; extrapolation]
- Sourdough spice can be added to a yeast bread at about 20 % of the flour weight (100 g spice per 500 g flour) without changing the rest of the recipe. — `{"spice_pct_of_flour": 20, "spice_g": 100, "flour_g": 500}` [breadcode-2432; yt:bVrSZzpVUnY 7:55; creator-rule]
- A champagne loaf had a crust with a slight tang and a soft crumb; the champagne aroma was subtle. [breadcode-2478; yt:v85zeRckFt0 7:37; anecdote]
- The crust tasted slightly floury; he thinks the dusting flour did not merge into the dough because the dough had frozen. [breadcode-2487; yt:Typy5Uzp-EM 5:04; anecdote]
- A loaf made with baking-powder-treated starter tasted much milder than his usual sourdough. [breadcode-2493; yt:c8GId0ByASo 7:15; anecdote] *(low confidence)*
- Some lactic acid evaporates during baking, so bread tastes less sour after baking than the raw dough does. [breadcode-2496; yt:c8GId0ByASo 2:56; extrapolation]
- Baked sourdough starter ground into a 'sourdough spice' has strong aroma; he mixed it into a Greek yogurt and buttermilk batter and suggests it as a pizza base or a flavour for quick yeast dough. [breadcode-2607; yt:rzakvUe7eIE 7:24; anecdote] *(low confidence)*
- In a single tasting by a neighbour, the cola-water bread did not taste sweet or of cola; he thinks the starter fermentation consumed the sugar. [breadcode-2692; yt:0oKA_iTytiw 5:02; anecdote]
- Commercial-style 'fake' sourdough: sourdough spice supplies the aroma while dry yeast does the leavening; the recipe is 100 g spice, 300 g cake flour, 280 g water, 8 g salt, 2 g dry yeast. — `{"spice_pct_of_flour": 33.3, "dry_yeast_pct_of_flour": 0.67, "hydration_pct": 93.3, "salt_pct": 2.67}` [breadcode-2709; yt:lG9ERQFFn5I description; creator-rule]
- The pasta dough is fermented slowly over about four weeks; the creator says this gives it a slight tang and good flavour. — `{"duration_h": 672}` [breadcode-2787; yt:A5blYerC8VE 0:00; creator-rule]
- Choose sourdough for a distinct sour taste and poolish for a strong yeasty flavour; both give similar dough properties. [breadcode-2794; yt:VlD2YuNq1w0 description; creator-rule]
- Add M&M-style inclusions at shaping rather than at mixing, otherwise the bread may not taste sweet. [breadcode-2985; yt:vqN2HIjHzJI description; creator-rule]

### flavour.ancient-grain-taste

- Einkorn tasted rye-like, a little more acidic and nutty. He says emmer tasted more like wheat bread (ASR unclear); the emmer was slightly more acidic than regular sourdough. [breadcode-1928; yt:PehhnV1lW2o 49:55-51:17; anecdote] *(low confidence)*

### flavour.colour

- Colour mix-ins (for example cocoa, squid ink, beetroot juice, tomato juice) significantly change the look. [breadcode-0716; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Cocoa can replace about 10 % of the flour for a black loaf and suits sweet toppings. — `{"cocoa_replaces_flour_pct": 10}` [breadcode-0727; SF Ch 9 Mix-ins §9.2 Examples > Flours; creator-rule]
- Natural colourants: activated charcoal (black), beetroot juice (red), blueberry juice (blue), butterfly pea flower (blue), carrot juice (orange), pear juice (pink), spinach juice (green), squid ink (black), strawberry juice (red), tomato juice (red). [breadcode-0734; SF Ch 9 Mix-ins §9.2 Examples > Colors; creator-rule]

### flavour.inclusion

- Chopped chocolate at about 20 % of flour weight (100 g on 500 g flour) can be mixed into a sourdough roll dough. — `{"chocolate_pct_of_flour": 20, "chocolate_g": 100}` [breadcode-2676; yt:9dsgUixpOBM description; creator-rule]

### flavour.mix-in-categories

- Small bits (seeds, whole spices) stay visible but are small enough to spread through the dough; chunks are larger and show up in occasional bites. [breadcode-0714; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
- Rule of thumb: about 10–20 % of flour weight for most mix-ins; about 1–5 % for spices. — `{"mixin_pct_of_flour": [10, 20], "spice_pct_of_flour": [1, 5]}` [breadcode-0721; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]

### flavour.sourness

- Longer baking lets more acetic acid leave with the steam while lactic acid stays, so a longer bake gives a less sour loaf. [breadcode-0656; SF Ch 10 Baking §10.1 The process of baking; extrapolation]

### flavour.texture

- Texture mix-ins change mouthfeel: cheese gives gummy texture, seeds give crunch, olives give squishy chunks. [breadcode-0717; SF Ch 9 Mix-ins §9.1 Categories; creator-rule]
