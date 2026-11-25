# 6. Fibres, hydrocolloids, seeds and soakers

Fibres and seeds take water that the flour then lacks. Two numbers matter and they differ: the water an ingredient holds when soaked to saturation (the soaker ratio), and the extra water a dough needs at equal consistency when the ingredient is added. The calculator uses the second (`model-flour.json` → `hydration_method.addition_coefficients`) and keeps the first as `soak_ratio`.

## Arabinoxylans and β-glucan

- Arabinoxylans are 2–3 % of wheat flour (1.35–2.75 % dm, of which 0.3–1.4 % water-extractable) yet hold about a quarter of dough water (science-flour-0050, -0051). Their water binding is quoted as 5–10 g/g (BAKERpedia), 10–11 g/g (Lovegrove), 15 g/g (Feillet via Boucheham) or 3.5–6.3 (extractable) and 6.7–9.9 g/g (unextractable) in an aleurone review (science-flour-0053); Novita cites 15–20 and up to 10 times (novita-0506, -0641).
- Added arabinoxylan needs 0.5–2 points more water per 1 % to keep farinograph consistency (science-flour-0054).
- In UK flours β-glucan, not arabinoxylan, was the fibre that best improved the prediction of absorption (science-flour-0047).

## Bran

- Refined vs wholemeal flour of the same mill: +5.5 to +8.0 points of farinograph absorption and +8 to +10 points of baking absorption (science-flour-0035). Treating wholemeal as 83 % refined flour + 17 % bran and germ assigns about 1.0–1.1 g water per g of bran at equal consistency (model coefficient 1.1, extrapolation).
- A cold soaker saturates wheat bran at about 2 g/g (Plötzblog; science-flour-0068).
- Creator rule: +5 g liquid per 120 g whole wheat flour (science-flour-0039).

## Oat bran

Oat bran saturates at 1.6 g water per g dry bran (61.5 % water, aw 0.998); pre-hydrated to saturation for 2 h and replacing 20 % of white flour it gave the best enriched bread (4.21 cm³/g vs 4.68 white, 3.53 under-hydrated), at a softer consistency (366 BU) than the farinograph standard (science-flour-0064).

## Psyllium

- At equal consistency in a wholegrain wheat dough, 5 % psyllium husk took the water from 55 to 100 (flour 100 → 95): about 9.5 g per g of psyllium (science-flour-0065, derived from Torbica 2025 Table 1).
- Swelling factor about 15 mL/g; a soaker lexicon gives 20–40 g/g for husk and 10 g/g for whole seed (science-flour-0066, -0068).
- Gluten-free breads used 2.9–17.1 % psyllium with 82–118 % water, and up to 10 % psyllium was added *without* adjusting water in one study; rice breads with 0–4 % psyllium needed 90–110 g water per 100 g flour; in wheat bread about 3.5 % psyllium at 75 % hydration was best (science-flour-0066). The "no extra water" result disagrees with the 9.5 g/g measured in wheat dough.

## Flax, chia and other seeds

- Flaxseed mucilage binds 16–30 g/g of its solids and is about 6 % of the seed (science-flour-0067).
- Ground chia needed about 5.5 g water per g at equal consistency (science-flour-0065); 5 % chia flour left farinograph absorption unchanged and 15 % raised it; whole seeds slightly lowered short-test absorption and hydrate later (science-flour-0067). Pre-hydrating chia raises absorption and shortens development.
- Flax and chia reduce extensibility; seed additions up to 10–15 % keep sensory acceptance (science-flour-0067).
- Soaker guide values per 100 g solids (Plötzblog; science-flour-0068): sunflower, sesame, pumpkin seeds and nuts 50 g; cracked grains, oat flakes, buckwheat, quinoa, semolina 100 g; whole linseed and corn grits 150 g; wheat bran and cracked linseed 200 g; dried old bread 200 g cold or 300 g boiling water; potato flakes 400 g; psyllium seed 1000 g, husk 2000–4000 g. Soak 1:1–1:2 at 10–30 °C for 4–20 h (or 8–12 h at 6–8 °C); coarse material ≤ 30–60 % of the cereal products.
- Industry soakers (IREKS; science-flour-0069): soaked grain TA 160–250, up to 60 % of flour, enzymes active; hot-soaked TA 200–250, ≤ 20 %; boiled TA 350–400, ≤ 15 %. Too much soaker gives water streaks, sticky crumb and lower volume (science-flour-0090).

## Gluten-free powders

Centrifuge water-holding (1 g in 10 mL, 25 °C): maize 5.31, chickpea 4.84, faba 4.69, semolina 2.48, rice 2.36 g/g (science-flour-0052). These rank powders; they are not dough absorptions.

## Coefficients adopted (g extra water per g at equal consistency)

| Ingredient | k | range | evidence |
|---|---|---|---|
| vital gluten | 1.5 | 1.35–2.15 | literature |
| wheat bran | 1.1 | 1.0–2.0 | extrapolation (soak 2.0) |
| oat bran | 1.6 | 1.2–1.9 | measured |
| psyllium husk | 9.5 | 6–15 | measured, one study |
| chia, ground | 5.5 | 3–8 | measured, one study |
| chia, whole, unsoaked | 0 | 0–1 | literature |
| flaxseed whole / cracked, soaked | 1.5 / 2.0 | 1–2.5 | creator-rule |
| sunflower, sesame, pumpkin seeds | 0.5 | 0.3–0.7 | creator-rule |
| cracked grain, oat flakes, groats | 1.0 | 0.6–1.5 | creator-rule |
| dried old bread | 2.0 | 2–3 | creator-rule |
| arabinoxylan supplement | 1.25 | 0.5–2 | literature |

## Gaps

No per-gram water coefficient was found for β-glucan concentrates, sesame or sunflower at equal dough consistency, or for whole flaxseed in dough; psyllium and chia rest on one study each.
