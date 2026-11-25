//! Plan a lean sourdough loaf from a preferment whose batch has already been weighed.

use crate::ferment::{Band, Vigour};
use crate::leaven::{
    CHECKED_STARTER_FEED_MAX, DoughFactors, Endpoint, FermentNote, QuickFerment, QuickStarter,
    Solve, StarterState, iota_from_starter, sourdough::IOTA_DOMAIN,
};
use crate::{EPS, EngineError, check_range};

pub const MAX_OBSERVATIONS: usize = 4;
const PRIOR_SIGMA: f64 = 0.35;
const OBS_SIGMA: f64 = 0.20;

#[derive(Debug, Clone, Copy)]
pub enum Observation {
    Peak {
        observed_h: f64,
        temp_c: f64,
        seed_hydration_pct: f64,
        flour_parts: f64,
        water_parts: f64,
    },
    Bulk {
        observed_h: f64,
        temp_c: f64,
        starter_hydration_pct: f64,
        dose_pct: f64,
        hydration_pct: f64,
        salt_pct: f64,
        whole_grain_pct: f64,
        endpoint: Endpoint,
    },
}

#[derive(Debug)]
pub struct QuickPreferment {
    pub seed_g: f64,
    pub seed_hydration_pct: f64,
    pub seed_state: StarterState,
    pub feed_flour_g: f64,
    pub feed_water_g: f64,
    pub feed_c: f64,
    pub age_h: f64,
    pub baked_g: f64,
    pub loss_pct: f64,
    pub hydration_pct: f64,
    pub salt_pct: f64,
    pub whole_grain_pct: f64,
    pub bulk_c: f64,
    pub bulk_h: f64,
    pub endpoint: Endpoint,
    pub extra_hydration_pct: f64,
    pub manual_vigour: Option<f64>,
    pub observations: Vec<Observation>,
}

impl Default for QuickPreferment {
    fn default() -> Self {
        Self {
            seed_g: 10.0,
            seed_hydration_pct: 100.0,
            seed_state: StarterState::Ripe,
            feed_flour_g: 50.0,
            feed_water_g: 50.0,
            feed_c: 21.0,
            age_h: 8.0,
            baked_g: 800.0,
            loss_pct: 14.0,
            hydration_pct: 75.0,
            salt_pct: 2.0,
            whole_grain_pct: 0.0,
            bulk_c: 21.0,
            bulk_h: 8.0,
            endpoint: Endpoint::Rise50,
            extra_hydration_pct: 100.0,
            manual_vigour: None,
            observations: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Mix {
    pub dough_g: f64,
    pub baked_g: f64,
    pub total_flour_g: f64,
    pub preferment_g: f64,
    pub additional_levain_g: f64,
    pub flour_g: f64,
    pub water_g: f64,
    pub salt_g: f64,
}

#[derive(Debug)]
pub enum TargetPlan {
    Mix { mix: Mix, hours: Band, extrapolated: bool },
    NoDose { too_slow: bool },
    WaterConflict,
}

#[derive(Debug)]
pub struct Alternatives {
    pub same_weight: Option<Mix>,
    pub same_weight_pct: f64,
    pub same_weight_hours: Option<Band>,
    pub same_time: Option<Mix>,
}

#[derive(Debug)]
pub struct PrefermentPlan {
    pub available_g: f64,
    pub preferment_hydration_pct: f64,
    pub peak: Band,
    pub peak_extrapolated: bool,
    pub vigour: Option<Vigour>,
    pub target: TargetPlan,
    pub alternatives: Option<Alternatives>,
}

fn invalid(field: &str, reason: &str) -> EngineError {
    EngineError::Invalid { field: field.into(), reason: reason.into() }
}

fn checked_iota(iota: f64) -> bool {
    (IOTA_DOMAIN.0..=IOTA_DOMAIN.1).contains(&iota)
}

impl Observation {
    fn reference(&self) -> Result<(f64, f64, f64), EngineError> {
        let (observed, temp, expected) = match *self {
            Self::Peak { observed_h, temp_c, seed_hydration_pct, flour_parts, water_parts } => {
                check_range("observation-temp", temp_c, 15.0, 30.0)?;
                check_range("observation-feed", flour_parts, 0.2, CHECKED_STARTER_FEED_MAX)?;
                let peak = QuickStarter {
                    flour_parts,
                    water_parts,
                    seed_hydration_pct,
                    temp_c,
                    ..QuickStarter::default()
                }
                .solve()?;
                (observed_h, temp_c, peak.hours.mid_h)
            }
            Self::Bulk {
                observed_h,
                temp_c,
                starter_hydration_pct,
                dose_pct,
                hydration_pct,
                salt_pct,
                whole_grain_pct,
                endpoint,
            } => {
                check_range("observation-temp", temp_c, 15.0, 30.0)?;
                let iota = iota_from_starter(dose_pct, starter_hydration_pct);
                check_range(
                    "observation-pff",
                    iota * 100.0,
                    IOTA_DOMAIN.0 * 100.0,
                    IOTA_DOMAIN.1 * 100.0,
                )?;
                let bulk = QuickFerment {
                    dose_pct,
                    starter_hydration_pct,
                    endpoint,
                    dough_c: temp_c,
                    room_c: temp_c,
                    dough: DoughFactors {
                        hydration_pct,
                        salt_pct,
                        whole_grain_pct,
                        ..DoughFactors::default()
                    },
                    ..QuickFerment::default()
                }
                .solve()?;
                (observed_h, temp_c, bulk.hours.mid_h)
            }
        };
        check_range("observation-hours", observed, 0.25, 96.0)?;
        Ok((observed, expected, temp))
    }
}

struct Dough {
    hydration: f64,
    salt: f64,
    baked_fraction: f64,
    preferment_hydration: f64,
    extra_hydration: f64,
}

impl Dough {
    fn mass(&self, flour: f64) -> f64 {
        flour * (1.0 + self.hydration + self.salt)
    }

    fn mix(&self, flour: f64, preferment: f64, extra: f64) -> Option<Mix> {
        let added_flour = flour - preferment - extra;
        let added_water = flour * self.hydration
            - preferment * self.preferment_hydration
            - extra * self.extra_hydration;
        let tolerance = EPS * flour;
        if added_flour < -tolerance || added_water < -tolerance {
            return None;
        }
        let dough_g = self.mass(flour);
        Some(Mix {
            dough_g,
            baked_g: dough_g * self.baked_fraction,
            total_flour_g: flour,
            preferment_g: preferment * (1.0 + self.preferment_hydration),
            additional_levain_g: extra * (1.0 + self.extra_hydration),
            flour_g: added_flour.max(0.0),
            water_g: added_water.max(0.0),
            salt_g: flour * self.salt,
        })
    }
}

impl QuickPreferment {
    fn calibration(&self) -> Result<(Option<Vigour>, Option<f64>), EngineError> {
        if self.observations.len() > MAX_OBSERVATIONS {
            return Err(invalid("observations", "too many observations"));
        }
        let mut vigour = Vigour::prior(PRIOR_SIGMA);
        if let Some(factor) = self.manual_vigour {
            check_range("manual_vigour", factor, 0.1, 10.0)?;
            if !self.observations.is_empty() {
                return Err(invalid(
                    "observations",
                    "use either a factor or observations, not both",
                ));
            }
            vigour.log_mean = factor.ln();
            return Ok((Some(vigour), None));
        }
        if self.observations.is_empty() {
            return Ok((None, None));
        }
        let mut temperature_sum = 0.0;
        for observation in &self.observations {
            let (observed, expected, temperature) = observation.reference()?;
            // With a ripe seed/starter and a constant temperature, progress scales with vigour.
            vigour = vigour.update(observed / expected * vigour.factor(), OBS_SIGMA)?;
            temperature_sum += temperature;
        }
        Ok((Some(vigour), Some(temperature_sum / f64::from(vigour.n))))
    }

    pub fn solve(&self) -> Result<PrefermentPlan, EngineError> {
        for (field, value, min, max) in [
            ("seed_g", self.seed_g, 0.01, 20_000.0),
            ("seed_hydration_pct", self.seed_hydration_pct, 30.0, 300.0),
            ("feed_flour_g", self.feed_flour_g, 0.01, 100_000.0),
            ("feed_water_g", self.feed_water_g, 0.0, 100_000.0),
            ("age_h", self.age_h, 0.0, 168.0),
            ("baked_g", self.baked_g, 50.0, 20_000.0),
            ("loss_pct", self.loss_pct, 0.0, 45.0),
            ("hydration_pct", self.hydration_pct, 1.0, 200.0),
            ("extra_hydration_pct", self.extra_hydration_pct, 30.0, 300.0),
            ("bulk_c", self.bulk_c, 10.0, 30.0),
        ] {
            check_range(field, value, min, max)?;
        }
        let (vigour, anchor_c) = self.calibration()?;
        let peak = QuickStarter {
            flour_parts: self.feed_flour_g / self.seed_g,
            water_parts: self.feed_water_g / self.seed_g,
            seed_hydration_pct: self.seed_hydration_pct,
            temp_c: self.feed_c,
            seed_state: self.seed_state,
            vigour,
            ..QuickStarter::default()
        }
        .solve()?;
        let preferment_flour =
            self.feed_flour_g + self.seed_g / (1.0 + self.seed_hydration_pct / 100.0);
        let available_g = self.seed_g + self.feed_flour_g + self.feed_water_g;
        let dough = Dough {
            hydration: self.hydration_pct / 100.0,
            salt: self.salt_pct / 100.0,
            baked_fraction: 1.0 - self.loss_pct / 100.0,
            preferment_hydration: peak.hydration_pct / 100.0,
            extra_hydration: self.extra_hydration_pct / 100.0,
        };
        let dough_g = self.baked_g / dough.baked_fraction;
        let flour = dough_g / dough.mass(1.0);
        let bulk = QuickFerment {
            solve: Solve::Dose,
            target_h: self.bulk_h,
            endpoint: self.endpoint,
            dough_c: self.bulk_c,
            room_c: self.bulk_c,
            dough_g,
            starter_hydration_pct: peak.hydration_pct,
            dough: DoughFactors {
                hydration_pct: self.hydration_pct,
                salt_pct: self.salt_pct,
                whole_grain_pct: self.whole_grain_pct,
                ..DoughFactors::default()
            },
            vigour,
            anchor_c,
            ..QuickFerment::default()
        };
        let dose = bulk.solve()?;
        let (target, target_iota) = if let Some(&FermentNote::NoDoseFits { too_slow }) =
            dose.notes.iter().find(|note| matches!(note, FermentNote::NoDoseFits { .. }))
        {
            (TargetPlan::NoDose { too_slow }, None)
        } else {
            let iota = dose.iota.expect("sourdough predicts prefermented flour");
            let required = iota * flour;
            let mut used = required.min(preferment_flour);
            // Keep as much preferment as possible, with the rest of the required flour supplied
            // by the extra starter. More of the wetter starter uses more of the water budget.
            let difference = dough.preferment_hydration - dough.extra_hydration;
            if difference > 0.0 {
                let bound =
                    (dough.hydration * flour - dough.extra_hydration * required) / difference;
                used = used.min(bound.max(0.0));
            }
            let extrapolated = dose.notes.contains(&FermentNote::Extrapolation);
            let target = match dough.mix(flour, used, required - used) {
                Some(mix) => TargetPlan::Mix { mix, hours: dose.hours, extrapolated },
                None => TargetPlan::WaterConflict,
            };
            (target, Some(iota))
        };

        let needs_alternatives = match &target {
            TargetPlan::Mix { mix, .. } => {
                let tolerance = EPS * flour;
                available_g - mix.preferment_g > tolerance || mix.additional_levain_g > tolerance
            }
            _ => true,
        };
        let alternatives = if needs_alternatives {
            let iota = preferment_flour / flour;
            let same_weight = dough.mix(flour, preferment_flour, 0.0);
            let same_weight_hours = if same_weight.is_some() && checked_iota(iota) {
                let time = QuickFerment {
                    solve: Solve::Time,
                    dose_pct: available_g / (flour - preferment_flour) * 100.0,
                    ..bulk
                }
                .solve();
                match time {
                    Ok(result) => Some(result.hours),
                    // The optional alternative can run past the engine's five-day horizon.
                    Err(EngineError::Invalid { field, .. }) if field == "schedule" => None,
                    Err(error) => return Err(error),
                }
            } else {
                None
            };
            let same_time = target_iota
                .filter(|iota| checked_iota(*iota))
                .and_then(|iota| dough.mix(preferment_flour / iota, preferment_flour, 0.0));
            Some(Alternatives {
                same_weight,
                same_weight_pct: iota * 100.0,
                same_weight_hours,
                same_time,
            })
        } else {
            None
        };
        Ok(PrefermentPlan {
            available_g,
            preferment_hydration_pct: peak.hydration_pct,
            peak: peak.hours,
            peak_extrapolated: peak.notes.contains(&FermentNote::Extrapolation),
            vigour,
            target,
            alternatives,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target_mix(plan: &PrefermentPlan) -> &Mix {
        match &plan.target {
            TargetPlan::Mix { mix, .. } => mix,
            other => panic!("expected a mix, got {other:?}"),
        }
    }

    #[test]
    fn large_preferment_conflicts_with_fixed_size_and_time() {
        let plan = QuickPreferment {
            seed_g: 5.0,
            feed_flour_g: 200.0,
            feed_water_g: 200.0,
            baked_g: 720.0,
            loss_pct: 10.0,
            hydration_pct: 58.0,
            ..QuickPreferment::default()
        }
        .solve()
        .unwrap();
        let target = target_mix(&plan);
        assert!(target.preferment_g < 405.0 && target.additional_levain_g == 0.0);
        assert!(plan.peak_extrapolated);
        let multiplier = 405.0 / target.preferment_g;
        let alternatives = plan.alternatives.unwrap();
        let all = alternatives.same_weight.unwrap();
        assert!((all.flour_g - 297.5).abs() < 1e-6);
        assert!((all.water_g - 87.5).abs() < 1e-6);
        assert!(alternatives.same_weight_hours.is_none());
        let scaled = alternatives.same_time.unwrap();
        assert!((scaled.baked_g - 720.0 * multiplier).abs() < 1e-6);
    }

    #[test]
    fn top_up_uses_the_extra_starters_own_hydration() {
        let plan = QuickPreferment {
            seed_g: 2.0,
            feed_flour_g: 5.0,
            feed_water_g: 5.0,
            extra_hydration_pct: 50.0,
            ..QuickPreferment::default()
        }
        .solve()
        .unwrap();
        let mix = target_mix(&plan);
        assert!((mix.preferment_g - 12.0).abs() < 1e-6);
        assert!(mix.additional_levain_g > 0.0);
        let water = mix.water_g + 6.0 + mix.additional_levain_g / 3.0;
        assert!((water / mix.total_flour_g - 0.75).abs() < 1e-9);
    }

    #[test]
    fn starter_observations_calibrate_both_forecasts() {
        let mut input = QuickPreferment::default();
        let reference = input.solve().unwrap();
        input.observations = vec![Observation::Peak {
            observed_h: 4.5,
            temp_c: 24.0,
            seed_hydration_pct: 100.0,
            flour_parts: 5.0,
            water_parts: 5.0,
        }];
        let personal = input.solve().unwrap();
        assert!(personal.peak.mid_h < reference.peak.mid_h);
        assert!(target_mix(&personal).preferment_g < target_mix(&reference).preferment_g);
    }

    #[test]
    fn never_label_a_clamped_dose_a_match() {
        let plan = QuickPreferment { bulk_h: 0.5, ..QuickPreferment::default() }.solve().unwrap();
        assert!(matches!(plan.target, TargetPlan::NoDose { too_slow: true }));
        assert!(plan.alternatives.unwrap().same_time.is_none());
    }
}
