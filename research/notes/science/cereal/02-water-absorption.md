# 2. Water absorption and the hydration method

Farinograph absorption is the water (% of flour, normally at 14 % moisture, 500 BU, no salt) that brings a dough to a standard consistency (science-flour-0055). It ranks flours; it is not the hydration of a bread. The calculator procedure is `model-flour.json` → `hydration_method`.

## Published absorption equations

| Equation | Data | Source |
|---|---|---|
| FA = 42.7 + 1.03 P + 0.248 D (P protein %, D damaged starch, Farrand units); r² 0.993, SE 0.42 | 20 Canadian HRS flour streams, P 11.5–19.2 %, D 15–89 FU | Tipples, Meredith & Holas 1978, Cereal Chem 55:652, Table V (science-flour-0048) |
| FA = 58.0 + 0.207 D; r² 0.897 | same | same |
| WA = 68.26 + 0.878 P + 0.334 SD − 1.97 M (M flour moisture %) | Farrand 1969, as quoted by Lovegrove et al. 2020; explains 83–86 % of UK variation, about 90 % with fibre components, β-glucan the best single addition | [doi:10.1002/cche.10316](https://doi.org/10.1002/cche.10316) (science-flour-0047) |
| Baking absorption RBA = 22.8 + 10.9 (wet gluten/protein) + 0.49 P; r² 0.929 | long remix process, 0.3 % malt | Tipples 1978 Table IV (science-flour-0049) |

About 1 point of absorption per 1 % protein and per 4 Farrand units of damaged starch; in a long malted process damaged starch *lowers* baking absorption (1 point per 13 FU) because amylase digests it and frees its water. Lovegrove does not state the unit of damaged starch; with damage in % of flour the Farrand intercept gives implausibly low values (53 % at 11.5 % protein, 7 % damage, 14 % moisture), so the model uses only its slopes.

Summing component capacities does not work: damaged starch is given as 1.75, 2 or 3–4 g/g, gluten 1.35–2.15 g/g, arabinoxylan 3.5–15 g/g depending on the source (science-flour-0050 to -0053).

## Steps adopted

1. **Flour absorption**: the mill value, else the type default, shifted by +1.0 point per protein point above the type's protein (Tipples). Default for common wheat: A₁₄ ≈ 59 + 8 (ash % dm − 0.55) + 1.0 (protein − 11.5), ± 3 points (science-flour-0070, extrapolation anchored on a measured type-530 flour at 60.6 %, an unsalted flour at 62.8 %, the T55→T150 recipe gap of +8 points and refined→wholemeal gaps of +5.5 to +8). It underestimates hard red spring flours with high starch damage (66.0 % measured at 12.7 % protein; science-flour-0035).
2. **Moisture**: A_M = (A₁₄ + 14)(100 − M)/86 − M (science-flour-0056). Mass balance at constant water per dry matter; at A = 60 it gives −1.86 points per moisture point, close to Farrand's empirical −1.97.
3. **Blend**: mass-weighted mean. An industry blend table (wheat 60–62 %, rye 82–86 %; science-flour-0030) departs from the linear mean by −0.8 to +2.4 points, most at 80/20 wheat/rye (science-flour-0057): ±2.5 points for wheat–rye blends.
4. **Base hydration**: mode A uses the types' typical bread hydration; mode B keeps a working recipe's hydration and adds only the absorption difference of the new flours. Mode B exists because process sets hydration as much as flour does: the same T65 takes 62 % in a straight dough and 68 % in a long-fermented tradition dough (science-flour-0006).
5. **Additions** (g water per g, at equal consistency where measured; see note 6).
6. **Scald** shift (note 4).
7. **Salt**, only when comparing with unsalted farinograph values: 62.8 → 59.7 → 58.9 % at 0 / 1 / 2 % NaCl in one flour (science-flour-0058).
8. **Allocation**: total water = H × F; the dough gets what is left after scald, soakers and preferments; a negative remainder is an error.

## Reference hydrations

- French reference recipes (Bourre 2008 annex; science-flour-0006): T55 and T65 62 %, T65 levain 64 %, T65 tradition 68 %, T80 66 %, organic T110 levain 66 %, T150 boule 70 %; rye bread 66/33 rye/wheat with 1 % gluten 65 % (science-flour-0007).
- German dough yields (Loderbauer via Wikipedia; science-flour-0031): wholemeal/meal TA 176–180, rye 175–178, ciabatta 170–175, baguette 165, wheat/toast 154–158, rolls 156–158, pretzel 150. IREKS gives water by wheat/rye ratio from 55–57 % (wheat rolls) to 84–88 % (coarse rye) (science-flour-0030).
- US HRS flours: baking absorption 68.3–71.2 % refined, 78.3–79.2 % wholemeal (science-flour-0035).
- Measured farinograph values: spelt white 55.95 % (SD 2.07; science-flour-0026); ancient Italian soft wheats 52.3–59.0 %, khorasan semolato 55.8 %, durum semolato 62.0 % (science-flour-0027); Italian type 0 / 1 / integrale of ancient varieties 54.7–64.9 / 56.7–63.8 / 57.1–66.0 % (science-flour-0028); light rye 59.0–68.6 % (Mixolab), wholemeal rye 63.8–70.6 % (science-flour-0034); micronized wholemeals wheat 87.4, rye 83.4, spelt 76.4, barley 70.6, buckwheat 63.4 % (science-flour-0029, blend-derived).
- Creator rules: whole wheat +5 g liquid per 120 g flour (science-flour-0039); spelt about 10 % less water at 50 % substitution (science-flour-0040); einkorn 15–20 % less liquid (science-flour-0041); buckwheat ≤ 30 % of bread flour (science-flour-0042). Flour strength maps to hydration: W 150–250 50–60 %, 250–350 60–70 %, > 350 70–90 % (Sourdough Framework, CC BY-SA; science-flour-0043).

## Worked examples (recomputed)

Mode A: 800 g T65 + 200 g rye T130 → (800 × 66 + 200 × 80)/1000 = 68.8 %; add 20 g vital gluten × 1.5 + 100 g sunflower seeds × 0.5 + 50 g oat bran × 1.6 = 160 g → 84.8 % (848 g water). Flour uncertainty (800 × 5 + 200 × 5.5)/1000 = 5.1 points, plus 2.5 points for a wheat–rye blend with 20 % rye = 7.6 points (76 g); total √(76² + 8² + 20² + 17.5²) = 80.9 g = ± 8.1 points; 130 g goes into the soakers, 718 g to the dough.

Mode B: a T65 recipe at 68 % moved to 70 % T65 + 30 % T150: 68 + (0.7 × 60 + 0.3 × 67 − 60) = 70.1 % (± 1.6); if the new flours are at 12 % moisture, A = (62.1 + 14) × 88/86 − 12 = 65.87 and H = 73.9 %.

## Disagreements

- Damaged starch: soft wheat 1–4 %, hard wheat 6–12 % of starch, ideal falling number 250–300 s (BAKERpedia; science-flour-0037) vs 300–360 s "normal" for type 550 (IREKS; science-flour-0032). Above about 8 % damage the gluten network suffers (science-flour-0038).
- High-fibre doughs: a 500 BU target misjudged absorption; pre-saturated oat-bran bread was best at 366 BU (science-flour-0064).

## Gaps

No open text of Rezette 2024 (minor components and absorption; captcha) or Sapirstein 2018 was available. Farinograph values were not found for einkorn, emmer, oat or chestnut flour; their model values are extrapolations.
