"""Validation against a private bake log: aggregates only.

Input: research/data/bakes-project.csv. That file is private and git-ignored. This script
reads it if it is present and writes ONLY per-regime aggregates (n, median and range of the
log residual, share inside the uncalibrated band). No row, id, date, flour or text from the
log is written anywhere. If the file is absent the script says so and writes nothing.

Rows are classified by generic rules on their columns (leavening, phase labels, numbers found
in the endpoint text); rows without a duration or an endpoint are counted as unusable.

Run:  py validate_private_bakes.py      Output: private-bakes-validation.json
"""
import csv
import json
import math
import re
from pathlib import Path

import numpy as np

import fermodel as fm

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parents[3] / "data" / "bakes-project.csv"
Z80 = 1.2816
BAND_SOURDOUGH = math.exp(Z80 * math.sqrt(0.35 ** 2 + 0.20 ** 2))
BAND_YEAST = math.exp(Z80 * math.sqrt(0.45 ** 2 + 0.20 ** 2))
TO_IDY = {"instant": 1.0, "fresh": 0.33, "active": 0.75}


def num(text):
    m = re.search(r"-?\d+(?:\.\d+)?", text or "")
    return float(m.group()) if m else float("nan")


def duration(text):
    """'8-10' -> geometric mean; '>2.5' -> (2.5, 'lower bound'); '' -> nan."""
    text = (text or "").strip()
    if not text:
        return float("nan"), ""
    bound = "lower" if text.startswith(">") else ""
    vals = [float(v) for v in re.findall(r"\d+(?:\.\d+)?", text)]
    if not vals:
        return float("nan"), ""
    return math.sqrt(vals[0] * vals[-1]), bound


def phases(row):
    out = []
    labels = [x.strip() for x in (row["phase_labels"] or "").split(";")]
    for i, part in enumerate((row["phases"] or "").split(";")):
        if ":" not in part:
            continue
        temp, dur = part.split(":", 1)
        hours, bound = duration(dur)
        out.append({"temp": num(temp), "hours": hours, "bound": bound, "label": labels[i] if i < len(labels) else ""})
    return out


def rise_target(text):
    """Mean of the first '+a-b%' or '+a%' found in the endpoint text, as a fraction."""
    m = re.search(r"\+\s*(\d+)(?:\s*-\s*(\d+))?\s*%", text or "")
    if not m:
        return float("nan")
    lo = float(m.group(1))
    hi = float(m.group(2)) if m.group(2) else lo
    return (lo + hi) / 200.0


def classify(row):
    """Returns (regime, ln(observed / engine), bound) or (None, reason, '')."""
    lev = row["leavening"].lower()
    ph = phases(row)
    if not ph or not (ph[0]["hours"] == ph[0]["hours"]):
        return None, "no duration", ""
    first = ph[0]
    temp, hours, bound = first["temp"], first["hours"], first["bound"]
    endpoint = row["endpoint"] or ""
    hyd = num(row["hydration_pct"])
    salt = num(row["salt_pct"])

    if "refresh" in lev:
        m = re.search(r"1\s*:\s*(\d+(?:\.\d+)?)", row["dose_basis"] or "")
        if not m:
            return None, "no feed ratio", ""
        seed_h = num(row["starter_hydration"])
        seed_h = 100.0 if seed_h != seed_h else seed_h
        iota = fm.inoculation_from_feed(float(m.group(1)), seed_h)
        pred = float(fm.tau_starter_peak(temp, iota, seed_h))
        return "sourdough and starter at room temperature", math.log(hours / pred), bound

    if lev.startswith("sourdough"):
        if first["label"] != "bulk":
            return None, "no bulk phase", ""
        rise = rise_target(endpoint)
        if rise != rise:
            if "finished" in endpoint.lower():
                rise, bound = 0.5, "upper"
            else:
                return None, "no endpoint", ""
        dose = num(row["dose_pct_flour"])
        seed_h = num(row["starter_hydration"])
        seed_h = 100.0 if seed_h != seed_h else seed_h
        starter_flour = dose / (1 + seed_h / 100.0)
        iota = starter_flour / 100.0 if "incl" in (row["dose_basis"] or "") else starter_flour / (100.0 + starter_flour)
        mod = 1.0
        if hyd == hyd:
            mod *= fm.hydration_modifier_sourdough(hyd)
        if salt == salt:
            mod *= fm.salt_modifier_sourdough(salt)
        pred = float(fm.tau_sourdough(temp, iota, rise, modifier=mod))
        return "sourdough and starter at room temperature", math.log(hours / pred), bound

    kind = next((k for k in TO_IDY if k in lev), None)
    if kind is None:
        return None, "leavening not modelled", ""
    idy = num(row["dose_pct_flour"]) * TO_IDY[kind]
    label = first["label"].lower()
    if "poolish" in lev and ("poolish" in label or "preferment" in label):
        m = re.search(r"(\d+(?:\.\d+)?)\s*%\s*yeast on poolish flour", endpoint)
        if m:
            idy = float(m.group(1)) * TO_IDY[kind]
        if "barely" in endpoint.lower():
            return "yeast preferments, rich doughs and fridge cases", float("nan"), "qualitative"
        pred = float(fm.tau_yeast(temp, idy, fm.YEAST_ENDPOINT_FACTOR["ripe_poolish"]))
        return "yeast preferments, rich doughs and fridge cases", math.log(hours / pred), bound
    if label == "bulk" and temp >= 15:
        rise = rise_target(endpoint)
        if rise != rise:
            return None, "no endpoint", ""
        factor = np.interp(rise, [0.0, 0.5, 0.75, 1.0], [0.0, fm.YEAST_ENDPOINT_FACTOR["rise_50"],
                                                         fm.YEAST_ENDPOINT_FACTOR["rise_75"], 1.0])
        pred = float(fm.tau_yeast(temp, idy, float(factor)))
        return "yeast, direct dough at room temperature", math.log(hours / pred), bound
    if "retard" in label:
        if "not risen" in endpoint.lower():
            return "yeast preferments, rich doughs and fridge cases", float("nan"), "qualitative"
        return None, "no endpoint", ""
    return None, "not classified", ""


def main():
    if not SOURCE.exists():
        print("private log not present: nothing written")
        return
    with SOURCE.open(encoding="utf-8") as f:
        rows = list(csv.DictReader(f))
    regimes, unusable = {}, 0
    for row in rows:
        regime, value, bound = classify(row)
        if regime is None:
            unusable += 1
            continue
        regimes.setdefault(regime, []).append((value, bound))
    out = {"dataset": "private bake log (not published); aggregates only",
           "n_rows": len(rows), "n_unusable": unusable,
           "residual": "ln(observed hours / engine hours at vigour 1); positive = slower than the engine",
           "caveat": "most temperatures in the log are room or target temperatures, not probe readings; "
                     "the rows come from one kitchen, so the sourdough rows share one culture",
           "regimes": {}}
    for regime, vals in sorted(regimes.items()):
        numeric = [v for v, _ in vals if v == v]
        band = BAND_SOURDOUGH if regime.startswith("sourdough") else BAND_YEAST
        entry = {"n": len(vals), "n_numeric": len(numeric),
                 "n_bounds_only": sum(1 for v, b in vals if v == v and b in ("upper", "lower")),
                 "n_qualitative_far_less_rise_than_engine": sum(1 for v, _ in vals if v != v)}
        if numeric:
            entry.update({"median_ln": round(float(np.median(numeric)), 2),
                          "min_ln": round(min(numeric), 2), "max_ln": round(max(numeric), 2),
                          "share_inside_uncalibrated_80pct_band": round(
                              sum(1 for v in numeric if abs(v) <= math.log(band)) / len(numeric), 2)})
            if len(numeric) > 1:
                entry["sd_ln_around_own_median"] = round(float(np.std(numeric, ddof=1)), 2)
        out["regimes"][regime] = entry
    (HERE / "private-bakes-validation.json").write_text(json.dumps(out, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
