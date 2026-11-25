# sourdough-microbiology — The Bread Code / The Sourdough Framework

Claims: 206 (book 136, videos 69, repos 1). Evidence: creator-rule 132, measured 25, literature 20, anecdote 16, extrapolation 13.
Source of truth: `research/data/claims-breadcode.jsonl` (ids below). Book-derived lines are adapted from *The Sourdough Framework* by Hendrik Kleinwächter, CC BY-SA 4.0; this file inherits CC BY-SA for those lines. Video and repo lines are paraphrased facts. Generic baker guidance only.

## Synthesis

Yeast vs lactic/acetic bacteria balance is steered by hydration (stiff → yeast, liquid → bacteria, lactic/dairy), temperature and feeding ratio; acidity self-limits fermentation and protects against moulds; pH falls from ≈6 to ≈3.8–4.0 through the process (Table 7.4). Liquid starters cut off oxygen and favour lactic over acetic acid (lNCHzD8HxBA). Many mechanism claims in Ch 2 are the author's reasoning (labelled `extrapolation`, low confidence where speculative).

## Claims

### sourdough-microbiology

- In traditional sourdough, wild yeast provides the leavening (airy structure) while lactic acid bacteria mainly produce acidity, which also extends shelf life; the two are described as a symbiosis. [breadcode-0001; SF Ch 1 The history of sourdough; creator-rule]
- Traditional lactic acid bacteria decompose the flour-water mix and consume dough proteins; modern fast yeast-only breads lack this step and therefore contain more gluten. [breadcode-0012; SF Ch 1 The history of sourdough; literature]
- Yeast and bacteria cannot make their own food; once the enzymes are activated, the food they need becomes available and they can feed and multiply. [breadcode-0020; SF Ch 2 How sourdough works; creator-rule]
- Endophytes live inside plants without harm, helping the host against other pathogens, water and heat stress and nutrient availability; under stress they can turn pathogenic. [breadcode-0057; SF Ch 2 How sourdough works; literature]
- Bacteria outnumber yeast in sourdough by about 100 to 1. — `{"ratio": 100}` [breadcode-0065; SF Ch 2 How sourdough works; creator-rule]
- Yeast provides the leavening, while bacteria create the distinctive sourdough flavours through acidic by-products of their metabolism. [breadcode-0066; SF Ch 2 How sourdough works; creator-rule]
- Acids produced by sourdough bacteria can significantly extend the shelf life of sourdough bread. [breadcode-0067; SF Ch 2 How sourdough works; literature]
- Homofermentative lactic acid bacteria produce a single compound during fermentation: lactic acid. [breadcode-0070; SF Ch 2 How sourdough works; creator-rule]
- Heterofermentative lactic acid bacteria produce lactic acid plus acetic acid, ethanol and some carbon dioxide. [breadcode-0071; SF Ch 2 How sourdough works; creator-rule]
- Yeast and bacteria compete for the same food source, sugar. [breadcode-0072; SF Ch 2 How sourdough works; creator-rule]
- Reports differ on sugar preference: some say bacteria consume mostly maltose and yeast prefer glucose, while others say each feeds on the other's by-products. [breadcode-0073; SF Ch 2 How sourdough works; literature] *(low confidence)*
- Yeast tolerate the acidic environment created by bacteria and are thus protected from other pathogens; the author's understanding of the yeast-bacteria symbiosis is that they coexist and sometimes, but not always, benefit each other. [breadcode-0074; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- Yeast and bacteria both produce compounds that prevent the other from metabolising food. [breadcode-0075; SF Ch 2 How sourdough works; literature]
- The author speculates that a stand-off between yeast and bacteria in a liquid dough occurs throughout the dough, not in isolated patches; unverified. [breadcode-0076; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- Homofermentative bacteria metabolise the ethanol produced by yeast and by heterofermentative lactic acid bacteria. [breadcode-0081; SF Ch 2 How sourdough works; creator-rule]
- In sourdough, most of the resulting compounds from these microbial metabolisms are organic acids. [breadcode-0082; SF Ch 2 How sourdough works; creator-rule]
- The optimal fermentation temperature for a starter's lactic acid bacteria depends on its strains; generally it is the temperature at which the starter was originally cultivated. [breadcode-0085; SF Ch 2 How sourdough works; creator-rule]
- A study of lactic acid bacteria on corn leaves lowered ambient temperature from 20 to 25 °C to about 5 to 10 °C and found previously unseen bacterial varieties. — `{"ambient_from_temp_c": [20, 25], "ambient_to_temp_c": [5, 10]}` [breadcode-0086; SF Ch 2 How sourdough works; literature]
- Flour is contaminated with a very large number of wild microbes (the author says 'millions'); they are surface microbes of the plant, not endophytes inside the grain. [breadcode-0094; SF Ch 3 Making a sourdough starter; creator-rule]
- In the first days, wild yeasts, bacteria and other microbes (including mould and pathogens) are activated by water and compete; only the most adapted survive, and new microbes struggle to enter an established starter. [breadcode-0100; SF Ch 3 Making a sourdough starter; creator-rule]
- A study identified more than 150 different yeast species on a single leaf. [breadcode-0101; SF Ch 3 Making a sourdough starter; literature] *(low confidence)*
- The microbes that survive a starter's early days are those that metabolise glucose and maltose quickly, and those that consume the waste of other microbes. [breadcode-0103; SF Ch 3 Making a sourdough starter; creator-rule]
- Competing microbes try to win by consuming food fastest, by producing agents that block others' food uptake, and by producing bactericides or fungicides. [breadcode-0104; SF Ch 3 Making a sourdough starter; creator-rule]
- Which microbes first dominate depends on the flour's origin; hand and air microbes may play a minor role. [breadcode-0105; SF Ch 3 Making a sourdough starter; creator-rule]
- Yeast produces ethanol, which the bacteria in the starter metabolise; this is why sourdough contains no meaningful alcohol. [breadcode-0106; SF Ch 3 Making a sourdough starter; creator-rule]
- In user reports, mould damaged starters that had dried out a lot. [breadcode-0114; SF Ch 3 Making a sourdough starter; anecdote] *(low confidence)*
- The key readiness sign is bubbles in the starter: yeast metabolises the flour and produces CO2, which is trapped in the mass and shows as bubbles, often at the container edges. [breadcode-0125; SF Ch 3 Making a sourdough starter; creator-rule]
- Acidity is toxic to most microbes; the yeast and bacteria in the starter are adapted to it, which makes a well-fed starter hard to invade. [breadcode-0141; SF Ch 3 Making a sourdough starter; creator-rule]
- Pickled food has been shown to stay good for a long time. [breadcode-0142; SF Ch 3 Making a sourdough starter; literature] *(low confidence)*
- Spores are more vulnerable to external pathogens such as mould than the active starter, though they are very resilient. [breadcode-0144; SF Ch 3 Making a sourdough starter; creator-rule]
- Spores reported as 250 million years old are still viable. [breadcode-0145; SF Ch 3 Making a sourdough starter; literature] *(low confidence)*
- Mould is aerobic and grows poorly under water, so a starter kept under water is protected from mould. [breadcode-0154; SF Ch 3 Making a sourdough starter; literature] *(low confidence)*
- The starter's microbial profile is carried into the main dough: more bacterial fermentation in the starter gives slightly more bacterial fermentation in the dough, and more yeast fermentation gives more yeast fermentation. [breadcode-0175; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- Remember that the dough is essentially a large starter; the dough's own response to the inoculum shows what to change next. [breadcode-0179; SF Ch 4 Sourdough starter types, §4.1 Introduction; creator-rule]
- The flavour of a regular starter is lactic (dairy), acetic (vinegary) or a mix of both, depending on the bacteria cultivated. [breadcode-0191; SF Ch 4 Sourdough starter types, §4.2 Regular starter; creator-rule]
- The water layer limits oxygen during fermentation, so the starter stops producing acetic acid; heterofermentative lactic acid bacteria thrive and the starter develops creamy dairy notes. [breadcode-0193; SF Ch 4 Sourdough starter types, §4.3 Liquid starter; creator-rule]
- A fruit-based starter can lack the bacterial part of fermentation, and acidity is important to clean the starter of possible pathogens. [breadcode-0235; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; creator-rule]
- A lievito madre should acidify below pH 4.2 to show it produces sufficient acidity. — `{"ph_max": 4.2}` [breadcode-0236; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; creator-rule]
- Lower pH means higher acidity; a pH meter helps check whether a starter made from fruit acidifies properly. [breadcode-0237; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; creator-rule]
- Bathing lievito madre in water to remove acidity was not confirmed by the author's experiments: the acidity stayed the same. [breadcode-0238; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; anecdote] *(low confidence)*
- A water bath could only lower acidity if it boosted anaerobic microorganisms, which requires staying in that environment for a long time, not a few hours. [breadcode-0239; SF Ch 4 Sourdough starter types, §4.5 Lievito madre or pasta madre; extrapolation] *(low confidence)*
- Acidity increases the longer fermentation is pushed, regardless of starter type. [breadcode-0241; SF Ch 4 Sourdough starter types, §4.6 Conclusion; creator-rule]
- At a 100 % volume increase, the liquid starter has produced the most acidity, then the regular starter, then the stiff starter. — `{"volume_increase_pct": 100}` [breadcode-0242; SF Ch 4 Sourdough starter types, §4.6 Conclusion; creator-rule]
- A long fermentation to build sourness needs either a loaf pan or a very strong gluten flour that can withstand long fermentation times. [breadcode-0244; SF Ch 4 Sourdough starter types, §4.6 Conclusion; creator-rule]
- Feeding is the first lever against imbalance: a starter left unfed becomes bacteria-dominant as acid accumulates, and feeding restores yeast activity. [breadcode-0362; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- When food is scarce, microbes sporulate; they must regrow from spores, which slows fermentation. [breadcode-0375; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Less starter makes it easier for microbes to reproduce, and a strong starter outcompetes other germs present in flour. [breadcode-0377; SF Ch 7 Wheat sourdough §7.2 Readying your starter; creator-rule]
- Wetter dough is more bacterially driven and loses gluten faster than stiffer dough; the starter must be in good condition to start such a dough. [breadcode-0401; SF Ch 7 Wheat sourdough §7.4 Hydration; creator-rule]
- Fermentation is self-limiting: microbes multiply until food runs out, and rising ethanol and acidity suppress growth and block newcomers. [breadcode-0426; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- A very bacterial starter makes the main dough bacteria-dominant, giving a less fluffy, too-sour bread. [breadcode-0427; SF Ch 7 Wheat sourdough §7.5 How much starter?; creator-rule]
- Fermentation is exothermic: the metabolism of the dough produces heat. [breadcode-0511; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > Aliquot jar limitations; creator-rule]
- Author gives an unvalidated idea that microbes produce much CO2 before dying near 60 °C and that this drives expansion; he says it needs research. — `{"temp_c": 60}` [breadcode-0649; SF Ch 10 Baking §10.1 The process of baking; extrapolation] *(low confidence)*
- Author speculates spores formed under heat stress might survive baking and be reactivated to start a starter from a baked loaf; untested. [breadcode-0664; SF Ch 10 Baking §10.1 The process of baking; extrapolation] *(low confidence)*
- Mold spores in the air contaminate bread from the moment it leaves the oven; they grow best in humid conditions, so a closed container favours mold. [breadcode-0779; SF Ch 11 Storing bread §11.2 Room temperature in a container; creator-rule]
- A plain yeast-based dough kept in a closed container starts to mold within a few days. [breadcode-0780; SF Ch 11 Storing bread §11.2 Room temperature in a container; anecdote]
- Sourdough bread keeps longer than yeast bread in a container because its acidity inhibits mold. [breadcode-0781; SF Ch 11 Storing bread §11.2 Room temperature in a container; creator-rule]
- Acetic acid is produced by heterofermentative lactic acid bacteria and by acetic acid bacteria during fermentation; it gives a vinegary, sharp tang and lowers the dough's pH. [breadcode-0795; SF Glossary > Acetic Acid; creator-rule]
- Lactic acid, produced by lactic acid bacteria, gives a mild, yogurt-like tang and together with acetic acid makes up the bread's acidity. [breadcode-0850; SF Glossary > Lactic Acid; creator-rule]
- pH measures acidity on a 0–14 scale, where 7 is neutral; below 7 is acidic and above 7 is alkaline. — `{"ph_neutral": 7}` [breadcode-0862; SF Glossary > pH; creator-rule]
- Fermented foods with a pH below 4.2 are generally considered food-safe. — `{"ph_food_safe_below": 4.2}` [breadcode-0863; SF Glossary > pH; creator-rule]
- A pH meter can track a sourdough's fermentation progress. [breadcode-0864; SF Glossary > pH; creator-rule]
- Wild yeasts live on the surface of plants in symbiosis with them: the plant shields the yeast from pathogens and the yeast receives sugars from photosynthesis; when the plant weakens, the yeast can become parasitic. [breadcode-0882; SF Glossary > Wild Yeast; creator-rule]
- Lactic acid tastes milder and yogurt-like, whereas acetic acid can taste pungent; acetic acid suits dark rye but not a fluffy ciabatta-style loaf. [breadcode-0905; SF Ch 12 Troubleshooting §12.1.4 What's the benefit of using a liquid sou; creator-rule]
- A liquid starter gives more bacterial acidity, which makes the bread more acidic (but milder), and the dough degrades faster during fermentation. [breadcode-0907; SF Ch 12 Troubleshooting §12.1.4 What's the benefit of using a liquid sou; creator-rule]
- Hooch is not harmful: it protects the starter from aerobic mold entering through the top. [breadcode-0921; SF Ch 12 Troubleshooting §12.1.7 Liquid on top of my starter; creator-rule]
- A mature starter rarely goes moldy, because the symbiosis of yeast and bacteria keeps pathogens such as mold out. [breadcode-0924; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; creator-rule]
- The low pH created by the bacteria is hostile to pathogens; anything below pH 4.2 is generally considered food-safe, the same principle as pickling. — `{"ph_food_safe_below": 4.2}` [breadcode-0925; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; literature]
- Young starters can go moldy because each flour contains mold spores that compete with wild yeast and bacteria for the flour; mold can sometimes win. [breadcode-0926; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; anecdote]
- A mature starter goes moldy only when conditions change, for example when stored in the fridge with its surface dried out. [breadcode-0928; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; anecdote]
- Mold needs aerobic conditions, meaning it needs oxygen to grow. [breadcode-0931; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; creator-rule]
- Converting a starter to liquid shifts it from acetic acid to lactic acid production, because acetic acid production also needs oxygen; the author reports this worked for them. [breadcode-0932; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; anecdote]
- Lactic acid bacteria produce metabolites that inhibit mold growth. [breadcode-0933; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; literature]
- After pickling, the flour settles and oxygen is depleted, so anaerobic lactic acid bacteria thrive and the smell changes from acetic to dairy. [breadcode-0935; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; creator-rule]
- Returning to 100 % hydration after pickling should eliminate the mold, according to the author. — `{"hydration_pct": 100}` [breadcode-0937; SF Ch 12 Troubleshooting §12.1.8 Fixing a moldy sourdough starter; anecdote] *(low confidence)*
- A starter that is too sour gives more bacterial than yeast activity, producing a tangier loaf that is less fluffy; balance needs fluffiness from yeast and moderate tang from bacteria. [breadcode-0938; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- Feeding with a low starter ratio gives the microorganisms a fresh environment, similar to using a 10 % or 20 % starter dose in dough. — `{"starter_pct_of_flour": [10, 20]}` [breadcode-0942; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- A sour (acidic) environment lets the desired microorganisms outcompete pathogens, because acidity is toxic to most pathogens. [breadcode-0943; SF Ch 12 Troubleshooting §12.1.9 My sourdough starter is too sour; creator-rule]
- A starter that smells of vinegar or acetone has produced a lot of acetic acid; once food runs out, some of its bacteria convert ethanol into acetic acid. [breadcode-0945; SF Ch 12 Troubleshooting §12.1.10 Why does my starter smell like vinegar ; creator-rule]
- Ethanol is oxidized to acetic acid by oxygen, producing water: CH3CH2OH + O2 → CH3COOH + H2O (reaction scheme). [breadcode-0947; SF Ch 12 Troubleshooting §12.1.10 Why does my starter smell like vinegar ; literature]
- Excess bacterial fermentation can be checked by taste (very sour), by a dough that suddenly becomes very sticky after a few hours of work, and by smell (should not be too pungent). [breadcode-0978; SF Ch 12 Troubleshooting §12.3 Bread > My bread stays flat; creator-rule]
- Over a long fermentation the bacteria metabolise most of the ethanol produced by the yeast; they mainly produce lactic and acetic acid. [breadcode-0979; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Acetic-acid-producing bacteria die out of a starter kept too liquid over time. [breadcode-0987; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule] *(low confidence)*
- Yeast produces CO2 and ethanol from sugars released by amylase; once sugar runs out, bacteria consume the ethanol, so acidity builds up with time. [breadcode-0998; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; creator-rule]
- For the same volume increase, a dough with more yeast and less bacteria produces less acidity. [breadcode-0999; SF Ch 12 Troubleshooting §12.3 Bread > My bread is too sour; creator-rule]
- A starter is a mix of lactic acid bacteria and wild yeast; bacteria produce acids and yeast produces carbon dioxide. Whole grain and rye flour bring more microbes than white flour. [breadcode-1141; repo:the-bread-code recipes/sourdough/make-sourdough-starter.md; creator-rule]
- Yeast produces CO2 for leavening and ethanol that evaporates in the bake; lactic acid bacteria add acidity. [breadcode-1328; yt:-60DxtcJ9Fo 7:49; creator-rule]
- Heterofermentative lactic acid bacteria make acetic acid (vinegar notes) when oxygen is present; homofermentative bacteria make lactic acid (yogurt notes). [breadcode-1329; yt:-60DxtcJ9Fo 8:23; creator-rule]
- Fermentation is the main control variable in sourdough; he says most problems are under- or over-fermentation. [breadcode-1362; yt:o_xtVmR4vlc 3:19; creator-rule]
- Yeast in the starter makes CO2 and ethanol, while bacteria make acids and consume gluten, so a stiff starter favours yeast over bacteria. [breadcode-1383; yt:msqU-ylXWUs 7:06; creator-rule]
- Excess acid from an over-sour starter breaks down the gluten structure, so the dough collapses and rises less in the oven. [breadcode-1460; yt:qM9BXKaIr60 1:12; creator-rule]
- Two starters fed differently and left overnight had different pH; he reported about 4.1 for the more acidic one. [breadcode-1462; yt:qM9BXKaIr60 2:18; 2:52; measured] *(low confidence)*
- After a long, stiff, heavily kneaded bulk (~14 h), a pH meter read about 4 near the end of fermentation, meaning the dough was very close to over-fermentation, while the sample jar had risen less than the author expected. — `{"ph_approx": 4}` [breadcode-1520; yt:LQPCXMpxO4c 26:31; 27:09; measured] *(low confidence)*
- A really bad smell in a starter can mean too much acetic acid has built up. [breadcode-1572; yt:yYkTrGHNW2w 14:17; creator-rule]
- A starter with less base acidity starts fermenting from about pH 4.2, so its acid rises more slowly and bulk lasts longer. — `{"starter_ph": 4.2}` [breadcode-1574; yt:yYkTrGHNW2w 4:07; extrapolation] *(low confidence)*
- Yeast favours cool temperatures, while lactic acid bacteria favour warm ones. [breadcode-1576; yt:yYkTrGHNW2w 5:50; creator-rule]
- A starter measured at about pH 4.3 was seen to reach about 3.9–4.1 after a 1:1:1 feed. — `{"starter_ph": [3.9, 4.3]}` [breadcode-1580; yt:yYkTrGHNW2w 17:38; anecdote] *(low confidence)*
- The literature standard for yeast to lactic acid bacteria in sourdough is about 1:50 to 1:100, but real starters deviate widely. — `{"yeast_to_lab_ratio": "1:50 to 1:100"}` [breadcode-1593; yt:nQWlG1buMM8 7:25; literature]
- Feeding a different flour for a long time gradually changes a starter's microbial families; the creator only tested up to 10 days. — `{"tested_days": 10}` [breadcode-1596; yt:nQWlG1buMM8 10:15; extrapolation] *(low confidence)*
- In a yeast dough with some starter, the starter adds flavour and acid while yeast does most of the leavening. — `{"starter_pct_of_flour": 20}` [breadcode-1607; yt:0kFYyj3WKpE 2:32; creator-rule]
- A starter that smells strongly of vinegar is too acidic to be used alone and can weaken the gluten during shaping. [breadcode-1608; yt:0kFYyj3WKpE 3:51; creator-rule]
- In a laminated sourdough, acid builds up during long ferments and degrades the gluten, so keep the total fermentation time short. [breadcode-1620; yt:dY9GeoZX8DY 0:39; creator-rule]
- Keep chilled rests short with sourdough, since acid keeps building even in the fridge. [breadcode-1627; yt:dY9GeoZX8DY 13:24; creator-rule]
- The guest suspects that after a certain time in a cold ferment the gluten structure breaks down, probably because acidity builds (unproven). He left loaves 36 h in the fridge: very sour, and he thinks 24 h is pushing it. — `{"fridge_h": [24, 36]}` [breadcode-1684; yt:l_PLhPWE-8E 9:30; 10:05; anecdote] *(low confidence)*
- A longer build gives more acid; a 50 % whole-wheat levain gave more acetic and sour notes than a bread-flour levain. — `{"whole_wheat_levain_pct": 50}` [breadcode-1688; yt:l_PLhPWE-8E 16:45–17:58; anecdote]
- Levito madre and stiff-from-liquid starters measured about pH 4.2; regular about 3.8–3.9; yeast-only about pH 5.2 with the lowest inflation. — `{"ph_range": [3.8, 5.2]}` [breadcode-1725; yt:lNCHzD8HxBA 14:56-15:40; 16:54; measured] *(low confidence)*
- A bulk pH of about 4.1 was measured; below about 4.0 signals over-fermentation risk. — `{"ph_range": [4, 4.1]}` [breadcode-1731; yt:9yI7QRpAgow 7:21; creator-rule]
- A starter that is not fed for long enough, or given too little new flour, becomes very sour; the acid attacks the gluten network and the dough turns into a gooey mess. [breadcode-1746; yt:AKJxiROZasU 1:50; creator-rule]
- At lower temperature the yeast part of the sourdough is more active than the bacterial part, which slows over-fermentation. [breadcode-1786; yt:dl7crk2Nvj8 16:12; creator-rule]
- Lactic to acetic acid balance matters: for panettone the guest aims for about 4 parts lactic to 1 part acetic, favouring sweet, yogurt-like notes over vinegar notes. — `{"ratio": 4}` [breadcode-1793; yt:l_PLhPWE-8E 14:21–14:58; anecdote] *(low confidence)*
- Measure pH on a dough sample and on the main dough, and use a size increase only as an early marker; the target pH depends on the starter and the dough. [breadcode-1810; yt:dQEebR6pIng 10:41; anecdote]
- A starter below about pH 4.2 is a hostile environment for yeast and lactic bacteria, but they can survive it; the more acidic it gets, the harder it is for them. — `{"ph_hostile_below": 4.2}` [breadcode-1876; yt:rOu-phxyQbQ 9:15; creator-rule]
- In a sourdough, the creator says there are typically about 1 to 100 yeast to bacteria (ratio as he phrases it). — `{"yeast_to_bacteria_ratio": [1, 100]}` [breadcode-1883; yt:TUkPI34Go7k 9:34; creator-rule] *(low confidence)*
- An acetone smell in a starter is normal; it comes from acetic-acid bacteria and much of it evaporates during baking. [breadcode-1984; yt:m7J6CBZcq8o 5:32; creator-rule]
- He used a pH meter; a fermented dough at about pH 4.1 was his end marker. — `{"ph": 4.1}` [breadcode-1992; yt:0BA-Lka_QMI 6:34; creator-rule] *(low confidence)*
- The acid in a starter has a pickling effect that suppresses unwanted germs, so a starter started from wild fruit must become acidic at some point. — `{"pH_note": "low pH, no number given"}` [breadcode-2024; yt:m7J6CBZcq8o 2:13; creator-rule]
- In a 500 % (liquid) starter, the acetic-acid bacteria are suppressed, which he links to the loss of sharp acetic flavour and a dairy taste. — `{"hydration_pct": 500}` [breadcode-2034; yt:m7J6CBZcq8o 3:53; anecdote] *(low confidence)*
- A sourdough dough is normally around pH 4.2 when shaped. — `{"ph": 4.2}` [breadcode-2036; yt:vf5RbwlW8Xs 4:08; creator-rule]
- The longer a dough ferments, the more its gluten network is broken down (by flour protease and lactic acid bacteria), the more acid accumulates and the lower the pH falls. [breadcode-2037; yt:vf5RbwlW8Xs 4:08; creator-rule]
- An overheated, over-kneaded dough rose a lot but tasted less sour, which suggests heat favours yeast leavening over acid production. [breadcode-2038; yt:vf5RbwlW8Xs 1:44; anecdote] *(low confidence)*
- Lower pH from a sourdough resists mould better than a neutral dough. [breadcode-2052; yt:OcOpIISBK6c [11:05]; creator-rule]
- Dough measured at pH 4.1 at the stage noted in the video. — `{"ph": 4.1}` [breadcode-2070; yt:NrKHO2xZTEU [2:29]; measured] *(low confidence)*
- Dough at about 10 h, when shaped, measured about pH 4.05. — `{"ph": 4.05, "time_h": 10}` [breadcode-2079; yt:MPWypFY0wyE [9:14]; measured] *(low confidence)*
- A stiff starter favours yeast over bacteria: less sour and less risk of over-fermentation. [breadcode-2117; yt:_P1B7PxAwk0 [6:05]; creator-rule]
- Sourdough made only from flour, water and salt stays mould-free for days because of its acidity. [breadcode-2134; yt:zfxOxu3_4bE [1:09]; anecdote]
- During fermentation, lactic acid bacteria gradually soften the gluten network, so the dough becomes more extensible the longer it ferments. [breadcode-2167; yt:bJxpCVc2wYE 9:12; creator-rule]
- Fruit flies spread wild yeast between fermenting substrates, much as bees pollinate flowers. [breadcode-2198; yt:xEwk5yHf_Gc 0:36; extrapolation] *(low confidence)*
- A liquid (5x water) starter favours bacteria over yeast, so the bacterial population can out-compete mould. — `{"hydration_pct": 500}` [breadcode-2200; yt:xEwk5yHf_Gc 3:10; creator-rule] *(low confidence)*
- A new starter often has an imbalance of bacteria and yeast, which gives flat bread; more yeast activity is needed to rise. [breadcode-2202; yt:xEwk5yHf_Gc 8:04; creator-rule]
- A surface layer on a very liquid starter blocks oxygen, so the culture makes more lactic acid and less acetic acid. [breadcode-2206; yt:ES7x28d10pg 1:06-1:39; creator-rule]
- As an unfed starter becomes more acidic, yeast activity fades while lactic and acetic acid bacteria dominate, which gives sour, weak dough. [breadcode-2218; yt:ngcISK7ZDjc 2:21; creator-rule]
- Yeast produce CO2 that inflates the dough, while bacteria produce acid that gives sourdough its flavour. [breadcode-2237; yt:nIOPCeLPqrM 3:03; creator-rule]
- Yeast mainly produces ethanol and CO2, lactic acid bacteria mainly produce lactic acid, and acetic acid bacteria mainly produce acetic acid, which needs oxygen. [breadcode-2305; yt:0v1QhtyUic4 0:37; creator-rule]
- Acetic acid (about 118 C) and lactic acid (about 122 C) only start to evaporate well above the about 92 C core temperature at which the creator considers the bread finished, so little acid is lost in a normal bake. — `{"core_c": 92}` [breadcode-2306; yt:0v1QhtyUic4 1:52; extrapolation] *(low confidence)*
- Yeast in a starter produces ethanol and CO2; the CO2 inflates the dough and supplies most of the oven spring. [breadcode-2418; yt:HgcGcG7rdZM 1:07; creator-rule]
- Lactic acid bacteria are either homofermentative (make essentially only lactic acid) or heterofermentative (make lactic acid, CO2 and a little ethanol); he says the bacteria in his starter are mostly heterofermentative. [breadcode-2419; yt:HgcGcG7rdZM 1:07; creator-rule]
- Acetic acid (the vinegary acid) requires oxygen to form; a layer of water on top of the starter limits oxygen so the bacteria shift toward lactic acid (yoghurt-like) and away from acetic acid. [breadcode-2420; yt:HgcGcG7rdZM 1:40; creator-rule]
- Before the hooch was stirred back in, the sour starter liquid measured pH 3.9 with a probe pH meter. — `{"ph": 3.9}` [breadcode-2489; yt:c8GId0ByASo 2:21; measured]
- Stirring a small amount of baking powder into the sour starter raised its pH from about 3.9 to about 4.1 within five minutes, with visible gas bubbles. — `{"ph_before": 3.9, "ph_after": 4.1, "wait_min": 5}` [breadcode-2490; yt:c8GId0ByASo 4:45; measured]
- In the dough made with that starter, pH was about 4.25 after adding flour; the baking powder counteracts the acid increase as the dough rises. — `{"ph": 4.25}` [breadcode-2491; yt:c8GId0ByASo 5:22; measured]
- After baking, the pH of the loaf was higher again, and he says the bread tasted much less sour. [breadcode-2492; yt:c8GId0ByASo 6:36; measured] *(low confidence)*
- The liquid on top of a starter (hooch) is fermentation liquid rich in microbes, not something to throw away; stir it back in. [breadcode-2494; yt:c8GId0ByASo 1:10; creator-rule]
- Longer fermentation builds up lactic and acetic acid, which makes the dough more sour and helps keep moulds out, which is why sourdough rarely grows mould. [breadcode-2495; yt:c8GId0ByASo 1:10; creator-rule]
- A sourdough starter is a symbiosis: yeast make ethanol and CO2, lactic acid bacteria make lactic acid (sour, dairy notes), and acetic acid bacteria make acetic acid (vinegary notes). [breadcode-2524; yt:nPSsWa4nGgU 0:00; creator-rule]
- After feeding, yeast is most active at first and produces CO2 and ethanol; bacteria then take over and acidify, and the yeast eventually goes dormant. [breadcode-2525; yt:nPSsWa4nGgU 0:38; creator-rule]
- At the same volume increase, a liquid-starter dough is more acidic (pH about 3.9–4.0) than a regular 100 % starter dough (about 4.2). — `{"ph": [3.9, 4.2]}` [breadcode-2527; yt:nPSsWa4nGgU 2:28; measured] *(low confidence)*
- In these enriched buns the sourdough is mostly for flavour; yeast does most of the leavening. — `{"sourdough_pct": 10, "yeast_dry_pct": 2}` [breadcode-2546; yt:Dx7xM8IThCI description; creator-rule]
- At about 6 °C, yeast and bacteria remain active, so a cold-stored dough keeps fermenting slowly over weeks and becomes very acidic. — `{"temp_c": 6, "cold_storage_weeks": 3}` [breadcode-2645; yt:lFzLCivvyes 3:52; anecdote]
- During baking, lactic and acetic acid evaporate at about 120 °C; acetic acid boils off faster, so a longer bake reduces tang. — `{"temp_c": 120}` [breadcode-2646; yt:lFzLCivvyes 4:29 (≈120 °C acid loss); 5:08; extrapolation]
- Some sourdough microbes come from the air, but many enter through the flour used to feed the starter, so feeding with local flour changes the culture. [breadcode-2725; yt:T4YvXrlp4eQ description; creator-rule]
- Sourdough is fermented by wild yeast and lactic bacteria carried from a mother dough, which gives the bread a mild sour taste. [breadcode-2792; yt:VlD2YuNq1w0 0:00; creator-rule]
- Sourdough starters produce CO2, lactic acid and acetic acid. [breadcode-2902; yt:FKlQFryXYmY description; creator-rule]
- More gas from a starter should give a fluffier loaf (his inference). [breadcode-2904; yt:FKlQFryXYmY description; extrapolation] *(low confidence)*
- Final pH: stiff starter about 4.5; regular and liquid starters about 4.0. — `{"ph_stiff": 4.5, "ph_regular_liquid": 4.0}` [breadcode-2914; yt:UYBJZ-xmRFM description; measured]
- A stiffer starter favours yeast activity over bacterial activity, which makes fermentation easier. [breadcode-2959; yt:40s3HVk462Y 0:00; creator-rule]

### sourdough-microbiology.acids

- The two predominant acids in sourdough bread are lactic acid (dairy notes) and acetic acid (vinegar taste). [breadcode-0068; SF Ch 2 How sourdough works; creator-rule]
- Both lactic and acetic acid are produced by homofermentative and heterofermentative lactic acid bacteria. [breadcode-0069; SF Ch 2 How sourdough works; creator-rule]
- Acetic acid production requires oxygen. [breadcode-0083; SF Ch 2 How sourdough works; literature]
- Depriving the starter of oxygen favours homofermentative lactic acid bacteria, which over time outcompete acetic-acid-producing bacteria, shifting the flavour profile toward lactic acid. [breadcode-0084; SF Ch 2 How sourdough works; literature]
- Some sources say that lower-temperature fermentation increases acetic acid production, while warmer fermentation boosts lactic acid; the author could not verify this and no source is cited. [breadcode-0088; SF Ch 2 How sourdough works; extrapolation] *(low confidence)*
- Smell indicates the dominant acid: lactic acid smells dairy/yogurt-like; acetic acid smells pungent and vinegary, sometimes described as glue or acetone. [breadcode-0128; SF Ch 3 Making a sourdough starter; creator-rule]
- Longer baking evaporates more of the acidity, so a too-sour dough can be balanced by cooking longer. [breadcode-0299; SF Ch 6 Bread types; creator-rule]
- Acidity stops rising after about 48 h of rye bulk fermentation because the microbes have used most nutrients; longer waits change flavour little. — `{"plateau_h": 48}` [breadcode-0333; SF Ch 8 Non wheat sourdough; creator-rule]
- Dough tastes far more acidic than the loaf; much of the acid evaporates in the bake. [breadcode-0335; SF Ch 8 Non wheat sourdough; creator-rule]
- Longer baking evaporates acidity, so a too-sour rye loaf can be softened by baking longer. [breadcode-0342; SF Ch 8 Non wheat sourdough; creator-rule]
- Acetic acid boils at about 118 °C but is volatile and leaves the dough with steam, mostly from the drying crust, well before that temperature. — `{"bp_c": 118}` [breadcode-0652; SF Ch 10 Baking §10.1 The process of baking; creator-rule]
- Lactic acid barely evaporates during baking; the quoted 122 °C boiling point is a value measured under strong vacuum. — `{"bp_c": 122}` [breadcode-0653; SF Ch 10 Baking §10.1 The process of baking; literature]
- In a wine-analysis steam separation, steam carried at least 99.5 % of the acetic acid and at most 0.5 % of the lactic acid. — `{"acetic_steam_carryover_pct_min": 99.5, "lactic_steam_carryover_pct_max": 0.5}` [breadcode-0654; SF Ch 10 Baking §10.1 The process of baking; literature]
- Cited sourdough measurements show less acid in the crust than in the crumb. [breadcode-0655; SF Ch 10 Baking §10.1 The process of baking; literature]
- Lactic acid is chemically more acidic than acetic acid but is often not perceived as sour. [breadcode-0980; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Acetic acid is perceived as sourer than its chemical acidity suggests, so starters producing more acetic acid taste sharper. [breadcode-0984; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- Acetic acid production requires oxygen; a starter that is too liquid submerges the flour, limits oxygen and favours lactic over acetic acid. [breadcode-0986; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; creator-rule]
- During baking, acetic acid leaves with the water vapour mainly from the crust, while lactic acid barely evaporates. [breadcode-0989; SF Ch 12 Troubleshooting §12.3 Bread > I want more tang in my bread; literature]
- Bacteria produce mostly acid, and the more acid accumulates, the less active the yeast becomes. [breadcode-1055; SF Ch 12 Troubleshooting §12.4 Debugging your crumb structure > Underferm; creator-rule]

### sourdough-microbiology.cold-rate

- At colder temperatures the microorganisms work at a slower rate; the author doubts that cold changes their biochemistry. [breadcode-0590; SF Ch 7 Wheat sourdough §7.13 Proofing; creator-rule]
- A retarded dough first proofs at the same rate as a room-temperature dough and slows as it cools. [breadcode-0609; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- There is a limit to how long dough can be fridge proofed; the author attributes it to continued fermentation at low temperatures. [breadcode-0612; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); extrapolation] *(low confidence)*

### sourdough-microbiology.ph

- Lactic and acetic acid bacteria build up acidity during bulk, which lowers dough pH; pH is logarithmic, so each unit is a tenfold change in acidity. [breadcode-0512; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- In the author's example, a sourdough starts at pH about 6.0 and is about pH 4.0 shortly before baking, a difference of roughly 100× in acidity (two pH units). — `{"ph_start": 6.0, "ph_pre_bake": 4.0}` [breadcode-0513; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- The acceptable pH window depends on the starter, water and flour, so each baker should determine target pH values for their own bread. [breadcode-0514; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- A stronger, higher-gluten flour can be fermented longer, so its acceptable pH endpoint differs. [breadcode-0515; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- Measure pH at six breakpoints: starter before use, after mixing, before dividing/pre-shaping, before shaping, before and after proofing, and after baking. [breadcode-0516; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- If a bake turns out well, reuse its pH values as the reference for the next batch; if not, shorten or extend the fermentation a little and recheck. [breadcode-0517; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- Example pH measured by the author at the starter ready (before use) breakpoint: pH 4.20. — `{"ph": 4.2}` [breadcode-0518; SF Table 7.4; measured]
- Example pH measured by the author at the after mixing breakpoint: pH 6.00. — `{"ph": 6.0}` [breadcode-0519; SF Table 7.4; measured]
- Example pH measured by the author at the before dividing/pre-shaping breakpoint: pH 4.10. — `{"ph": 4.1}` [breadcode-0520; SF Table 7.4; measured]
- Example pH measured by the author at the before shaping breakpoint: pH 4.05. — `{"ph": 4.05}` [breadcode-0521; SF Table 7.4; measured]
- Example pH measured by the author at the before proofing breakpoint: pH 4.03. — `{"ph": 4.03}` [breadcode-0522; SF Table 7.4; measured]
- Example pH measured by the author at the after proofing breakpoint: pH 3.80. — `{"ph": 3.8}` [breadcode-0523; SF Table 7.4; measured]
- Example pH measured by the author at the after baking breakpoint: pH 3.90. — `{"ph": 3.9}` [breadcode-0524; SF Table 7.4; measured]
- Once a good working pH range is found, the same fermentation level can be reproduced in each subsequent dough, which helps consistency in large bakeries. [breadcode-0525; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter; creator-rule]
- The author's pH values are only a rough guide: the bacteria balance in each starter differs, so your own values must be found. [breadcode-0526; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter limitations; creator-rule]
- A pH meter must be certified for liquid and semi-solid media, be properly calibrated, be temperature-compensated (pH depends on temperature), and must be recalibrated frequently because it drifts over time. [breadcode-0527; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter limitations; creator-rule]
- Rinse a pH meter for at least one minute before inserting it into dough, because the liquid around its head is not food-safe. — `{"rinse_min": 1}` [breadcode-0528; SF Ch 7 Wheat sourdough §7.9 Bulk fermentation > pH meter limitations; creator-rule]
- A pH meter measures the acidity that has accumulated in the dough, which can be used to control proofing consistency. [breadcode-0618; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- Test the pH at several proofing times to find the pH value that gives the best bread for your process, then aim for that value on every dough. [breadcode-0619; SF Ch 7 Wheat sourdough §7.13 Proofing > Cold-proofing (retarding); creator-rule]
- Example pH at the 'Starter ready' step of one baker's sourdough process: 4.20. — `{"ph": 4.2}` [breadcode-0621; SF Table 7.4; measured]
- Example pH at the 'Mixing' step of one baker's sourdough process: 6.00. — `{"ph": 6.0}` [breadcode-0622; SF Table 7.4; measured]
- Example pH at the 'Dividing/preshaping' step of one baker's sourdough process: 4.10. — `{"ph": 4.1}` [breadcode-0623; SF Table 7.4; measured]
- Example pH at the 'Shaping' step of one baker's sourdough process: 4.05. — `{"ph": 4.05}` [breadcode-0624; SF Table 7.4; measured]
- Example pH at the 'Before proofing' step of one baker's sourdough process: 4.03. — `{"ph": 4.03}` [breadcode-0625; SF Table 7.4; measured]
- Example pH at the 'After proofing' step of one baker's sourdough process: 3.80. — `{"ph": 3.8}` [breadcode-0626; SF Table 7.4; measured]
- Example pH at the 'After baking' step of one baker's sourdough process: 3.90. — `{"ph": 3.9}` [breadcode-0627; SF Table 7.4; measured]
- Measuring the dough pH before baking, with a pH meter, helps judge fermentation progress in later bakes. [breadcode-1009; SF Ch 12 Troubleshooting §12.3 Bread > My bread flattens out when removin; creator-rule]
- A pH meter measures the acidity produced by lactic and acetic acid bacteria; measure pH repeatedly to find a bulk endpoint that works for your dough. [breadcode-1102; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]
- The author ends bulk fermentation at about pH 4.1, but the endpoint is individual and should be found per dough. — `{"ph": 4.1}` [breadcode-1103; SF Ch 12 Troubleshooting §12.5 Misc > Baking in the tropics; creator-rule]

### sourdough-microbiology.yeast-bacteria-balance

- Using old starter directly can make the bread more sour because the balance of yeast to bacteria may be off. [breadcode-0292; SF Ch 6 Bread types; creator-rule]
