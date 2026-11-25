"""Starter time-to-peak: the bulk growth model applied to a refresh.

A ripe starter has population density 1 by definition, so a refresh seeded at iota (seed flour /
total flour) is back at ripe density when exp(G) * iota = K_peak with K_peak = 1:
    tau_peak = ln(K_peak / iota) / (mu_ref * v * M_no_salt * M_hydration * gamma_s(T))
with M_no_salt = 1.18 and M_hydration = 1 + 0.005 (H - 75) kept within 0.85-1.15 (1.125 for a
100 % starter, 0.90 for a 55 % one).
No starter-specific rate is fitted; mu_ref and gamma_s come from fit_sourdough.py. The few
published statements we could read are used to check K_peak (they rarely give a temperature).

Run:  py fit_starter_peak.py     Outputs: starter-peak-fit.json, starter-peak-residuals.csv
"""
import csv
import json
import math
from pathlib import Path

import numpy as np

import fermodel as fm

HERE = Path(__file__).resolve().parent
NO_SALT = fm.NO_SALT_SOURDOUGH
ASSUMED_TEMP_C = 23.0      # used only where the source gives no temperature


def main():
    with (HERE / "data" / "starter_peak_observations.csv").open(encoding="utf-8") as f:
        obs = list(csv.DictReader(f))
    rows, k_implied = [], []
    for r in obs:
        if not r["seed_flour_fraction"]:
            continue
        iota = float(r["seed_flour_fraction"])
        temp = float(r["temp_c"]) if r["temp_c"] else ASSUMED_TEMP_C
        lo, hi = float(r["hours_low"]), float(r["hours_high"])
        mid = math.sqrt(lo * hi)
        pred = float(fm.tau_starter_peak(temp, iota, 100.0))
        rate = fm.SOURDOUGH["mu_ref"] * NO_SALT * fm.hydration_modifier_sourdough(100.0) * float(fm.gamma_sourdough(temp))
        k = iota * math.exp(rate * mid)
        volume_peak = "CO2" not in r["conditions"]
        if volume_peak:
            k_implied.append(k)
        rows.append({"id": r["id"], "feed_ratio": r["feed_ratio"], "iota": iota, "temp_c_used": temp,
                     "temp_assumed": int(not r["temp_c"]), "hours_low": lo, "hours_high": hi,
                     "hours_engine_K1": round(pred, 2), "ln_obs_over_engine": round(math.log(mid / pred), 3),
                     "implied_K_peak": round(k, 2), "used_for_K": int(volume_peak), "evidence": r["evidence"]})
    with (HERE / "starter-peak-residuals.csv").open("w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)

    res = [r["ln_obs_over_engine"] for r in rows if r["used_for_K"]]
    table = {}
    for hyd, label in ((100, "liquid_100pct"), (55, "stiff_55pct")):
        table[label] = {}
        for parts in (1, 2, 3, 5, 10, 20):
            iota = fm.inoculation_from_feed(parts, hyd)
            table[label][f"1:{parts}"] = {
                "iota": round(float(iota), 4),
                **{f"{t}C_h": round(float(fm.tau_starter_peak(t, iota, hyd)), 1) for t in (18, 21, 24, 27, 30)}}
    out = {
        "model": "tau_peak = ln(K_peak / iota) / (mu_ref * v * 1.18 * M_hydration * gamma_s(T)), K_peak = 1",
        "assumed_temperature_where_unstated_c": ASSUMED_TEMP_C,
        "n_statements_used": len(res),
        "median_ln_obs_over_engine": round(float(np.median(res)), 3),
        "sd_ln": round(float(np.std(res, ddof=1)), 3),
        "implied_K_peak_median": round(float(np.median(k_implied)), 2),
        "implied_K_peak_range": [round(min(k_implied), 2), round(max(k_implied), 2)],
        "prediction_table_hours": table,
        "note": "four statements, three of them without a temperature: this is a consistency check, not a fit. "
                "The pressure-vessel test (peak of CO2 generation at 20 C after 12-14 h) is a different event "
                "from the volume peak and is listed but not used."}
    (HERE / "starter-peak-fit.json").write_text(json.dumps(out, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
