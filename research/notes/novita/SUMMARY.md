# Novita Listyani (+ Seraphine Lishe) — research summary

Channel: Novita Listyani (shared with her daughter Seraphine Lishe, who presents most science videos). Plus the site novitalistyani.com and its BakingSmart recipe XML files. All sources are all-rights-reserved; the notes paraphrase and record facts only. Videos run 2014–2026; baking content runs 2019–2026.

## Outputs
| Path | Content |
|---|---|
| `data/claims-novita.jsonl` | **869 claims**: 514 creator-rule, 284 literature, 43 anecdote, 19 extrapolation, 9 measured |
| `data/recipes-novita.json` | **86 recipes**: 10 from BakingSmart XML (2026 method versions) + 76 from videos (68 with quantities from descriptions, 7 from transcripts, 1 with on-screen-only quantities recorded as unknown). Every recipe has `total_flour_g`, baker's % on every weighed ingredient, and `quantities_from` |
| `data/references-novita.json` | **102 references**: 90 with a DOI resolved via the Crossref API, 12 without one (thesis, books, patents, decree). Each has `cited_by` |
| `notes/novita/<topic>.md` | 26 taxonomy topics. Each has a claim list grouped by sub-topic and dated; scalds, preferment, proof, baking, mixing, dough-temperature, hydration, yeast, salt, staling-storage and starter also have a synthesis |
| `notes/novita/bakingsmart.md` | BakingSmart features and XML data model (design input) |
| `raw/web/novita/youtube/GZEvf4buPf4.*` | Seraphine Lishe's science companion to DtsBfGiq1H0 (fetched with `tools/fetch-youtube.mjs`) |
| `raw/novita/` | triage.json, shard lists, shards (haiku extraction), Crossref cache (git-ignored) |

## Coverage (reconciles with `raw/youtube/novita/_all.txt`: 341 entries)
- **Processed: 151 videos.** 147 produced at least one claim or recipe. The other 4 were processed but are empty, for these reasons:
  - `NDta4tmRYyU` (spice cake): music only and not bread; the description recipe is a non-yeasted cake.
  - `M5M7KQp5zYw` (fruit yeast water Q&A): no captions; the description lists only topics.
  - `oRwhCZlgiGI` (fruit yeast water guide): music, no narration.
  - `6yo3l-yA1VA` (bika ambon): one-line transcript; its recipe is a yeasted batter cake, which is outside the "doughs only" rule.
- 10 of the processed videos were non-bread food (mooncake, potstickers, coffee cookies, kombucha, noodles, spice cake, brownies, cream puffs, soufflé pancakes, chocolate-chip cookies). For these I kept only science claims.
- **Extra:** `GZEvf4buPf4` (Seraphine Lishe channel), the full science breakdown for DtsBfGiq1H0: 10 claims.
- **Skipped: 189** (listed at the end with reasons).
- **Failed: 1**: `IfkdYvvITus` (rendang, an off-topic savoury dish; no captions).
- 151 + 189 + 1 = 341.
- **Dropped recipes:** 3 extracted recipes were removed as batters rather than doughs: bika ambon ×2 and yeast-water muffins. Their videos still have claims.
- **Read in full by me:** ocBV6-O5SL4, DtsBfGiq1H0, GZEvf4buPf4, plus the top science videos 7bwsLYvd6go, doZTKqN8Lls, FBKZzNnnRKQ, CClhjpjcAd8, 1OD5Kex5v4g, PEoTSDswa58, 8ICSP9xq9o0, DjQ7EtvzK_w, _mFYeiLzLpo, OpK1CeW9xm8 and yevjgX3SvlM. That gave 144 claims. The other 136 videos were extracted by 10 haiku sub-agents (shards 01–10). Shard 07 was rewritten from scratch after it produced invalid JSON and unsupported claims. Shard 02 had its approximate locators replaced with exact ones, and 5 unsupported claims were deleted.

## Quality checks
- **Automated locator check (all 869 claims):** every timestamp locator falls on a transcript paragraph. In 770 claims, every number also appears in that paragraph or its neighbours. The remaining 99 contain at least one number not found literally nearby. Most are unit conversions (1 h ↔ 60 min, ratios such as 1:1.5 or % computed from grams); some are figures stated in an adjacent paragraph. Spot checks found no fabricated numbers in that set, but they were not each re-verified.
- **Literature claims:** 284, of which 152 link to a reference entry. The rest cite papers the video does not name ("a study", "a Japanese paper"); their notes say "Unresolved source(s)" and their confidence is capped at medium.
- **10 random claims re-verified by me against the raw transcript** (seed 20261124):

| claim | video @ locator | result |
|---|---|---|
| novita-0332 | j187SM6HwVo @ 4:10 | verified; 1.7% (6 g) in adjacent paragraph |
| novita-0850 | rvdsfasP0pM @ 4:09 | verified |
| novita-0148 | bIP-8B2P1A8 @ 36:53 | verified |
| novita-0190 | 3x4p_bECQbM @ 1:44 | verified (2021 shape/sensory paper) |
| novita-0847 | t--K8le97fc @ 5:06 | verified |
| novita-0867 | rvWSBsQLZXM @ 6:33 | content verified; locator covered only bulk → extended to proof/bake paragraphs |
| novita-0296 | zNTRKWhFOXQ @ 14:48 | verified (FDA Food Code quote) |
| novita-0779 | YR2e3Kka0TA @ 13:46 | content verified; "wait 5 min" is at 5:37 → locator extended |
| novita-0634 | uNoxI1-jA88 @ 6:25 | verified |
| novita-0660 | pLdkpbxYSak @ 7:03 | verified; added omitted "no significant sensory difference 1.5–6%" to notes |

Result: 10 of 10 claims are supported by the transcript. Three needed a locator extension or a note, and those fixes have been applied. None was fabricated.

## Distinctive methods
1. **Tangzhong + poolish as the base of almost every bread** (2022–2026). Both are made the night before and used cold: poolish 0.2 % yeast on total flour, 1 h at room temperature then fridge; tangzhong 15–20 % of flour at 1:1.5–1:2.
2. **The "24-hour" tangzhong** (ocBV6-O5SL4, Oct 2026): scald with high-protein bread flour at 1:1.5–2, heated to 75–80 °C (microwave 1–2 min) rather than 65 °C. Use it immediately or after exactly ~24 h in the fridge; avoid ~12 h at room temperature and 48 h cold. It is based on the Ke Yuwen thesis and on Shikishima's patent for cold-aged yudane (15–72 h).
3. **Refrigerated sponge for 100 % whole wheat** (DtsBfGiq1H0 / GZEvf4buPf4, Aug 2026): 45 % of the flour as a poolish with ~1.2 % yeast on that flour, 18 h in the fridge; 10 % tangzhong at 1:2; 77 % hydration; ≥13.5 % protein whole wheat flour. One dough is used for rolls, pullman loaf, buns, pita and cinnamon rolls.
4. **Quick 3-hour cold ferment** (CClhjpjcAd8): 5 °C with ~2 % instant yeast (6 % fresh). Even 2–3 h in the cold limits lipid-oxidation off-flavours.
5. **Mixing:** ice water; liquids and broken-up tangzhong first; bassinage (hold back 1/4 of the liquid); butter only after 3–5 min of development.
6. **Crisp without steam:** a gelatinized rice-flour glaze, plus an AP/bread-flour blend, on shio pan.
7. **Salt as a sourness lever in the levain:** 2 % for milder, about 0.7 % for more sour.
8. **Feeding the levain with tangzhong** (rye shokupan, qhvJ22U5SBI).
9. **Salt-stressed yeast** (7 % salt solution, 40 min) for sweet doughs.
10. **Pan-fill dough density:** ~257–286 g of dough per litre of pan for shokupan. She scales recipes to the pan with this figure.

## Experimental findings (her own side-by-side tests; single trials)
- Starter feeding dilutions 1:1:1, 1:2:2, 1:5:5 and 1:11 → she chose 1:2:2 (novita-0232). A 1:1:1 and a 1:5:5 starter converge at around 4–5 h (novita-0454).
- Tangzhong 35 at ~69 % hydration staled faster than tangzhong 20; at 73–74 % it stayed softer (novita-0825).
- Two coats of rice glaze on shio pan: crisper at first but much worse crispness retention; one coat is better (novita-0574).
- Salt-free dough was stickier and weaker by hand (novita-0734). Yudane aged 3 days: small gluten difference, little visible effect on the bread (novita-0784).
- No volume difference between a 20 % rye loaf and the wheat version (tyoILEdRiK8).
- Most "experiments" in her science videos are literature reviews, not her own measurements. The evidence labels reflect that: 284 literature claims against 9 measured.

## Top 15 insights for the calculators
1. **Scald model.** Input the scald as % of total flour plus a flour:water ratio (min 1:1, default 1:2). Its flour and water count in totals. Offer a "weigh after cooking" correction for evaporation.
2. **Scald hydration offset.** Optimal absorption: 68 % (control), 71 % (20 % scald), 74 % (40 % scald), i.e. ≈ +1.5 points per 10 % scalded (Yamauchi 2014). Her rule: +2 points per 10 % above 10 %. Default +1.5 ± 0.5 points per 10 %, confidence medium, flour-dependent.
3. **Scald-level guide with warnings:** 10 % mild; 20 % standard for milk bread; 30 % maximum; ≥40 % big volume loss. Proof gets longer (84 → 86 min at 20 % under industrial conditions), more at higher levels.
4. **Scald temperature and timing.** Gelatinization starts around 65 °C and completes around 75 °C. β-amylase dies above ~70 °C and α-amylase at ~80 °C. Heated gluten helps at 75–80 °C. Use the scald at 0 h or ~24 h cold; flag 8–16 h at room temperature and ≥48 h cold.
5. **Poolish presets:** 100 % hydration. Overnight: 0.2 % instant yeast on total flour (≈0.6–1.0 % of poolish flour), 1 h at 27–28 °C, then fridge ~8–12 h. Same day: ~3–3.5 h at 27 °C. Fruit yeast water: 10–18 h at room temperature. Prefermented flour 20–50 %.
6. **Refrigerated-sponge preset for whole wheat:** 45–50 % flour, ~1.2 % yeast on sponge flour, 18 h at 4 °C. +2–3 points water for whole wheat; flag flours under 13.5 % protein.
7. **Cold-ferment yeast dose:** 5 °C, 2 % instant (6 % fresh) gives ~3 h to target rise. Fresh:instant ≈ 3:1. Warn when yeast is very low and the ferment very long at room-warm temperatures (FDA 4–6 h danger-zone rule cited).
8. **Dough temperature:** water = 3×DDT − flour − room − friction. Target lower DDT (≈17–24 °C) for strength; warn above 27 °C (sticky). Cold scald and poolish lower the dough temperature, which the model should account for.
9. **Bassinage split** presets 50/50 or 40/40/20. **Autolyse** 15–30 min, cap 60 min.
10. **All her timings are for 27–28 °C.** The engine should rescale them to the user's temperature (BakingSmart cannot). Bulk endpoint ≈ 1.5× without extra yeast after a mature poolish, ≈2× otherwise. Pan proof endpoint is 1–2 cm below the rim.
11. **Pan scaling:** dough weight ≈ pan volume (L) × 257–286 g/L for shokupan.
12. **Bake presets:** enriched rolls 170–175 °C, 15–25 min. Loaf 180–190 °C, 30–40 min (with a bottom-heat finish). Lean Dutch oven 240 °C preheat, 230 °C covered 20 min, 200 °C uncovered 10–15 min. Pita on a 200–220 °C pan.
13. **Steam advisor:** volume against steam is U-shaped (≈0.33 L/m³ low, 0.66–1.0 medium, ≥1.33 high). High steam: gloss, but crispness fades fast. Covered vessels need little or no added water.
14. **Ingredient water factors:** milk 0.87, egg 0.75, butter 0.16, honey 0.17 (BakingSmart convention). Show which are counted in hydration.
15. **Frozen dough mode:** freeze unfermented; honey 6–8 %; slightly more yeast (or osmotolerant yeast); solid fat; ≤1 month; avoid thaw cycles. Optional tangzhong.

## Contradictions with common advice
- Tangzhong does **not** make bread fluffier: it lowers volume. The aim is soft, chewy and slow to stale (vs widespread "fluffy" claims; 2026).
- Tangzhong does not need 75 % hydration, and 1:5 is a poor ratio (vs the popular 1:5 method and its "add water" fix).
- The windowpane test is valid, not a myth, but it is judged after rests and folds, not only straight after mixing.
- Autolyse is not special. Delaying salt is about proteolysis, not saving the yeast. Salt does not kill a starter.
- More steam is not always better (U-shaped volume, faster loss of crispness).
- Cold fermentation helps even at 2–3 h; it does not need 6–72 h. Tiny yeast doses for very long cold ferments are discouraged.
- Cold water strengthens gluten; it is not only a way to slow yeast.

## Contradictions between her own videos over the years
| Topic | Earlier | Later |
|---|---|---|
| Using the tangzhong straight away | Don't; refrigerate 8–24 h for maltose and retrogradation (DjQ7EtvzK_w 2023, OpK1CeW9xm8 2023) | Immediate use (0 h) is also fine; only intermediate holds are bad (ocBV6-O5SL4 2026) |
| Scald temperature | Stop at 65 °C; overshooting ~5 °C inactivates β-amylase (DjQ7EtvzK_w 2023, Wz0PiKOWY1g 2023) | Aim for 75–80 °C for heated-gluten benefits (2026). Her own 2023 enzyme numbers imply less maltose at that temperature; this is unreconciled |
| Denatured gluten in the scald | Blamed for volume loss (2022–23) | Heated gluten raises volume (2026, citing a yudane heated-gluten paper) |
| Scald ratio | 1:5 as used by everyone (before Oct 2022); 1:1 or 1:2 (2022–23) | 1:1.5–1:2 (2026). 1:5 is used only as a glaze (shio pan 2026; rice glaze recipes give 1:9–1:15) |
| Scald level | 5–8 % (early 1:5 breads) | 20 % standard, 10–35 % range |
| Extra water for scalds | +2 points per 10 % scalded (2022) / 68–71–74 % table (2023) | ~72 % at 20 % (+2–3 points) (2026). Roughly consistent |
| Which scald level is softest | Yamauchi: 40 % softest (cited 2023) | Ke Yuwen: 10–20 % softest, >20 % harder (cited 2026). She explains the gap by differences in water absorption |
| Poolish routine | Fruit yeast water 10–18 h at room temperature (2020–21); ~2 h at room temperature then fridge, with extra yeast in the final dough (2022–23) | 1 h at room temperature then fridge, no extra yeast; bulk takes longer (2026 XML) |
| Retarding the final dough after a poolish | Unnecessary, may be counterproductive (tZKoPwHGcWU 2024) | She now also retards final doughs (CClhjpjcAd8 2025) |
| Yeast for long ferments | "Very little yeast" is enough for flavour ferments (novita-0577, KLHi11p2VrE 2022) | Tiny-yeast long cold ferments are a bad idea (zNTRKWhFOXQ 2024, CClhjpjcAd8 2025) |
| Crustless shokupan bake | Video settings (2023) | 160 °C 25 min + 20 min bottom heat only (2026 XML) |

Source disagreements she points out herself: the Ke Yuwen thesis vs Yamauchi on how softness changes with scald level; the 1:5 ratio's origin (Yvonne Chen's book) vs Japanese patents (100:80–120) and papers (1:1, 1:2).

## Gaps and blockers
- Several papers she names have no DOI: the Ke Yuwen 2004 thesis (Taiwan; it returned no match on Crossref, and I searched no other catalogue), Japanese patents, Calvel's book, Galal & Johnson 1977, and Fanari 2021 (Chem. Eng. Trans.). The Japanese yudane papers resolved through Crossref title search: Yamada 2015 doi:10.3136/nskkk.62.547, heated gluten 2017 doi:10.3136/nskkk.64.90, Yamauchi 2014 doi:10.3136/fstr.20.1071, and the RSD paper doi:10.1002/jsfa.12565. Title-search matches carry a `match` field. I fetched no open-access full text. All literature claims paraphrase her paraphrase, so verify them against the primary paper before relying on them.
- 132 literature claims cite papers the video does not name.
- Haiku-extracted claims (shards) are less precise than mine. 99 claims contain numbers that are not literally in the cited paragraph (mostly conversions); they were not individually re-verified.
- Recipes: some video recipes have null grams (tsp/egg counts only), and a few stated totals differ from the itemised sums. Both are noted per recipe under "merge check".
- No quantitative temperature–time fermentation data beyond her 27–28 °C timings and the 5 °C cold-ferment paper. She gives no Q10 or rate model.

## Skipped videos (189)

**non-baking cooking (Indonesian savoury dishes)** (7): `ukXeXllkPJk` Opor Ayam | Indonesian Chicken Curry Recipe; `HTDikVZp198` The Poisonous Superfood Eaten By Millions | Cassava Leaves a; `6IcvYq6zgPQ` How to Cook Bok Choy with Oyster Sauce Mix! | Best Blanched ; `Qd58g2lXAkk` Authentic Indonesian Fried Chicken with Crunchy Spiced Flake; `Bw9NIxHrAGI` Authentic Indonesian Balado Sauce; `h0loPxRHn04` Soto Ayam - Traditional Indonesian Chicken Soup; `YTXw4oYx9OY` Authentic Indonesian Chicken Curry | Gulai Ayam | The Best R

**aquarium/betta fish** (51): `Rri5NpQeYqo` Probiotics for Your Betta Fish; `eW_vMvgZkI0` Our New Outdoor Betta Fish Setup; `0TS77v-ce1U` 5 Interesting Betta Fish Types | Double Tail, Extended Red, ; `9yBdr9RwBbI` Betta Fish | From Ornamental to the Next Big Model Lab Anima; `7mOGBL8VwOI` Shocking Betta Fish Transformation Within 3 Months!; `K0_JS_k16rw` A Cat in the Betta Fish Room; `EjMxtcn5rdg` Life of Betta Fish |  A Quick Glance at Each Phase of a Bett; `N2D20uevmi0` Betta Fish Aggression | They Fight as Soon as They Can Swim!; `UoMFLiG5Y8A` Changing a Betta Fish from Female to Male | The Research Beh; `geq44ZM_C2U` You're more related to Betta fish than you think - and why t; `IM_0jpLYkjo` Betta Fish in a Music Video | Are You Alive or Just Breathin; `fLUenALzRVw` New! Amazing Betta Fish Facts; `lwkKbnu_7dE` How to Raise a Giant-sized Betta Fish | Giant Bettas; `3InvND5XTU8` Bagan Betta Fish : The Ultimate Fighting Fish; `R9f0w_ACFv0` Wild Betta Fish | Not Your Everyday Bettas; `vfZmCdC6y6Y` 4000 Years of Domestication: What Have We Done to Betta Fish; `bPewiUcmKkA` Ladies of the Betta Sorority Tank | Introducing Trouble; `yUI7tFGokec` Betta Fish | Moments of Love; `Gh62co4gZRs` Simply Betta Fish: Stars in the Night Sky with aquarium side; `F1PS2BNCIPI` Get to Know More About Your Betta Fish: Selective Breeding a; `z8BkBpl3tdM` Betta Fish Breeding: How to Successfully Raise Betta Fry; `Xx5pqHsXZ_U` Betta Fish Breeding: Romance is Hard; `ySrAw0v5qFA` Betta Fish Breeding: Love Story; `XhdToIKd4qE` Betta Fish Flaring; `y9wKUO4mlPs` Dry Leaves for Your Betta Fish (pH Test); `gGpwTNLBlgY` Basic Betta Fish Color Genetics; `P_7ToY_CdCw` A Beautiful Betta Fish World | It's a Beautiful World; `wDx-QFUND-s` What Your Betta Wants to Eat; `SBcaYQOOvrk` Cherry: A Shrimp Life; `rGdaed4UYGY` What Determines A Betta Spawn's Sex Ratio? | More Males or F; `z5cHX5-oTPI` Two Bettas in Love | Betta Love Story; `lEKZQ_DQ_DU` Pond Snails | Pros & Cons; `rGs1631VoqI` Do Guppies Like Seaweed or Maybe Carrots? | Best Food for Gu; `qlSPcF2_DkQ` Giant Betta Fish; `jn5wUF8GvJA` Juvenile Bettas Living Peacefully with Guppies; `lmrWmdrHja0` Jedi Fish | Rise of the Guppies; `Ud19X9MqCvI` The Best Leaves for Your Betta Fish | Tropical Almond Leaves; `YCGbbrdErm0` Betta Parents and Their Offsprings | Tales from My Sorority ; `UBLpQI-HDmY` Important Tips on Betta Fish Care; `e-84inGmAXM` When A Guppy Fish Having Babies; `6Qbv0TjTQmQ` Amazing Facts About Guppy Fish; `2Gbhjh7gFXs` A Closer Look at Betta Fish; `0NKM8zz17Oc` More Amazing Betta Fish Facts; `-mJrl_4jcKk` The World According to Gill | Escape from the Green Island; `yN63ZOjFKZk` Betta Fish Saga | The Story Begins; `FA64O6_gqck` Amazing Betta Fish Facts; `_K0sfcUp3ew` The Complete Betta Fish Life Cycle in 3 Minutes; `iCelAgw7OwY` Betta Fish #Shorts : Easy Afternoon; `iHU3oYBNbt4` Betta Fish #Shorts : Like a Boss; `_jkdR6bYaos` Betta Fish #Shorts : A Rebellious Son; `ntRwGrIlOJ8` Betta Fish #Shorts : HUNDREDS of Betta Fry| Under the Water 

**nature/animals** (36): `KF3Shqxi1oY` We found a pregnant cat; `fnpIFeX8Ur0` Weaver Ants  | A Closer Look at the Colony; `ckdQ47nKyjM` Amazing Facts about Mammals; `pmCm0-Tswu8` The Story of Orchids - Aphrodite's Sandal | Paphiopedilum; `Ef2TpHHL3VI` The Impostor - Kerengga Ant-Like Jumper | Myrmaplata platale; `PhH4SRvN67k` Weaver Ants | The queens, the princesses, the drones and the; `qtW0hoUzBW4` Moringa - The Super Tree; `MWywJgi_s_U` Shot In The Neck! | Saba Turtle Conservation; `gwPIHIcFYg0` This Adorable Eagle won't eat unless... | TWA Gunung Tunak; `FA2XT22eczE` Incredible Butterfly Park!; `PhtUi4hSmYg` Heading Into A Super Dark Cave Filled With Bats!; `JAs_ha_K2pM` Adorable Lion Cubs at Play; `UQ_2FOFoXFY` A Wild Day under the Sun; `mulyEi4-nfA` Why the Sea Smells like That; `UzPl5oPK-DM` Big Ant Story; `x4Ma9T9iZkg` A Brief Dive Down a Tropical Underwater World; `JLifMe0p5rs` Amazing Facts About Ferns; `K9k5vXU18nM` Ants and Aphids | A Story of Masters and Slaves; `exI9D706TMg` When Ants Meet Candy; `3TT61JyICS4` Up Close With Exotic Animals; `qUYkO3Gp39I` Fun at the Zoo; `jJJN14ZtJvA` Cute and Majestic Animals at Bali Zoo; `aBIO_pQl290` Why do roosters crow?; `la3LhlVpYB0` And So We Fly; `DY0Fz9_fhsE` Because It's Tough To be a Bird; `xdr4uB2JWu4` Greetings from the Animal Kingdoms; `ZRP1WDm-yTg` Binturong - The cute bearcat; `33EsV-fVUR8` Good Morning, Greetings from Bali Zoo; `tZVsm2nmilI` A Day with Elephants in Bali; `vh6uev6yYo8` Exotic and Awesome Animals of Bali Zoo; `8GGpCMRcHvo` The Cutest of Them All; `R6_kqdbtQAU` Relax with Orchids 蘭の花でリラックス; `rE2f-PLngpY` Tropical Flowers - Take Two HD; `EIoUqBcCMf4` The Story of Orchids - Aphrodite's Sandal (Paphiopedilum - O; `vrjDdyrBHIs` Tropical Flowers HD; `J6ZG5r69iWg` Have you ever seen a spider pretending to be an ant? | Ant m

**unrelated (culture/language, photography, essays, DIY, channel meta)** (24): `CTYyQFVUK5w` What is Chinese WangHong?; `QPlJ8vXTdkA` Anything but Passion - Red (红 赤 朱) in Chinese Culture | Lang; `CDAeeWWrPco` The Chinese Grue Character: Qīng 青 | Languages and Cultures ; `xEgJkEN1OQQ` Should You Be A Specialist or Generalist?; `e52OuDvmlKQ` How Things Got This Cheap? | Gen Z, Capitalism, and Technolo; `DVP_1WfWe54` Artificial Intelligence | AI; `KdLxDAJpvXE` Why This Channel Has Such Diverse Videos? | Betta Fish, Food; `GjEash8dbTI` 5 Homemade Natural Mosquito Repellents - Which ones work? | ; `PWtv0PfB3qA` DIY Shower Gel How To | DIY with Seraphine; `tNCCLDTYt4M` Drone Test Flight | DJI Spark; `X5TSHSslw6E` The Cuteness Power; `coevGUJ4zU0` GoPro Stop Recording, Please!; `myuvzWz4psc` 7 Reasons Why Gossipers Aren't Happy; `uREIsWEoGro` Headshot Photography - Tips on How to Look Better; `x2pQDZ1BKwM` HDR Photography - Photoshop Tutorial; `Lkh6RHNbBdA` Macro Focus Stacking in Photoshop; `XBoPBF8AG2o` The Pleasure of Reading Feynman's Books; `TKoYYNsSXLw` Basic Eye Make Up with Photoshop CC; `-h38W65_eNc` Raw File Editing in Photoshop; `E4O1KHNHOHA` Portrait Retouching with Photoshop CC; `19asEHFJ3pc` General Tips on Sunrise and Sunset Photography; `UvPvWOtmoYk` Photo Composition Tips; `tqzVhDBugCU` Just-world Fallacy; `jvkhIsrMfNI` Tales of a Parallel Universe - The Violent Beginning

**food but not baking (stingless bee honey)** (1): `UfNG-CCIhkE` The Many Benefits of Stingless Bee Honey | Tetragonula Laevi

**travel / Bali-Lombok life / culture** (66): `jEWjhVFXcWI` The Story of a Resort Owner during the Pandemic | Meet You T; `tiw8UFo9UxM` Life in a Traditional Balinese Village during the Pandemic |; `hN_lXgaEp1o` The Sacred Sanghyang Dance of Bali; `fPd4-hiPCtg` The Traditional Ceremony with Hundreds of Roasted Pigs: Usab; `xXjJfPqdbUE` Jimbaran Fish Market | Parrotfish, Octopus, and More Fresh S; `4Fwhpow1OGg` Sons of Sembalun; `3R06AuhIgTk` 10 of the Most Beautiful Destinations of Bali!; `IjckVclH0Aw` Jagasatru Waterfall, Bali; `98ZyOVMiA6s` Trying on Traditional Balinese Dance Costume and Makeup!; `xGN2G-ILfT8` A Trip to Taman Nusa | Bali; `NO1iWULAgEk` A Trip to Goa Gajah (Elephant Cave) | Bali; `0i4VTFMb254` These plants felt... | Swimming in a River in Bali | Babahan; `T5aSDbk6G_I` A Trip to GWK (Garuda Wisnu Kencana Cultural Park) | Bali; `-7-MC9Cpuh8` HOLD ON TIGHT! | Taro ATV Ride; `U2uODQww1mU` You have to see them! | AMAZING Green Destinations of Bali; `DcgCcN7LvoY` Watch out for the waves! | Pantai Bias Tugel, Bali; `1iOacCJdxNY` Giant Bird Nest, Clam Hunting | This Day was Amazing! | Lomb; `8BOXMrNfbgM` Island Hopping | Why Travel?; `dQ74bUi30bs` Fun Jumping; `g-4csS2b678` Fun Adventure!; `U27A3weAHVc` Bilasayak Beach at Gunung Tunak National Park Lombok; `_dsyL7rXZ4M` Keramas Beach - ATV Adventure; `zXByTYygx1s` Incredible Royal Record Breaking Ceremony!; `e0L9lkb69O8` Buffalo Ride at Jatiluwih, Bali; `TmyXyUOo8dI` Rainy Season at Jatiluwih and a Visit to Yeh Hoo Waterfall; `4cNAQP9jN6U` Gunung Kawi Tampaksiring, Bali; `bZrBfeM0KFo` Candi Tebing Tegallingah | 800-Year-Old Cliff Temple In Bali; `QqwVjzB_MD0` Hidden Canyon of Beji Guwang, Bali; `EsujqRz6Zsg` Mount Sangkareang; `vtd48hmK0Qk` Mount Agung; `QG1xybfw0G0` Ijen Crater | One of Only Two Blue Flames in the World; `6cAsbbh64z8` Eating Gourmet Chocolate and Trying Out Jungle Buggies in Ba; `VkVSI8aRzNw` Traditional Beauty of Bali; `uLLHEOMKVAs` Sembalun - The Hidden Beauty at the Foothills of Mount Rinja; `XtqJpuLW3Hs` Mangku Sakti Waterfall, Lombok; `RGXmVPGbNLY` Deep into the Sesaot Forest, Lombok; `SW3fwXVdF4c` Pergasingan Hill, Lombok - What an Amazing Hike!; `GxnQA77k3Ww` The Incredible Mount Ijen in Banyuwangi; `mXvFZoVTdYQ` ATV Ride, Bali; `NvtnJg3dN68` Elephant Ride, Bali; `9UkcwoCYTXQ` Exploring Paradise; `WTQsYgDL3co` Goodbye Jakarta; `u718bstJ6jE` One Day in Jakarta; `hFpBxQKF3qw` 1,200 km Road Trip; `kwKnxKV65sI` Getting Ready for a Big Journey; `1kHADoMeWLc` YouTube Happy Hour Bali 2016; `gevNlpe7DTI` Colors of Bali; `wb_XNwDmjpQ` An Afternoon at Kelating Beach, Bali; `Bh0BLQnWSvU` Monkeying Around With Monkeys: Sangeh Monkey Forest, Bali; `9k86kpKPARA` Beautiful Yoga on a Paddy Field; `191RElu30c4` The Hunt for Pokémon, Kuta, Bali; `e5_seUUz8Co` Beautiful Yoga at Sanur Beach, Bali; `D8CRrODKTK4` A Wonderful Day in Munduk, Bali; `ZYHmyaXOWhM` Exotic Sumbawa: Wild Horses and More; `QnBZEUQXKAE` A Story of Wangsean Village | ワンセアン村の物語 | Sebuah Cerita dari; `-vIjWF4RFyA` Her First Time Tubing at Wangsean Village; `0pRi2KAJmts` Cendrawasih Dance | チャンドラワシィ（極楽鳥）の踊り| Tarian Cendrawasih; `mYH8w1K9V9k` Omed Omedan, the Kissing Ritual of Sesetan, Bali; `L8gVCiKFePA` Ogoh Ogoh Denpasar Bali - Parade; `rAlADd2pjgc` Wild Horses and Buffaloes of Doro Ncanga, Sumbawa(HD); `R_f9Nf5PVBI` Bali Arts Festival - Pesta Kesenian Bali (FULL HD); `fGMHaW5NEyU` MOUNT RINJANI | GUNUNG RINJANI (FULL HD) - Lombok Indonesia; `MlyrNu4N450` Omed Omedan, The Kissing Ritual of Bali; `hif5kvvwo3Y` Ogoh ogoh Denpasar Bali HD; `iVUHRnIT5Ig` Bali Sanur (Pantai Sanur) HD; `Iw2MWaZghCo` Sights and Sounds of Bali: Procession at Nitimandala Today

**travel/food vlog (fruit market)** (1): `SmhOdUFEmHg` How much fruit can you get for $5 in Bali?

**non-baking cooking (soy milk)** (1): `OPttxh4dvg4` Soy Milk 101: Making Delicious and Nutritious Soy Milk Based

**non-bread fermentation (tempeh); description only, no transcript content** (1): `v8lWkIbmrmg` Tempeh | How to Make This Healthy Super-food

**travel/culture (palm-wine tapping vlog)** (1): `W8rSJ_v_-R8` The Making of a Traditional Drink: Tuak
