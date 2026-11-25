"""Sourdough bulk-time model: fit and validation.

Model (constant dough temperature T, inoculation iota = starter flour / total flour):
    rise R(t) = iota * (exp(G) - 1),   G = mu_ref * v * M * gamma_s(T) * t
    tau(R)    = ln(K / iota) / (mu_ref * v * M * gamma_s(T)),   K = R + iota
so K = 0.5 + iota for an aliquot at +50 %, 0.75 + iota for +75 %, 1 + iota for doubled.

Steps
1. gamma_s: compact cardinal-exponential form fitted to the equal-weight blend of the published
   Gaenzle 1998 curves for C. milleri and L. sanfranciscensis (literature, no bread data).
2. mu_ref: fitted to the Sourdough Journey timetables (measured, n = 40, 19-23 C, 2 % salt).
   The same data are also fitted with the power-law form and with a free exponential
   temperature slope, to show what each form buys.
3. Other sources are NOT fitted: each gets a vigour factor v (median) and a residual spread.
   The spread of ln v across sources is the prior width for an unknown starter.
4. Table 7.1 of The Sourdough Framework is checked against the 8-12 h window.

Run:  py fit_sourdough.py     Outputs: sourdough-fit.json, sourdough-residuals.csv
"""
import csv
import json
import math
from pathlib import Path

import numpy as np
from scipy.optimize import least_squares

import fermodel as fm

HERE = Path(__file__).resolve().parent
TRAPZ = getattr(np, "trapezoid", None) or np.trapz      # numpy 2 renamed trapz
DATA = HERE / "data"
SALT_FREE = 1.18      # rate multiplier without salt relative to 2 % salt (Wraith: 1.15 on doubling, 1.21 on bulk)


def read(name):
    with (DATA / name).open(encoding="utf-8") as f:
        return list(csv.DictReader(f))


def num(x):
    try:
        return float(x)
    except (TypeError, ValueError):
        return float("nan")


def main():
    out = {"residual": "ln(observed hours / engine hours); positive = slower than the engine",
           "engine_defaults": fm.SOURDOUGH}
    res_rows = []

    def log_row(dataset, rid, temp, iota, rise, hours, pred, modifier=1.0, note=""):
        res_rows.append({"dataset": dataset, "id": rid, "temp_c": round(float(temp), 2),
                         "iota": round(float(iota), 4), "rise_fraction": rise, "modifier": modifier,
                         "hours_observed": hours, "hours_engine": round(float(pred), 2),
                         "ln_obs_over_engine": round(math.log(hours / pred), 4), "note": note})

    # ---------------------------------------------------------------- 1. temperature curve
    grid = np.arange(4.0, 35.01, 1.0)
    target = fm.gamma_blend_published(grid)

    def f_shape(p):
        return np.log(fm.gamma_form(grid, *p)) - np.log(target)

    s = least_squares(f_shape, [3.0, 0.3, 40.0], bounds=([0.5, 0.05, 36.5], [6, 0.6, 45]))
    out["gamma_s_fit_to_published_blend"] = {
        "target": "0.5 * C. milleri LTH H198 + 0.5 * L. sanfranciscensis LTH2581 (Gaenzle 1998 Table 2, model 1a), "
                  "each normalised to its optimum, blend normalised at 25 C, grid 4-35 C",
        "b": s.x[0], "c_per_C": s.x[1], "t_max": s.x[2], "t_opt": s.x[2] - s.x[0] / s.x[1],
        "rmse_ln": fm.rmse(s.fun), "max_abs_ln": float(np.max(np.abs(s.fun)))}
    out["gamma_s_values"] = {str(int(t)): round(float(fm.gamma_sourdough(t)), 3)
                             for t in (3, 4, 6, 8, 10, 12, 15, 18, 20, 22, 24, 25, 26, 28, 30, 32, 35, 38)}
    out["gamma_published_strain_values"] = {
        str(int(t)): {"yeast_H198": round(float(fm.gaenzle_1a_normalised(t, 3.0, 0.34, 36.0)), 3),
                      "lab_LTH2581": round(float(fm.gaenzle_1a_normalised(t, 2.0, 0.22, 41.0)), 3)}
        for t in (4, 10, 15, 20, 25, 27, 30, 32, 35)}

    # ---------------------------------------------------------------- 2. mu_ref on measured data
    sj = read("sourdough_sj_timetables.csv")
    t = np.array([num(r["temp_c"]) for r in sj])
    io = fm.inoculation(np.array([num(r["starter_pct_of_flour"]) for r in sj]))
    rise = np.array([num(r["rise_pct"]) / 100 for r in sj])
    h = np.array([num(r["hours"]) for r in sj])
    g = fm.gamma_sourdough(t)

    def f_mu(p):
        return np.log(np.log1p(rise / io) / (math.exp(p[0]) * g)) - np.log(h)

    s1 = least_squares(f_mu, [math.log(0.3)])

    def f_mu_c(p):
        return np.log(np.log1p(rise / (math.exp(p[1]) * io)) / (math.exp(p[0]) * g)) - np.log(h)

    s2 = least_squares(f_mu_c, [math.log(0.3), 0.0])

    def f_free_slope(p):
        return np.log(np.log1p(rise / io) / (math.exp(p[0]) * np.exp(p[1] * (t - 25)))) - np.log(h)

    s3 = least_squares(f_free_slope, [math.log(0.3), 0.1])
    pl = {}
    for target_rise in (0.75, 1.0):
        m = rise == target_rise
        x = np.c_[np.ones(m.sum()), -np.log(io[m]), -(t[m] - 25)]
        beta, *_ = np.linalg.lstsq(x, np.log(h[m]), rcond=None)
        pl[str(target_rise)] = {"A_h": math.exp(beta[0]), "a": beta[1], "b_per_C": beta[2],
                                "rmse_ln": fm.rmse(x @ beta - np.log(h[m]))}
    out["sj_timetables_fit"] = {
        "n": len(h), "domain": "18.9-23.3 C, starter 5-20 % of flour (iota 0.024-0.091), rise +75 % and +100 %, 2 % salt",
        "log_growth_with_literature_gamma": {"mu_ref_per_h": math.exp(s1.x[0]), "parameters": 1,
                                             "rmse_ln": fm.rmse(s1.fun)},
        "log_growth_free_K_scale": {"mu_ref_per_h": math.exp(s2.x[0]), "c_K": math.exp(s2.x[1]),
                                    "parameters": 2, "rmse_ln": fm.rmse(s2.fun)},
        "log_growth_free_exponential_slope": {"mu_ref_per_h": math.exp(s3.x[0]), "k_per_C": s3.x[1],
                                              "q10": math.exp(10 * s3.x[1]), "parameters": 2,
                                              "rmse_ln": fm.rmse(s3.fun)},
        "power_law_per_endpoint": {"parameters": 6, "fits": pl,
                                   "rmse_ln": fm.rmse([pl["0.75"]["rmse_ln"], pl["1.0"]["rmse_ln"]])},
        "reading": "one growth rate reproduces the dose slope AND the +75 % to +100 % spacing; the measured "
                   "temperature slope (Q10 about 2.0 over 19-23 C) is flatter than the literature curve (about 3.4)"}
    pred = fm.tau_sourdough(t, io, rise)
    for r, tt, ii, rr, hh, pp in zip(sj, t, io, rise, h, pred):
        log_row("sj_timetables (measured)", f"{r['temp_f']}F-{r['starter_pct_of_flour']}%-{r['rise_pct']}", tt, ii, rr, hh, pp)
    eng = np.log(h / pred)
    out["sj_timetables_fit"]["engine_rmse_ln"] = fm.rmse(eng)
    out["sj_timetables_fit"]["engine_mean_residual_by_temp_c"] = {
        str(round(float(tt), 1)): round(float(eng[t == tt].mean()), 3) for tt in sorted(set(t))}

    sources = {}

    # ---------------------------------------------------------------- 3a. SJ guideline table
    obs = read("sourdough_observations.csv")
    v_list = []
    for r in obs:
        if r["id"].startswith("sj-guideline"):
            tt, ii, rr, hh = num(r["temp_c"]), fm.inoculation(num(r["starter_pct_of_flour"])), num(r["rise_pct"]) / 100, num(r["hours"])
            pp = float(fm.tau_sourdough(tt, ii, rr))
            log_row("sj_guidelines", r["id"], tt, ii, rr, hh, pp)
            v_list.append(math.log(hh / pp))
    sources["sj_guidelines (18-27 C, 20 % starter, endpoints +30..+100 %)"] = v_list

    # ---------------------------------------------------------------- 3b. Wraith table (doubling)
    wr = read("sourdough_wraith_table.csv")
    w_res = {0: [], 2: []}
    for r in wr:
        tt, ii, salt, hh = num(r["temp_c"]), num(r["inoculation_pct_prefermented_flour"]) / 100, int(num(r["salt_pct"])), num(r["hours_to_double"])
        mod = SALT_FREE if salt == 0 else 1.0
        pp = float(fm.tau_sourdough(tt, ii, 1.0, modifier=mod))
        if ii <= 0.15:          # above 15 % Wraith adds a gluten penalty that this model does not have
            w_res[salt].append(math.log(hh / pp))
        log_row("wraith_table (doubling)", f"{r['temp_f']}F-{r['inoculation_pct_prefermented_flour']}%-salt{salt}", tt, ii, 1.0, hh, pp, mod,
                "" if ii <= 0.15 else "inoculation > 15 %: excluded from the vigour estimate")
    sources["wraith_table (10-29 C, iota 0.5-15 %, 2 % salt, doubling)"] = w_res[2]
    salt_ratio = [num(r["hours_to_double"]) for r in wr if int(num(r["salt_pct"])) == 2]
    nosalt = [num(r["hours_to_double"]) for r in wr if int(num(r["salt_pct"])) == 0]
    out["wraith_salt_ratio_2pct_over_0pct"] = {"median": float(np.median(np.array(salt_ratio) / np.array(nosalt))),
                                               "min": float(np.min(np.array(salt_ratio) / np.array(nosalt))),
                                               "max": float(np.max(np.array(salt_ratio) / np.array(nosalt)))}

    # ---------------------------------------------------------------- 3c. Craig sourdough chart
    cr = read("sourdough_craig_chart.csv")
    ct = np.array([num(r["temp_c"]) for r in cr])
    ci = fm.inoculation(np.array([num(r["starter_pct_of_flour"]) for r in cr]))
    ch = np.array([num(r["hours_total_to_ready"]) for r in cr])
    ok = (ch >= 4) & (ct >= 4) & (ct <= 30)

    def f_craig(p):       # his endpoint is "ready to bake": fit an effective rise and a vigour
        return np.log(np.log1p(math.exp(p[1]) / ci[ok]) / (fm.SOURDOUGH["mu_ref"] * math.exp(p[0]) * fm.gamma_sourdough(ct[ok]))) - np.log(ch[ok])

    sc = least_squares(f_craig, [0.0, math.log(0.4)])
    v_c, r_c = math.exp(sc.x[0]), math.exp(sc.x[1])
    out["craig_chart"] = {"n_fitted": int(ok.sum()), "domain": "4-30 C, starter 1-40 % of flour, cells >= 4 h",
                          "effective_rise_of_ready_to_bake": r_c, "vigour": v_c, "rmse_ln": fm.rmse(sc.fun),
                          "note": "pizza dough (about 60 % hydration, 2.5-3 % salt); temperature curve built by Craig "
                                  "on the same Gaenzle C. milleri parameters, so its shape is not independent"}
    for r, tt, ii, hh in zip(cr, ct, ci, ch):
        pp = float(fm.tau_sourdough(tt, ii, r_c, vigour=v_c))
        log_row("craig_sourdough_chart (ready to bake)", f"{r['temp_f']}F-{r['starter_pct_of_flour']}%", tt, ii, round(r_c, 3), hh, pp, 1.0,
                f"engine evaluated with fitted effective rise {r_c:.2f} and vigour {v_c:.2f}")
    sources["craig_chart (pizza dough, vigour only)"] = [math.log(1 / v_c)]

    # ---------------------------------------------------------------- 3d. single statements
    for r in obs:
        rid = r["id"]
        if rid.startswith("breadcode-author"):
            tt, ii, hh = num(r["temp_c"]), fm.inoculation(num(r["starter_pct_of_flour"])), num(r["hours"])
            pp = float(fm.tau_sourdough(tt, ii, 0.5))
            log_row("breadcode_author_statement", rid, tt, ii, 0.5, hh, pp, 1.0, "endpoint assumed +50 %")
            sources["bread code author (22 C, 20 %, +50 % assumed)"] = [math.log(hh / pp)]

    # ---------------------------------------------------------------- 3e. cold-proofer bulk (non-isothermal)
    cold = {}
    for r in obs:
        if r["id"].startswith("sj-coldproofer"):
            tt, ii = num(r["temp_c"]), fm.inoculation(num(r["starter_pct_of_flour"]))
            ts = np.linspace(0, 24, 500)
            temps = fm.newton_temperature(ts, 21.0, tt, 2.0)
            growth = float(TRAPZ(fm.SOURDOUGH["mu_ref"] * fm.gamma_sourdough(temps), ts))
            cold[r["id"]] = {"set_temp_c": tt, "G_after_24h": round(growth, 2),
                             "engine_rise_pct": round(100 * float(ii) * (math.exp(growth) - 1)),
                             "observed": r["conditions"]}
    out["cold_proofer_24h_check"] = {"assumption": "dough at 21 C placed in still air at the set temperature, "
                                                   "time constant 2 h", "runs": cold}

    # ---------------------------------------------------------------- 4. Framework Table 7.1
    table = {}
    for temp, fed, starving in ((30, 5, 2.5), (25, 10, 5), (20, 15, 10)):
        row = {}
        for label, pct in (("recently_fed", fed), ("starving", starving)):
            for hyd in (100, 55):
                ii = float(fm.inoculation(pct, hyd))
                row[f"{label}_{pct}pct_hydration{hyd}"] = {
                    "iota": round(ii, 4),
                    "hours_plus50": round(float(fm.tau_sourdough(temp, ii, 0.5)), 1),
                    "hours_plus100": round(float(fm.tau_sourdough(temp, ii, 1.0)), 1)}
        table[f"{temp}C"] = row
    fed50 = [table[k][n]["hours_plus50"] for k, n in (("30C", "recently_fed_5pct_hydration100"),
                                                      ("25C", "recently_fed_10pct_hydration100"),
                                                      ("20C", "recently_fed_15pct_hydration100"))]
    out["framework_table_7_1"] = {
        "book_window_h": [8, 12], "engine_vigour": 1.0, "predictions": table,
        "recently_fed_plus50_hydration100_h": fed50,
        "geometric_mean_h": round(float(np.exp(np.mean(np.log(fed50)))), 1),
        "vigour_that_centres_on_10h": round(float(np.exp(np.mean(np.log(fed50))) / 10.0), 2),
        "note": "the table gives doses, not times; the starving rows are a lower dose for acid load, "
                "not a claim of equal time"}
    for r in obs:
        if r["id"].startswith("framework-table"):
            tt, ii, hh = num(r["temp_c"]), fm.inoculation(num(r["starter_pct_of_flour"])), num(r["hours"])
            pp = float(fm.tau_sourdough(tt, ii, 0.5))
            log_row("framework_table_7_1 (window midpoint 10 h, +50 %)", r["id"], tt, ii, 0.5, hh, pp)
    sources["framework table 7.1 (window midpoint, +50 %, 100 % hydration starter)"] = [math.log(10.0 / x) for x in fed50]

    # ---------------------------------------------------------------- vigour table and prior width
    vig = {}
    means = []
    for name, vals in sources.items():
        m = float(np.mean(vals))
        vig[name] = {"n": len(vals), "mean_ln_obs_over_engine": round(m, 3),
                     "vigour_v": round(math.exp(-m), 2),
                     "sd_within_ln": round(float(np.std(vals, ddof=1)), 3) if len(vals) > 1 else None}
        means.append(m)
    means.append(0.0)      # the reference source itself (Sourdough Journey timetables)
    out["vigour_by_source"] = vig
    out["prior_width"] = {
        "sources": len(means), "sd_ln_v_between_sources": round(float(np.std(means, ddof=1)), 3),
        "min_v": round(math.exp(-max(means)), 2), "max_v": round(math.exp(-min(means)), 2),
        "recommended_sigma_ln_v": 0.35,
        "within_source_sd_ln": round(fm.rmse(eng), 3)}

    with (HERE / "sourdough-residuals.csv").open("w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=list(res_rows[0]))
        w.writeheader()
        w.writerows(res_rows)
    (HERE / "sourdough-fit.json").write_text(json.dumps(out, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
