# 6. Dough temperature: base temperature, DDT, friction and heat balance

Fermentation time depends on dough temperature more than on anything else the baker controls at mixing. One degree changes the rate by 12–15 % for yeast between 20 and 27 °C. For sourdough it is 15 % at 20 °C, 8 % at 25 °C and 4 % at 27 °C (engine curves, notes 1–2). The French rule of thumb says the same thing: 1 °C on the dough is worth 10 % of the yeast ([Technomitron](https://technomitron.aainb.com/petrissage/methodes-de-petrissage/), "Utilité du chiffre de base").

## French base temperature

water temperature = base − (room temperature + flour temperature)

| kneading | base | mixer settings |
|---|---|---|
| slow (PVL) | 68–72 | 10 min at 40 rpm |
| improved (PA) | 60–64 | 10–13 min at 80 rpm |
| intensive (PI) | 52–54 | 18–20 min at 80 rpm |

All for an oblique-axis mixer after 3–5 min of frasage, the lower figure in summer. To move the dough by 1 °C, change the base by about 3 (same page, "Chiffres de base utilisées").

## Desired dough temperature (DDT)

- Three factors: water = 3 × DDT − room − flour − FF.
- Four factors, with a preferment: water = 4 × DDT − room − flour − preferment − FF.

([King Arthur Baking, Desired dough temperature](https://www.kingarthurbaking.com/blog/2018/05/29/desired-dough-temperature).) The two systems are the same arithmetic: base = 3 × DDT − FF. A DDT of 24–26 °C is the usual target for wheat yeast breads ([King Arthur, friction factor](https://www.kingarthurbaking.com/blog/2018/08/27/determining-the-friction-factor-in-baking)).

## Friction factors

FF is a number in the units of the formula, not the warming of the dough. With n factors, the dough itself warms by about FF / n.

| mixing | FF (°C, 3-factor formula) | real warming (°C) | evidence |
|---|---|---|---|
| gentle folds by hand | 0–2.2 | 0–0.7 | creator-rule |
| vigorous hand kneading, 8 min | 3.3–4.4 | 1.1–1.5 | measured by one baker |
| planetary stand mixer, 3 min stir + 4 min speed 2 | 12.2–13.3 | 4.1–4.4 | measured by one baker |
| "most mixers", 3 min first + 4 min second speed (Hamelman, as quoted) | 13.3–15.6 | 4.4–5.2 | creator-rule |
| oblique mixer, slow kneading | 0–4 | 0–1.3 | derived from the base |
| oblique mixer, improved kneading | 8–12 | 2.7–4.0 | derived from the base |
| oblique mixer, intensive kneading | 18–20 | 6.0–6.7 | derived from the base |

- The first four rows are from the King Arthur article, converted from °F differences (× 5/9).
- The last three are FF = 3 × 24 − base.
- No figure specific to spiral mixers was found. The engine starts spiral mixers on the "most mixers" row and asks for one logged mix.
- To calibrate: FF = n × (measured dough temperature) − (sum of the other n temperatures). To correct after a miss: FF_new = FF_old + n × (measured − target). The King Arthur article corrects by the plain difference, which under-corrects by a factor n.

## Heat balance

The DDT formulas weight room, flour and water equally. A heat balance weights each ingredient by mass × specific heat and handles milk, eggs, butter, scalds and preferments without new rules.

T_dough = Σ(m·c·T) / Σ(m·c) + ΔT_mix + k_room · (T_room − T_blend)

- ΔT_mix is the real warming from the table above.
- k_room is the share of the gap to room temperature closed during mixing: 0.1 by default (range 0–0.25, an assumption). The 3-factor formula uses 1/3, which is too much for a short mix.
- Solve for water: T_water = (C_total · (DDT − ΔT_mix) − Σ_others m·c·T) / (m_water · c_water).

| ingredient | c (J/g·K) | range | source |
|---|---:|---|---|
| water | 4.18 | 4.18–4.19 | handbook |
| ice (near 0 °C) | 2.09 | 1.9–2.1 | handbook |
| wheat flour | 1.8 | 1.59–2.0 | handbook 1.59; composition rule about 2.0 at 14 % moisture |
| whole milk | 3.85 | 3.77–3.93 | handbook |
| skim milk | 4.0 | 3.98–4.02 | handbook |
| whole egg | 3.18 | 3.1–3.3 | handbook |
| butter | 2.4 | 2.0–2.72 | handbook 2.72; depends on how much fat melts |
| oil | 1.97 | 1.67–2.0 | handbook |
| sugar | 1.25 | 1.2–1.6 | recalled handbook value, not re-read |
| salt | 0.88 | 0.85–0.9 | handbook |
| levain, 100 % hydration | 2.99 | — | mixture rule |
| stiff levain, 55 % | 2.64 | — | mixture rule |
| dough, 70 % hydration | 2.78 | — | mixture rule |

Handbook values are from The Engineering ToolBox ([foods](https://www.engineeringtoolbox.com/specific-heat-capacity-food-d_295.html), [solids](https://www.engineeringtoolbox.com/specific-heat-solids-d_154.html)). The mixture rule is (c_flour + h · c_water) / (1 + h) for hydration h.

Worked check: 1000 g flour at 20 °C and 700 g water. Flour holds 1800 J/K, water 2926 J/K. To land at 25 °C with 4 °C of mixing heat and k_room = 0, the blend must be 21 °C, so the water must be 21.6 °C. The 3-factor formula with FF = 12 and a 20 °C room gives 23 °C. The two agree within 1.5 °C here and drift apart when the room is far from the flour temperature.

## Ice

Latent heat of melting: 334 J/g. With all ice at 0 °C and all of it melting:

m_ice = (Σ_others m·c·(T − T_target) + M_water · c_water · (T_liquid − T_target)) / (334 + c_water · T_liquid)

T_target is DDT − ΔT_mix and M_water the total water including ice. If the result is negative or larger than the water, the target cannot be reached that way; the engine says so instead of clamping.

## Limits

- Friction depends on the mixer, the speed, the dough mass and its stiffness. One logged mix is worth more than any table.
- Flour specific heat varies with moisture; the range above changes the water temperature by about 1 °C.
- Evaporation and bowl heat capacity are folded into k_room.
