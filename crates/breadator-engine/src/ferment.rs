//! Fermentation progress over temperature phases.
//!
//! A rate model gives τ(T): the time, in hours, a dough held at a constant temperature T needs to
//! reach a defined endpoint (for instance an aliquot jar +50 %). Over a varying temperature the
//! progress is P(t) = ∫ v / τ(T(s)) ds, where v is the culture's vigour factor (1 = the model's
//! reference culture; learned from the baker's own bakes, see [`Vigour`]). The endpoint is reached
//! when P = 1. This is the isothermal-rate integration used in predictive microbiology for
//! fluctuating temperatures; it assumes the culture's state depends only on accumulated progress.
//!
//! Dough temperature is not the room's: when dough moves to a fridge or out of it, its core
//! follows Newton's law T(t) = T_env + (T_0 − T_env)·e^(−t/k), with k the time constant of the
//! dough piece in its container. Phases integrate that transient instead of assuming the dough
//! takes the room temperature instantly.

use serde::{Deserialize, Serialize};

use crate::check_range;
use crate::error::EngineError;

/// Integration step, hours (1 minute): far below any τ or cooling constant met in practice.
const STEP_H: f64 = 1.0 / 60.0;
/// Longest schedule integrated, hours (two weeks): bounds the work on absurd inputs.
const MAX_HOURS: f64 = 24.0 * 14.0;

/// τ(T) of a culture at a given dose. Implementations are the parameterized models.
pub trait RateModel {
    /// Hours to reach the endpoint at constant `temp_c`; `None` when the culture does not progress
    /// at that temperature (outside its growth range).
    fn tau_h(&self, temp_c: f64) -> Option<f64>;
}

/// Temperature the dough core follows during a phase.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Phase {
    /// The dough stays at `temp_c` (measured dough temperature, or a proofer that holds it).
    Constant { temp_c: f64, hours: f64 },
    /// The dough sits in an environment at `env_c` and moves toward it with time constant
    /// `time_constant_h`, starting from the dough temperature at the end of the previous phase.
    Environment { env_c: f64, hours: f64, time_constant_h: f64 },
}

impl Phase {
    fn hours(&self) -> f64 {
        match self {
            Self::Constant { hours, .. } | Self::Environment { hours, .. } => *hours,
        }
    }

    fn validate(&self, i: usize) -> Result<(), EngineError> {
        match *self {
            Self::Constant { temp_c, hours } => {
                check_range(&format!("phases[{i}].temp_c"), temp_c, -5.0, 50.0)?;
                check_range(&format!("phases[{i}].hours"), hours, 0.0, MAX_HOURS)
            }
            Self::Environment { env_c, hours, time_constant_h } => {
                check_range(&format!("phases[{i}].env_c"), env_c, -25.0, 50.0)?;
                check_range(&format!("phases[{i}].hours"), hours, 0.0, MAX_HOURS)?;
                check_range(&format!("phases[{i}].time_constant_h"), time_constant_h, 0.01, 48.0)
            }
        }
    }

    /// Dough temperature `t` hours into the phase, starting from `start_c`.
    fn temp_at(&self, start_c: f64, t: f64) -> f64 {
        match *self {
            Self::Constant { temp_c, .. } => temp_c,
            Self::Environment { env_c, time_constant_h, .. } => {
                env_c + (start_c - env_c) * (-t / time_constant_h).exp()
            }
        }
    }
}

/// Progress reached at the end of each phase.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PhaseProgress {
    /// Cumulated progress at the end of the phase (1 = endpoint).
    pub progress: f64,
    /// Dough temperature at the end of the phase, °C.
    pub end_temp_c: f64,
    /// Hours into this phase at which the endpoint was reached, if it was.
    pub endpoint_at_h: Option<f64>,
}

/// Where the progress was made, weighted by progress, up to the endpoint (or the whole schedule
/// when it is not reached): tells how far a prediction leans on the model's cold or hot ends.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct TemperatureStats {
    /// Progress-weighted mean dough temperature, °C.
    pub mean_c: f64,
    pub share_below_8: f64,
    pub share_below_10: f64,
    pub share_above_30: f64,
    pub share_above_35: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleProgress {
    pub phases: Vec<PhaseProgress>,
    pub progress: f64,
    /// Hours from the start of the schedule at which the endpoint is reached.
    pub endpoint_at_h: Option<f64>,
    pub temperature_stats: TemperatureStats,
}

/// Integrates progress over the given phases. `start_c` is the dough temperature at mixing.
pub fn progress(
    model: &impl RateModel,
    vigour: f64,
    start_c: f64,
    phases: &[Phase],
) -> Result<ScheduleProgress, EngineError> {
    check_range("start_c", start_c, -5.0, 50.0)?;
    check_range("vigour", vigour, 0.01, 100.0)?;
    let total: f64 = phases.iter().map(Phase::hours).sum();
    check_range("total_hours", total, 0.0, MAX_HOURS)?;
    for (i, p) in phases.iter().enumerate() {
        p.validate(i)?;
    }

    let mut done = 0.0;
    let mut temp = start_c;
    let mut elapsed = 0.0;
    let mut endpoint_at_h = None;
    let mut out = Vec::with_capacity(phases.len());
    // Progress-weighted temperature sums, counted up to the endpoint.
    let (mut w, mut wt, mut below8, mut below10, mut above30, mut above35) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for phase in phases {
        let hours = phase.hours();
        let phase_start = temp;
        let mut endpoint_in_phase = None;
        let mut t = 0.0;
        while t < hours {
            let dt = STEP_H.min(hours - t);
            // Midpoint rule: rate at the dough temperature in the middle of the step.
            let mid_c = phase.temp_at(phase_start, t + dt / 2.0);
            let rate = model.tau_h(mid_c).map_or(0.0, |tau| vigour / tau);
            let step = rate * dt;
            if endpoint_at_h.is_none() {
                let counted = if done + step >= 1.0 { 1.0 - done } else { step };
                w += counted;
                wt += counted * mid_c;
                below8 += if mid_c < 8.0 { counted } else { 0.0 };
                below10 += if mid_c < 10.0 { counted } else { 0.0 };
                above30 += if mid_c > 30.0 { counted } else { 0.0 };
                above35 += if mid_c > 35.0 { counted } else { 0.0 };
                if done + step >= 1.0 {
                    let frac = if step > 0.0 { (1.0 - done) / step } else { 0.0 };
                    endpoint_in_phase = Some(t + frac * dt);
                    endpoint_at_h = Some(elapsed + t + frac * dt);
                }
            }
            done += step;
            t += dt;
        }
        temp = phase.temp_at(phase_start, hours);
        elapsed += hours;
        out.push(PhaseProgress {
            progress: done,
            end_temp_c: temp,
            endpoint_at_h: endpoint_in_phase,
        });
    }
    let share = |x: f64| if w > 0.0 { x / w } else { 0.0 };
    let temperature_stats = TemperatureStats {
        mean_c: if w > 0.0 { wt / w } else { start_c },
        share_below_8: share(below8),
        share_below_10: share(below10),
        share_above_30: share(above30),
        share_above_35: share(above35),
    };
    Ok(ScheduleProgress { phases: out, progress: done, endpoint_at_h, temperature_stats })
}

/// Hours needed in a final, open-ended phase (`last`) after the fixed `phases` to reach the
/// endpoint; `None` if it is never reached within the integration limit.
pub fn time_to_endpoint(
    model: &impl RateModel,
    vigour: f64,
    start_c: f64,
    phases: &[Phase],
    last: Phase,
) -> Result<Option<f64>, EngineError> {
    let fixed: f64 = phases.iter().map(Phase::hours).sum();
    let mut all = phases.to_vec();
    let open = MAX_HOURS - fixed;
    if open <= 0.0 {
        return Err(EngineError::OutOfRange {
            field: "total_hours".into(),
            value: fixed,
            min: 0.0,
            max: MAX_HOURS,
        });
    }
    all.push(match last {
        Phase::Constant { temp_c, .. } => Phase::Constant { temp_c, hours: open },
        Phase::Environment { env_c, time_constant_h, .. } => {
            Phase::Environment { env_c, hours: open, time_constant_h }
        }
    });
    let p = progress(model, vigour, start_c, &all)?;
    Ok(p.endpoint_at_h.map(|h| (h - fixed).max(0.0)))
}

/// Finds the dose (yeast %, inoculation %…) for which the endpoint is reached at `target_h`.
/// `model_at(dose)` builds the rate model for a dose; more dose must mean faster fermentation.
/// Bisection on log(dose) between `min_dose` and `max_dose`.
pub fn dose_for_time<M: RateModel>(
    model_at: impl Fn(f64) -> M,
    vigour: f64,
    start_c: f64,
    phases: &[Phase],
    target_h: f64,
    min_dose: f64,
    max_dose: f64,
) -> Result<Option<f64>, EngineError> {
    check_range("target_h", target_h, 0.05, MAX_HOURS)?;
    if !(min_dose > 0.0 && max_dose > min_dose) {
        return Err(EngineError::Invalid {
            field: "dose".into(),
            reason: "empty dose range".into(),
        });
    }
    let total: f64 = phases.iter().map(Phase::hours).sum();
    if total + 1e-9 < target_h {
        return Err(EngineError::Invalid {
            field: "phases".into(),
            reason: "the schedule is shorter than the target time".into(),
        });
    }
    let reach = |dose: f64| -> Result<f64, EngineError> {
        Ok(progress(&model_at(dose), vigour, start_c, phases)?
            .endpoint_at_h
            .unwrap_or(f64::INFINITY))
    };
    // Too fast even at the lowest dose, or too slow at the highest: no dose fits.
    if reach(min_dose)? < target_h || reach(max_dose)? > target_h {
        return Ok(None);
    }
    let (mut lo, mut hi) = (min_dose.ln(), max_dose.ln());
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if reach(mid.exp())? > target_h {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Ok(Some((0.5 * (lo + hi)).exp()))
}

/// Multiplicative uncertainty on a predicted duration: ln(observed/predicted) ~ N(0, σ²).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Band {
    pub low_h: f64,
    pub mid_h: f64,
    pub high_h: f64,
}

/// 80 % band (z = 1.2816) around `hours` for a log-normal spread `sigma`.
pub fn band80(hours: f64, sigma: f64) -> Band {
    let z = 1.281_551_565_545 * sigma.max(0.0);
    Band { low_h: hours * (-z).exp(), mid_h: hours, high_h: hours * z.exp() }
}

/// A culture's vigour factor, learned from bakes: log v ~ N(mean, variance).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vigour {
    pub log_mean: f64,
    pub log_variance: f64,
    /// Number of bakes learned from.
    pub n: u32,
}

impl Vigour {
    /// Prior for a culture never baked with: the model's reference culture, with `prior_sigma`
    /// (log scale) of culture-to-culture spread.
    pub fn prior(prior_sigma: f64) -> Self {
        Self { log_mean: 0.0, log_variance: prior_sigma * prior_sigma, n: 0 }
    }

    pub fn factor(&self) -> f64 {
        self.log_mean.exp()
    }

    /// Learns from one bake: with the vigour assumed (`self.factor()`), the model predicted
    /// `predicted_progress` at the moment the endpoint was actually observed. The vigour that
    /// would have predicted exactly 1 is factor / predicted_progress; it is weighed against the
    /// prior with the observation noise `obs_sigma` (log scale): conjugate normal update.
    pub fn update(&self, predicted_progress: f64, obs_sigma: f64) -> Result<Self, EngineError> {
        check_range("predicted_progress", predicted_progress, 0.01, 100.0)?;
        check_range("obs_sigma", obs_sigma, 0.01, 5.0)?;
        let observed = self.log_mean - predicted_progress.ln();
        let obs_var = obs_sigma * obs_sigma;
        let variance = 1.0 / (1.0 / self.log_variance + 1.0 / obs_var);
        let mean = variance * (self.log_mean / self.log_variance + observed / obs_var);
        Ok(Self { log_mean: mean, log_variance: variance, n: self.n + 1 })
    }

    /// Spread of a prediction made with this vigour: model error and vigour uncertainty combined.
    pub fn prediction_sigma(&self, model_sigma: f64) -> f64 {
        (model_sigma * model_sigma + self.log_variance).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// τ = 10 h at 25 °C, Q10 = 2.
    struct Q10;
    impl RateModel for Q10 {
        fn tau_h(&self, t: f64) -> Option<f64> {
            Some(10.0 * 2f64.powf((25.0 - t) / 10.0))
        }
    }

    struct Dose(f64);
    impl RateModel for Dose {
        fn tau_h(&self, t: f64) -> Option<f64> {
            Q10.tau_h(t).map(|tau| tau * (10.0 / self.0).powf(0.7))
        }
    }

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    #[test]
    fn constant_temperature_reaches_the_endpoint_at_tau() {
        let p =
            progress(&Q10, 1.0, 25.0, &[Phase::Constant { temp_c: 25.0, hours: 20.0 }]).unwrap();
        close(p.endpoint_at_h.unwrap(), 10.0, 1e-6);
        close(p.progress, 2.0, 1e-9);
        let open =
            time_to_endpoint(&Q10, 2.0, 25.0, &[], Phase::Constant { temp_c: 35.0, hours: 0.0 })
                .unwrap();
        close(open.unwrap(), 2.5, 1e-6);
    }

    #[test]
    fn phases_add_their_fractions() {
        // 5 h at 25 °C (half the way), then 15 °C (τ = 20 h): 10 more hours.
        let fixed = [Phase::Constant { temp_c: 25.0, hours: 5.0 }];
        let h =
            time_to_endpoint(&Q10, 1.0, 25.0, &fixed, Phase::Constant { temp_c: 15.0, hours: 0.0 })
                .unwrap();
        close(h.unwrap(), 10.0, 1e-6);
    }

    #[test]
    fn newton_cooling_matches_the_analytic_integral() {
        // From 25 °C into a 5 °C fridge with k = 2 h: T(t) = 5 + 20·e^(−t/2).
        // Rate = 2^((T−25)/10)/10; integrate analytically by fine quadrature as the reference.
        let phase = Phase::Environment { env_c: 5.0, hours: 12.0, time_constant_h: 2.0 };
        let p = progress(&Q10, 1.0, 25.0, &[phase]).unwrap();
        let n = 200_000;
        let dt = 12.0 / n as f64;
        let reference: f64 = (0..n)
            .map(|i| {
                let t = (i as f64 + 0.5) * dt;
                let temp = 5.0 + 20.0 * (-t / 2.0).exp();
                2f64.powf((temp - 25.0) / 10.0) / 10.0 * dt
            })
            .sum();
        close(p.progress, reference, 1e-6);
        close(p.phases[0].end_temp_c, 5.0 + 20.0 * (-6f64).exp(), 1e-9);
        // The transient matters: a dough assumed at 5 °C at once would make 12/40 = 0.30; cooling
        // adds 0.05·Σ(ln 4)^k/(k·k!) ≈ 0.103 (substituting u = e^(−t/2)), about a third more.
        let instant = 12.0 / Q10.tau_h(5.0).unwrap();
        close(p.progress / instant, (0.30 + 0.103) / 0.30, 0.01);
    }

    #[test]
    fn dose_for_time_inverts_the_model() {
        let phases = [Phase::Constant { temp_c: 22.0, hours: 48.0 }];
        let dose = dose_for_time(Dose, 1.0, 22.0, &phases, 8.0, 0.1, 100.0).unwrap().unwrap();
        let reached = progress(&Dose(dose), 1.0, 22.0, &phases).unwrap().endpoint_at_h.unwrap();
        close(reached, 8.0, 1e-3);
        // Unreachable: even the largest dose is too slow at 2 °C for a 1 h target.
        let cold = [Phase::Constant { temp_c: 2.0, hours: 48.0 }];
        assert_eq!(dose_for_time(Dose, 1.0, 2.0, &cold, 1.0, 0.1, 30.0).unwrap(), None);
    }

    #[test]
    fn vigour_learns_toward_observations_and_narrows() {
        let prior = Vigour::prior(0.4);
        // The endpoint came when the model said only 0.5 of the way: the culture is ~2× faster.
        let v = prior.update(0.5, 0.2).unwrap();
        assert!(v.factor() > 1.0 && v.factor() < 2.0);
        assert!(v.log_variance < prior.log_variance);
        let mut many = prior;
        for _ in 0..30 {
            many = many.update(0.5 * many.factor(), 0.2).unwrap();
        }
        close(many.factor(), 2.0, 0.05);
        let b = band80(10.0, 0.3);
        assert!(b.low_h < 10.0 && b.high_h > 10.0);
        close(b.low_h * b.high_h, 100.0, 1e-9);
    }
}
