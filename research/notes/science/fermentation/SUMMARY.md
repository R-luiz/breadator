# Fermentation model: summary and recommended engine

The engine now returns a time, an 80 % band and an evidence label for every case, instead of declining. All parameters are in `research/data/model-fermentation.json`; every fit is reproducible from `fits/`.

## Recommended engine

1. **Dough temperature.** Compute the mixed dough temperature with a heat balance (or the 3- and 4-factor DDT and French base-temperature formulas, which are the same arithmetic). Follow it afterwards with Newton cooling or warming, or with the baker's probe readings (notes 6–7).
2. **Progress.** Integrate P = Σ Δt / τ(T_dough, inputs) across bulk, bench, proof and fridge. The endpoint is P = 1. Progress carries over between phases.
3. **Commercial yeast.** τ_doubled = A · d^(−a) · exp(−c (T − 25)) · ((Tmax − 25) / (Tmax − T))^b. Other endpoints are a fraction or multiple of the doubling time (note 1, note 8).
4. **Sourdough.** Rise R = ι · (e^G − 1) with G = ∫ μ_ref · v · M · γ_s(T) dt, so τ = ln(K / ι) / μ with K = R + ι, plus a lag for a starter that is not at its peak (notes 2–3).
5. **Starter peak.** Same law with K = 1 and no salt (note 9).
6. **Modifiers.** Multiply the rate by the factors of note 4.
7. **Calibration.** One vigour factor per culture, log-normal prior, updated from logged bakes (note 8).
8. **Uncertainty.** An 80 % band from the prior, the bake noise, the distance from the calibration temperature, the modifiers in use and domain penalties. Extrapolation is flagged (note 8).

### Parameters

| parameter | value | range | unit | evidence | valid domain |
|---|---:|---|---|---|---|
| yeast A | 0.91 | 0.6–1.6 | h at 1 % instant yeast, 25 °C, doubled | creator-rule (chart), checked | lean dough |
| yeast a (dose exponent) | 0.72 | 0.60–0.90 | — | creator-rule; 0.70 and 0.90 in two independent tables | 0.003–1 % instant yeast |
| yeast c | 0.190 | 0.12–0.20 | 1/°C | creator-rule (chart) | 1.7–35 °C |
| yeast b | 1.32 | 1.0–3.0 | — | creator-rule (chart) | 1.7–35 °C |
| yeast Tmax | 45 | 42–46.1 | °C | literature (growth) | — |
| yeast endpoint factors | +50 % 0.70, +75 % 0.85, tripled 1.6, ripe poolish 1.9 | see note 8 | × τ_doubled | extrapolation; creator-rule for poolish | — |
| sourdough μ_ref | 0.30 | 0.15–0.48 | 1/h at 25 °C | measured (n = 40) | 19–23 °C measured; 10–30 °C by curve |
| sourdough c | 0.314 | 0.25–0.34 | 1/°C | literature | 4–35 °C |
| sourdough b | 3.64 | 3.0–4.4 | — | literature | 4–35 °C |
| sourdough Tmax | 40.4 | 36–41 | °C | literature | — |
| sourdough K | R + ι | — | — | measured | R 0.25–1.0, ι 0.5–20 % |
| lag, starter from the fridge (about 7 days) | 2 | 1–3 | h at 25 °C | measured (one test) | — |
| starter K_peak | 1.0 | 0.7–1.5 | — | extrapolation, 4 statements | 1:1:1 to 1:20:20 |
| no-salt factor (sourdough) | 1.18 | 1.06–1.21 | × rate | creator-rule | — |
| fresh → instant | 0.33 | 0.25–0.42 | × weight | manufacturers | — |
| active dry → instant | 0.75 | 0.67–1.00 | × weight | creator-rule | — |
| osmotolerant switch | 10 | 8–17 | % sugar | manufacturer figure | — |
| friction, stand mixer | 12.8 | 12.2–13.3 | °C in the 3-factor formula | measured by one baker | 7 min mix |
| base temperature PVL / PA / PI | 68–72 / 60–64 / 52–54 | — | °C sum | creator-rule | oblique mixer |
| cooling τ_c, 650 g loaf in a banneton | 2.8 | ±35 % | h | measured (one loaf) | home fridge |
| cooling scaling | (m / 0.65 kg)^(1/3) | — | — | physics | 0.2–2 kg |
| prior sd of ln v | 0.35 sourdough, 0.45 yeast | — | — | spread across sources | — |
| noise of one logged bake | 0.20 | 0.10–0.30 | ln units | assumed | — |

### Expected accuracy by regime

80 % band, as a factor either way on the predicted time.

| regime | no log | after 3 logged bakes in the regime | what the fits show |
|---|---:|---:|---|
| yeast, warm (18–30 °C, lean dough) | 1.9 | 1.35 | law vs chart 5 % rms (n = 753); independent doubling statements median +4 % (n = 5); recipe schedules 1.2–1.6 times longer |
| yeast, cold (fridge, with cool-down) | 2.7 | 1.55 | published cold schedules end at 75 % of doubling, quartiles 54–133 % (n = 41) |
| sourdough, warm (18–28 °C) | 1.7 | 1.35 | measured doughs 10 % rms (n = 40); other sources within their vigour factor (0.5–1.6) |
| sourdough, retard (≤ 8 °C) | 2.3 | 1.5 | fridge rate from an extrapolated curve; agrees with one practitioner estimate; 80 % of the effect is the cool-down, which rests on one measured loaf |
| starter peak | 1.7 | 1.35 (after 2 peaks) | four statements, median +8 %, scatter 0.24 |

Add 3 % per °C between the prediction and the temperature the logs were made at, up to 10 °C.

## Coverage

**Processed (read in full unless stated).**
- Gänzle, Ehmann & Hammes 1998 (growth parameters, pH, salt, acids).
- Salvadó et al. 2011 (cardinal temperatures of *Saccharomyces*).
- Carcea et al. 2020 (salt and gas production); Codină et al. 2013 (gas over time); Šeruga et al. 2005 (dough conductivity); Aslankoohi et al. 2015 (water activity of sweet dough); Di Cagno et al. 2014 (abstract only).
- TXCraig1's yeast thread and final chart (918 cells transcribed) and his sourdough chart (14 of 61 rows), from Wayback copies; the cf-calculator source.
- Bill Wraith's table (all 512 values) and method post.
- The Sourdough Journey: bulk timetables (40 measured times), percentage-rise article (guidelines, fridge cooling curve), unfed-starter, flour-type and cold-proofer experiments, two FAQ pages.
- The Sourdough Framework ch. 7 and Tables 7.1, 7.3; the Bread Code feeding-ratio and fermentation-table pages.
- Technomitron (French training site of the INBP alumni association): base temperature, poolish table, yeast forms. BAKERpedia and Weekend Bakery poolish tables.
- Lallemand Baking Update 1/3, 1/4, 1/6, 2/9; Red Star and Fleischmann's conversion pages; King Arthur DDT and friction articles; Engineering ToolBox specific heats; US patent 5,827,724.
- A third-party compilation of 195 yeast table and recipe rows, used for validation only and not re-verified row by row.
- A private log of 13 home bakes, used for validation only. Nothing from it is published except aggregates per regime (`fits/private-bakes-validation.json`); it is not a source for any claim.

**Skipped or not reachable.**
- Calvel, Hamelman, Suas and INBP publications themselves: books were not available. Their poolish practice enters through the Technomitron and BAKERpedia tables; Hamelman's friction figure through King Arthur's quotation.
- Lesaffre and AB Mauri professional data sheets: not reachable; their consumer brands were read.
- Brandt et al. 2004 (rye sourdough, inoculum size), Romano et al. 2007 and Stanke et al. 2014 (dough volume kinetics): paywalled, not used.
- `research/data/recipes-breadcode.json`: its recipes give dose and endpoint but no temperature; they serve as a range check only (4–24 h "depending on temperature" against 5–23 h from the engine at 30–15 °C).
- Web search was not available in this session; sources were reached through known addresses, site maps, the Wayback index and literature APIs.

## Key insights

- The cf-calculator formula is a fridge-range regression. It is exact below 8 °C and too short by a factor of 2 to 4 at room temperature.
- A cardinal factor on top of the exponential, the form Gänzle used, fits Craig's chart twice as well as a single exponential and five times as well as a CTMI.
- For sourdough, the rise is inoculation × (growth factor − 1). One measured rate then explains the effect of the dose and of the rise target together. The power law of the yeast model fits no better and cannot translate between endpoints.
- The same rate and the same curve predict starter peaks, with the peak defined as the return to ripe density.
- The Sourdough Framework's Table 7.1 lands at 8.1–12.1 h at +50 % without any tuning.
- During a retard the cool-down dominates. A 26 °C loaf does about 80 % of its fridge fermentation while cooling.
- Starters differ by a factor of three across sources. One logged bake removes most of that. A private log (aggregate only) shows the pattern the calibration assumes: four sourdough and starter observations shared one offset from the reference and scattered by 0.09 in log units around it.

## Disagreements (both sides recorded)

1. **Temperature sensitivity of yeast, 18–30 °C.** Craig's chart: Q10 3.4–3.9. Compiled tables and recipes with several temperatures: about 2.1. French rule of thumb (1 °C = 10 % of the yeast): 2.0–2.6. The engine keeps the chart and widens the band with distance from the calibration temperature.
2. **Temperature sensitivity of sourdough, 19–23 °C.** Measured doughs: Q10 about 2.0. Gänzle's growth curves: about 3.4. The engine keeps the literature curve because it also has to cover the fridge; it is 15 % fast at 19 °C and 10 % slow at 22 °C on the measured data.
3. **Poolish tables.** The French table (22–24 °C) and the BAKERpedia table (26–29 °C) give nearly the same times for the same dose. Either one temperature is loosely stated or the ripeness criteria differ.
4. **Fresh to dry yeast.** Professional guidance: 3–4 to 1. Consumer packs: 2.4 to 1.
5. **What sets the bulk target.** Flour protein (The Sourdough Framework) or dough temperature before a retard (The Sourdough Journey). The engine reconciles them by integrating the retard.
6. **Does the fridge stop fermentation?** The Framework says fermentation comes to a halt near 4 °C, with a caveat. A commercial yeast kept producing gas at 3–5 °C, and a practitioner puts 24 h at 3 °C at up to 30 min of warm bulk. The engine uses 3 % of the 25 °C rate at 4 °C for sourdough and 5 % for yeast.
7. **Stiff or liquid starter, which favours yeast.** The Framework says stiff; Di Cagno et al. found the highest yeast counts in liquid sourdoughs. No rise-time comparison was found; the engine applies no factor.
8. **Starter ratios.** A FAQ illustration (1:20:20 only doubles the time of 1:1:1) is 45 % faster than a pure growth law at high ratios.
9. **Friction-factor correction.** King Arthur corrects FF by the plain temperature miss; the formula's own algebra needs n times the miss.

## Open gaps

- No measured gas rate of commercial yeast in dough from 4 to 35 °C at several doses. This would settle disagreement 1.
- No timed series of starter peaks at several ratios and known temperatures.
- No measured effect of fat, milk, egg, cinnamon, cocoa or scalds on gas rate; those modifiers are reasoned ranges.
- Sugar modifiers for sourdough are borrowed from yeast.
- One cooling curve only; no warm-up curve; no measurement for bowls, tubs or trays.
- Spiral-mixer friction is a quoted "most mixers" figure.
- Yeasted direct doughs at bakery doses judged at +50 %: recipe schedules run 1.2–1.6 times the engine, and a private log (aggregate only, n = 2, temperatures not probe-measured) ran slower still. Treat the upper half of the band as the likely side until calibrated.
- Rich doughs and very heavily yeasted preferments in the fridge: private observations (aggregate only) showed far less visible rise than predicted.
- Proof after shaping: progress is predicted, visible volume is not.
- Rye-dominant and gluten-free doughs are outside the model.
- The home-log noise (0.20) and the domain penalties are judgement; the first real bake logs should replace them.

## Files

- `01`–`09`: topic notes. `fits/`: `fermodel.py` (reference implementation), `fit_yeast.py`, `fit_sourdough.py`, `fit_starter_peak.py`, `fit_cooling.py`, `calibration_demo.py`, `fit_framework.py`, `validate_private_bakes.py` (aggregates only), `build_model_json.py`, `build_data_craig_yeast.py`, input tables in `fits/data/`, and a residual CSV and a JSON summary for each fit.
- `research/data/model-fermentation.json`, `claims-science-fermentation.jsonl` (82 claims), `references-science-fermentation.json` (23 references).
