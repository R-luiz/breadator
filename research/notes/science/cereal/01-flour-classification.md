# 1. Flour classification and composition

National flour "types" are defined by ash (mineral residue after incineration), a proxy for extraction rate, not for strength. Protein, fibre, absorption and enzyme activity vary within a type and must come from the bag or the mill when they matter. The calculator data are in `research/data/model-flour.json` → `flours` (40 types); claims are `science-flour-*`.

## The standards

**France, wheat** (ash on dry matter, six regulatory types since 1963; [FranceAgriMer 2012](https://www.franceagrimer.fr/sites/default/files/rdd/documents/ETU-CER-farine%202010_2.pdf) p. 3, same table in the [2024 miller circular](https://boulangerie.org/wp-content/uploads/sites/2/2024/12/PJ-2-a-la-Circulaire-n%C2%B0124-Les-types-de-farines.pdf); science-flour-0001):

| Type | Ash % dm | Usual use (science-flour-0100) |
|---|---|---|
| T45 | < 0.50 | cakes, pastry |
| T55 | 0.50–0.60 | everyday white bread, viennoiserie |
| T65 | 0.62–0.75 | most breads, baguette de tradition, campagne |
| T80 | 0.75–0.90 | pain bis |
| T110 | 1.00–1.20 | semi-wholemeal |
| T150 | > 1.40 | wholemeal (complet, intégral) |

T130 (1.20–1.40 %) is a trade name, not a regulatory type (science-flour-0002). The ranges leave gaps (0.60–0.62, 0.90–1.00, 1.20–1.40) that belong to no type. In 2010, T65 was 66 % of conventional bread-making flour and T55 32 %; organic bakers used mostly T80 and T110 (science-flour-0004). Tradition flour may contain only wheat flour, gluten, α-amylase, wheat malt, soy and fava flour.

**France, rye and méteil**: T70 0.60–1.00 %, T85 0.75–1.25 %, T130 1.20–1.50 %, T170 > 1.50 %; the ranges overlap and the circular does not state the basis (science-flour-0003). Méteil now means a 50/50 wheat–rye blend.

**Germany** (DIN 10355, % dm; table transcribed on [Wikipedia](https://de.wikipedia.org/wiki/Mehl); science-flour-0011 to -0013, uses in -0101): wheat 405 ≤ 0.50, 550 0.51–0.63, 812 0.64–0.90, 1050 0.91–1.20, 1600 1.21–1.80; rye 815 ≤ 0.90, 997 0.91–1.10, 1150 1.11–1.30, 1370 1.31–1.60, 1740 1.61–1.80; spelt 630 ≤ 0.70, 812 0.71–0.90, 1050 0.91–1.20. **Rye 1800 is Roggenbackschrot**, a meal without germ (≤ 2.20 %); wholemeal products keep the germ and carry no type number. The type number is the mean ash in mg per 100 g dry matter.

**Italy** ([DPR 187/2001](https://www.normattiva.it/uri-res/N2Ls?urn:nir:presidente.repubblica:decreto:2001-02-09;187) art. 1–2, on dry matter, protein N × 5.70; science-flour-0014, -0015):

| Type | Ash max % dm | Protein min % dm |
|---|---|---|
| 00 | 0.55 | 9.00 |
| 0 | 0.65 | 11.00 |
| 1 | 0.80 | 12.00 |
| 2 | 0.95 | 12.00 |
| integrale | 1.30–1.70 | 12.00 |
| semola (durum) | 0.90 | 10.50 |
| semolato | 0.90–1.35 | 11.50 |

Moisture ≤ 14.50 %. Re-milled semola ("rimacinata") is permitted only for bread making and retail. Secondary tables putting 00 at ≤ 0.50 % ash do not match the decree.

**United States** (21 CFR 137; science-flour-0016 to -0018): "flour" is not typed by ash; its ash (moisture-free) may not exceed protein/20 + 0.35 (12 % protein → 0.95 %). All-purpose, bread and high-gluten flour are commercial protein classes, not standards. Whole wheat flour keeps all natural constituents; semolina ash ≤ 0.92 %.

## What a type tells you about the rest

Pooled French flours (Bourre et al. 2008, [doi:10.1051/mnut/2008442049](https://doi.org/10.1051/mnut/2008442049), Table 2; science-flour-0005) had about the same protein in every wheat type, 11.4–11.9 % (N × 6.25, as is), while fibre rose from 3.9 % (T55) to 12.5 % (T150). Ciqual 2020 gives lower protein for T55 (9.9 %) (science-flour-0008). **Protein is chosen by wheat blend, not by type.** US classes: all-purpose about 10.3–11.7 %, bread 11.5–13.5 %, high-gluten about 14 % (science-flour-0019, -0021, -0022).

Measured analogues fill the types without their own analysis: German types borrow French values of the same ash class, Italian types their legal floors. These carry `evidence: extrapolation` in the model.

## Bases and factors

- **Ash**: French, German, Italian and US limits are on dry matter. Composition tables are as sold.
- **Protein**: EU labels use N × 6.25 on the food as sold (Regulation 1169/2011 Annex I); the Italian minima use N × 5.70 on dry matter, so 9.00 / 11.00 / 12.00 % become about 8.4 / 10.3 / 11.3 % on a label at 14.5 % moisture (science-flour-0062). US mill specifications are usually on a 14 % moisture basis (novita-0636).
- N × 5.7 = 0.912 × (N × 6.25). Ciqual prints both.

## Data-quality warnings

- Ciqual's T65 row (14.9 % protein, ash 1.06 % as is = 1.22 % dm) is inconsistent with its own type and is not used (science-flour-0008).
- Ciqual's T170 rye row is the USDA "dark rye" entry; US dark rye is a clear-flour stream, not wholemeal (science-flour-0009, -0102). Wholemeal rye has about 11 % protein on dry matter (science-flour-0029).
- Bourre et al. describe their rye bread as T130 rye in the text and as T85 rye in the recipe annex (science-flour-0007).

## Label equivalences are approximate

The Sourdough Framework equates cake = T405 = T45 = 00, all-purpose = T550 = T55 = 0, T812 = T80 = 1, T1050 = T110 = 2, whole = Vollkorn = T150 = integrale (science-flour-0045). By ash, Italian 0 (≤ 0.65 %) spans T55 and the lower T65, German 812 spans T65 and T80, and US classes have no ash equivalent.

## Ancient grains, rye and non-wheat flours

- Spelt, einkorn, emmer, khorasan and durum are all *Triticum*, i.e. wheat under EU gluten law; rye, barley and oats are gluten cereals too, oats only qualify for "gluten-free" when specially produced and ≤ 20 mg/kg (Regulation 828/2014; science-flour-0046).
- Wholemeal einkorn, emmer and spelt had 14.5–15.2 % protein on dry matter and 2.1–2.2 % ash (science-flour-0025); stone-milled emmer and spelt flours 15.9–16.0 %, khorasan 11.5 % (science-flour-0099).
- Rye: 86.5 % endosperm, 10 % bran, 3.5 % germ; arabinoxylans, not gluten, bind its water (science-flour-0098). Modern hybrid ryes are poorer in enzymes and absorb up to 10 % more water than 1960s ryes (science-flour-0033).
- Buckwheat, oat, barley, corn, rice and chestnut composition: Ciqual and USDA (science-flour-0010, -0020). Chestnut flour is very dry (5.5 % water) and starch-poor (46.9 %).

## Enzyme activity per flour

Falling number (FN) is an inverse measure of α-amylase. Blends mix linearly in liquefaction number LN = 6000/(FN − 50), not in FN (science-flour-0063); the model expresses activity as LN relative to a 320 s white flour. Normal bands: wheat 550 300–360 s, rye 997/1150 180–240 s (science-flour-0032). Measured: wholemeal wheat 324 s, rye 200 s, spelt 238 s (wholemeal) to 383 s (white), barley 389 s, buckwheat 918 s (science-flour-0029, -0026); khorasan semolato 591 s and an ancient durum semolato 473 s (science-flour-0027).

## Cold-retard tolerance

No study measures retard limits per flour type. The model's classes (high 48–72 h, medium 12–24 h, low ≤ 12 h; science-flour-0071) are an extrapolation from flour strength and fermentation length (science-flour-0043), common 24 h fridge proofs (breadcode-0611), multi-day pizza doughs (breadcode-1323), weak ancient-wheat gluten (science-flour-0023, -0026, -0027) and high-amylase dampness (science-flour-0032).

## Gaps

- The 1963 French arrêté text itself is only a scanned Journal officiel; the type ranges come from FranceAgriMer and the 2024 miller circular. The basis of the rye ranges is not stated anywhere read.
- No measured protein distribution for Italian 00/0 or German types; no falling number for einkorn, emmer, oat, maize, rice or chestnut; no water absorption for einkorn, emmer, oat or chestnut.
- The Codex standard for wheat flour (CXS 152) and for wheat protein products (CXS 163) could not be downloaded.
