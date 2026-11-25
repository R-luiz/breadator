+++
title = "Fermentation time: temperature, dose and endpoint"
description = "Three things set how long a dough ferments: its temperature, the amount of yeast or starter, and the rise you wait for. What the data say about each, why a clock time alone fails, and how to calibrate."
topic = "bulk"
order = 1
updated = "2026-11-25"
+++
Three things set a fermentation time: the temperature of the dough (see [dough temperature](/en/learn/dough-temperature)), the dose of yeast or starter, and the endpoint you are waiting for. A recipe that prints only a duration has fixed all three for its author's kitchen. That is why stated bulk times are unreliable: flour, starter activity and temperature shift them, and one or two hours either way is enough to under- or over-ferment a dough [@breadcode-0493; @breadcode-1756].

## Temperature

Yeasts and bacteria work faster as the dough warms, up to an optimum, then collapse. Ten *Saccharomyces cerevisiae* strains grown in broth (wine, sake, agave and wild strains, none a baker's strain) had optima of 30–35 °C and stopped growing above 42–46 °C and below 0.4–5 °C [@salvadoz2011]. In broth, a sourdough yeast was fastest near 27 °C and its lactobacilli near 32–33 °C [@ganzlemg1998].

A blend of those sourdough curves gives, relative to 25 °C, a rate of 0.03 at 4 °C, 0.11 at 10 °C, 0.27 at 15 °C, 0.58 at 20 °C, 1.17 at 28 °C, 1.15 at 30 °C and 0.51 at 35 °C [@science-ferm-0040]. A sourdough therefore gains almost nothing above 28 °C; in one home test a dough at 30 °C was only modestly ahead of the same dough at 25 °C [@breadcode-1995].

The sources disagree on how steep the slope is at room temperature. For yeast, a pizza-dough chart tuned on bakers' feedback implies that 10 °C divide the time by 3.4–3.9 between 20 and 25 °C [@science-ferm-0015]; tables and recipes that list several temperatures imply about 2.1 [@science-ferm-0016]. For sourdough, timed doughs between 19 and 23 °C give about 2.0, against 3.4 for the literature curve [@science-ferm-0044]. Per degree, that is somewhere between 7 and 15 %.

## Commercial yeast: the dose law

Across a whole chart of pizza-dough fermentation times, the hours follow 0.91 × dose^−0.72 × a temperature factor, with the dose in percent of instant yeast and 0.91 h the time at 1 % and 25 °C [@science-ferm-0006]. The exponent matters: halving the yeast multiplies the time by 1.65, not 2. Two independent professional poolish tables give exponents of 0.70 and 0.90 [@science-ferm-0012]. [Commercial yeast](/en/learn/commercial-yeast) covers the forms of yeast and how to convert between them.

| Instant yeast (% of flour) | 4 °C | 10 °C | 18 °C | 21 °C | 25 °C | 30 °C |
|---|---|---|---|---|---|---|
| 0.05 % | 165 h | 65 h | 20 h | 13 h | 8 h | 4.5 h |
| 0.1 % | 100 h | 39 h | 12 h | 8 h | 5 h | 2.5 h |
| 0.3 % | 45 h | 18 h | 5.5 h | 3.5 h | 2 h | 1 h |
| 1 % | 19 h | 7.5 h | 2.5 h | 1.5 h | 1 h | 0.5 h |

These hours are for a lean dough held at one temperature until it has doubled. The chart behind them covers 0.003–0.32 % instant yeast (the 1 % row extends the law) and is a model adjusted to forum feedback, not a controlled experiment [@science-ferm-0001]; five independent statements with a doubling endpoint sit at a median 1.04 times the law [@science-ferm-0007]. A published cold-fermentation formula agrees with the same chart within 5 % below 8 °C but is too short by a factor of about 2 at 18 °C and 4 at 30 °C [@science-ferm-0005].

At a bakery dose the gas production stays roughly constant between measurements (58.5, 65.5 and 57.5 mL/g at 1, 2 and 3 hours with 3 % yeast and 1.5 % salt at 30 °C), which suggests a rise that is close to linear [@codinagg2013].

## Sourdough: growth from the prefermented flour

A starter does not add a fixed gas rate; it seeds a population that grows. Writing the rise as inoculation × (e^growth − 1), with the inoculation as starter flour over total flour, one growth rate of 0.30 per hour at 25 °C reproduces 40 measured bulk times within 10 % [@science-ferm-0043]. Those tests found, for a rise of +75 % with 20 % starter, 13 h at 19 °C and 8.5 h at 23 °C; with 5 % starter, 18.5 h and 14 h [@science-ferm-0041]. Going on from +75 % to +100 % took 1 to 2 hours more [@science-ferm-0042].

Two consequences follow. The dose acts logarithmically: halving the starter from 20 % to 10 % lengthens bulk by about a third, it does not double it [@science-ferm-0047]. And the endpoint is part of the answer. The same law, applied to a liquid starter of average vigour and a rise of +50 %, gives roughly these times [@science-ferm-0043]:

| Starter (% of final-mix flour) | 18 °C | 21 °C | 24 °C | 27 °C | 30 °C |
|---|---|---|---|---|---|
| 5 % | 24 h | 15.5 h | 11 h | 9 h | 9 h |
| 10 % | 19 h | 12.5 h | 9 h | 7 h | 7 h |
| 20 % | 14.5 h | 9.5 h | 7 h | 5.5 h | 5.5 h |

## Table 7.1 of The Sourdough Framework

The Framework's Table 7.1 (read in full in [levain and inoculation](/en/learn/levain-and-inoculation)) gives doses rather than times: 5 % of recently fed starter at 30 °C, 10 % at 25 °C and 15 % at 20 °C, and 2.5, 5 and 10 % for a starving starter [@science-ferm-0054]. Its author aims for 8 to 12 hours of bulk, with about 10 % starter for 8 hours and 5 % for 12 [@science-ferm-0053]. Run through the measured rate, the three recently-fed rows reach +50 % in 8.9, 8.1 and 12.1 hours, inside that window with no adjustment; to doubling, the 20 °C row needs 15.7 hours [@science-ferm-0055]. The table is a sound rule for a +50 % target and an average starter, not for doubling.

## Why one clock time is wrong

- **Temperature moves.** One baker expected his sample to double in 8–10 hours at about 21 °C; it took 12 hours in winter and 6 in summer [@breadcode-1371].
- **Starters differ.** Across six sources the vigour of the culture runs from 0.51 to 1.59 times the measured reference, a factor of three [@science-ferm-0052]. One published table says 6.5 hours where the baker who republished it observes about 10, at 22 °C with 20 % starter [@science-ferm-0051]. A starter kept a week in the fridge peaked about 2 hours later than the same starter freshly fed, in one test [@science-ferm-0059].
- **The formula counts.** In a model tuned on test doughs, 2 % salt lengthens the time to double a sourdough by about 15 % [@science-ferm-0027]. In yeasted dough, total gas over three hours fell to 0.78–0.84 of the unsalted value at 3 % salt [@carceam2020].
- **The dough is not at one temperature.** A 26 °C loaf put in the fridge for 12 hours ferments as much as in about 1.9 hours at 25 °C, of which 1.5 hours happen while it cools; assuming 4 °C from the start would give 0.4 hours [@science-ferm-0067]. [Retarding dough in the fridge](/en/learn/cold-retard) follows this through.

## Uncertainty and calibration

An honest prediction is a range. For a sourdough bulk at 25 °C with an unknown starter, the 80 % band is the predicted time multiplied or divided by 1.68; after three logged bakes it narrows to about 1.34 [@science-ferm-0079]. In a replay with a starter 1.6 times faster than the reference, the median error fell from 57 % with no log to 12 % after one logged bake and 4 % after three [@science-ferm-0080].

So use a predicted time to decide when to start watching, and an endpoint to decide when to stop: see [bulk fermentation endpoints](/en/learn/bulk-fermentation-endpoints). Write down the dough temperature, the dose, the endpoint and the hours it took. The [fermentation calculator](/en/calculators/fermentation) applies these laws along the dough's own temperature, room then fridge, and returns the range with the time.
