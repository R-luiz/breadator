# 3. Protein, gluten and vital gluten

Protein content is not gluten strength. What forms the network is the glutenin polymer; gliadin makes dough extensible. The calculator data are in `model-flour.json` → `flours[].gluten` and `vital_gluten`.

## Gluten composition by species

Across 15 cultivars per species at four German sites (Geisslitz et al. 2019, [doi:10.3390/foods8090409](https://doi.org/10.3390/foods8090409); science-flour-0023), the gliadin/glutenin ratio averaged 2.5 for common wheat, 3.3 spelt, 4.0 durum, 4.9 emmer and 6.5 einkorn; glutenin fell from 16.6–19.0 mg/g (common wheat, spelt) to 12.8 (emmer) and 10.3 mg/g (einkorn). The ancient wheats had *more* protein than common wheat yet weaker dough. Gluten was 60–80 % of wholemeal protein against the usual 80–90 % in white flour (science-flour-0024), so a wholemeal label overstates gluten-forming protein.

Measured consequences:
- Spelt (90 genotypes): gluten index 59 vs 97–100 for bread wheats, farinograph absorption 6 points lower (science-flour-0026).
- Ancient Italian common wheats: alveograph W 47–87; khorasan W 57 with P/L 1.6; durum 'Senatore Cappelli' W 55; all T. turgidum samples gluten index < 10 % (science-flour-0027).
- Stone-milled ancient flours at 72–100 % extraction: farinograph stability only 1.6–3.1 min (science-flour-0028).
- The Sourdough Framework treats rye, emmer, einkorn, rice and corn doughs as "homogenize only" (no kneading, folding or shaping), spelt and wheat above 70 % of the flour as kneadable (science-flour-0044).
- Rye forms no gluten network: arabinoxylans and acid-protected starch build the crumb (science-flour-0098).

Strength classes in the model: very strong (US high-gluten), strong (US bread flour, German 550 "backstark"), medium (T55 with W 160–250, novita-0104; T65; Italian 0), weak or variable (pastry T45/405, spelt, khorasan), very weak (einkorn), none (rye, barley, oat, buckwheat, maize, rice, chestnut).

## Protein targets by bread

Creator and reference values (science-flour-0097, a compilation, `creator-rule`): steamed buns 10.5–11.5 % (novita-0772); soft rolls and lighter fried doughs 11–12 % (all-purpose); baguette about 12 % (novita-0380) and French bread flours about 11.5 % N × 6.25 (science-flour-0005); doughnuts 12–13 % (novita-0256); soft pan loaves and enriched rolls 13 % (novita-0845, novita-0552); US bread flour 11.5–13.5 % (science-flour-0021); 100 % whole wheat ≥ 13.5 % (novita-0032); bagels and chewy hearth breads 14 % high-gluten (science-flour-0022). UK bread-making flours run 10.5–12.5 % (science-flour-0050).

## Vital wheat gluten

Composition: about 73 % protein (78 % dry basis), 6 % fat, 6 % moisture, < 1 % ash; absorbs 140–180 % of its weight; dried above 55 °C it loses function (BAKERpedia; science-flour-0060). USDA: 75.16 % protein, 8.2 % water (81.9 % on dry matter; science-flour-0061). Usage 1–4 % of flour in yeast doughs, up to 12 % in high-fibre breads.

**Dose** (mass balance, science-flour-0096): to raise flour of protein P_f to P_t with gluten of protein P_g, add

G = F × (P_t − P_f) / (P_g − P_t)

with all values on the same basis and nitrogen factor. 1000 g at 11.0 % to 13.0 % with 75 % gluten: G = 1000 × 2.0/62.0 = 32.3 g (check: (11 000 + 32.26 × 75)/1032.26 = 13.0 %). If total flour must stay 1000 g, replace x = F (P_t − P_f)/(P_g − P_f) = 31.25 g of flour. On a dry-matter basis (flour 14 %, gluten 8.2 % moisture) G = 32.6 g, 1.2 % more. BAKERpedia's "+0.6 protein point per 1 % gluten" is the same rule: (100 × 11 + 73)/101 = 11.61 %.

**Water**: +1.5 points of dough water per 1 % gluten (BAKERpedia), within the 1.35–2.15 g/g given for gluten in component tables (science-flour-0051, -0052); the worked example needs 48 g extra water.

**Bases**: N × 5.7 = 0.912 × (N × 6.25); EU labels are N × 6.25 as sold; Italian minima N × 5.7 on dry matter; US specs at 14 % moisture (science-flour-0062, novita-0636).

## What protein does not tell

- Process changes the water a flour carries more than protein does (note 2).
- Wholemeal needs about 1.5 points more protein for the same gluten (science-flour-0024, novita-0032).
- Weak, extensible ancient-wheat doughs tolerate long cold retards poorly; the model's retard classes are extrapolations (science-flour-0071).

## Gaps

The Codex standard for wheat protein products (CXS 163) could not be retrieved, so the regulatory minimum protein of vital gluten is not stated. No alveograph W was found for French T65, German types or typical Italian 00 and 0 flours.
