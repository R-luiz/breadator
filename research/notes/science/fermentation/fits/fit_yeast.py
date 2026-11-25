"""Commercial-yeast time model: fit and validation.

1. Fits the dose and temperature laws to TXCraig1's final chart (918 cells, 1.7-35 C,
   0.003-0.32 % IDY) and compares four temperature forms:
   M_cf   the cf-calculator regression of the same chart (30.91 * d^-0.72 * exp(-0.110 (F - 35)))
   M_exp  one free exponential
   M_ctmi cardinal temperature model with inflection
   M_card exponential with a cardinal cut-off, exp(c (T - 25)) ((Tmax - T) / (Tmax - 25))^b
2. Validates the chosen form against independent tables (French, BAKERpedia and Weekend Bakery
   poolish tables; Bread Code recipes) and a third-party compilation of recipes and tables.
3. Checks cold and mixed profiles with the progress integral P = sum(dt / tau(T_dough)).

Run:  py fit_yeast.py      Outputs: yeast-fit.json, yeast-chart-residuals.csv,
                                    yeast-validation-residuals.csv
"""
import csv
import json
import math
from pathlib import Path

import numpy as np
from scipy.optimize import least_squares

import fermodel as fm

HERE = Path(__file__).resolve().parent
DATA = HERE / "data"


def read(name):
    with (DATA / name).open(encoding="utf-8") as f:
        return list(csv.DictReader(f))


def num(x):
    try:
        return float(x)
    except (TypeError, ValueError):
        return float("nan")


def region_rmse(res, temp):
    out = {"all": fm.rmse(res)}
    for label, lo, hi in (("cold_below_8C", -99, 8), ("mid_8_to_28C", 8, 28), ("warm_28_to_35C", 28, 99)):
        m = (temp >= lo) & (temp < hi)
        out[label] = fm.rmse(res[m]) if m.any() else None
    return out


def main():
    chart = read("yeast_craig_chart.csv")
    t_all = np.array([num(r["temp_c"]) for r in chart])
    tf_all = np.array([num(r["temp_f"]) for r in chart])
    d_all = np.array([num(r["idy_pct"]) for r in chart])
    h_all = np.array([num(r["hours"]) for r in chart])
    # The chart is rounded to whole hours and floored at 2 h: fit on cells of 4 h or more.
    use = h_all >= 4
    t, tf, d, y = t_all[use], tf_all[use], d_all[use], np.log(h_all[use])

    summary = {"dataset": "yeast_craig_chart.csv", "n_cells": int(len(h_all)), "n_fitted": int(use.sum()),
               "fit_rule": "cells >= 4 h (integer rounding and the 2 h floor distort shorter cells)",
               "residual": "ln(predicted / chart)", "models": {}}

    # M_cf: published regression, nothing fitted
    r_cf = math.log(30.91) - 0.72 * np.log(d) - 0.110 * (tf - 35) - y
    summary["models"]["M_cf_calculator"] = {
        "formula": "h = 30.91 * IDY%^-0.72 * exp(-0.110 * (degF - 35))", "parameters_fitted": 0,
        "rmse_ln": region_rmse(r_cf, t)}

    # M_exp: one exponential
    x = np.c_[np.ones_like(t), -np.log(d), -(t - 25.0)]
    beta, *_ = np.linalg.lstsq(x, y, rcond=None)
    r_exp = x @ beta - y
    summary["models"]["M_exp"] = {
        "formula": "h = A * d^-a * exp(-b (T - 25))", "A_h": math.exp(beta[0]), "a": beta[1],
        "b_per_C": beta[2], "q10": math.exp(10 * beta[2]), "parameters_fitted": 3,
        "rmse_ln": region_rmse(r_exp, t)}

    # M_ctmi
    def f_ctmi(p):
        g = np.maximum(fm.ctmi(t, p[2], p[3], p[4]), 1e-9) / fm.ctmi(25.0, p[2], p[3], p[4])
        return p[0] - p[1] * np.log(d) - np.log(g) - y

    s = least_squares(f_ctmi, [0.0, 0.7, -3.0, 36.0, 45.0],
                      bounds=([-5, 0.3, -30, 28, 36], [5, 1.5, 1.5, 45, 60]))
    summary["models"]["M_ctmi"] = {
        "formula": "h = A * d^-a / (CTMI(T) / CTMI(25))", "A_h": math.exp(s.x[0]), "a": s.x[1],
        "t_min": s.x[2], "t_opt": s.x[3], "t_max": s.x[4], "parameters_fitted": 5,
        "note": "t_opt runs into t_max: the chart never turns down below 35 C, and the CTMI's "
                "quadratic foot does not follow the chart's near-exponential cold tail",
        "rmse_ln": region_rmse(s.fun, t)}

    # M_card: exponential x cardinal cut-off, Tmax fixed at 45 C (Salvado 2011: 45.4 C; Craig: 45 C)
    def f_card(p):
        return p[0] - p[1] * np.log(d) - np.log(fm.gamma_form(t, p[2], p[3], 45.0)) - y

    s = least_squares(f_card, [0.0, 0.7, 1.5, 0.2])
    card = {"A_h": math.exp(s.x[0]), "a": s.x[1], "b": s.x[2], "c_per_C": s.x[3], "t_max": 45.0}
    card["t_opt"] = 45.0 - card["b"] / card["c_per_C"]
    jac = s.jac
    cov = np.linalg.inv(jac.T @ jac) * (np.sum(s.fun ** 2) / (len(y) - 4))
    card["se"] = {"ln_A": math.sqrt(cov[0, 0]), "a": math.sqrt(cov[1, 1]), "b": math.sqrt(cov[2, 2]),
                  "c": math.sqrt(cov[3, 3])}
    summary["models"]["M_card"] = {
        "formula": "h = A * d^-a * exp(-c (T - 25)) * ((Tmax - 25) / (Tmax - T))^b", **card,
        "parameters_fitted": 4, "rmse_ln": region_rmse(s.fun, t)}
    summary["chosen"] = "M_card"
    summary["engine_defaults"] = fm.YEAST
    summary["local_q10_of_engine_curve"] = {
        str(tt): float(fm.gamma_yeast(tt + 5) / fm.gamma_yeast(tt - 5)) for tt in (7, 15, 20, 25, 30)}

    # residuals of the ENGINE defaults (rounded parameters) on every chart cell
    pred = fm.tau_yeast(t_all, d_all)
    with (HERE / "yeast-chart-residuals.csv").open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["temp_f", "temp_c", "idy_pct", "chart_hours", "engine_hours", "ln_residual",
                    "used_in_fit", "cf_calculator_hours"])
        for i in range(len(h_all)):
            cf = 30.91 * d_all[i] ** -0.72 * math.exp(-0.110 * (tf_all[i] - 35))
            w.writerow([int(tf_all[i]), t_all[i], d_all[i], int(h_all[i]), round(float(pred[i]), 2),
                        round(math.log(pred[i] / h_all[i]), 4), int(use[i]), round(cf, 2)])
    r_eng = np.log(pred[use]) - y
    summary["engine_rmse_ln_on_chart"] = region_rmse(r_eng, t)

    # ------------------------------------------------------------------ independent validation
    rows = []

    def add(group, rid, source, medium, endpoint, idy, temp, hours, note=""):
        if not (idy > 0 and hours > 0 and temp == temp):
            return
        p = float(fm.tau_yeast(temp, idy))
        rows.append({"group": group, "id": rid, "source": source, "medium": medium, "endpoint": endpoint,
                     "idy_pct": idy, "temp_c": temp, "hours_observed": hours,
                     "hours_engine_doubled": round(p, 2), "ln_obs_over_engine": round(math.log(hours / p), 4),
                     "note": note})

    for r in read("yeast_observations.csv"):
        temp = num(r["temp_c"])
        hours = math.sqrt(num(r["hours_low"]) * num(r["hours_high"]))
        note = ""
        if temp != temp:
            temp, note = 21.0, "temperature not stated: 21 C assumed"
        add("own:" + r["id"].rsplit("-", 1)[0], r["id"], r["source"][:60], r["medium"], r["endpoint"],
            num(r["idy_pct_equiv"]), temp, hours, note)

    ext = read("yeast_external_compilation.csv")
    cold_rows = []
    for r in ext:
        idy, temp, hours = num(r["idy_pct_fermenting_flour"]), num(r["rt_c"]), num(r["rt_h"])
        if r["fridge_h"]:
            cold_rows.append(r)
            continue
        ep = r["endpoint"].lower()
        if "doubl" in ep and "2.5" not in ep and "toward" not in ep:
            kind = "doubled"
        elif any(k in ep for k in ("ripe", "peak", "domed", "collaps", "reced")):
            kind = "ripe"
        elif "unspecified" in ep or "watch" in ep:
            kind = "unspecified"
        else:
            kind = "schedule/other"
        add("ext:" + r["medium"].split(" (")[0] + ":" + kind, r["id"], r["source"][:60], r["medium"],
            r["endpoint"][:60], idy, temp, hours)

    groups = {}
    for r in rows:
        groups.setdefault(r["group"], []).append(r["ln_obs_over_engine"])
    summary["validation_isothermal"] = {
        g: {"n": len(v), "median_ln_obs_over_engine": round(float(np.median(v)), 3),
            "time_factor": round(float(np.exp(np.median(v))), 2),
            "sd_ln": round(float(np.std(v, ddof=1)), 3) if len(v) > 1 else None}
        for g, v in sorted(groups.items())}

    ripe = [r["ln_obs_over_engine"] for r in rows if r["medium"] == "poolish"]
    summary["ripe_poolish_factor"] = {
        "n": len(ripe), "median": round(float(np.exp(np.median(ripe))), 2),
        "quartiles": [round(float(np.exp(q)), 2) for q in np.percentile(ripe, [25, 75])],
        "meaning": "hours to a ripe poolish divided by engine hours to 'doubled' at the same dose and temperature"}

    # dose exponent inside each independent table
    def slope(prefix):
        sel = [r for r in rows if r["id"].startswith(prefix)]
        xs = np.log([r["idy_pct"] for r in sel])
        ys = np.log([r["hours_observed"] for r in sel])
        return round(float(-np.polyfit(xs, ys, 1)[0]), 3), len(sel)

    summary["dose_exponent_independent"] = {
        "technomitron_poolish_22_24C": slope("technomitron"), "bakerpedia_poolish_26_29C": slope("bakerpedia"),
        "weekendbakery_poolish": slope("weekendbakery")}

    # temperature slope inside sources that list several temperatures (independent of the chart)
    by_group = {}
    for r in ext:
        idy, temp, hours = num(r["idy_pct_fermenting_flour"]), num(r["rt_c"]), num(r["rt_h"])
        if r["fridge_h"] or not (idy > 0 and hours > 0 and temp == temp):
            continue
        by_group.setdefault(r["source"][:45] + "|" + r["medium"][:12], []).append((idy, temp, hours))
    multi = {g: v for g, v in by_group.items() if len(v) >= 3 and len({t for _, t, _ in v}) >= 2}
    cols, ys = [], []
    names = sorted(multi)
    for gi, g in enumerate(names):
        for idy, temp, hours in multi[g]:
            row = [0.0] * len(names)
            row[gi] = 1.0
            cols.append(row + [-math.log(idy), -(temp - 25.0)])
            ys.append(math.log(hours))
    beta, *_ = np.linalg.lstsq(np.array(cols), np.array(ys), rcond=None)
    summary["within_source_temperature_slope"] = {
        "method": "ln h = source effect - a ln d - b (T - 25) over sources with at least 3 rows and 2 temperatures",
        "groups": len(names), "n": len(ys), "a": round(float(beta[-2]), 3), "b_per_C": round(float(beta[-1]), 4),
        "time_q10": round(math.exp(10 * float(beta[-1])), 2),
        "resid_sd_ln": round(float(np.std(np.array(cols) @ beta - np.array(ys))), 3),
        "note": "18-30 C; dominated by one calculator table and by proof-stage pairs; the chart's own slope there is 0.137 per C"}

    # ------------------------------------------------------------------ cold / mixed profiles
    def profile_progress(r, tau_c_h=1.5, warm_tail_h=0.0):
        """P for 'rt_h at rt_c, then fridge_h at fridge_c' with Newton cooling of the dough."""
        idy, rt_c, rt_h = num(r["idy_pct_fermenting_flour"]), num(r["rt_c"]), num(r["rt_h"])
        fr_c, fr_h = num(r["fridge_c"]), num(r["fridge_h"])
        if not (idy > 0 and fr_h > 0 and fr_c == fr_c):
            return None
        if rt_c != rt_c:
            rt_c, rt_h = 22.0, 0.0
        if rt_h != rt_h:
            rt_h = 0.0
        p_warm = rt_h / float(fm.tau_yeast(rt_c, idy))
        ts = np.linspace(0.0, fr_h, 400)
        temps = fm.newton_temperature(ts, rt_c, fr_c, tau_c_h)
        p_cold = fm.progress(ts, temps, lambda tt: fm.tau_yeast(tt, idy))
        p_iso = fr_h / float(fm.tau_yeast(fr_c, idy))
        return p_warm, p_cold, p_iso

    cold_out = []
    for r in cold_rows:
        res = profile_progress(r)
        if res is None:
            continue
        p_warm, p_cold, p_iso = res
        total = p_warm + p_cold
        cold_out.append({"group": "ext:cold-profile", "id": r["id"], "source": r["source"][:60],
                         "medium": r["medium"], "endpoint": r["endpoint"][:60],
                         "idy_pct": num(r["idy_pct_fermenting_flour"]), "temp_c": num(r["fridge_c"]),
                         "hours_observed": num(r["fridge_h"]),
                         "hours_engine_doubled": "", "ln_obs_over_engine": round(math.log(total), 4),
                         "note": f"profile: {num(r['rt_h'])} h at {num(r['rt_c'])} C then fridge; P_warm={p_warm:.2f} "
                                 f"P_fridge_with_cooling={p_cold:.2f} P_fridge_isothermal={p_iso:.2f}; "
                                 "ln_obs_over_engine column = ln(total progress P), 0 means 'doubled' exactly at the end"})
    lp = [r["ln_obs_over_engine"] for r in cold_out]
    if lp:
        summary["validation_cold_profiles"] = {
            "n": len(lp), "median_progress_P": round(float(np.exp(np.median(lp))), 2),
            "quartiles_P": [round(float(np.exp(q)), 2) for q in np.percentile(lp, [25, 75])],
            "sd_ln_P": round(float(np.std(lp, ddof=1)), 3),
            "assumption": "dough ball cooling time constant 1.5 h; warm phase before the fridge counted; "
                          "warm-up before baking not counted",
            "reading": "P is the engine's progress toward 'doubled' at the end of the published schedule; "
                       "published cold schedules sit above P = 1 when they are conservative"}

    with (HERE / "yeast-validation-residuals.csv").open("w", newline="", encoding="utf-8") as f:
        cols = ["group", "id", "source", "medium", "endpoint", "idy_pct", "temp_c", "hours_observed",
                "hours_engine_doubled", "ln_obs_over_engine", "note"]
        w = csv.DictWriter(f, fieldnames=cols)
        w.writeheader()
        w.writerows(rows + cold_out)

    (HERE / "yeast-fit.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
