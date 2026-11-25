"""Assembles research/data/model-fermentation.json from the fit outputs in this folder.

Run the fit scripts first (fit_yeast.py, fit_sourdough.py, fit_starter_peak.py, fit_cooling.py,
calibration_demo.py), then:  py build_model_json.py
"""
import json
from pathlib import Path

import fermodel as fm

HERE = Path(__file__).resolve().parent
OUT = HERE.parents[3] / "data" / "model-fermentation.json"


def load(name):
    return json.loads((HERE / name).read_text(encoding="utf-8"))


def r(x, n=3):
    return round(float(x), n)


def main():
    y = load("yeast-fit.json")
    s = load("sourdough-fit.json")
    sp = load("starter-peak-fit.json")
    co = load("cooling-fit.json")
    ca = load("calibration-demo.json")
    yc = y["models"]["M_card"]
    private_rows = []
    private_file = HERE / "private-bakes-validation.json"
    if private_file.exists():
        pv = load("private-bakes-validation.json")
        for regime, st in pv["regimes"].items():
            text = (f"n numeric {st['n_numeric']} (bounds only: {st['n_bounds_only']}), qualitative 'far less rise than the engine': "
                    f"{st['n_qualitative_far_less_rise_than_engine']}")
            if "median_ln" in st:
                text += f"; median ln(obs / engine) {st['median_ln']}, range {st['min_ln']} to {st['max_ln']}"
            if "sd_ln_around_own_median" in st:
                text += f", sd around own median {st['sd_ln_around_own_median']}"
            private_rows.append({"dataset": "private bake log, aggregate only: " + regime, "n": st["n"], "residual": text,
                                 "file": "fits/private-bakes-validation.json", "evidence": "anecdote",
                                 "note": "one kitchen; temperatures mostly not probe-measured; no rows published"})
    sj = s["sj_timetables_fit"]

    SRC = {
        "craig_yeast": "TXCraig1, pizzamaking.com topic 26831, reply #260 chart (2015-08-22), Wayback copy of attachment 221784",
        "craig_method": "TXCraig1, pizzamaking.com topic 26831, reply #4 (2013-08-07): Gaenzle-form growth curve, Tmax 45 C; Wayback 2014-01-14",
        "craig_sd": "TXCraig1, pizzamaking.com topic 22649, chart Rev1 (Wayback copy of attachment 223566)",
        "cf_calc": "derwille1/cf-calculator index.html line 119 (A=30.91, ALPHA=0.72, BETA=0.110, REF_TEMP=35)",
        "gaenzle": "doi:10.1128/AEM.64.7.2616-2623.1998, Table 2 (model 1a) and Results text for Tmax",
        "salvado": "doi:10.1128/AEM.01861-10, Table 2",
        "sj_tt": "The Sourdough Journey, Bulk Fermentation Timetables V1.0 (Dec 2022), PDF p. 1",
        "sj_rise": "The Sourdough Journey, 'The mystery of percentage rise in bulk fermentation' (guideline table; fridge chart; phases)",
        "sj_unfed": "The Sourdough Journey, 'Can you use an unfed starter directly from the refrigerator?'",
        "sj_flour": "The Sourdough Journey, 'White, wheat, or rye? What is the best flour to feed your starter?'",
        "sj_cold": "The Sourdough Journey, 'Experiment: 24-hour bulk fermentation using a cold proofer'",
        "sj_faq": "The Sourdough Journey, FAQ 'Starter maintenance'",
        "wraith": "Bill Wraith rise-time table (2007-08) preserved at the-bread-code.io/fermentation-table/; method post thefreshloaf.com/node/4129",
        "framework": "The Sourdough Framework ch. 7.5 'How much starter?', Table 7.1, Table 7.3 (CC BY-SA 4.0)",
        "technomitron_poolish": "Technomitron (AAINB), 'Les methodes de fermentation sur levure', poolish dose table (22-24 C)",
        "technomitron_base": "Technomitron (AAINB), 'Methodes de petrissage', sections 'Chiffres de base utilisees' and 'Utilite du chiffre de base'",
        "technomitron_yeast": "Technomitron (AAINB), 'La levure de boulangerie' (dry matter of yeast forms; doses)",
        "bakerpedia": "BAKERpedia, 'Poolish', table of compressed-yeast inoculation vs fermentation time (26-29 C)",
        "weekendbakery": "Weekend Bakery, 'Artisan bread baking tips: Poolish & biga', section on instant-yeast percentages",
        "lbu13": "Lallemand Baking Update vol. 1 no. 3 'Yeast Characteristics', figure 'Effect of sugar on yeast activity' (Wayback 2003)",
        "lbu14": "Lallemand Baking Update vol. 1 no. 4 (yeast dose), list of factors affecting gas production and retention (Wayback 2003)",
        "lbu16": "Lallemand Baking Update vol. 1 no. 6 'Dry Yeast' (Wayback 2003)",
        "lbu29": "Lallemand Baking Update vol. 2 no. 9 'Working with Instant Yeast' (Wayback 2003)",
        "redstar": "Red Star Yeast, 'Yeast conversion chart' (0.6 oz cake = one 1/4 oz packet dry)",
        "fleischmann": "Fleischmann's Yeast FAQ, 'How do I substitute dry yeast for Fresh Active Yeast?'",
        "carcea": "doi:10.3390/foods9070952, section 3 Rheofermentometer results and Figure 9",
        "codina": "doi:10.17221/216/2012-CJFS, Table 2",
        "nestec": "US patent 5,827,724 (Nestec), description of Figs. 7-8 and claims",
        "ka_ddt": "King Arthur Baking, 'Desired dough temperature' (2018-05-29)",
        "ka_ff": "King Arthur Baking, 'Determining the friction factor in baking' (2018-08-27)",
        "et_food": "Engineering Toolbox, 'Food and foodstuff - specific heat' (above-freezing column)",
        "et_solids": "Engineering Toolbox, 'Solids - specific heats' and 'Liquids and fluids - specific heats'",
        "seruga": "doi:10.17221/3385-CJFS, Tables 3-4",
        "external": "third-party compilation RomaricErnst/baker-hub docs/yeast-model/DATASET.csv (rows not re-verified)",
        "breadcode_tools": "research/notes/breadcode/tools.md sections 4a-4b (The Bread Code pages and MIT repositories)",
        "dicagno": "doi:10.1128/AEM.00309-14, abstract",
        "aslankoohi": "doi:10.1371/journal.pone.0119364, Methods and Results",
    }

    URL = {
        "craig_yeast": "http://web.archive.org/web/20151010043650/http://www.pizzamaking.com/forum/index.php?action=dlattach;topic=26831.0;attach=221784;image",
        "craig_method": "http://web.archive.org/web/20140114131230/http://www.pizzamaking.com:80/forum/index.php/topic,26831.0.html",
        "craig_sd": "http://web.archive.org/web/20151017014811/http://www.pizzamaking.com/forum/index.php?action=dlattach;topic=22649.0;attach=223566;image",
        "cf_calc": "https://raw.githubusercontent.com/derwille1/cf-calculator/main/index.html",
        "gaenzle": "https://pmc.ncbi.nlm.nih.gov/articles/PMC106434/", "salvado": "https://pmc.ncbi.nlm.nih.gov/articles/PMC3067424/",
        "sj_tt": "https://thesourdoughjourney.com/wp-content/uploads/2023/01/Bulk-Fermentation-Timetables-V1.0.pdf",
        "sj_rise": "https://thesourdoughjourney.com/the-mystery-of-percentage-rise-in-bulk-fermentation/",
        "sj_unfed": "https://thesourdoughjourney.com/can-you-use-an-unfed-starter-directly-from-the-refrigerator/",
        "sj_flour": "https://thesourdoughjourney.com/white-wheat-or-rye-what-is-the-best-flour-to-feed-your-starter/",
        "sj_cold": "https://thesourdoughjourney.com/experiment-24-hour-bulk-fermentation-using-a-cold-proofer/",
        "sj_faq": "https://thesourdoughjourney.com/faq-starter-maintenance/",
        "wraith": "https://www.the-bread-code.io/fermentation-table/", "framework": "https://github.com/hendricius/the-sourdough-framework",
        "technomitron_poolish": "https://technomitron.aainb.com/la-fermentation/les-methodes-de-fermentation-sur-levure/",
        "technomitron_base": "https://technomitron.aainb.com/petrissage/methodes-de-petrissage/",
        "technomitron_yeast": "https://technomitron.aainb.com/constituants-pain-et-pate/la-levure-de-boulangerie/",
        "bakerpedia": "https://bakerpedia.com/processes/poolish/",
        "weekendbakery": "https://www.weekendbakery.com/posts/more-artisan-bread-baking-tips-poolish-biga/",
        "lbu13": "http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/1_3YEAST.PDF",
        "lbu14": "http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/1_4DOSE.PDF",
        "lbu16": "http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/1_6DRYYE.PDF",
        "lbu29": "http://web.archive.org/web/2003/http://www.lallemand.com:80/BakerYeastNA/eng/PDFs/LBU%20PDF%20FILES/2_9INST.PDF",
        "redstar": "https://redstaryeast.com/yeast-baking-lessons/yeast-conversion-table/",
        "fleischmann": "https://www.fleischmannsyeast.com/frequently-asked-questions/",
        "carcea": "https://pmc.ncbi.nlm.nih.gov/articles/PMC7404662/", "codina": "https://www.agriculturejournals.cz/publicFiles/216_2012-CJFS.pdf",
        "nestec": "https://patents.google.com/patent/US5827724A/en",
        "ka_ddt": "https://www.kingarthurbaking.com/blog/2018/05/29/desired-dough-temperature",
        "ka_ff": "https://www.kingarthurbaking.com/blog/2018/08/27/determining-the-friction-factor-in-baking",
        "et_food": "https://www.engineeringtoolbox.com/specific-heat-capacity-food-d_295.html",
        "et_solids": "https://www.engineeringtoolbox.com/specific-heat-solids-d_154.html",
        "seruga": "https://www.agriculturejournals.cz/publicFiles/50561.pdf",
        "external": "https://github.com/RomaricErnst/baker-hub/blob/main/docs/yeast-model/DATASET.csv",
        "breadcode_tools": "https://www.the-bread-code.io/tutorial/2023/05/06/sourdough-starter-feeding-ratio.html",
        "dicagno": "https://doi.org/10.1128/AEM.00309-14", "aslankoohi": "https://doi.org/10.1371/journal.pone.0119364",
    }

    model = {
        "schema_version": "2.0",
        "source_index": [{"source": SRC[k], "url": URL[k]} for k in SRC],
        "claims_file": "research/data/claims-science-fermentation.jsonl",
        "references_file": "research/data/references-science-fermentation.json",
        "notes": "research/notes/science/fermentation/ (SUMMARY.md and notes 01-09); reference implementation fits/fermodel.py",
        "generated": "2026-11-24",
        "status": "parameterised engine with calibration; every default is replaceable by a logged-bake calibration",
        "units": {"temperature": "deg C", "time": "h", "dose": "baker's % (flour = 100)", "mass": "g", "energy": "J"},
        "evidence_labels": ["measured", "literature", "creator-rule", "anecdote", "extrapolation"],

        "progress_integration": {
            "equation": "P = sum over phases of dt / tau(T_dough(t), inputs); the endpoint is reached when P = 1",
            "sourdough_equivalent": "G = integral of mu(T_dough(t)) dt; rise R = iota (exp(G) - 1); endpoint when G = ln(K / iota)",
            "time_step": "integrate T_dough(t) in steps of 5 min or less during cooling and warming, 15 min otherwise",
            "dough_temperature": "use a probe reading when the user gives one; otherwise the Newton model in 'cooling_warming'",
            "phase_carry_over": "P (or G) carries over between bulk, bench, shaped proof and fridge; shaping does not reset it",
            "evidence": "extrapolation",
            "note": "additivity of rate over a temperature history is the standard assumption of predictive microbiology; "
                    "it ignores adaptation lag after a temperature step, which matters most on warm-up from the fridge",
        },

        "endpoints": {
            "definition": "fractional volume rise R of an aliquot taken right after mixing, straight-sided jar, same temperature as the dough",
            "sourdough": {
                "mapping": "K = R + iota  (tau = ln(K / iota) / mu)",
                "rise_25": {"R": 0.25, "K": "0.25 + iota"}, "rise_50": {"R": 0.50, "K": "0.50 + iota"},
                "rise_75": {"R": 0.75, "K": "0.75 + iota"}, "doubled": {"R": 1.00, "K": "1.00 + iota"},
                "starter_peak": {"K": 1.0, "note": "ripe-starter density is the unit of the model"},
                "evidence": "measured",
                "sources": [SRC["sj_tt"], SRC["wraith"]],
                "support": "one growth rate reproduces both the dose slope and the +75 % to +100 % spacing of the "
                           f"Sourdough Journey timetables (rmse {r(sj['log_growth_with_literature_gamma']['rmse_ln'])} in ln h, n = {sj['n']}); "
                           "Wraith's doubling times follow ln(1 + 1/iota) to within 4 % up to 15 % inoculation",
                "target_by_flour_and_temperature": {
                    "framework_table_7_3": {"protein_8_10": 0.25, "protein_10_12": 0.50, "protein_12_15": 1.00,
                                            "evidence": "creator-rule", "source": SRC["framework"], "license": "CC-BY-SA-4.0"},
                    "sourdough_journey_with_12_16h_cold_retard": {"27C": 0.30, "24C": 0.50, "21C": 0.75, "18C": 1.00,
                                                                  "evidence": "creator-rule", "source": SRC["sj_rise"]},
                    "note": "the two rules disagree in what drives the target (flour strength vs dough temperature before a retard); "
                            "the engine removes the second effect by integrating the retard explicitly",
                },
            },
            "yeast": {
                "mapping": "tau(endpoint) = f_endpoint * tau_doubled",
                "f_endpoint": {
                    "rise_50": {"value": 0.70, "range": [0.55, 0.85], "evidence": "extrapolation"},
                    "rise_75": {"value": 0.85, "range": [0.75, 0.93], "evidence": "extrapolation"},
                    "doubled": {"value": 1.00, "range": [1.0, 1.0], "evidence": "creator-rule"},
                    "pizza_ball_ready_to_bake": {"value": 1.00, "range": [0.8, 1.3], "evidence": "creator-rule"},
                    "tripled": {"value": 1.60, "range": [1.4, 2.0], "evidence": "extrapolation"},
                    "ripe_poolish": {"value": 1.90, "range": y["ripe_poolish_factor"]["quartiles"],
                                     "table_to_table_range": [1.6, 3.2], "evidence": "creator-rule",
                                     "n": y["ripe_poolish_factor"]["n"]},
                },
                "sources": [SRC["craig_yeast"], SRC["technomitron_poolish"], SRC["bakerpedia"], SRC["weekendbakery"],
                            SRC["codina"], SRC["external"]],
                "basis": "at a bakery dose the gas production stays roughly constant over three hours "
                         "(58.5, 65.5 and 57.5 mL/g measured after 60, 120 and 180 min at 30 C with 3 % yeast, Codina 2013 Table 2), which gives a near-linear rise after a short lag "
                         "(+50 % at 0.60 of the doubling time); a dough that still grows its yeast rises like a sourdough (+50 % at about 0.79); "
                         "the engine takes the midpoint. The chart baseline matches independent 'doubled' statements "
                         f"(median factor {y['validation_isothermal']['ext:direct dough:doubled']['time_factor']}, n = {y['validation_isothermal']['ext:direct dough:doubled']['n']})",
            },
        },

        "commercial_yeast": {
            "equation": "tau_doubled = A * d^(-a) * exp(-c (T - T0)) * ((Tmax - T0) / (Tmax - T))^b / (v * M)",
            "equivalent_form": "tau_doubled = A * d^(-a) / (v * M * gamma_y(T)),  gamma_y(T) = exp(c (T - T0)) ((Tmax - T) / (Tmax - T0))^b",
            "inputs": {"d": "instant-dry-yeast equivalent, % of the flour in the fermenting mass (preferment flour for a poolish)",
                       "T": "dough temperature", "v": "personal vigour factor (default 1)", "M": "product of modifiers"},
            "parameters": {
                "A": {"value": 0.91, "unit": "h at d = 1 % IDY, T0 = 25 C, endpoint 'doubled'", "range": [0.6, 1.6],
                      "fit_value": r(yc["A_h"], 4), "evidence": "creator-rule", "confidence": "medium",
                      "sources": [SRC["craig_yeast"], SRC["external"]],
                      "valid_domain": "lean dough, 55-70 % hydration, 2-3 % salt",
                      "note": "range = spread of independent sources with a stated doubling endpoint"},
                "a": {"value": 0.72, "unit": "-", "range": [0.60, 0.90], "fit_value": r(yc["a"], 4),
                      "evidence": "creator-rule", "confidence": "medium",
                      "sources": [SRC["craig_yeast"], SRC["technomitron_poolish"], SRC["bakerpedia"], SRC["cf_calc"]],
                      "independent_estimates": y["dose_exponent_independent"],
                      "valid_domain": "0.003-0.32 % IDY (chart); the French poolish table gives the same exponent up to 0.75 % IDY equivalent"},
                "c": {"value": 0.190, "unit": "1/C", "range": [0.12, 0.20], "fit_value": r(yc["c_per_C"], 4),
                      "evidence": "creator-rule", "confidence": "medium", "sources": [SRC["craig_yeast"], SRC["craig_method"]],
                      "valid_domain": "1.7-35 C"},
                "b": {"value": 1.32, "unit": "-", "range": [1.0, 3.0], "fit_value": r(yc["b"], 4),
                      "evidence": "creator-rule", "confidence": "medium", "sources": [SRC["craig_yeast"]],
                      "valid_domain": "1.7-35 C; controls the flattening toward the optimum"},
                "Tmax": {"value": 45.0, "unit": "C", "range": [42.0, 46.1], "evidence": "literature", "confidence": "medium",
                         "sources": [SRC["salvado"], SRC["craig_method"]],
                         "note": "growth Tmax of ten S. cerevisiae strains 42.0-46.1 C (none a baker's strain); fixed, not fitted"},
                "T0": {"value": 25.0, "unit": "C", "evidence": "definition"},
                "Topt_derived": {"value": r(45.0 - 1.32 / 0.190, 1), "unit": "C", "formula": "Tmax - b / c", "evidence": "extrapolation",
                                 "note": "beyond the 35 C edge of the chart: extrapolation"},
            },
            "local_time_q10": {k + "C": r(v, 2) for k, v in y["local_q10_of_engine_curve"].items()},
            "fit": {"dataset": "fits/data/yeast_craig_chart.csv", "n_fitted": y["n_fitted"],
                    "rmse_ln_by_form": {k: {kk: (r(vv) if vv is not None else None) for kk, vv in v["rmse_ln"].items()}
                                        for k, v in y["models"].items()},
                    "reading": "the cardinal-exponential form is the only one within the chart's rounding everywhere; "
                               "the cf-calculator regression is exact below 8 C and wrong by a factor 2-5 above 15 C; "
                               "a CTMI cannot follow the chart's cold tail"},
            "cardinal_reference": {
                "S_cerevisiae_growth_CTMI": {"Tmin": [0.4, 5.0], "Topt": [30.0, 34.8], "Tmax": [42.0, 46.1], "mu_opt_per_h": [0.28, 0.45],
                                             "species_mean": {"Topt": 32.3, "Tmax": 45.4},
                                             "evidence": "measured", "source": SRC["salvado"],
                                             "note": "growth in YNB broth; gas production in dough continues below the growth Tmin"},
                "cold_gas_commercial_yeast": {"statement": "more than 40 mL CO2 per g pressed yeast after 6-7 d at 3-5 C, 5-6 d at 5-9 C, 3-4 d at 10 C (2 % maltose medium)",
                                              "evidence": "measured", "source": SRC["nestec"]},
            },
            "yeast_equivalence": {
                "to_idy_equivalent": {
                    "instant_dry": {"factor": 1.00, "range": [1.0, 1.0]},
                    "active_dry": {"factor": 0.75, "range": [0.67, 1.00]},
                    "fresh_compressed": {"factor": 0.33, "range": [0.25, 0.42]},
                    "osmotolerant_instant": {"factor": 1.00, "range": [0.6, 1.0],
                                             "note": "below about 10 % sugar it is slower than regular yeast; see modifier 'sugar'"},
                },
                "evidence": "creator-rule", "confidence": "medium",
                "sources": [SRC["lbu29"], SRC["lbu16"], SRC["redstar"], SRC["fleischmann"], SRC["craig_yeast"], SRC["cf_calc"],
                            SRC["technomitron_yeast"]],
                "data": {"lallemand": "instant has about 3.3 times the solids of fresh (96 %); rule 3:1; one pound of instant replaces 3-4 lb of fresh",
                         "red_star_and_fleischmanns": "0.6 oz cake = one 1/4 oz packet of dry yeast (2.4:1), any dry type",
                         "craig_chart_header": "CY : ADY : IDY = 1 : 0.42 : 0.32",
                         "dry_matter": "fresh 31-34 %, active dry 93 %, instant 95 %"},
                "handling": "instant: mix into the flour; active dry: rehydrate at about 40 C; add 2 % water per 1 % instant yeast when replacing fresh",
            },
            "lag": {"value_h": 0.2, "range_h": [0.1, 0.4], "applies": "added to tau when tau < 1.5 h (d above about 0.7 % IDY at 25 C)",
                    "evidence": "extrapolation", "confidence": "low",
                    "note": "the chart is floored at 2 h, so very short proofs are outside its domain; flag any prediction under 45 min"},
            "valid_domain": {"T_C": [1.7, 35.0], "idy_pct": [0.003, 1.0], "hydration_pct": [55, 80], "salt_pct": [1.5, 3.0],
                             "sugar_pct": [0, 5], "fat_pct": [0, 5]},
        },

        "sourdough": {
            "equation": "tau = ln(K / iota) / (mu_ref * v * M * gamma_s(T)) + lag / gamma_s(T),   K = R + iota",
            "rise_curve": "R(t) = iota * (exp(G) - 1),  G = integral of mu_ref * v * M * gamma_s(T(t)) dt",
            "gamma_s": "gamma_s(T) = exp(c (T - T0)) * ((Tmax - T) / (Tmax - T0))^b,  0 at T >= Tmax",
            "inputs": {"iota": "prefermented flour fraction = starter flour / (starter flour + added flour)",
                       "iota_from_recipe": "starter_flour = starter_pct / (1 + starter_hydration); iota = starter_flour / (100 + starter_flour)",
                       "R": "target rise (0.5 = aliquot +50 %)", "v": "personal vigour factor", "M": "product of modifiers"},
            "parameters": {
                "mu_ref": {"value": 0.30, "unit": "1/h at 25 C", "range": [0.15, 0.48], "fit_value": r(sj["log_growth_with_literature_gamma"]["mu_ref_per_h"], 4),
                           "evidence": "measured", "confidence": "medium", "sources": [SRC["sj_tt"]],
                           "valid_domain": "white-flour dough with 10 % whole wheat, 2 % salt, about 75 % hydration, liquid starter fed 1:2:2 and used at peak; dough 19-23 C, starter 5-20 % of flour",
                           "note": "range = vigour 0.5-1.6 found across the other sources; in broth C. milleri reaches 0.42 1/h at its optimum"},
                "c": {"value": 0.314, "unit": "1/C", "range": [0.25, 0.34], "fit_value": r(s["gamma_s_fit_to_published_blend"]["c_per_C"], 4),
                      "evidence": "literature", "confidence": "medium", "sources": [SRC["gaenzle"]]},
                "b": {"value": 3.64, "unit": "-", "range": [3.0, 4.4], "fit_value": r(s["gamma_s_fit_to_published_blend"]["b"], 4),
                      "evidence": "literature", "confidence": "medium", "sources": [SRC["gaenzle"]]},
                "Tmax": {"value": 40.4, "unit": "C", "range": [36.0, 41.0], "fit_value": r(s["gamma_s_fit_to_published_blend"]["t_max"], 2),
                         "evidence": "literature", "confidence": "medium", "sources": [SRC["gaenzle"]],
                         "note": "no growth of C. milleri at 36 C and of L. sanfranciscensis at 41 C after 7 days"},
                "T0": {"value": 25.0, "unit": "C", "evidence": "definition"},
                "Topt_derived": {"value": r(s["gamma_s_fit_to_published_blend"]["t_opt"], 1), "unit": "C", "formula": "Tmax - b / c", "evidence": "literature",
                                 "note": "yeast alone 27 C, lactobacilli 32-33 C (Gaenzle 1998)"},
                "lag": {"unit": "h at 25 C, divided by gamma_s(T)", "evidence": "measured", "confidence": "low",
                        "sources": [SRC["sj_unfed"], SRC["sj_tt"]],
                        "values": {"fed_and_used_at_peak": {"value": 0.0, "range": [0.0, 0.5]},
                                   "past_peak_or_unfed_12_24h_room": {"value": 0.5, "range": [0.0, 1.5], "evidence": "extrapolation"},
                                   "from_fridge_unfed_about_7_days": {"value": 2.0, "range": [1.0, 3.0]},
                                   "unfed_more_than_2_weeks": {"value": None, "route": "refresh once or twice first; not modelled"}},
                        "note": "one pressure-vessel test: a starter kept 7 days in the fridge peaked about 2 h later than the same starter freshly fed, in a 1:1:1 feed"},
            },
            "gamma_s_table": s["gamma_s_values"],
            "published_strain_parameters": {
                "model_1a": "mu = mu_opt * a' * x^b * exp(-c x), x = Tmax - T",
                "strains": fm.GAENZLE,
                "model_1b_Tmin_C": {"C_milleri_LTH_H198": 8.0, "L_sanfranciscensis_LTH2581": 3.0, "L_sanfranciscensis_LTH1729": 4.1},
                "evidence": "measured", "source": SRC["gaenzle"],
                "note": "growth in mMRS broth; the authors say model 1a fits poorly near Tmin; a' is rounded in print, normalise at x = b / c",
                "compact_fit_quality": {"rmse_ln": r(s["gamma_s_fit_to_published_blend"]["rmse_ln"]),
                                        "max_abs_ln": r(s["gamma_s_fit_to_published_blend"]["max_abs_ln"]), "grid": "4-35 C"},
            },
            "fit": {"dataset": "fits/data/sourdough_sj_timetables.csv", "n": sj["n"],
                    "rmse_ln": {"engine (1 parameter, literature temperature curve)": r(sj["engine_rmse_ln"]),
                                "same form, free exponential temperature slope (2 parameters)": r(sj["log_growth_free_exponential_slope"]["rmse_ln"]),
                                "power law A * iota^-a * exp(-b (T - T0)) per endpoint (6 parameters)": r(sj["power_law_per_endpoint"]["rmse_ln"])},
                    "measured_q10_19_23C": r(sj["log_growth_free_exponential_slope"]["q10"], 2),
                    "engine_mean_residual_by_temp_c": sj["engine_mean_residual_by_temp_c"],
                    "reading": "the growth form needs one rate where the power law needs three numbers per endpoint; the measured "
                               "temperature slope is flatter than the literature curve, which the uncertainty term s_T covers"},
            "table_7_1_check": s["framework_table_7_1"],
            "stiff_vs_liquid": {
                "accounting": "use the starter's real flour content in iota (a 55 % starter carries 29 % more flour per gram than a 100 % starter)",
                "rate_multiplier": {"value": 1.0, "range": [0.85, 1.15], "evidence": "extrapolation", "confidence": "low",
                                    "note": "no factor for the starter type as such; in a refresh the general hydration term applies to the starter's own hydration"},
                "sources": [SRC["framework"], SRC["dicagno"]],
                "note": "the Framework rates stiff starters as more yeast-active; Di Cagno 2014 found the highest yeast counts in liquid sourdoughs and more acids in firm ones; no rise-time comparison was found",
            },
            "valid_domain": {"T_C": [10.0, 30.0], "T_measured_C": [18.9, 23.3], "iota": [0.005, 0.20], "rise": [0.25, 1.0],
                             "salt_pct": [0, 2.5], "hydration_pct": [60, 85]},
        },

        "starter_peak": {
            "equation": "tau_peak = ln(K_peak / iota) / (mu_ref * v * M_no_salt * M_hydration * gamma_s(T)) + lag / gamma_s(T)",
            "M_hydration": "1 + 0.005 (H - 75) within 0.85-1.15, H = hydration of the starter itself (1.125 at 100 %, 0.90 at 55 %)",
            "iota_from_feed": "seed:flour:water = 1:f:w, seed hydration h: iota = (1 / (1 + h)) / (1 / (1 + h) + f)",
            "parameters": {
                "K_peak": {"value": 1.0, "range": [0.7, 1.5], "evidence": "extrapolation", "confidence": "low",
                           "implied_by_statements": sp["implied_K_peak_range"], "sources": [SRC["sj_faq"], SRC["sj_tt"], SRC["breadcode_tools"]]},
                "M_no_salt": {"value": 1.18, "range": [1.06, 1.21], "evidence": "creator-rule", "sources": [SRC["wraith"]]},
            },
            "check": {"n": sp["n_statements_used"], "median_ln_obs_over_engine": sp["median_ln_obs_over_engine"], "sd_ln": sp["sd_ln"],
                      "temperature_assumed_where_unstated_C": sp["assumed_temperature_where_unstated_c"]},
            "prediction_table_hours": sp["prediction_table_hours"],
            "note": "published statements almost never give the temperature; treat the table as a first guess and log two peaks",
        },

        "modifiers": {
            "use": "M = product of the factors that apply; each multiplies the rate (time is divided by M). Reference dough: lean, 2 % salt, "
                   "65-75 % hydration, white flour. Ranges are meant as 80 % intervals and enter the band as ln(high / low) / 2.56.",
            "table": [
                {"name": "salt", "applies_to": "yeast", "formula": "M = (1 - 0.021 s^2) / 0.916, s = salt % of flour",
                 "values": {"0": 1.09, "1": 1.07, "1.5": 1.04, "2": 1.00, "2.5": 0.95, "3": 0.88}, "range_pct": 5,
                 "evidence": "measured", "confidence": "medium", "sources": [SRC["carcea"], SRC["lbu14"]],
                 "data": "total gas in 3 h: 1518 / 1487 / 1272 mL and 1584 / 1519 / 1242 mL at 0 / 1.5 / 3 % salt (two flours)",
                 "valid_domain": "0-3 % salt"},
                {"name": "salt", "applies_to": "sourdough", "formula": "M = 1 + 0.09 (2 - s)",
                 "values": {"0": 1.18, "1": 1.09, "2": 1.00, "3": 0.91}, "range": {"0": [1.06, 1.21]},
                 "evidence": "creator-rule", "confidence": "medium", "sources": [SRC["wraith"], SRC["gaenzle"]],
                 "data": f"Wraith table: 2 % salt lengthens doubling by a factor {r(s['wraith_salt_ratio_2pct_over_0pct']['median'], 2)} and bulk by 1.21; "
                         "in broth the lactobacilli stopped at 4 % NaCl (w/v) while the yeast tolerated 8 %",
                 "valid_domain": "0-2.5 % salt; above 2.5 % extrapolation"},
                {"name": "sugar", "applies_to": "yeast, regular (lean-dough) strain",
                 "values": {"0": 1.00, "5": 1.00, "10": 0.80, "15": 0.70, "20": 0.60, "25": 0.50, "30": 0.40},
                 "range": {"5": [0.9, 1.1], "10": [0.7, 1.0], "20": [0.5, 0.8], "30": [0.35, 0.6]},
                 "evidence": "literature", "confidence": "medium", "sources": [SRC["lbu13"], SRC["lbu14"], SRC["lbu29"], SRC["aslankoohi"]],
                 "data": "manufacturer figure: lean-dough yeast falls from 100 to about 40 % relative activity between 0 and 30 % sugar; a general-purpose "
                         "strain peaks near 10 % (75 -> 100 -> 60 %); gas production rises up to about 5 % sugar, then falls"},
                {"name": "sugar", "applies_to": "yeast, osmotolerant strain",
                 "values": {"0": 0.57, "10": 0.78, "20": 1.00, "30": 1.25}, "range_pct": 15,
                 "evidence": "literature", "confidence": "medium", "sources": [SRC["lbu13"]],
                 "osmotic_threshold": {"switch_to_osmotolerant_above_sugar_pct": 10, "range": [8, 17],
                                       "note": "the lean-dough and sugar-tolerant lines cross near 10 % sugar, the general-purpose line near 17 %; the manufacturer advises more yeast above 10 %"},
                 "data": "same figure, values relative to a lean-dough strain in a sugar-free dough"},
                {"name": "sugar", "applies_to": "sourdough", "values": {"0": 1.0, "5": 1.0, "10": 0.8, "20": 0.5},
                 "range": {"10": [0.6, 1.0], "20": [0.3, 0.8]}, "evidence": "extrapolation", "confidence": "low",
                 "sources": [SRC["lbu13"]], "calibration_route": "log one sweet-dough bulk and store it as a recipe-specific vigour"},
                {"name": "fat (butter, oil)", "applies_to": "yeast and sourdough",
                 "values": {"0-10": 1.00, "10-30": 0.90, "above_30": 0.80}, "range": {"0-10": [0.95, 1.05], "10-30": [0.75, 1.0], "above_30": [0.6, 1.0]},
                 "evidence": "extrapolation", "confidence": "low", "sources": [SRC["lbu14"]],
                 "note": "no measured gas-rate effect of fat was found; rich doughs are slow mainly through sugar, low dough temperature and lower water; "
                         "the heat balance already handles cold butter", "calibration_route": "recipe-specific vigour"},
                {"name": "milk", "applies_to": "yeast and sourdough", "values": {"any": 1.00}, "range": {"any": [0.9, 1.05]},
                 "evidence": "extrapolation", "confidence": "low", "sources": [SRC["lbu13"]],
                 "note": "baker's yeast lives on glucose, fructose, sucrose and maltose; milk mainly changes heat capacity and total water (about 87-88 % of milk is water)"},
                {"name": "eggs", "applies_to": "yeast and sourdough", "values": {"any": 1.00}, "range": {"any": [0.9, 1.05]},
                 "evidence": "extrapolation", "confidence": "low", "sources": [],
                 "note": "count about 75 % of whole egg as water for hydration and heat balance"},
                {"name": "cinnamon and other antimicrobial spices mixed into the dough", "applies_to": "yeast and sourdough",
                 "values": {"below_0.5pct": 1.00, "1pct": 0.80, "3pct": 0.60, "in_a_filling_or_swirl": 1.00},
                 "range": {"1pct": [0.5, 1.0], "3pct": [0.3, 0.9]}, "evidence": "extrapolation", "confidence": "low", "sources": [],
                 "note": "no dose-response measurement could be read; direction only (cinnamaldehyde is antifungal)",
                 "calibration_route": "keep the spice in the filling, or log one bake"},
                {"name": "cocoa powder", "applies_to": "yeast and sourdough", "values": {"5-10pct": 0.90}, "range": {"5-10pct": [0.75, 1.0]},
                 "evidence": "extrapolation", "confidence": "low", "sources": [], "calibration_route": "log one bake"},
                {"name": "whole grain or bran share w (0-1) of the flour", "applies_to": "sourdough", "formula": "M = 1 + 0.3 w",
                 "range": "1 + 0 w to 1 + 0.6 w", "evidence": "measured", "confidence": "low", "sources": [SRC["sj_flour"], SRC["lbu14"]],
                 "data": "one pressure-vessel test of 1:1:1 feeds at 20 C: whole wheat made almost twice the CO2 of bread flour early on, peaked later (14 h vs 12 h) and ended with less",
                 "note": "bran also lowers gas retention: choose a lower rise target rather than a longer time"},
                {"name": "whole grain or bran share w (0-1) of the flour", "applies_to": "yeast", "formula": "M = 1 + 0.1 w",
                 "range": "1 + 0 w to 1 + 0.25 w", "evidence": "extrapolation", "confidence": "low", "sources": [SRC["lbu14"]]},
                {"name": "scalded flour (tangzhong, yudane, empois)", "applies_to": "yeast and sourdough", "values": {"5-20pct_of_flour_scalded": 1.00},
                 "range": {"5-20pct_of_flour_scalded": [0.95, 1.10]}, "evidence": "extrapolation", "confidence": "low", "sources": [],
                 "note": "the scald enters the heat balance as an ingredient at its own temperature; that effect is usually larger than any change in gas rate"},
                {"name": "hydration H (% of flour)", "applies_to": "sourdough dough and starter refresh", "formula": "M = 1 + 0.005 (H - 75), clipped to 0.85-1.15",
                 "range": "slope 0 to 0.015 per %", "evidence": "extrapolation", "confidence": "low", "sources": [SRC["gaenzle"], SRC["craig_sd"], SRC["wraith"]],
                 "data": f"direction: higher dough yield acidifies faster (Gaenzle 1998 discussion); Craig's 60 % pizza dough with 2.5-3 % salt runs at vigour {r(s['craig_chart']['vigour'], 2)} of the reference"},
                {"name": "hydration H (% of flour)", "applies_to": "yeast", "formula": "M = 1 + 0.004 (H - 65), clipped to 0.9-1.1",
                 "range": "slope 0 to 0.01 per %", "evidence": "literature", "confidence": "low", "sources": [SRC["lbu14"]],
                 "data": "higher absorption increases gas production and lowers gas retention (manufacturer note, no numbers)"},
                {"name": "acidity", "applies_to": "sourdough",
                 "rule": "no multiplier inside the rise window; growth stalls once iota * exp(G) passes about 1.5-2 (pH near 3.8)",
                 "evidence": "literature", "confidence": "medium", "sources": [SRC["gaenzle"]],
                 "data": "L. sanfranciscensis: pH minimum 3.9-4.2, optimum 5.4-5.5; C. milleri: no pH effect from 3.5 to 7, no growth above about 0.15 mol/L undissociated acetic acid"},
                {"name": "acidity (added levain or acids in a yeasted dough)", "applies_to": "yeast", "values": {"pH_above_4.5": 1.00, "pH_3.8_4.5": 0.90},
                 "range": {"pH_3.8_4.5": [0.7, 1.0]}, "evidence": "extrapolation", "confidence": "low", "sources": [SRC["gaenzle"], SRC["lbu13"]]},
            ],
            "hybrid_doughs": "run the yeast and the sourdough rise side by side and add the rises: R = R_yeast(t) + R_sourdough(t); never add the times",
        },

        "dough_temperature": {
            "base_temperature": {
                "equation": "T_water = base - (T_room + T_flour)",
                "base_values_C": {"slow_kneading_PVL": [68, 72], "improved_kneading_PA": [60, 64], "intensive_kneading_PI": [52, 54]},
                "conditions": "oblique-axis mixer; frasage 3-5 min at 40 rpm, then 10 min at 40 rpm (PVL), 10-13 min at 80 rpm (PA), 18-20 min at 80 rpm (PI); lower value in summer",
                "correction": "raise or lower the base by 3 to move the dough by 1 C",
                "evidence": "creator-rule", "confidence": "medium", "sources": [SRC["technomitron_base"]],
            },
            "ddt_formulas": {
                "three_factor": "T_water = 3 * DDT - T_room - T_flour - FF",
                "four_factor": "T_water = 4 * DDT - T_room - T_flour - T_preferment - FF",
                "link_to_base": "base = 3 * DDT - FF",
                "ff_update": "FF_new = FF_old + n * (T_actual - DDT), n = number of factors; the dough itself warmed by about FF / n",
                "evidence": "creator-rule", "confidence": "medium", "sources": [SRC["ka_ddt"], SRC["ka_ff"], SRC["technomitron_base"]],
            },
            "friction_factor_C": {
                "unit_note": "FF in the units of the n-factor formula (a temperature difference); divide by n for the actual warming of the dough",
                "hand_gentle_folds": {"value": 1, "range": [0, 2.2], "actual_rise_C": [0, 0.7], "evidence": "creator-rule", "sources": [SRC["ka_ff"]]},
                "hand_vigorous_8_min": {"value": 4, "range": [3.3, 4.4], "actual_rise_C": [1.1, 1.5], "evidence": "measured", "sources": [SRC["ka_ff"]]},
                "stand_mixer_planetary_3_min_stir_4_min_speed_2": {"value": 12.8, "range": [12.2, 13.3], "actual_rise_C": [4.1, 4.4], "evidence": "measured", "sources": [SRC["ka_ff"]]},
                "professional_mixers_3_min_first_4_min_second_speed": {"value": 14.4, "range": [13.3, 15.6], "actual_rise_C": [4.4, 5.2], "evidence": "creator-rule", "sources": [SRC["ka_ff"]],
                                                                       "note": "Hamelman's figure as quoted by King Arthur; use it as the default for spiral mixers until a mixing log exists"},
                "oblique_slow_PVL": {"value": 2, "range": [0, 4], "actual_rise_C": [0, 1.3], "evidence": "extrapolation", "sources": [SRC["technomitron_base"]]},
                "oblique_improved_PA": {"value": 10, "range": [8, 12], "actual_rise_C": [2.7, 4.0], "evidence": "extrapolation", "sources": [SRC["technomitron_base"]]},
                "oblique_intensive_PI": {"value": 19, "range": [18, 20], "actual_rise_C": [6.0, 6.7], "evidence": "extrapolation", "sources": [SRC["technomitron_base"]]},
                "derivation": "oblique-mixer rows: FF = 3 * 24 C - base, from the French base values",
                "calibration": "FF = n * T_actual - (sum of the other n temperatures), from one mix with a probe",
            },
            "heat_balance": {
                "equation": "T_dough = sum(m_i c_i T_i) / sum(m_i c_i) + dT_mix + k_room (T_room - T_blend)",
                "solve_for_water": "T_water = (C_total (DDT - dT_mix) - sum over other ingredients of m_i c_i T_i) / (m_water c_water)",
                "dT_mix": "actual warming by mixing = FF / n from the table above",
                "k_room": {"value": 0.1, "range": [0.0, 0.25], "evidence": "extrapolation",
                           "note": "share of the gap to room temperature closed during mixing (bowl, air); the 3-factor formula weights the room at 1/3, which overstates it for a short mix"},
                "specific_heat_J_per_g_K": {
                    "water": {"value": 4.18, "range": [4.18, 4.19], "evidence": "literature", "sources": [SRC["et_solids"]]},
                    "ice": {"value": 2.09, "range": [1.9, 2.1], "evidence": "literature", "sources": [SRC["et_solids"]], "note": "near 0 C"},
                    "wheat_flour": {"value": 1.8, "range": [1.59, 2.0], "evidence": "literature", "sources": [SRC["et_food"]],
                                    "note": "handbook 1.59; composition rule (Choi & Okos) for 14 % moisture gives about 2.0"},
                    "milk_whole": {"value": 3.85, "range": [3.77, 3.93], "evidence": "literature", "sources": [SRC["et_food"], SRC["et_solids"]]},
                    "milk_skim": {"value": 4.0, "range": [3.98, 4.02], "evidence": "literature", "sources": [SRC["et_food"]]},
                    "egg_whole": {"value": 3.18, "range": [3.1, 3.3], "evidence": "literature", "sources": [SRC["et_food"]]},
                    "butter": {"value": 2.4, "range": [2.0, 2.72], "evidence": "literature", "sources": [SRC["et_food"]],
                               "note": "handbook 2.72 above freezing; apparent value depends on how much fat melts between 10 and 30 C"},
                    "oil": {"value": 1.97, "range": [1.67, 2.0], "evidence": "literature", "sources": [SRC["et_solids"]]},
                    "sugar": {"value": 1.25, "range": [1.2, 1.6], "evidence": "extrapolation", "sources": [], "note": "handbook value recalled, not re-read"},
                    "salt": {"value": 0.88, "range": [0.85, 0.9], "evidence": "literature", "sources": [SRC["et_solids"]]},
                    "preferment_or_dough": {"formula": "(c_flour + h c_water) / (1 + h), h = hydration as a fraction",
                                            "values": {"levain_100pct": 2.99, "stiff_levain_55pct": 2.64, "dough_70pct": 2.78},
                                            "evidence": "extrapolation", "note": "mass-weighted mixture"},
                },
                "ice": {"latent_heat_J_per_g": 334,
                        "equation": "m_ice = (sum_other m_i c_i (T_i - T_target) + M_water c_water (T_liquid - T_target)) / (L_f + c_water T_liquid), "
                                    "T_target = DDT - dT_mix, ice at 0 C, all of it melting",
                        "limits": "0 <= m_ice <= M_water; report 'not reachable' otherwise", "evidence": "literature"},
            },
            "rule_of_thumb": {"statement": "1 C on the dough is worth about 10 % of the yeast dose",
                              "evidence": "creator-rule", "sources": [SRC["technomitron_base"]],
                              "engine_value": "one degree changes the rate by 12-15 % at 20-27 C (yeast); for sourdough by 15 % at 20 C, 8 % at 25 C and 4 % at 27 C"},
        },

        "cooling_warming": {
            "equation": "T(t) = T_env + (T0 - T_env) exp(-t / tau_c)",
            "measured_curve": {"description": "650 g shaped loaf (330 g flour, 78 % hydration), core probe, refrigerator set to 1.7 C, 21 readings over 10 h",
                               "tau_c_h": r(co["measured_curve"]["newton_fit_free_env"]["tau_c_h"], 2),
                               "T_env_fit_C": r(co["measured_curve"]["newton_fit_free_env"]["t_env_c"], 1),
                               "rmse_C": r(co["measured_curve"]["newton_fit_free_env"]["rmse_c"], 2),
                               "evidence": "measured", "confidence": "medium", "sources": [SRC["sj_rise"]],
                               "note": "one loaf; the fitted surroundings sit 1.6 C above the set point"},
            "tau_c_defaults_h": co["tau_c_table"],
            "scaling_rule": co["scaling_rule"],
            "range": "plus or minus 35 % on any tau_c: packing of the fridge, shelf contact, fan cycling and dough aeration are not modelled",
            "physics": {**co["physics"], "h_eff_calibrated_W_m2K": r(co["h_eff_calibrated_W_m2K"], 1),
                        "evidence": "extrapolation", "sources": [SRC["seruga"], SRC["sj_rise"]],
                        "note": "Biot number 0.4-1.9: the core lags the surface, so the first-eigenvalue time constant is used instead of the lumped one; "
                                "thermal conductivity was measured for dense unleavened dough at 39-68 C"},
            "warm_up": "same tau_c toward room temperature (range 0.7-1.0 of the cooling value; condensation adds heat)",
            "fridge_default_C": {"value": 4.0, "range": [2.0, 7.0]},
            "fermentation_during_transient": co["fermentation_during_transient"],
            "reading": "over a 12 h retard of a 26 C loaf, about 80 % of the fermentation happens while it cools; a model that drops the dough to 4 C at once under-predicts the retard by a factor 5",
        },

        "personal_calibration": {
            "parameter": "v multiplies the rate (mu_ref for sourdough and starters, 1 / tau for yeast); one v per culture, optionally one per recipe family",
            "prior": {"distribution": "ln v ~ Normal(0, s0^2)",
                      "s0_sourdough": {"value": 0.35, "from": f"sd of ln v across {s['prior_width']['sources']} sources = {s['prior_width']['sd_ln_v_between_sources']} (v from {s['prior_width']['min_v']} to {s['prior_width']['max_v']})"},
                      "s0_yeast": {"value": 0.45, "from": "sd of ln(observed / engine) inside groups of independent tables and recipes, 0.35-0.56"},
                      "evidence": "measured"},
            "observation": "y = ln(engine hours at v = 1 over the logged temperature history / observed hours to the logged endpoint) = ln v + e",
            "noise": {"se": {"value": 0.20, "range": [0.10, 0.30], "note": "tabulated sources scatter by 0.10; home logs add endpoint and thermometer error"}},
            "update": "precision = 1 / s0^2 + n / se^2; mean = (sum(y) / se^2) / precision; v = exp(mean)",
            "posterior_sd_after_n_bakes": ca["posterior_sd_after_n_bakes"],
            "replay": ca["replays"],
            "anchor": "store the mean dough temperature of the logged bakes (T_anchor); the band widens with distance from it",
            "evidence": "extrapolation",
        },

        "uncertainty": {
            "band": "80 % interval = hours * exp(-/+ 1.2816 * sd)",
            "sd": "sd^2 = sd_v^2 + se^2 + (s_T * min(|T - T_anchor|, 10))^2 + sum(modifier terms^2) + sum(domain penalties^2)",
            "terms": {
                "sd_v": "posterior sd of ln v (prior: 0.35 sourdough, 0.45 yeast)",
                "se": 0.20,
                "s_T_per_C": {"value": 0.03, "T_anchor_default": 25.0,
                              "from": "half the gap between the engine's temperature slope and the flatter slopes of independent data "
                                      "(sourdough 0.122 vs 0.071 per C at 19-23 C; yeast 0.137 vs 0.074 per C at 18-30 C)"},
                "modifier": "ln(range high / range low) / 2.56 for each modifier that differs from 1",
                "domain_penalties": {
                    "yeast_cold_below_8C": 0.50, "sourdough_cold_below_10C": 0.40,
                    "sourdough_above_30C": 0.30, "yeast_above_35C": 0.40,
                    "dose_outside_validated_range": 0.25, "prediction_under_45_min": 0.30,
                    "starter_state_unknown": 0.20, "enriched_dough_sugar_above_10_or_fat_above_20": 0.25},
                "domain_penalty_relief": "halve a penalty once at least two logged bakes lie in that same domain",
            },
            "band_factor_examples": {"sourdough_uncalibrated_at_25C": ca["band_factor_no_calibration"],
                                     "sourdough_after_3_bakes_at_anchor": ca["replays"][0]["logged_bakes"]["3"]["band_factor_at_anchor"],
                                     "yeast_uncalibrated_at_25C": 1.88, "yeast_cold_uncalibrated_4C": 2.66, "sourdough_retard_uncalibrated_4C": 2.28,
                                     "yeast_cold_after_3_cold_bakes": 1.55, "sourdough_retard_after_3_retards": 1.47, "starter_peak_after_2_peaks": 1.36},
            "extrapolation_flag": "set whenever a domain penalty applies, a modifier with confidence 'low' differs from 1, or P is integrated mostly outside 10-30 C",
            "evidence": "extrapolation",
        },

        "validation": [
            {"dataset": "TXCraig1 yeast chart, cells >= 4 h (fitted)", "n": y["n_fitted"], "residual": "rmse ln 0.052 (cold 0.044, mid 0.052, warm 0.059)",
             "file": "fits/yeast-chart-residuals.csv", "evidence": "creator-rule"},
            {"dataset": "cf-calculator regression on the same cells", "n": y["n_fitted"],
             "residual": f"rmse ln {r(y['models']['M_cf_calculator']['rmse_ln']['all'])} (cold {r(y['models']['M_cf_calculator']['rmse_ln']['cold_below_8C'])}, mid {r(y['models']['M_cf_calculator']['rmse_ln']['mid_8_to_28C'])}, warm {r(y['models']['M_cf_calculator']['rmse_ln']['warm_28_to_35C'])})",
             "file": "fits/yeast-fit.json", "evidence": "creator-rule"},
            *[{"dataset": "yeast, independent: " + g, "n": v["n"],
               "residual": f"median ln(obs / engine doubled) {v['median_ln_obs_over_engine']} (factor {v['time_factor']}), sd {v['sd_ln']}",
               "file": "fits/yeast-validation-residuals.csv", "evidence": "creator-rule"}
              for g, v in y["validation_isothermal"].items() if v["n"] >= 3],
            {"dataset": "yeast, published cold schedules (warm start, Newton cooling, tau_c 1.5 h)", "n": y["validation_cold_profiles"]["n"],
             "residual": f"median progress P {y['validation_cold_profiles']['median_progress_P']}, quartiles {y['validation_cold_profiles']['quartiles_P']}, sd ln P {y['validation_cold_profiles']['sd_ln_P']}",
             "file": "fits/yeast-validation-residuals.csv", "evidence": "creator-rule"},
            {"dataset": "Sourdough Journey timetables (fitted, 1 parameter)", "n": sj["n"], "residual": f"rmse ln {r(sj['engine_rmse_ln'])}; mean by temperature {sj['engine_mean_residual_by_temp_c']}",
             "file": "fits/sourdough-residuals.csv", "evidence": "measured"},
            *[{"dataset": "sourdough, not fitted: " + g, "n": v["n"],
               "residual": f"mean ln(obs / engine) {v['mean_ln_obs_over_engine']} (vigour {v['vigour_v']}), sd within {v['sd_within_ln']}",
               "file": "fits/sourdough-residuals.csv", "evidence": "creator-rule"} for g, v in s["vigour_by_source"].items()],
            {"dataset": "Craig sourdough chart, 4-30 C (effective rise and vigour fitted)", "n": s["craig_chart"]["n_fitted"],
             "residual": f"rmse ln {r(s['craig_chart']['rmse_ln'])}; effective rise {r(s['craig_chart']['effective_rise_of_ready_to_bake'], 2)}, vigour {r(s['craig_chart']['vigour'], 2)}",
             "file": "fits/sourdough-residuals.csv", "evidence": "creator-rule"},
            {"dataset": "Sourdough Framework Table 7.1, recently fed rows at +50 % (window 8-12 h)", "n": 3,
             "residual": f"engine hours {s['framework_table_7_1']['recently_fed_plus50_hydration100_h']} with a 100 % starter; geometric mean {s['framework_table_7_1']['geometric_mean_h']} h",
             "file": "fits/sourdough-fit.json", "evidence": "creator-rule"},
            {"dataset": "24 h bulk in a cold proofer at 12 / 15 / 18 C (20 % starter)", "n": 3,
             "residual": "engine rise " + ", ".join(f"{v['set_temp_c']:.0f} C: +{v['engine_rise_pct']} %" for v in s["cold_proofer_24h_check"]["runs"].values())
                         + "; observed: 18 C slightly overproofed, 15 C and 12 C good loaves",
             "file": "fits/sourdough-fit.json", "evidence": "measured"},
            {"dataset": "The Sourdough Framework flatbread and pancake recipes in research/data/recipes-breadcode.json (20 % starter, +50 % or bubbles; 4-24 h or 6-12 h 'depending on temperature')", "n": 3,
             "residual": "engine at +50 % with a 100 % starter: "
                         + ", ".join(f"{t} C {float(fm.tau_sourdough(t, fm.inoculation(20), 0.5)):.1f} h" for t in (15, 20, 25, 30))
                         + "; the recipes give no temperature, so this is a range check only",
             "file": "fits/sourdough-fit.json", "evidence": "creator-rule"},
            {"dataset": "starter time-to-peak statements", "n": sp["n_statements_used"],
             "residual": f"median ln(obs / engine) {sp['median_ln_obs_over_engine']}, sd {sp['sd_ln']} (temperature assumed 23 C in three of four)",
             "file": "fits/starter-peak-residuals.csv", "evidence": "creator-rule"},
            *private_rows,
            {"dataset": "fridge cooling curve, 650 g loaf", "n": co["measured_curve"]["n"],
             "residual": f"Newton rmse {r(co['measured_curve']['newton_fit_free_env']['rmse_c'], 2)} C", "file": "fits/cooling-residuals.csv", "evidence": "measured"},
            {"dataset": "calibration replay", "n": sum(x["rows"] for x in ca["replays"]),
             "residual": "median error after 3 logged bakes: " + ", ".join(f"{x['logged_bakes']['3']['median_abs_error_pct']} %" for x in ca["replays"]),
             "file": "fits/calibration-demo.csv", "evidence": "extrapolation"},
        ],

        "failure_regions": [
            {"region": "sourdough below 10 C and any retard", "why": "the temperature curve is a broth growth model that its authors call poor near Tmin; the alternative model puts the yeast's Tmin at 8 C. One practitioner estimate (24 h at 3 C is worth at most 30 min of warm bulk) agrees with the engine (32 min)",
             "behaviour": "predict, flag, widen (penalty 0.40); express fridge time as equivalent hours at 25 C"},
            {"region": "sourdough above 30 C", "why": "yeast and lactobacilli part ways (optima 27 and 32-33 C): acidity outruns gas; proteolysis can ruin the dough before the rise target", "behaviour": "predict, flag, widen; advise a lower rise target"},
            {"region": "sourdough past the endpoint (iota exp(G) above about 1.5)", "why": "acid stall and gluten breakdown; the exponential rise law no longer holds", "behaviour": "report 'over-fermented by x h equivalent', do not report a rise"},
            {"region": "inoculation above 20 % or below 0.5 %", "why": "Wraith adds a gluten penalty above 15 %; below 0.5 % no data", "behaviour": "flag, penalty 0.25"},
            {"region": "starter not at peak, or of unknown age", "why": "lag from one test only", "behaviour": "use the lag table, penalty 0.20"},
            {"region": "yeast direct doughs at bakery doses (0.4 % IDY and more) judged at +50 %", "why": "recipe schedules run 1.2-1.6 times the engine; a private log (aggregate only, n = 2, temperatures not probe-measured) ran slower still", "behaviour": "report the upper half of the band as the likely side; calibrate on the first bake"},
            {"region": "rich dough (sugar above 10 %, fat above 30 %) or a preferment dosed far above the validated range, in the fridge", "why": "private observations (aggregate only) showed far less visible rise than predicted; cold fat stiffens the dough and the modifiers were not measured cold", "behaviour": "flag; do not promise a visible rise in the fridge"},
            {"region": "yeast doses above 1 % IDY or predictions under 45 min", "why": "chart floored at 2 h; adaptation lag not in the power law", "behaviour": "add the 0.2 h lag, flag"},
            {"region": "yeast between 18 and 30 C far from the calibration temperature", "why": "the chart's temperature slope (Q10 3.4-3.9) is steeper than rules of thumb and compiled tables (Q10 about 2.1)", "behaviour": "s_T term; calibrate at the temperature actually used"},
            {"region": "yeast in the fridge", "why": "chart rows were tuned to schedules that include cooling and warm-up; published cold schedules scatter by a factor 2.2", "behaviour": "integrate the cooling curve, penalty 0.50"},
            {"region": "sweet and rich doughs (sugar above 10 %, fat above 20 %), spices, cocoa", "why": "modifiers are manufacturer curves or reasoned ranges, not dough-time measurements", "behaviour": "flag, widen, offer a recipe-specific calibration"},
            {"region": "rye-dominant, gluten-free and very wet batters", "why": "rise targets and gas retention differ; no data fitted", "behaviour": "rate only (G), no rise prediction"},
            {"region": "hybrid doughs", "why": "no dataset; rises are added by assumption", "behaviour": "flag"},
            {"region": "dough over 2 kg, or containers not in the table", "why": "cooling constants are scaled from one 650 g loaf", "behaviour": "ask for a probe reading"},
        ],
    }
    OUT.write_text(json.dumps(model, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("wrote", OUT, OUT.stat().st_size, "bytes")


if __name__ == "__main__":
    main()
