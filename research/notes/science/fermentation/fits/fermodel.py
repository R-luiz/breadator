"""Reference implementation of the Breadator fermentation model (research/data/model-fermentation.json).

Pure functions, numpy only. Every fit script in this folder imports from here, so the numbers
in the residual CSVs are produced by exactly these equations.

Units: temperatures in deg C, times in hours, doses in baker's percent (flour = 100).
"""
import math

import numpy as np

# --------------------------------------------------------------------------- temperature curves
T_REF = 25.0


def gamma_form(temp_c, b, c, t_max, t_ref=T_REF):
    """Relative rate, 1.0 at t_ref:  exp(c (T - Tref)) * ((Tmax - T) / (Tmax - Tref))**b.

    This is Gaenzle et al. 1998 model 1a (mu = a x^b exp(-c x), x = Tmax - T) normalised at the
    reference temperature instead of at the optimum. Optimum at Tmax - b / c. Zero at and
    above Tmax. At low temperature it tends to an exponential with slope c - b / (Tmax - T).
    """
    t = np.asarray(temp_c, dtype=float)
    x = np.maximum(t_max - t, 1e-12)
    g = np.exp(c * (t - t_ref)) * (x / (t_max - t_ref)) ** b
    return np.where(t < t_max, g, 0.0)


def gaenzle_1a(temp_c, a_prime, b, c, t_max):
    """Gaenzle 1998 Table 2, model 1a, as published: mu/mu_opt = a' x^b exp(-c x)."""
    t = np.asarray(temp_c, dtype=float)
    x = np.maximum(t_max - t, 1e-12)
    return np.where(t < t_max, a_prime * x ** b * np.exp(-c * x), 0.0)


def gaenzle_1a_normalised(temp_c, b, c, t_max):
    """Same curve rescaled so that its maximum (x = b / c) is exactly 1 (a' is rounded in print)."""
    xo = b / c
    peak = xo ** b * math.exp(-c * xo)
    t = np.asarray(temp_c, dtype=float)
    x = np.maximum(t_max - t, 1e-12)
    return np.where(t < t_max, x ** b * np.exp(-c * x) / peak, 0.0)


# Published strain parameters (Gaenzle, Ehmann & Hammes 1998, Table 2, model 1a; Tmax from text)
GAENZLE = {
    "C_milleri_LTH_H198": {"mu_opt": 0.42, "a_prime": 0.029, "b": 3.0, "c": 0.34, "t_max": 36.0},
    "L_sanfranciscensis_LTH2581": {"mu_opt": 0.71, "a_prime": 0.097, "b": 2.0, "c": 0.22, "t_max": 41.0},
    "L_sanfranciscensis_LTH1729": {"mu_opt": 0.67, "a_prime": 0.19, "b": 1.5, "c": 0.19, "t_max": 41.0},
}


def gamma_blend_published(temp_c, w_yeast=0.5, t_ref=T_REF):
    """Equal-weight blend of the yeast (H198) and LAB (LTH2581) curves, 1.0 at t_ref."""
    y = GAENZLE["C_milleri_LTH_H198"]
    l = GAENZLE["L_sanfranciscensis_LTH2581"]

    def blend(t):
        return (w_yeast * gaenzle_1a_normalised(t, y["b"], y["c"], y["t_max"])
                + (1 - w_yeast) * gaenzle_1a_normalised(t, l["b"], l["c"], l["t_max"]))

    return blend(temp_c) / blend(t_ref)


def ctmi(temp_c, t_min, t_opt, t_max):
    """Cardinal temperature model with inflection (Rosso et al. 1993); 1.0 at t_opt."""
    t = np.asarray(temp_c, dtype=float)
    num = (t - t_max) * (t - t_min) ** 2
    den = (t_opt - t_min) * ((t_opt - t_min) * (t - t_opt) - (t_opt - t_max) * (t_opt + t_min - 2 * t))
    g = num / den
    return np.where((t <= t_min) | (t >= t_max), 0.0, np.maximum(g, 0.0))


# --------------------------------------------------------------------------- engine defaults
SOURDOUGH = {"mu_ref": 0.30, "b": 3.64, "c": 0.314, "t_max": 40.4}      # h^-1 at 25 C, reference dough
YEAST = {"A": 0.91, "a": 0.72, "b": 1.32, "c": 0.190, "t_max": 45.0}    # h at 1 % IDY, 25 C, "doubled"
YEAST_ENDPOINT_FACTOR = {"rise_50": 0.70, "rise_75": 0.85, "doubled": 1.00, "ripe_poolish": 1.90}
NO_SALT_SOURDOUGH = 1.18


def gamma_sourdough(temp_c, p=SOURDOUGH):
    return gamma_form(temp_c, p["b"], p["c"], p["t_max"])


def gamma_yeast(temp_c, p=YEAST):
    return gamma_form(temp_c, p["b"], p["c"], p["t_max"])


def inoculation(starter_pct, starter_hydration_pct=100.0):
    """Prefermented-flour fraction iota from starter mass as % of ADDED flour (recipe style)."""
    starter_flour = np.asarray(starter_pct, dtype=float) / (1.0 + starter_hydration_pct / 100.0)
    return starter_flour / (100.0 + starter_flour)


def inoculation_from_feed(flour_parts, starter_hydration_pct=100.0, seed_parts=1.0):
    """iota for a starter refresh seed:flour:water = seed_parts:flour_parts:..."""
    seed_flour = seed_parts / (1.0 + starter_hydration_pct / 100.0)
    return seed_flour / (seed_flour + flour_parts)


def sourdough_growth_needed(rise_fraction, iota):
    """G* = ln(K / iota) with K = rise + iota  (rise 0.5 = aliquot +50 %, 1.0 = doubled)."""
    return np.log1p(np.asarray(rise_fraction, dtype=float) / np.asarray(iota, dtype=float))


def tau_sourdough(temp_c, iota, rise_fraction, vigour=1.0, modifier=1.0, lag_h_ref=0.0, p=SOURDOUGH):
    """Hours at constant dough temperature to reach the rise endpoint."""
    rate = p["mu_ref"] * vigour * modifier * gamma_sourdough(temp_c, p)
    return sourdough_growth_needed(rise_fraction, iota) / rate + lag_h_ref / gamma_sourdough(temp_c, p)


def hydration_modifier_sourdough(hydration_pct):
    """Rate multiplier for the water content of a sourdough system: 1 + 0.005 (H - 75), within 0.85-1.15."""
    return float(np.clip(1.0 + 0.005 * (float(hydration_pct) - 75.0), 0.85, 1.15))


def salt_modifier_sourdough(salt_pct):
    """Rate multiplier for salt in a sourdough, 1.0 at 2 % of flour."""
    return 1.0 + 0.09 * (2.0 - float(salt_pct))


def tau_starter_peak(temp_c, iota, starter_hydration_pct=100.0, vigour=1.0, k_peak=1.0, lag_h_ref=0.0, p=SOURDOUGH):
    """Hours from feeding to volume peak: no salt (x 1.18) and the hydration term of the starter itself."""
    modifier = NO_SALT_SOURDOUGH * hydration_modifier_sourdough(starter_hydration_pct)
    rate = p["mu_ref"] * vigour * modifier * gamma_sourdough(temp_c, p)
    return np.log(k_peak / np.asarray(iota, dtype=float)) / rate + lag_h_ref / gamma_sourdough(temp_c, p)


def tau_yeast(temp_c, idy_pct, endpoint_factor=1.0, vigour=1.0, modifier=1.0, p=YEAST):
    """Hours at constant dough temperature; idy_pct = instant-dry-yeast equivalent, % of flour."""
    return (endpoint_factor * p["A"] * np.asarray(idy_pct, dtype=float) ** (-p["a"])
            / (vigour * modifier * gamma_yeast(temp_c, p)))


def progress(times_h, temps_c, tau_fn):
    """P = sum(dt / tau(T)) over a piecewise-constant temperature history; endpoint at P = 1."""
    times_h = np.asarray(times_h, dtype=float)
    temps_c = np.asarray(temps_c, dtype=float)
    dt = np.diff(times_h)
    mid = 0.5 * (temps_c[1:] + temps_c[:-1])
    return float(np.sum(dt / tau_fn(mid)))


def newton_temperature(t_h, t0_c, t_env_c, tau_c_h):
    """Lumped (Newton) dough temperature after t_h hours in surroundings at t_env_c."""
    return t_env_c + (t0_c - t_env_c) * np.exp(-np.asarray(t_h, dtype=float) / tau_c_h)


def rmse(x):
    x = np.asarray(x, dtype=float)
    return float(np.sqrt(np.mean(x ** 2)))
