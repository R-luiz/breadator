# 7. Cooling, warming and the fridge

A dough put in the fridge keeps fermenting while it cools. For a typical loaf most of the fermentation of an overnight retard happens in the first hours. The engine therefore models the dough temperature and integrates the rate along it.

## Newton cooling

T(t) = T_env + (T_0 − T_env) · exp(−t / τ_c)

One measured curve is available: a 650 g shaped loaf (330 g flour, 78 % hydration) with a core probe in a refrigerator set to 1.7 °C, read every 30 minutes for 10 hours ([The Sourdough Journey](https://thesourdoughjourney.com/the-mystery-of-percentage-rise-in-bulk-fermentation/), chart "Cold Retard Temperature Change Over 10 Hours"). It went from 25.6 °C to 20.6 °C in 1 h, 13.9 °C in 2 h, 9.4 °C in 4 h, 6.1 °C in 6 h and 4.4 °C in 8 h.

The fit gives τ_c = 2.8 h toward 3.3 °C, with an rms error of 0.4 °C (`fits/fit_cooling.py`). The first half hour cools slower than the law, because the core lags the surface. The fitted surroundings sit 1.6 °C above the set point.

## Default time constants

The measured loaf fixes an effective surface coefficient of 5.9 W/m²·K. Other masses and containers are scaled with an equal-volume sphere and a correction for conduction inside the dough (density 1100 kg/m³, 2.7 J/g·K, 0.40 W/m·K).

| container | 0.2 kg | 0.5 kg | 1 kg | 2 kg |
|---|---:|---:|---:|---:|
| lined banneton in a bag (the measured case) | 1.8 h | 2.6 h | 3.3 h | 4.4 h |
| covered bowl or tub, thin plastic or metal | 1.4 h | 2.0 h | 2.6 h | 3.5 h |
| thick glass or ceramic bowl with lid | 1.8 h | 2.5 h | 3.3 h | 4.3 h |
| flattened pieces in a closed tray (pizza balls) | 1.2 h | 1.7 h | 2.2 h | 2.9 h |
| bare on a tray, fan-assisted fridge | 0.8 h | 1.2 h | 1.5 h | 2.1 h |

- Rule: τ_c ≈ 2.8 h × (m / 0.65 kg)^(1/3) × (5.9 / h_eff) / shape factor.
- Every value carries ±35 %. Fridge loading, shelf contact, fan cycling and how gassy the dough is are not modelled.
- A heavy bowl adds its own heat: add 20–40 % for thick glass or ceramic (glass 0.84 J/g·K, porcelain 1.07 J/g·K).
- Only the first row is anchored on a measurement. The surface coefficients of the other rows (6–12 W/m²·K) are engineering estimates for still and moving air.
- Warming up follows the same law toward room temperature. Use 0.7–1.0 of the cooling constant: condensation on a cold dough adds heat.
- Default fridge: 4 °C, range 2–7 °C. A fridge at 7 °C more than doubles the cold rate of a sourdough compared with 3 °C (note 2).

The thermal conductivity comes from measurements on dense unleavened dough at 39–68 °C: 0.347 W/m·K at 39 °C, up to 0.585 W/m·K with bran ([Šeruga et al. 2005](https://www.agriculturejournals.cz/publicFiles/50561.pdf), Tables 3–4). An aerated dough conducts less. The Biot number of these cases is 0.4–1.9, so a loaf is not at one temperature inside; the time constant above is that of the core's long-term approach, not a promise for every point of the loaf.

## Fermentation during the transient

Equivalent hours at 25 °C for a dough placed in a 4 °C fridge for 12 hours (`fits/cooling-fit.json`):

| start and time constant | sourdough | of which cooling period | yeast |
|---|---:|---:|---:|
| 21 °C, τ_c 1.5 h | 0.85 h | 0.47 h | 1.00 h |
| 21 °C, τ_c 2.8 h | 1.25 h | 0.87 h | 1.36 h |
| 26 °C, τ_c 1.5 h | 1.17 h | 0.79 h | 1.30 h |
| 26 °C, τ_c 2.8 h | 1.85 h | 1.47 h | 1.92 h |
| 26 °C, τ_c 4.0 h | 2.46 h | 2.08 h | 2.48 h |
| at 4 °C from the start | 0.38 h | — | 0.57 h |

- A 26 °C loaf does about 80 % of its retard fermentation while cooling. A model that puts the dough at 4 °C at once is wrong by a factor of 5.
- A warm dough needs a lower rise before it goes in the fridge than a cool one. This is the reason behind the practitioner rule "+30 % at 27 °C, +75 % at 21 °C" (same article, guideline table). The engine reaches the same advice by integration.
- The practitioner's own estimates agree with the table: the last 2 hours of room-temperature bulk are worth 8–12 hours in the fridge, and 24 hours at 3 °C at most 30 minutes of peak bulk ([FAQ, bulk timing](https://thesourdoughjourney.com/faq-bulk-fermentation-timing/); "Phase 5").
- Two hours of warm-up at 22 °C after the fridge add only about 0.2 h at 25 °C, because the core stays cold.

## A cool bulk on purpose

A 20 % starter dough mixed at 21 °C and held 24 hours in a chamber at 18 °C came out slightly overproofed; at 15 °C and 12 °C it gave good loaves ([The Sourdough Journey, cold proofer experiment](https://thesourdoughjourney.com/experiment-24-hour-bulk-fermentation-using-a-cold-proofer/); one test each, followed by 16–21 h in the fridge). The engine gives a rise of +226 %, +67 % and +26 % for those three runs. The ordering and the verdicts match. The numbers are not validated beyond that.

## Limits

- One loaf, one fridge. The table is a starting point; a probe reading replaces it.
- The cold end of both temperature curves is extrapolated (notes 1–2). Fridge predictions are flagged and widened.
- After a temperature step, organisms take time to adapt. The engine ignores this, which matters most on warm-up.
