# 3. Sourdough: inoculation, time and vigour

The engine predicts bulk time from how much of the flour comes from the starter, the dough temperature and the rise the baker is waiting for. One measured data set sets the rate; every other source is used as a check and to size the spread between starters.

## The law

- Inoculation: ι = starter flour / (starter flour + added flour). A recipe with 20 % starter at 100 % hydration has ι = 0.091; with a 55 % starter, 0.114.
- Growth: G = μ_ref · v · M · γ_s(T) · t, with μ_ref = 0.30 per hour at 25 °C. γ_s is the temperature curve of note 2, v the starter's vigour, M the modifiers of note 4.
- Rise of the aliquot: R = ι · (e^G − 1).
- Time to a rise target R*: τ = ln(K / ι) / (μ_ref · v · M · γ_s(T)) with K = R* + ι.

So K = 0.5 + ι for +50 %, 0.75 + ι for +75 % and 1 + ι for doubled. A ripe starter has density 1 by definition: the dough has doubled when its population is back to that of a ripe starter.

Hours to +50 % with a liquid starter and v = 1:

| starter (% of flour) | ι | 18 °C | 21 °C | 24 °C | 27 °C | 30 °C |
|---|---:|---:|---:|---:|---:|---:|
| 5 % | 0.024 | 23.6 | 15.5 | 11.1 | 9.1 | 8.9 |
| 10 % | 0.048 | 18.7 | 12.3 | 8.9 | 7.2 | 7.1 |
| 20 % | 0.091 | 14.4 | 9.5 | 6.8 | 5.5 | 5.4 |
| 30 % | 0.130 | 12.1 | 8.0 | 5.7 | 4.7 | 4.6 |

## The measured data

The Sourdough Journey published bulk times for 20 combinations of starter (5, 10, 15, 20 %) and dough temperature (66–74 °F, 18.9–23.3 °C), each to +75 % and +100 % rise ([Bulk Fermentation Timetables V1.0](https://thesourdoughjourney.com/wp-content/uploads/2023/01/Bulk-Fermentation-Timetables-V1.0.pdf), page 1). The doughs were 100 g flour batches, 90 % bread flour and 10 % whole wheat with 2 % salt; the starter was fed 1:2:2 and used near its peak. The note says over 60 tests, no averaging, outliers dropped, minor smoothing, times rounded to half hours.

Fits (`fits/fit_sourdough.py`, n = 40):

| form | fitted numbers | rms error (ln h) |
|---|---:|---:|
| engine: one rate, literature temperature curve | 1 | 0.102 |
| same, with a free exponential temperature slope | 2 | 0.051 |
| power law A · ι^(−a) · exp(−b (T − 25)), fitted per rise target | 6 | 0.055 |

- The fitted rate is 0.2985 per hour; the engine uses 0.30.
- A single rate reproduces two things at once: the dose slope (about 4.2 h per e-fold of inoculation) and the 1–2 h step from +75 % to +100 %. That is the evidence for the growth form and for K = R + ι.
- The power law fits as well only because it has its own constants for each rise target (a = 0.31, b = 0.07 per °C at +75 %). It cannot map one endpoint to another.
- The free slope gives a Q10 of 2.0 against 3.4 for the literature curve. The engine keeps the literature curve, because a slope fitted on 19–23 °C would put fridge activity at 20 % of the 25 °C rate, which contradicts every cold observation. The cost is a 15 % error at the edges of the measured range.

## Other sources: vigour and spread

None of these was fitted. Each gives a vigour factor relative to the reference (`sourdough-fit.json`, `vigour_by_source`).

| source | what it is | vigour v |
|---|---|---:|
| Sourdough Journey guideline table (18–27 °C, 20 % starter, +30 to +100 %) | creator rule from the same tests | 0.95 |
| Sourdough Framework Table 7.1 at +50 %, window midpoint | creator rule | 0.96 |
| Author of the Bread Code page (22 °C, 20 %, about 10 h) | anecdote | 0.84 |
| Bill Wraith's table (2 % salt, doubling, ι up to 15 %) | model tuned to his test doughs | 1.59 |
| Craig's sourdough chart (pizza dough, 2.5–3 % salt, about 60 % hydration) | model built on forum data at 75 °F | 0.51 |

- [Wraith's model](https://www.thefreshloaf.com/node/4129) has the same structure: exponential growth to a fixed population relative to a ripe starter, salt and hydration as multipliers, and the Gänzle temperature curves. His doubling times follow ln(1 + 1/ι) within 4 %. His starter was 1.6 times faster than the reference. Above 15 % inoculation he adds a gluten penalty that the engine does not have.
- [Craig's chart](http://web.archive.org/web/20250323105308/https://www.pizzamaking.com/forum/index.php/topic,22649.0.html) is exactly logarithmic in dose (hours ∝ ln(89 % / starter %)). Its endpoint is "ready to bake" and its dough is saltier and stiffer, so its low vigour mixes culture, formula and endpoint.
- The page that preserves Wraith's table notes the gap itself: about 10 h in the author's kitchen against 6.5 h in the table ([fermentation table](https://www.the-bread-code.io/fermentation-table/), "What changes the time").

The standard deviation of ln v across the six sources is 0.36. The engine uses 0.35 as the prior width for a starter it knows nothing about (note 8; model file, `personal_calibration`). That is a factor of 1.6 either way at the 80 % level.

## The Sourdough Framework's table

Table 7.1 gives doses, not times: 5 % recently fed starter at 30 °C, 10 % at 25 °C, 15 % at 20 °C; for a starving starter 2.5, 5 and 10 %. The text says the author aims for 8 to 12 hours of bulk and uses about 10 % for 8 h and 5 % for 12 h (ch. 7.5, CC BY-SA 4.0). Engine hours with v = 1:

| row | liquid starter, +50 % | stiff (55 %) starter, +50 % | liquid starter, +100 % |
|---|---:|---:|---:|
| 30 °C, 5 % | 8.9 | 8.2 | 10.8 |
| 25 °C, 10 % | 8.1 | 7.4 | 10.3 |
| 20 °C, 15 % | 12.1 | 10.9 | 15.7 |

At the book's +50 % target the three rows land at 8.1–12.1 h with a geometric mean of 9.6 h, so the measured rate needs no further calibration to sit in the window. The 20 °C row is at the slow edge. At +100 %, which the book reserves for flour above 12 % protein, the 20 °C row leaves the window.

The starving-starter rows come out 2 h longer (10.2–14.1 h at +50 %) before any lag. The book gives them to limit the acid a tired starter brings, not to promise equal time.

The earlier dose-versus-temperature fits of this table (`fits/fit_framework.py`) remain valid as a description of the table: the recently-fed row is linear in temperature and the starving row exponential. They say nothing about time.

## Starter state

One pressure-vessel test compared a starter at its peak with the same starter kept 7 days in the fridge: the cold one started later and peaked about 2 hours later in a 1:1:1 feed ([The Sourdough Journey](https://thesourdoughjourney.com/can-you-use-an-unfed-starter-directly-from-the-refrigerator/)). The engine adds a lag in hours at 25 °C, divided by γ_s(T): 0 for a starter at its peak, 0.5 (0–1.5) for one that is past its peak, 2 (1–3) for one taken from the fridge after about a week. Only the last value has a measurement behind it, and only one.

## Limits

- The rate was measured on one starter and one flour blend between 19 and 23 °C.
- Outside 10–30 °C and outside ι = 0.5–20 % the engine extrapolates and says so.
- Past the endpoint the rise law stops being true: acid stalls the bacteria and the gluten weakens.
