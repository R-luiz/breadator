# 2. Sourdough organisms and the temperature curve

A sourdough rises on the gas of a wild yeast and of heterofermentative lactic acid bacteria. The engine takes its temperature response from the one study that measured both in the same conditions.

## Published growth parameters

Gänzle, Ehmann and Hammes (1998) grew two strains of *Lactobacillus sanfranciscensis* (now *Fructilactobacillus sanfranciscensis*) and *Candida milleri* LTH H198 (now placed with *Kazachstania humilis*) in broth ([full text](https://pmc.ncbi.nlm.nih.gov/articles/PMC106434/), Table 2 and Results). Their model 1a is μ = μopt · a′ · x^b · e^(−c·x) with x = Tmax − T.

| strain | μopt (1/h) | a′ | b | c (1/°C) | Tmax (°C) | optimum (°C) |
|---|---:|---:|---:|---:|---:|---:|
| *C. milleri* LTH H198 | 0.42 | 0.029 | 3.0 | 0.34 | 36 | 27 |
| *L. sanfranciscensis* LTH2581 | 0.71 | 0.097 | 2.0 | 0.22 | 41 | 32 |
| *L. sanfranciscensis* LTH1729 | 0.67 | 0.19 | 1.5 | 0.19 | 41 | 33 |

- Tmax was fixed at the lowest temperature with no visible growth after 7 days.
- a′ is rounded in print, so each curve is rescaled to 1 at x = b/c before use (`fits/fermodel.py`).
- Below 26 °C the three organisms respond to temperature alike, which the authors give as a reason for their stable association (Discussion).
- A second fit (model 1b, Ratkowsky type) puts the minimum growth temperature at 8.0 °C for the yeast and 3.0–4.1 °C for the lactobacilli. The authors note that model 1a fits poorly near the minimum.

## The engine's curve

The engine uses one compact curve for the consortium:

γ_s(T) = exp(0.314 (T − 25)) · ((40.4 − T) / 15.4)^3.64, zero at and above 40.4 °C

It was fitted to the equal-weight blend of the yeast and LTH2581 curves between 4 and 35 °C and follows it within 3.6 % rms (9 % at worst; `fits/sourdough-fit.json`, `gamma_s_fit_to_published_blend`). Its optimum is 28.8 °C.

| °C | 4 | 10 | 15 | 18 | 20 | 22 | 25 | 28 | 30 | 32 | 35 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| rate relative to 25 °C | 0.03 | 0.11 | 0.27 | 0.43 | 0.58 | 0.75 | 1.00 | 1.17 | 1.15 | 0.99 | 0.51 |

Why a blend: the yeast alone slows above 27 °C and stops at 36 °C, yet doughs keep rising faster up to about 28–30 °C in practice. Bill Wraith's table still shortens slightly from 27 to 29 °C, and Craig's sourdough chart is fastest at 27–29 °C (note 3). The lactobacilli, which also make CO2, carry the rate there.

## How the curve compares with bread data

- **Measured doughs, 19–23 °C.** The Sourdough Journey timetables give a Q10 of about 2.0; the curve has 3.4 in that range. The engine runs 15 % fast at 19 °C and 10 % slow at 22 °C (`sourdough-fit.json`, `engine_mean_residual_by_temp_c`). This gap is carried as uncertainty (note 3).
- **Cold.** A practitioner estimates that 24 hours at 3 °C are worth at most 30 minutes of peak bulk fermentation ([The Sourdough Journey](https://thesourdoughjourney.com/the-mystery-of-percentage-rise-in-bulk-fermentation/), "Phase 5"). The curve gives 32 minutes at 27 °C. The agreement is encouraging, but it rests on an estimate, not a measurement.
- **Hot.** Above 30 °C the two populations part ways: gas falls while acid production holds. The Framework's advice to cut the starter to 5 % at 30 °C (Table 7.1) fits the curve's 15 % gain over 25 °C.

## Acids, salt and pH

- The lactobacilli grew between pH 3.9 and 6.7 with an optimum near 5.5. The yeast was unaffected between pH 3.5 and 7 (Table 3). Bacterial growth therefore stalls as the dough nears pH 3.8, while the yeast carries on.
- The yeast was strongly inhibited by undissociated acetic acid, with a predicted limit near 0.15 mol/L, and much less by lactic acid (Table 4, Discussion).
- The lactobacilli were inhibited at 4 % NaCl, the yeast tolerated 8 % (percent of broth, not baker's percent; abstract).
- Typical dough levels are 100–200 mmol/L lactate and 40–60 mmol/L acetate (Discussion). These are observations, not targets.

## Stiff and liquid cultures

Four traditional sourdoughs kept for 28 days either firm (dough yield 160) or liquid (dough yield 280) diverged: the liquid ones ended with the lowest acidity and the highest yeast counts, the firm ones with more lactic and acetic acid ([Di Cagno et al. 2014](https://doi.org/10.1128/AEM.00309-14), abstract only). The Sourdough Framework rates a stiff starter as more yeast-active and less bacterial (Table 4.1, CC BY-SA 4.0). The two do not agree on which form favours yeast. No source read here compares their rise times. The engine therefore applies no factor for the starter type as such (range 0.85–1.15). It counts the starter's real flour content, and in a refresh it applies the general hydration term to the starter's own water content (note 9).

## Limits

- Broth is not dough: nutrients, buffering and the mixed population differ. The authors say their model needs validation against real sourdoughs.
- The strains came from one commercial starter. Other starters carry other species.
- Below 10 °C the curve is an extrapolation of a model that fits poorly there. Fridge predictions are flagged and widened.
