use breadator_engine::formula::{
    Basis, Flour, Formula, Ingredient, Inoculation, Seed, Stage, StageExtra, StageKind, Target,
    levain_flour_pct, solve,
};
use breadator_engine::ingredients::IngredientKind;
use breadator_engine::scald::{ScaldMethod, scald_stage};
use breadator_engine::temperature::{
    Component, WaterTempInput, flour_proximate, specific_heat, water_temperature,
};
use breadator_engine::yeast::convert;
use breadator_engine::{EngineError, Warning};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-6, "{a} != {b}");
}

fn flour(id: &str, pct: f64) -> Flour {
    Flour { id: id.into(), pct, kind: None, protein_pct: None }
}

fn ing(id: &str, kind: IngredientKind, pct: f64) -> Ingredient {
    Ingredient { id: id.into(), kind, pct, composition: None }
}

fn grams(lines: &[breadator_engine::formula::Line], id: &str) -> f64 {
    lines.iter().find(|l| l.id == id).map(|l| l.grams).unwrap_or(0.0)
}

fn lean(target: Target) -> Formula {
    Formula {
        flours: vec![flour("white", 80.0), flour("whole", 20.0)],
        ingredients: vec![
            ing("water", IngredientKind::Water, 70.0),
            ing("salt", IngredientKind::Salt, 2.0),
            ing("yeast", IngredientKind::InstantYeast, 0.5),
        ],
        stages: vec![],
        target,
        allowance_pct: 0.0,
    }
}

#[test]
fn dough_target_scales_the_total_formula() {
    let s = solve(&lean(Target::Dough { grams: 1000.0 })).unwrap();
    let f = 1000.0 / 1.725;
    close(s.metrics.total_flour_g, f);
    close(grams(&s.totals, "white"), 0.8 * f);
    close(grams(&s.totals, "water"), 0.7 * f);
    close(s.metrics.dough_g, 1000.0);
    close(s.metrics.hydration_water_pct, 70.0);
    close(s.metrics.salt_pct, 2.0);
    let mixed: f64 = s.final_mix.iter().map(|l| l.grams).sum();
    close(mixed, 1000.0);
}

#[test]
fn baked_and_pieces_targets_account_for_loss_and_allowance() {
    let mut f = lean(Target::Baked { count: 2, grams: 430.0, loss_pct: 14.0 });
    f.allowance_pct = 2.0;
    let s = solve(&f).unwrap();
    close(s.metrics.dough_g, 2.0 * 430.0 / 0.86 * 1.02);
    let s = solve(&lean(Target::Pieces { count: 8, grams: 90.0 })).unwrap();
    close(s.metrics.dough_g, 720.0);
}

#[test]
fn levain_comes_out_of_the_totals_and_reports_both_inoculations() {
    let mut f = lean(Target::Flour { grams: 1000.0 });
    f.ingredients[0].pct = 75.0;
    f.ingredients.truncate(2);
    let p = levain_flour_pct(50.0, 100.0, Inoculation::FinalMixFlour).unwrap();
    close(p, 20.0);
    f.stages.push(Stage {
        id: "levain".into(),
        kind: StageKind::Levain,
        flour_pct: p,
        flour: Some("white".into()),
        liquid_pct: 100.0,
        liquid: "water".into(),
        extras: vec![],
        evaporation_pct: None,
        seed: Some(Seed { pct: 20.0, hydration_pct: 100.0 }),
    });
    let s = solve(&f).unwrap();
    let lv = &s.stages[0];
    close(lv.mass_g, 400.0);
    close(grams(&s.final_mix, "white"), 600.0);
    close(grams(&s.final_mix, "whole"), 200.0);
    close(grams(&s.final_mix, "water"), 550.0);
    close(grams(&s.final_mix, "levain"), 400.0);
    let m = &s.metrics.levains[0];
    close(m.inoculation_final_mix_pct.unwrap(), 50.0);
    close(m.inoculation_total_flour_pct, 40.0);
    close(s.metrics.prefermented_flour_pct, 20.0);
    // 1:5:5 build: seed 1 part, fresh flour 5, fresh water 5.
    let seed = lv.seed.as_ref().unwrap();
    close(seed.fresh_flour_g, 200.0 / 1.1);
    close(seed.seed_g, 0.2 * 200.0 / 1.1);
    close(seed.fresh_water_g, 200.0 - seed.seed_g / 2.0);
    close(seed.fresh_flour_g * 5.0 / 5.0, seed.fresh_water_g);
}

#[test]
fn poolish_yeast_is_carved_from_the_total_yeast() {
    let mut f = lean(Target::Flour { grams: 500.0 });
    f.stages.push(Stage {
        id: "poolish".into(),
        kind: StageKind::Poolish,
        flour_pct: 40.0,
        flour: None,
        liquid_pct: 100.0,
        liquid: "water".into(),
        extras: vec![StageExtra { ingredient: "yeast".into(), pct: 0.1, basis: Basis::StageFlour }],
        evaporation_pct: None,
        seed: None,
    });
    let s = solve(&f).unwrap();
    let p = &s.stages[0];
    close(p.flour_g, 200.0);
    close(grams(&p.lines, "white"), 160.0);
    close(grams(&p.lines, "whole"), 40.0);
    close(grams(&p.lines, "yeast"), 0.2);
    close(grams(&s.final_mix, "yeast"), 2.5 - 0.2);
    let mixed: f64 = s.final_mix.iter().map(|l| l.grams).sum();
    close(mixed, s.metrics.dough_g);
}

#[test]
fn overallocated_stage_is_an_error_not_negative_grams() {
    let mut f = lean(Target::Flour { grams: 500.0 });
    f.stages.push(Stage {
        id: "poolish".into(),
        kind: StageKind::Poolish,
        flour_pct: 80.0,
        flour: None,
        liquid_pct: 100.0, // needs 400 g water, the formula has 350 g
        liquid: "water".into(),
        extras: vec![],
        evaporation_pct: None,
        seed: None,
    });
    match solve(&f) {
        Err(EngineError::Overallocated { ingredient, .. }) => assert_eq!(ingredient, "water"),
        other => panic!("expected Overallocated, got {other:?}"),
    }
}

#[test]
fn milk_scald_counts_milk_water_and_compensates_evaporation() {
    let mut f = Formula {
        flours: vec![flour("bread", 100.0)],
        ingredients: vec![
            ing("milk", IngredientKind::Milk, 60.0),
            ing("water", IngredientKind::Water, 10.0),
            ing("salt", IngredientKind::Salt, 1.8),
        ],
        stages: vec![],
        target: Target::Flour { grams: 400.0 },
        allowance_pct: 0.0,
    };
    f.stages.push(
        scald_stage("tangzhong", ScaldMethod::Tangzhong, 5.0, None, "milk", Some(10.0)).unwrap(),
    );
    let s = solve(&f).unwrap();
    let t = &s.stages[0];
    close(t.flour_g, 20.0);
    close(grams(&t.lines, "milk"), 100.0);
    close(t.water_g, 88.0);
    close(t.evaporation_water_g, 8.8);
    close(s.metrics.hydration_total_pct, (240.0 * 0.88 + 40.0) / 4.0);
    close(s.metrics.scalded_flour_pct, 5.0);
    assert!(s.warnings.is_empty());
    f.stages[0].evaporation_pct = None;
    let s = solve(&f).unwrap();
    assert_eq!(s.warnings, vec![Warning::ScaldEvaporationUnknown { stage: "tangzhong".into() }]);
}

#[test]
fn invalid_inputs_are_reported() {
    let mut f = lean(Target::Dough { grams: 1000.0 });
    f.flours[0].pct = 70.0;
    assert!(matches!(solve(&f), Err(EngineError::FlourSum { .. })));
    let mut f = lean(Target::Dough { grams: 1000.0 });
    f.ingredients[1].id = "water".into();
    assert!(matches!(solve(&f), Err(EngineError::DuplicateId { .. })));
    let f = lean(Target::Dough { grams: f64::NAN });
    assert!(matches!(solve(&f), Err(EngineError::OutOfRange { .. })));
}

#[test]
fn classic_method_matches_the_base_temperature_rule() {
    let r = water_temperature(&WaterTempInput::Classic {
        desired_dough_c: 24.0,
        flour_c: 20.0,
        room_c: 22.0,
        preferment_c: Some(26.0),
        friction_factor_c: 6.0,
    })
    .unwrap();
    close(r.water_c, 4.0 * 24.0 - 20.0 - 22.0 - 26.0 - 6.0);
}

#[test]
fn heat_balance_conserves_energy_with_and_without_ice() {
    let cp_flour = specific_heat(&flour_proximate(11.0, false), 20.0);
    assert!((1.8..2.1).contains(&cp_flour), "flour cp {cp_flour}");
    let components = vec![Component {
        id: "flour".into(),
        grams: 1000.0,
        temp_c: 20.0,
        specific_heat: cp_flour,
    }];
    // Everything already at the target: water at the target too.
    let r = water_temperature(&WaterTempInput::HeatBalance {
        desired_dough_c: 20.0,
        water_g: 700.0,
        components: components.clone(),
        mixing_rise_c: 0.0,
        tap_c: None,
    })
    .unwrap();
    close(r.water_c, 20.0);
    // Warm flour and a big mixing rise: ice from 12 °C tap water.
    let hot = vec![Component {
        id: "flour".into(),
        grams: 1000.0,
        temp_c: 30.0,
        specific_heat: cp_flour,
    }];
    let r = water_temperature(&WaterTempInput::HeatBalance {
        desired_dough_c: 24.0,
        water_g: 650.0,
        components: hot.clone(),
        mixing_rise_c: 8.0,
        tap_c: Some(12.0),
    })
    .unwrap();
    let ice = r.ice_g.unwrap();
    let tap = r.tap_water_g.unwrap();
    close(ice + tap, 650.0);
    let t_pre = 16.0;
    let balance = 1000.0 * cp_flour * (30.0 - t_pre) + tap * 4.18 * (12.0 - t_pre)
        - ice * (334.0 + 4.18 * t_pre);
    assert!(balance.abs() < 1e-6, "energy imbalance {balance}");
    // Impossible target: all water as ice and still too warm.
    let r = water_temperature(&WaterTempInput::HeatBalance {
        desired_dough_c: 5.0,
        water_g: 100.0,
        components: hot,
        mixing_rise_c: 10.0,
        tap_c: Some(12.0),
    })
    .unwrap();
    assert!(matches!(r.warnings[0], Warning::TargetUnreachable { .. }));
    assert!(r.expected_dough_c > 5.0);
}

#[test]
fn yeast_conversions_go_through_fresh_equivalence() {
    let c = convert(10.0, IngredientKind::FreshYeast, IngredientKind::InstantYeast).unwrap();
    close(c.grams, 3.3);
    assert!(c.low_g <= c.grams && c.grams <= c.high_g);
    let back = convert(c.grams, IngredientKind::InstantYeast, IngredientKind::FreshYeast).unwrap();
    close(back.grams, 10.0);
    assert!(convert(1.0, IngredientKind::Salt, IngredientKind::FreshYeast).is_err());
}

#[test]
fn solution_serializes_to_json() {
    let s = solve(&lean(Target::Dough { grams: 900.0 })).unwrap();
    let v = serde_json::to_value(&s).unwrap();
    assert!(v["metrics"]["hydration_water_pct"].is_number());
    let f: Formula = serde_json::from_value(serde_json::json!({
        "flours": [{"id": "t65", "pct": 100}],
        "ingredients": [{"id": "water", "kind": "water", "pct": 68}],
        "target": {"by": "pieces", "count": 2, "grams": 500}
    }))
    .unwrap();
    close(solve(&f).unwrap().metrics.dough_g, 1000.0);
}

/// A formula can come from anyone through the public API: one with tens of thousands of entries
/// is refused at once instead of being solved (it used to take seconds of CPU).
#[test]
fn an_oversized_formula_is_refused_before_any_work() {
    let mut f = lean(Target::Flour { grams: 1000.0 });
    f.flours = (0..90_000).map(|i| flour(&format!("{i:x}"), 0.0)).collect();
    let started = std::time::Instant::now();
    let err = solve(&f).unwrap_err();
    assert!(started.elapsed() < std::time::Duration::from_millis(100), "{:?}", started.elapsed());
    assert!(matches!(&err, EngineError::Invalid { field, .. } if field == "flours"), "{err:?}");

    let mut f = lean(Target::Flour { grams: 1000.0 });
    f.ingredients = (0..49).map(|i| ing(&format!("i{i}"), IngredientKind::Other, 1.0)).collect();
    assert!(matches!(solve(&f), Err(EngineError::Invalid { field, .. }) if field == "ingredients"));

    let mut f = lean(Target::Flour { grams: 1000.0 });
    f.flours[0].id = "x".repeat(65);
    assert!(matches!(solve(&f), Err(EngineError::Invalid { field, .. }) if field == "id"));
}
