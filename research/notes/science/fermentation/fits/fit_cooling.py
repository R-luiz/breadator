"""Dough cooling in a home refrigerator: Newton fit and a physics table.

1. Fits T(t) = T_env + (T0 - T_env) exp(-t / tau_c) to the one measured curve we have
   (The Sourdough Journey: 650 g shaped loaf, internal probe, refrigerator set to 1.7 C).
2. Derives an effective surface coefficient h_eff from that fit and builds the default table
   tau_c(mass, container) from tau = m cp / (h_eff A) with a first-eigenvalue correction for
   conduction inside the dough (sphere of equal volume).
3. Integrates fermentation during the transient: growth gained while a warm dough cools,
   compared with what the fridge temperature alone would give.

Run:  py fit_cooling.py      Outputs: cooling-fit.json, cooling-residuals.csv
"""
import csv
import json
import math
from pathlib import Path

import numpy as np
from scipy.optimize import brentq, least_squares

import fermodel as fm

HERE = Path(__file__).resolve().parent
TRAPZ = getattr(np, "trapezoid", None) or np.trapz      # numpy 2 renamed trapz
RHO = 1100.0      # kg/m3, mixed dough before much gas (range 600-1200 once aerated)
CP = 2700.0       # J/kg/K, 70 % hydration dough from mass-weighted flour (1.7) and water (4.18)
K_DOUGH = 0.40    # W/m/K (Seruga et al. 2005 measured 0.35-0.59 for dense doughs at 39-68 C)


def sphere(mass_kg, rho=RHO):
    r = (3 * mass_kg / (4 * math.pi * rho)) ** (1 / 3)
    return r, 4 * math.pi * r ** 2


def tau_hours(mass_kg, h_eff, shape_area_factor=1.0, cp=CP, k=K_DOUGH):
    """First-eigenvalue cooling time constant of an equal-volume sphere, hours."""
    r, area = sphere(mass_kg)
    area *= shape_area_factor
    r_eff = r / shape_area_factor            # a flatter piece has a shorter conduction path
    biot = h_eff * r_eff / k
    zeta = brentq(lambda z: 1 - z / math.tan(z) - biot, 1e-6, math.pi - 1e-6)
    alpha = k / (RHO * cp)
    tau_exact = r_eff ** 2 / (alpha * zeta ** 2) / 3600.0
    tau_lumped = mass_kg * cp / (h_eff * area) / 3600.0
    return tau_exact, tau_lumped, biot


def main():
    with (HERE / "data" / "cooling_sj_fridge_650g.csv").open(encoding="utf-8") as f:
        rows = list(csv.DictReader(f))
    t = np.array([float(r["hours"]) for r in rows])
    temp = np.array([float(r["temp_c"]) for r in rows])

    def resid(p):
        return fm.newton_temperature(t, p[0], p[1], p[2]) - temp

    s = least_squares(resid, [temp[0], 4.0, 2.5])
    t0, t_env, tau_c = s.x
    s_fix = least_squares(lambda p: fm.newton_temperature(t, p[0], 1.7, p[1]) - temp, [temp[0], 3.0])
    out = {"measured_curve": {
        "dataset": "cooling_sj_fridge_650g.csv", "n": len(t),
        "newton_fit_free_env": {"t0_c": t0, "t_env_c": t_env, "tau_c_h": tau_c, "rmse_c": fm.rmse(s.fun)},
        "newton_fit_env_at_setpoint_1.7C": {"t0_c": s_fix.x[0], "tau_c_h": s_fix.x[1], "rmse_c": fm.rmse(s_fix.fun)},
        "note": "labels are whole deg F (about +-0.3 C); the first half hour cools slower than Newton "
                "(core probe lag), and the asymptote sits about 2 C above the set point"}}
    with (HERE / "cooling-residuals.csv").open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["hours", "temp_c_measured", "temp_c_newton", "residual_c"])
        for ti, me, pr in zip(t, temp, fm.newton_temperature(t, t0, t_env, tau_c)):
            w.writerow([ti, me, round(float(pr), 2), round(float(pr - me), 2)])

    # effective surface coefficient that reproduces the measured tau for 0.65 kg
    h_cal = brentq(lambda h: tau_hours(0.65, h)[0] - tau_c, 0.5, 60.0)
    out["h_eff_calibrated_W_m2K"] = h_cal
    out["physics"] = {"rho_kg_m3": RHO, "cp_J_kgK": CP, "k_W_mK": K_DOUGH,
                      "formula": "tau = R^2 / (alpha zeta1^2), zeta1 from 1 - zeta cot(zeta) = Bi, Bi = h R / k, "
                                 "R = equal-volume sphere radius / shape factor; lumped limit tau = m cp / (h A)"}
    containers = {
        "banneton_with_liner_in_bag (calibrated on the measured loaf)": (h_cal, 1.0),
        "covered_bowl_or_tub_thin_plastic_or_metal": (8.0, 1.0),
        "thick_glass_or_ceramic_bowl_with_lid (bowl mass adds 20-40 %)": (6.0, 1.0),
        "pizza_balls_in_closed_tray (flattened, shape factor 1.4)": (6.0, 1.4),
        "bare_on_tray_fan_assisted_fridge": (12.0, 1.2),
    }
    table = {}
    for name, (h, sf) in containers.items():
        table[name] = {"h_eff_W_m2K": round(h, 1), "shape_area_factor": sf}
        for m in (0.2, 0.5, 1.0, 2.0):
            te, tl, bi = tau_hours(m, h, sf)
            table[name][f"{m}kg"] = {"tau_c_h": round(te, 2), "lumped_h": round(tl, 2), "biot": round(bi, 2)}
    out["tau_c_table"] = table
    out["scaling_rule"] = {"formula": "tau_c = 2.8 h * (m / 0.65 kg)^(1/3) * (h_ref / h_eff) / shape_area_factor",
                           "h_ref_W_m2K": round(h_cal, 1), "exponent_check": "sphere: tau proportional to m / A = m^(1/3)"}

    # fermentation during the transient
    def transient(gamma, t_start, t_env_c, tau, hours):
        ts = np.linspace(0, hours, 2000)
        temps = fm.newton_temperature(ts, t_start, t_env_c, tau)
        eq_ref = float(TRAPZ(gamma(temps), ts))            # equivalent hours at 25 C
        iso = hours * float(gamma(t_env_c))
        return {"equivalent_h_at_25C": round(eq_ref, 2), "isothermal_at_env_h_at_25C": round(iso, 2),
                "extra_from_transient_h_at_25C": round(eq_ref - iso, 2)}

    ex = {}
    for label, gamma in (("sourdough", fm.gamma_sourdough), ("yeast", fm.gamma_yeast)):
        for start in (21.0, 26.0):
            for tau in (1.5, 2.8, 4.0):
                ex[f"{label}: {start:.0f} C dough into a 4 C fridge, tau_c {tau} h, 12 h"] = transient(gamma, start, 4.0, tau, 12.0)
        ex[f"{label}: 4 C dough warming at 22 C, tau_c 2.8 h, 2 h"] = transient(gamma, 4.0, 22.0, 2.8, 2.0)
    out["fermentation_during_transient"] = ex

    (HERE / "cooling-fit.json").write_text(json.dumps(out, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
