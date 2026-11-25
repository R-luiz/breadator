# 8. Endpoints, personal calibration and uncertainty

## What "done" means

The engine works on the rise of an aliquot: a small sample taken right after mixing, pressed flat in a straight-sided jar, kept at the dough's temperature. R is its fractional volume gain: 0.5 for +50 %, 1.0 for doubled. A small jar cools faster than the dough in a fridge, so the aliquot is a bulk tool, not a retard tool.

### Sourdough

τ = ln(K / ι) / μ, with K = R + ι.

| endpoint | R | K |
|---|---:|---|
| +25 % | 0.25 | 0.25 + ι |
| +50 % | 0.50 | 0.50 + ι |
| +75 % | 0.75 | 0.75 + ι |
| doubled | 1.00 | 1.00 + ι |
| starter at its peak | — | 1 |

The mapping is supported by measurement: one growth rate reproduces both the dose slope and the +75 % to +100 % spacing of 40 timed doughs (note 3), and Wraith's doubling times follow ln(1 + 1/ι) within 4 %.

Which R to aim for is a baking decision, and two practitioners frame it differently:

- by flour strength: +25 % for 8–10 % protein, +50 % for 10–12 %, +100 % for 12–15 % (The Sourdough Framework, Table 7.3, CC BY-SA 4.0);
- by dough temperature before a cold retard: +30 % at 27 °C, +50 % at 24 °C, +75 % at 21 °C, +100 % at 18 °C ([The Sourdough Journey](https://thesourdoughjourney.com/the-mystery-of-percentage-rise-in-bulk-fermentation/), guideline table).

The second rule exists because a warm dough keeps fermenting while it cools (note 7). The engine integrates the retard, so it can state one target for the whole process (bulk plus retard) and derive the bulk cut-off for any dough temperature. The flour-strength rule then sets that overall target.

### Yeast

τ(endpoint) = f · τ_doubled.

| endpoint | f | range | evidence |
|---|---:|---|---|
| +50 % | 0.70 | 0.55–0.85 | extrapolation |
| +75 % | 0.85 | 0.75–0.93 | extrapolation |
| doubled; pizza ball ready to bake | 1.00 | 0.8–1.3 | creator-rule |
| tripled | 1.60 | 1.4–2.0 | extrapolation |
| ripe poolish | 1.90 | 1.4–2.3 (tables: 1.6–3.2) | creator-rule, 48 rows |

The fractions for +50 % and +75 % sit between two limits. If gas production is level, as measured at a bakery dose over three hours ([Codină et al. 2013](https://www.agriculturejournals.cz/publicFiles/216_2012-CJFS.pdf), Table 2), the rise is near-linear after a short lag and +50 % comes at 0.60 of the doubling time. If the yeast is still multiplying, as in long low-dose doughs, the rise accelerates like a sourdough and +50 % comes at about 0.79. The engine takes the midpoint and a wide range. The baseline "doubled" matches independent doubling statements (median factor 1.04, n = 5; note 1).

### Proof after shaping

Progress carries over: shaping does not reset the organisms. The proof target is expressed in the same variable. Structure is a separate matter: a degassed, shaped loaf shows less volume for the same fermentation, and the finger-poke test reads structure, not gas produced. No validated proof-height model was found; the engine reports fermentation progress and leaves the visual check to the baker.

## Personal calibration

Starters differ by a factor of three between sources (vigour 0.51 to 1.59, note 3). One number per culture absorbs most of it.

- **Vigour v** multiplies the rate: μ_ref for sourdough and starter peaks, 1/τ for yeast.
- **Prior:** ln v is normal with mean 0 and standard deviation 0.35 for sourdough (the spread across six sources) and 0.45 for yeast (the spread inside groups of independent tables and recipes, 0.35–0.56).
- **A logged bake** gives y = ln(engine hours at v = 1 along the logged temperatures / observed hours to the logged endpoint).
- **Noise per bake:** 0.20 in log units (range 0.10–0.30). Tabulated sources scatter by 0.10; home logs add reading and thermometer error.
- **Update:** precision = 1/s0² + n/se²; mean = (Σy / se²) / precision; v = exp(mean).

| logged bakes | 0 | 1 | 2 | 3 | 5 | 10 |
|---|---:|---:|---:|---:|---:|---:|
| sd of ln v (sourdough) | 0.35 | 0.17 | 0.13 | 0.11 | 0.09 | 0.06 |

Replay on two sources treated as one baker each (`fits/calibration_demo.py`): for a starter 1.6 times faster than the reference, the median error falls from 57 % with no log to 12 % after one bake and 4 % after three. For the reference source itself it stays near 9 %, which is the scatter of that table. The replay tests the update, not the transfer between kitchens.

A recipe family that departs from the reference (sweet, spiced, rye) can carry its own v on top of the culture's.

### Check on a private bake log

A private log of 13 home bakes was available for validation; only aggregates are reported (`fits/private-bakes-validation.json`). Three rows had no duration or endpoint.

- **Sourdough bulk and starter refresh at room temperature (n = 4, one an upper bound).** Median ln(observed / engine) −0.43, range −0.48 to −0.29, all inside the uncalibrated band. Around their own median the four scatter by only 0.09, so a single vigour factor would have brought them within 10 %. This is the pattern the calibration assumes: a home culture can sit well away from the reference, and one number absorbs it. The small scatter holds only with the hydration term applied to the starter itself (note 9).
- **Yeast, direct dough at room temperature (n = 2, one a lower bound).** Both ran slower than the engine (range of ln +0.45 to +1.26). The temperatures were planned, not measured. Together with recipe schedules running 1.2–1.6 times the engine (note 1), this marks bakery-dose doughs judged at +50 % as a case where the engine is likely short.
- **Preferments, a rich dough and fridge cases (n = 4).** Two numeric rows within ln −0.33 to +0.12. Two fridge cases showed far less visible rise than predicted. Rich doughs and preferments dosed far above the validated range are flagged as failure regions when cold.

## Uncertainty

80 % band = hours × exp(∓1.2816 · sd), with

sd² = sd_v² + se² + (0.03 · min(|T − T_anchor|, 10))² + Σ modifier terms² + Σ domain penalties²

- sd_v: what is left of the prior after the logs.
- se: 0.20.
- Temperature term: 3 % per °C away from the temperature the logs were made at (25 °C when there are none). It stands for the gap between the engine's temperature slopes and the flatter ones of independent data (notes 1 and 3).
- Modifier term: ln(high / low) / 2.56 for each modifier that is not 1.
- A penalty is halved once at least two logged bakes lie in that same domain.
- Domain penalties, in log units: yeast below 8 °C 0.50; sourdough below 10 °C 0.40; sourdough above 30 °C 0.30; yeast above 35 °C 0.40; dose outside the validated range 0.25; prediction under 45 min 0.30; starter state unknown 0.20; sugar above 10 % or fat above 20 % 0.25.

| case | band (× and ÷) |
|---|---:|
| sourdough, no log, 25 °C | 1.68 |
| sourdough, three logs, same temperature | 1.34 |
| yeast, no log, 25 °C | 1.88 |
| sourdough retard at 4 °C, no log | 2.3 |
| sourdough retard, three logged retards | 1.47 |
| yeast in the fridge, no log | 2.7 |
| yeast in the fridge, three logged cold bakes | 1.55 |
| starter peak, no log / two logged peaks | 1.68 / 1.36 |

The engine sets an extrapolation flag whenever a penalty applies, a low-confidence modifier is active, or most of the progress is integrated outside 10–30 °C.

The penalties are judgement, sized from the scatter seen in validation: published cold yeast schedules scatter by 0.8 in log units, recipe schedules by 0.4–0.6 (note 1).

## Limits

- The 0.20 noise for a home log is assumed, not measured.
- The band is conservative on tabulated data (it covered nearly every replayed row) and untested on real logs.
- A single v cannot fix a wrong temperature slope; logging at two temperatures is the remedy, and the engine does not yet fit a personal slope.
