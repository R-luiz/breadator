"""Personal calibration and the 80 % band: method and a replay on the two multi-row sourdough sources.

Prior:        ln v ~ Normal(0, s0^2), s0 = 0.35 (spread of ln v across sources, fit_sourdough.py)
Each bake:    y = ln(engine hours at v = 1 / observed hours) = ln v + e,  e ~ Normal(0, se^2)
Posterior:    precision = 1 / s0^2 + n / se^2;  mean = (sum(y) / se^2) / precision
Prediction:   sd^2 = posterior variance + se^2 + (s_T * min(|T - T_anchor|, 10))^2 + sum(domain penalties^2)
80 % band:    hours * exp(-/+ 1.2816 * sd)

Replay: treat a source as one baker's starter, feed its rows one at a time in random order,
predict every remaining row, and count how often the observation falls inside the band.

Run:  py calibration_demo.py     Outputs: calibration-demo.json, calibration-demo.csv
"""
import csv
import json
import math
import random
from pathlib import Path

import numpy as np

import fermodel as fm

HERE = Path(__file__).resolve().parent
S0 = 0.35          # prior sd of ln v
SE = 0.20          # sd of one logged bake (endpoint reading, thermometer, dough-to-dough noise)
S_T = 0.03         # per deg C away from the temperature the calibration bakes were made at
Z80 = 1.2816


def posterior(ys, s0=S0, se=SE):
    prec = 1 / s0 ** 2 + len(ys) / se ** 2
    return (sum(ys) / se ** 2) / prec, math.sqrt(1 / prec)


def predictive_sd(post_sd, temp, anchor, penalties=(), se=SE, s_t=S_T):
    return math.sqrt(post_sd ** 2 + se ** 2 + (s_t * min(abs(temp - anchor), 10.0)) ** 2 + sum(p ** 2 for p in penalties))


def load(name, rise_col=None):
    rows = []
    with (HERE / "data" / name).open(encoding="utf-8") as f:
        for r in csv.DictReader(f):
            if name.startswith("sourdough_sj"):
                rows.append((float(r["temp_c"]), float(fm.inoculation(float(r["starter_pct_of_flour"]))),
                             float(r["rise_pct"]) / 100, float(r["hours"])))
            elif int(float(r["salt_pct"])) == 2 and float(r["inoculation_pct_prefermented_flour"]) <= 15:
                rows.append((float(r["temp_c"]), float(r["inoculation_pct_prefermented_flour"]) / 100, 1.0,
                             float(r["hours_to_double"])))
    return rows


def replay(rows, label, n_runs=400, seed=1):
    rng = random.Random(seed)
    stats = {k: {"inside": 0, "n": 0, "abs_err": []} for k in (0, 1, 2, 3, 5)}
    for _ in range(n_runs):
        order = rows[:]
        rng.shuffle(order)
        for k in stats:
            train, test = order[:k], order[k:]
            ys = [math.log(float(fm.tau_sourdough(t, i, r)) / h) for t, i, r, h in train]
            mean, sd = posterior(ys)
            anchor = float(np.mean([t for t, *_ in train])) if train else 25.0
            for t, i, r, h in test[:8]:
                pred = float(fm.tau_sourdough(t, i, r, vigour=math.exp(mean)))
                band = Z80 * predictive_sd(sd, t, anchor)
                stats[k]["n"] += 1
                stats[k]["inside"] += int(abs(math.log(h / pred)) <= band)
                stats[k]["abs_err"].append(abs(math.log(h / pred)))
    out = {}
    for k, s in stats.items():
        out[str(k)] = {"coverage_of_80pct_band": round(s["inside"] / s["n"], 3),
                       "median_abs_error_pct": round(100 * (math.exp(float(np.median(s["abs_err"]))) - 1), 1),
                       "p90_abs_error_pct": round(100 * (math.exp(float(np.percentile(s["abs_err"], 90))) - 1), 1),
                       "band_factor_at_anchor": round(math.exp(Z80 * predictive_sd(posterior([0.0] * k)[1], 0, 0)), 2)}
    return {"source": label, "rows": len(rows), "logged_bakes": out}


def main():
    result = {
        "parameters": {"prior_sd_ln_v": S0, "bake_sd_ln": SE, "temperature_term_per_C": S_T, "z80": Z80},
        "band_factor_no_calibration": round(math.exp(Z80 * predictive_sd(S0, 0, 0)), 2),
        "posterior_sd_after_n_bakes": {str(n): round(posterior([0.0] * n)[1], 3) for n in (0, 1, 2, 3, 5, 10)},
        "replays": [replay(load("sourdough_sj_timetables.csv"), "Sourdough Journey timetables (true v about 1.0)"),
                    replay(load("sourdough_wraith_table.csv"), "Wraith table, 2 % salt, doubling (true v about 1.6)")],
        "note": "the replays use rows of a single source, so they test the update and the band, "
                "not the transfer between kitchens"}
    with (HERE / "calibration-demo.csv").open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["source", "logged_bakes", "coverage_of_80pct_band", "median_abs_error_pct", "p90_abs_error_pct",
                    "band_factor_at_anchor"])
        for rep in result["replays"]:
            for k, s in rep["logged_bakes"].items():
                w.writerow([rep["source"], k, s["coverage_of_80pct_band"], s["median_abs_error_pct"],
                            s["p90_abs_error_pct"], s["band_factor_at_anchor"]])
    (HERE / "calibration-demo.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
