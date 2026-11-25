# enzymes — The Bread Code / The Sourdough Framework

Claims: 77 (book 50, videos 26, repos 1). Evidence: creator-rule 64, literature 6, extrapolation 4, anecdote 3.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

Amylases release sugars (sweeter dough over a long autolyse), proteases weaken gluten — the clock that limits very low inoculations and long ferments; whole grain carries more enzymes. Malt (diastatic) adds amylase; heat-inactivation around 70 °C per Ch 2. See `mixing.md` (fermentolysis) and `bulk.md`.

## Claims

### enzymes

- When fermentation is sped up, bread flavour deteriorates; the sprouting enzyme activity that gives a fluffier texture and better crust takes time and cannot be accelerated indefinitely. [breadcode-0009; SF Ch 1 The history of sourdough; creator-rule]
- A dry seed is dormant and can be stored for years; contact with water starts germination, and the enzymes inside the seed convert stored nutrients into a form the growing plant can use. [breadcode-0016; SF Ch 2 How sourdough works; creator-rule]
- Amylase and protease are the two main enzymes driving sourdough fermentation. [breadcode-0021; SF Ch 2 How sourdough works; creator-rule]
- In a grain, most enzymes sit in the outer parts of the hull and are activated first when water arrives, which explains why whole-wheat flour gives a faster enzymatic response. [breadcode-0044; SF Ch 2 How sourdough works; literature]
- Fermentation speed, governed by enzyme and microbial activity, controls crumb softness, crust colour and how much gluten remains in the final bread. [breadcode-0046; SF Ch 2 How sourdough works; creator-rule]
- Flour enzymes break down starch and gluten over time; with a low inoculation rate (about 1 %), the starter can take about 24 h to regrow inside the bread dough. — `{"starter_pct_of_flour": 1, "regrowth_h": 24}` [breadcode-0180; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Whole-wheat flour has higher enzymatic activity, so adding some speeds up fermentation. [breadcode-0384; SF Ch 7 Wheat sourdough §7.3 Ingredients; creator-rule]
- Enzymes sweeten oats when oat milk is left to rest (analogy for autolysis). [breadcode-0442; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Whole-grain flour speeds up the effects of resting because the hull contains more enzymes. [breadcode-0443; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- Enzyme reaction rates are assumed to rise with temperature until the enzymes are denatured by heat; the author has not seen studies on this. [breadcode-0447; SF Ch 7 Wheat sourdough §7.6 Autolysis; extrapolation] *(low confidence)*
- Enzymatic malt adds malty flavour and improves enzymatic activity, especially in quicker yeast doughs. [breadcode-0726; SF Ch 9 Mix-ins §9.2 Examples > Flours; creator-rule]
- Alpha-amylase cuts starch molecules into shorter fragments, producing maltose and some glucose. [breadcode-0801; SF Glossary > Alpha-amylase; creator-rule]
- Amylase breaks starch down into simpler sugars that feed fermentation; in brewing, the mash is held at set temperatures for an extended time so most starch is converted to sugar. [breadcode-0804; SF Glossary > Amylase; creator-rule]
- Beta-amylase breaks the starch fragments produced by alpha-amylase down into maltose. [breadcode-0821; SF Glossary > Beta-amylase; creator-rule]
- Diastatic malt is dried, ground malted grain whose enzymes break starch into sugars, which can improve bread flavour, colour and shelf life. [breadcode-0829; SF Glossary > Diastatic Malt; creator-rule]
- Maltose is a sugar produced when amylases break down starch; it is a primary food source for yeast during fermentation. [breadcode-0854; SF Glossary > Maltose; creator-rule]
- Non-diastatic malt is dried at higher temperatures, which deactivates its enzymes; it is used mainly for flavour and colour. [breadcode-0855; SF Glossary > Non-diastatic Malt; creator-rule]
- In a wet dough the flour's own amylases keep working well above 50 °C. — `{"temp_c": [50, null]}` [breadcode-0856; SF Glossary > Non-diastatic Malt; literature]
- Flour amylases in a wet dough stay stable up to about 70 °C. — `{"temp_c": 70}` [breadcode-0857; SF Glossary > Non-diastatic Malt; literature]
- Flour amylases in a wet dough are inactivated as the dough heats further during baking. [breadcode-0858; SF Glossary > Non-diastatic Malt; literature]
- Protease breaks proteins, including gluten, into peptides and amino acids; moderate activity makes dough more extensible, while excessive activity weakens the gluten network and gives slack, sticky dough with poor volume. [breadcode-0870; SF Glossary > Protease; creator-rule]
- Protease activity rises with longer fermentation and with higher dough temperature. [breadcode-0871; SF Glossary > Protease; creator-rule]
- In sourdough, the acidic conditions activate cereal proteases, and flour from sprouted or malted grains has higher protease activity. [breadcode-0872; SF Glossary > Protease; creator-rule]
- Gluten also degrades on its own from the moment flour and water are mixed, because protease converts the gluten network into amino acids. [breadcode-0964; SF Ch 12 Troubleshooting §12.2.4 My dough completely tears after a long f; creator-rule]
- Flour amylase and protease enzymes break down starch and gluten during long fermentation, which affects the dough. [breadcode-1207; repo:pizza-dough README.md; creator-rule]
- If a dough or starter ferments for too long, enzymes break down the flour, so the dough becomes stickier. [breadcode-1436; yt:ZTHHI0iuamA 4:29; creator-rule]
- Flour enzymes activate when water is mixed in and start breaking down flour during a long rest; this adds flavour and makes the dough easier for yeast to digest. [breadcode-1442; yt:l0nAMbbYziI 1:50; 2:24; extrapolation]
- The author says the long rest breaks down the flour, which made the autolysed dough ferment faster and taste more sour; he did not measure this. [breadcode-1481; yt:JZBtmQXKWA8 19:58; 29:48; extrapolation]
- Whole-wheat flour contains considerably more enzymes than bread flour; adding a little of it speeds up fermentation. — `{"whole_wheat_g": 200}` [breadcode-1554; yt:36GdGT0XYwE 1:50; creator-rule]
- Malt extract at about 1 % of flour weight adds malt flavour; a small amount of yeast (about 0.25 %) was also used. — `{"malt_pct": 1, "yeast_pct": 0.25}` [breadcode-1622; yt:dY9GeoZX8DY description; creator-rule]
- Resting flour and water before adding leaven lets amylase and protease act; the dough becomes more extensible and needs less kneading. [breadcode-1802; yt:TUkPI34Go7k 2:12; anecdote]
- Autolysis that runs too long breaks gluten down so far that the dough loses structure; sourdough is affected more. [breadcode-1803; yt:TUkPI34Go7k 5:02; creator-rule]
- Autolysis length depends on flour protein, temperature, enzyme content and the crumb wanted. [breadcode-1804; yt:TUkPI34Go7k 5:35; creator-rule]
- For a dough with about 1% dry yeast (of flour weight), the creator suggests an autolysis of about 6 to 10 hours. — `{"dry_yeast_pct_flour": 1, "autolysis_h": [6, 10]}` [breadcode-1805; yt:TUkPI34Go7k 6:43; creator-rule]
- Active barley malt (about 2 % of flour in this bread) adds enzymes that convert starch into sugars, improving browning, and break down gluten, giving a softer crumb; the creator says resting flour and water for a few hours does much the same. — `{"malt_g": 20, "malt_pct_of_flour": 2}` [breadcode-1853; yt:1Dbbfq5SKi0 3:01; creator-rule]
- In a warm sourdough kitchen (above about 25 °C, main fermentation about 5 h), the creator suggests an autolysis of about 2 to 3 hours. — `{"temp_c_min": 25, "main_ferment_h": 5, "autolysis_h": [2, 3]}` [breadcode-1879; yt:TUkPI34Go7k 7:51; creator-rule]
- In cold conditions, sourdough can skip autolysis; a long room-temperature rest on top of a slow ferment may have negative effects, because the gluten breaks down further. [breadcode-1880; yt:TUkPI34Go7k 7:18; creator-rule]
- At about 22 °C the creator saw an autolysis limit of about 12 hours, and about 10 hours at about 25 °C; a stronger flour allows a slightly longer limit. — `{"temp_c": 22, "autolysis_limit_h": 12}` [breadcode-1881; yt:TUkPI34Go7k 6:43; creator-rule]
- During a long fermentation, amylase breaks starch down into sugars and protease breaks gluten down, which is why a long fermentation gives a more open crumb. [breadcode-1943; yt:G0nVQy-L3Bk 4:55; creator-rule]
- Amylase and protease degrade flour over time; a too-long fermentation harms the dough. [breadcode-2055; yt:0aylYlvA4K8 [2:13]–[2:47]; creator-rule]
- Enzymes and bacteria break down gluten over time. [breadcode-2104; yt:_cb4vCQ2shk [11:31]–[12:07]; creator-rule]
- Flour enzymes, notably protease, are activated by water, so dough degrades over time and the starter should be active enough to use that window. [breadcode-2339; yt:-6dlWqFlFSo 4:32; creator-rule]

### enzymes.amylase

- Amylase breaks complex starch molecules into simple, digestible sugars; in the grain the germ uses amylase to convert starch into sugars for growth. [breadcode-0022; SF Ch 2 How sourdough works; creator-rule]
- Maltose freed from starch stays largely hidden inside the germ and is not consumed by the microorganisms on the grain surface until grinding exposes it. [breadcode-0023; SF Ch 2 How sourdough works; creator-rule]
- Higher temperature speeds up the amylase reaction that converts starch to sugar. [breadcode-0024; SF Ch 2 How sourdough works; creator-rule]
- Amylase needs time to convert most of the starch into simple sugars; these sugars feed the yeast and the Maillard browning reaction in the oven, which is why a long fermentation matters for flavour and colour. [breadcode-0025; SF Ch 2 How sourdough works; creator-rule]
- Brewers keep mash at controlled temperatures so that the different amylases convert starch into sugar. [breadcode-0026; SF Ch 2 How sourdough works; literature]
- The iodine starch test (used in brewing) indicates unconverted starch when the sample turns blue or black after adding iodine. [breadcode-0027; SF Ch 2 How sourdough works; creator-rule]
- Whether the iodine starch test works for bread dough is unknown; the author has not established it. [breadcode-0028; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- Amylase in the flour is what breaks down starch when flour is wetted, turning it into sugars the microbes can use. [breadcode-0102; SF Ch 3 Making a sourdough starter; creator-rule]
- Amylase breaks starch into sugars, which feed yeast and bacteria and speed fermentation. [breadcode-1315; yt:KRO9EcIyXSI 25:39; creator-rule]

### enzymes.amylases

- Amylases convert starch into simpler sugars, which brown better during baking. [breadcode-0416; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Enzyme activity also speeds up with temperature: amylase and protease work faster, releasing more sugars and degrading gluten. [breadcode-1089; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- During a long wet fermentation, enzymes in the flour convert starch into sugars, so the dough weakens and stops holding together as well. [breadcode-2222; yt:ngcISK7ZDjc 5:54; creator-rule] *(low confidence)*
- Amylases and proteases are the key enzymes in bread and pizza making; amylases convert starch into sugars. [breadcode-2275; yt:-rwuxy5KS6Q 4:44; creator-rule]

### enzymes.cold-proof

- Cold proofing changes the enzyme activity and therefore the flavour of the bread compared with room-temperature proofing. [breadcode-1910; yt:b7Pd7UcZxd0 10:07; creator-rule] *(low confidence)*

### enzymes.malt

- Industrial bakers add enzyme-rich malted flour to speed up fermentation; if malt appears in a bread's ingredient list, this strategy was probably used. [breadcode-0029; SF Ch 2 How sourdough works; creator-rule]
- Enzymatically active malt is not heated above about 70 °C, the temperature above which amylases begin to degrade; heat-treated inactive malt has no effect on the flour. — `{"temp_c": 70}` [breadcode-0030; SF Ch 2 How sourdough works; creator-rule]
- Industrial bakeries add active malt, which carries more enzymes and shortens dough time; malt is often an ingredient in supermarket bread. [breadcode-0411; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Active malt at about 1 % of flour brings enzymes that convert starch to fermentable sugar, which speeds up the fermentation. — `{"malt_pct": 1}` [breadcode-2272; yt:-rwuxy5KS6Q 3:35; creator-rule]
- Inactive malt is extracted at high temperature and loses its enzymes; choose active malt for speed. [breadcode-2273; yt:-rwuxy5KS6Q 4:10; creator-rule]
- If you have no malt, about 2 % whole wheat flour of the total flour gives a similar enzymatic speed-up. — `{"whole_wheat_pct": 2}` [breadcode-2274; yt:-rwuxy5KS6Q 5:59; creator-rule]

### enzymes.protease

- Protease breaks complex proteins into simpler proteins and amino acids; in wheat dough its main target is gluten. [breadcode-0031; SF Ch 2 How sourdough works; creator-rule]
- Adding water to flour activates protease, which in sprouting grain breaks down gluten to supply amino acids to the germ. [breadcode-0032; SF Ch 2 How sourdough works; creator-rule]
- A wheat dough left at room temperature for several days can lose its gluten network: it tears, holds no structure and is no longer suitable for baking. [breadcode-0033; SF Ch 2 How sourdough works; creator-rule]
- In a dough made from a dried starter, a fermentation that stayed slow for 3 to 4 days let the gluten break down to the point of failure; the author attributes this to protease. — `{"duration_h": [72, 96]}` [breadcode-0034; SF Ch 2 How sourdough works; anecdote]
- A fast-proofing dough (1 to 2 hours with a large amount of active dry yeast) baked dense and not fluffy because protease was not given enough time. — `{"bulk_h": [1, 2]}` [breadcode-0035; SF Ch 2 How sourdough works; creator-rule]
- Proteolysis breaks some gluten bonds, which makes it easier for yeast to inflate the dough; a long fermentation is therefore critical for a fluffy, open crumb in sourdough. [breadcode-0037; SF Ch 2 How sourdough works; creator-rule]
- Protease accelerates proteolysis by breaking gluten into smaller, more easily metabolised amino acids. [breadcode-0079; SF Ch 2 How sourdough works; creator-rule]
- Protease breaks down storage proteins; over long fermentation it weakens the dough until it can no longer hold gas. [breadcode-1316; yt:KRO9EcIyXSI 26:13; creator-rule]

### enzymes.proteases

- A fast yeast dough (1 % dry yeast) is never as open as a sourdough dough, because protease cannot act in such a short fermentation. — `{"yeast_dry_pct_of_flour": 1}` [breadcode-0410; SF Ch 7 Wheat sourdough §7.4 Hydration; anecdote]
- A fermentation of about 1 h breaks down little gluten and gives a dense, not fluffy, crumb. — `{"fermentation_h": 1}` [breadcode-0412; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- With about 1 % starter, flour can break down before the desired fermentation stage, because protease acts before the microbes reproduce enough. — `{"starter_pct_of_flour": 1}` [breadcode-0431; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Over a day of flour and water resting, dough becomes more extensible hour by hour and tastes sweeter, as protease and amylase act. — `{"rest_h": 24}` [breadcode-0441; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- In that short 5 h, enzymes have not broken down enough flour: the dough is less elastic and too few sugars are released for flavour. — `{"bulk_h": 5}` [breadcode-0446; SF Ch 7 Wheat sourdough §7.6 Autolysis; creator-rule]
- During over-fermentation proteases break down flour gluten, and bacteria consume gluten by proteolysis. [breadcode-1027; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Overferme; literature]
- Fermentation has an upper time limit: he says flour and water left at room temperature for about two days start to break down the gluten, because the seeds try to sprout and convert starch and gluten. — `{"rt_days": 2}` [breadcode-2533; yt:nPSsWa4nGgU 6:42; creator-rule]
