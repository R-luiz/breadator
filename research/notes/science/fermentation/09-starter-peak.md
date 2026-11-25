# 9. Starter: time to peak

A refresh is a small dough without salt, inoculated at a known rate. The engine uses the same growth model as for bulk.

## The law

- Feed 1 : f : w (seed : flour : water), seed hydration h as a fraction.
- Inoculation: ι = seed flour / total flour = (1 / (1 + h)) / (1 / (1 + h) + f).
- Time to peak: τ = ln(K_peak / ι) / (μ_ref · v · 1.18 · M_h · γ_s(T)), with K_peak = 1.

K_peak = 1 is the model's own unit: a ripe starter has density 1, so the refresh peaks when its population is back there. The factor 1.18 is the rate without salt and M_h the hydration term of note 4 applied to the starter itself: 1.125 for a 100 % starter, 0.90 for a 55 % one. μ_ref = 0.30 per hour and γ_s are those of notes 2 and 3.

| feed (liquid, 100 %) | ι | 18 °C | 21 °C | 24 °C | 27 °C | 30 °C |
|---|---:|---:|---:|---:|---:|---:|
| 1:1:1 | 0.333 | 6.4 h | 4.2 h | 3.0 h | 2.4 h | 2.4 h |
| 1:2:2 | 0.200 | 9.3 h | 6.1 h | 4.4 h | 3.6 h | 3.5 h |
| 1:3:3 | 0.143 | 11.3 h | 7.4 h | 5.3 h | 4.3 h | 4.2 h |
| 1:5:5 | 0.091 | 13.9 h | 9.1 h | 6.6 h | 5.3 h | 5.2 h |
| 1:10:10 | 0.048 | 17.6 h | 11.6 h | 8.3 h | 6.8 h | 6.6 h |
| 1:20:20 | 0.024 | 21.5 h | 14.1 h | 10.2 h | 8.3 h | 8.1 h |

A stiff starter (55 % hydration) carries more flour per gram of seed, which shortens the refresh, and less water, which slows it: 1:5:2.75 has ι = 0.114 and peaks in 7.4 h at 24 °C, against 6.6 h for the liquid 1:5:5. Whether a stiff culture differs beyond its water content is open (note 2); the engine applies no further factor. The hydration term is an extrapolation (note 4); the private validation log is consistent with it (aggregate only, note 8).

## What it was checked against

Published statements rarely give a temperature. Four could be used, three with 23 °C assumed (`fits/fit_starter_peak.py`):

| statement | source | observed | engine |
|---|---|---:|---:|
| fed 1:2:2 about 4–6 h before mixing, used near peak | [Sourdough Journey timetables](https://thesourdoughjourney.com/wp-content/uploads/2023/01/Bulk-Fermentation-Timetables-V1.0.pdf), recipe note | 4–6 h | 4.9 h |
| a 1:1:1 feed that peaks in 4 h | [Sourdough Journey FAQ](https://thesourdoughjourney.com/faq-starter-maintenance/) | 4 h | 3.3 h |
| …needs about 1:20:20 to peak in 8 h | same | 8 h | 11.2 h |
| check a 1:5:5 feed after 6–12 h | [The Bread Code, feeding ratio](https://www.the-bread-code.io/tutorial/2023/05/06/sourdough-starter-feeding-ratio.html) | 6–12 h | 7.2 h |

The median gap is +8 % and the scatter 0.24 in log units. Three of four agree within 20 %. The 1:20:20 statement is an illustration in a FAQ and sits about 30 % below the law; the same FAQ says a 1:2:2 feed takes only 15 % longer than 1:1:1, where the law says 46 %. Either high ratios run faster than a pure growth law predicts, or the illustration is loose. No timed series of ratios at a known temperature was found to decide.

A fifth observation is not comparable: in a pressure vessel at 20 °C, CO2 generation of 1:1:1 feeds peaked after 12–14 h ([The Sourdough Journey](https://thesourdoughjourney.com/white-wheat-or-rye-what-is-the-best-flour-to-feed-your-starter/)). That is the gas-production peak, long after the volume peak; the starter is past ripe by then.

## Starter state and the next use

- A starter used at its peak needs no lag. One kept a week in the fridge peaked about 2 hours later in a 1:1:1 feed ([The Sourdough Journey](https://thesourdoughjourney.com/can-you-use-an-unfed-starter-directly-from-the-refrigerator/); one test). The engine adds 2 h (1–3 h) at 25 °C for that case.
- The time to peak is the best calibration data a baker can log: no shaping, no salt, a clear event. Two logged peaks cut the vigour uncertainty from 0.35 to 0.13 (note 8).
- The same v then serves the bulk prediction. This assumes that the starter's speed in a refresh carries over to the dough, which Wraith's method also assumes and which was not tested here.

## Limits

- Peak height depends on hydration and flour strength; the law predicts the time, not the height.
- At 30 °C and above the yeast slows while the bacteria do not (note 2): the starter may sour before it peaks.
- Feeds leaner than 1:20:20 and temperatures below 15 °C are extrapolation.
