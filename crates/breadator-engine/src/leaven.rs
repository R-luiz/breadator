//! Fermentation models: commercial yeast, sourdough, and the fermentation calculator.
//!
//! Parameters, sources and validation: `research/data/model-fermentation.json` and
//! `research/notes/science/fermentation/` (reference implementation `fits/fermodel.py`).
//!
//! - Temperature: γ(T) = e^(c (T − 25)) · ((T_max − T) / (T_max − 25))^b, 1 at 25 °C — Gänzle,
//!   Ehmann & Hammes 1998 model 1a normalised at 25 °C.
//! - Yeast: τ_doubled = A · d^(−a) / (M · v · γ_y(T)), d = instant-yeast equivalent % of flour;
//!   fitted to TXCraig1's pizzamaking.com chart (918 cells, 5 % rms). Other endpoints are a
//!   fraction of the doubling time.
//! - Sourdough: rise R = ι (e^G − 1), G = ∫ μ_ref · M · v · γ_s(T) dt, so at constant temperature
//!   τ = ln((R + ι) / ι) / (μ_ref · M · v · γ_s). ι = prefermented flour / total flour. μ_ref
//!   measured on 40 timed doughs (10 % rms); reproduces The Sourdough Framework Table 7.1.
//! - Uncertainty: ln(observed / predicted) ~ N(0, sd²), sd² = sd_v² + se² + (0.03·ΔT)² +
//!   Σ modifier terms² + Σ domain penalties².

use serde::{Deserialize, Serialize};

use crate::check_range;
use crate::error::EngineError;
use crate::ferment::{
    Band, Phase, RateModel, ScheduleProgress, Vigour, band80, dose_for_time, progress,
};

/// Reference temperature of every rate parameter, °C.
pub const T_REF: f64 = 25.0;

/// Relative rate at `temp_c` (1 at 25 °C, 0 at and above `t_max`).
pub fn gamma(temp_c: f64, b: f64, c: f64, t_max: f64) -> f64 {
    if temp_c >= t_max {
        return 0.0;
    }
    (c * (temp_c - T_REF)).exp() * ((t_max - temp_c) / (t_max - T_REF)).powf(b)
}

/// Commercial yeast, instant-yeast equivalent dose; "doubled" endpoint.
pub mod yeast {
    /// Hours to double at 1 % instant yeast, 25 °C (range 0.6–1.6).
    pub const A_H: f64 = 0.91;
    /// Dose exponent (0.60–0.90).
    pub const DOSE_EXP: f64 = 0.72;
    pub const C: f64 = 0.190;
    pub const B: f64 = 1.32;
    pub const T_MAX: f64 = 45.0;
    /// Validated dose range, % instant yeast of flour.
    pub const DOSE_DOMAIN: (f64, f64) = (0.003, 1.0);
    pub fn gamma(t: f64) -> f64 {
        super::gamma(t, B, C, T_MAX)
    }
}

/// Sourdough (LAB + yeast consortium of a ripe starter).
pub mod sourdough {
    /// Growth rate at 25 °C in the reference dough, 1/h (range 0.15–0.48).
    pub const MU_REF: f64 = 0.30;
    pub const C: f64 = 0.314;
    pub const B: f64 = 3.64;
    pub const T_MAX: f64 = 40.4;
    /// Delay of a starter taken from the fridge (about a week old) at 25 °C, h (1–3).
    pub const FRIDGE_LAG_H: f64 = 2.0;
    /// Validated prefermented-flour fraction.
    pub const IOTA_DOMAIN: (f64, f64) = (0.005, 0.2);
    pub fn gamma(t: f64) -> f64 {
        super::gamma(t, B, C, T_MAX)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Leavening {
    InstantYeast,
    ActiveDryYeast,
    FreshYeast,
    OsmotolerantYeast,
    Sourdough,
}

impl Leavening {
    /// Grams of instant yeast one gram of this form is worth: (typical, low, high).
    fn idy_equivalent(self) -> (f64, f64, f64) {
        match self {
            Self::InstantYeast | Self::OsmotolerantYeast => (1.0, 1.0, 1.0),
            Self::ActiveDryYeast => (0.75, 0.67, 1.0),
            Self::FreshYeast => (0.33, 0.25, 0.42),
            Self::Sourdough => (1.0, 1.0, 1.0),
        }
    }
}

/// The volume target that defines "done".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Endpoint {
    /// Aliquot jar +25 % (weak flours, The Sourdough Framework Table 7.3). Sourdough only.
    Rise25,
    Rise50,
    Rise75,
    Doubled,
    /// Yeast only.
    Tripled,
    /// A poolish or sponge at its peak (domed, starting to recede). Yeast only.
    RipePoolish,
}

impl Endpoint {
    /// Yeast: (factor on the doubling time, low, high).
    fn yeast_factor(self) -> Option<(f64, f64, f64)> {
        match self {
            Self::Rise25 => None,
            Self::Rise50 => Some((0.70, 0.55, 0.85)),
            Self::Rise75 => Some((0.85, 0.75, 0.93)),
            Self::Doubled => Some((1.0, 1.0, 1.0)),
            Self::Tripled => Some((1.6, 1.4, 2.0)),
            Self::RipePoolish => Some((1.9, 1.38, 2.24)),
        }
    }

    /// Sourdough: rise fraction R.
    fn sourdough_rise(self) -> Option<f64> {
        match self {
            Self::Rise25 => Some(0.25),
            Self::Rise50 => Some(0.5),
            Self::Rise75 => Some(0.75),
            Self::Doubled => Some(1.0),
            Self::Tripled | Self::RipePoolish => None,
        }
    }
}

/// Container a dough sits in, for its cooling/warming time constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Container {
    /// Lined banneton in a bag (the measured reference loaf).
    Banneton,
    /// Covered thin plastic or metal bowl or tub.
    Tub,
    /// Thick glass or ceramic bowl with a lid.
    ThickBowl,
    /// Dough balls in a closed tray.
    PizzaTray,
    /// Bare on a tray in a fan-assisted fridge.
    BareTray,
}

impl Container {
    /// (effective heat transfer coefficient W/m²K, shape area factor).
    fn transfer(self) -> (f64, f64) {
        match self {
            Self::Banneton => (5.9, 1.0),
            Self::Tub => (8.0, 1.0),
            Self::ThickBowl => (6.0, 1.0),
            Self::PizzaTray => (6.0, 1.4),
            Self::BareTray => (12.0, 1.2),
        }
    }

    /// Cooling time constant of `dough_g` grams, h: 2.8 h measured for a 650 g loaf in a
    /// banneton, scaled by (m / 0.65 kg)^(1/3) and by the container's heat transfer (±35 %).
    pub fn cooling_tau_h(self, dough_g: f64) -> f64 {
        let (h, shape) = self.transfer();
        2.8 * (dough_g / 650.0).cbrt() * (5.9 / h) / shape
    }
}

/// Dough composition that changes fermentation speed (all in % of total flour).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DoughFactors {
    pub salt_pct: f64,
    pub sugar_pct: f64,
    pub fat_pct: f64,
    pub hydration_pct: f64,
    /// Whole-grain or bran share of the flour, %.
    pub whole_grain_pct: f64,
    /// Cinnamon or similar spice mixed into the dough (not a filling), %.
    pub spice_pct: f64,
    pub cocoa_pct: f64,
    pub scalded_pct: f64,
    pub milk: bool,
    pub eggs: bool,
}

impl Default for DoughFactors {
    fn default() -> Self {
        Self {
            salt_pct: 2.0,
            sugar_pct: 0.0,
            fat_pct: 0.0,
            hydration_pct: 70.0,
            whole_grain_pct: 0.0,
            spice_pct: 0.0,
            cocoa_pct: 0.0,
            scalded_pct: 0.0,
            milk: false,
            eggs: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    Measured,
    Literature,
    CreatorRule,
    Extrapolation,
}

/// One rate factor applied to the reference dough, with its 80 % range.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Modifier {
    /// `salt`, `sugar`, `fat`, `milk`, `eggs`, `spice`, `cocoa`, `whole_grain`, `scald`,
    /// `hydration`, `endpoint`, `yeast_form`.
    pub code: &'static str,
    /// Multiplies the rate (divides the time).
    pub factor: f64,
    pub low: f64,
    pub high: f64,
    pub evidence: Evidence,
}

impl Modifier {
    fn sd(&self) -> f64 {
        if self.high > self.low { (self.high / self.low).ln() / 2.56 } else { 0.0 }
    }
}

/// Linear interpolation in a sorted (x, y) table, flat outside.
fn interp(table: &[(f64, f64)], x: f64) -> f64 {
    let (first, last) = (table[0], table[table.len() - 1]);
    if x <= first.0 {
        return first.1;
    }
    if x >= last.0 {
        return last.1;
    }
    for w in table.windows(2) {
        let ((x0, y0), (x1, y1)) = (w[0], w[1]);
        if x <= x1 {
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0);
        }
    }
    last.1
}

/// The modifiers that differ from 1 (or carry a range) for this dough and leavening.
pub fn modifiers(f: &DoughFactors, leavening: Leavening) -> Vec<Modifier> {
    use Evidence::*;
    let sourdough = leavening == Leavening::Sourdough;
    let m = |code, factor: f64, low: f64, high: f64, evidence| Modifier {
        code,
        factor,
        low,
        high,
        evidence,
    };
    let mut out = Vec::new();

    // Salt (Carcea et al. 2020 for yeast; Wraith's table for sourdough).
    if sourdough {
        let s = f.salt_pct.min(3.0);
        let x = 1.0 + 0.09 * (2.0 - s);
        out.push(m("salt", x, x * 0.94, x * 1.03, CreatorRule));
    } else {
        let s = f.salt_pct.min(3.5);
        let x = (1.0 - 0.021 * s * s) / 0.916;
        out.push(m("salt", x, x * 0.95, x * 1.05, Measured));
    }

    // Sugar (Lallemand Baking Update 1/3 relative-activity curves).
    if f.sugar_pct > 0.0 {
        let s = f.sugar_pct;
        let (x, lo, hi, ev) = match leavening {
            Leavening::OsmotolerantYeast => {
                let x = interp(&[(0.0, 0.57), (10.0, 0.78), (20.0, 1.0), (30.0, 1.25)], s);
                (x, x * 0.85, x * 1.15, Literature)
            }
            Leavening::Sourdough => (
                interp(&[(0.0, 1.0), (5.0, 1.0), (10.0, 0.8), (20.0, 0.5)], s),
                interp(&[(0.0, 1.0), (5.0, 1.0), (10.0, 0.6), (20.0, 0.3)], s),
                interp(&[(0.0, 1.0), (5.0, 1.0), (10.0, 1.0), (20.0, 0.8)], s),
                Extrapolation,
            ),
            _ => (
                interp(
                    &[
                        (0.0, 1.0),
                        (5.0, 1.0),
                        (10.0, 0.8),
                        (15.0, 0.7),
                        (20.0, 0.6),
                        (25.0, 0.5),
                        (30.0, 0.4),
                    ],
                    s,
                ),
                interp(&[(0.0, 1.0), (5.0, 0.9), (10.0, 0.7), (20.0, 0.5), (30.0, 0.35)], s),
                interp(&[(0.0, 1.0), (5.0, 1.1), (10.0, 1.0), (20.0, 0.8), (30.0, 0.6)], s),
                Literature,
            ),
        };
        out.push(m("sugar", x, lo, hi, ev));
    } else if leavening == Leavening::OsmotolerantYeast {
        out.push(m("sugar", 0.57, 0.48, 0.66, Literature));
    }

    if f.fat_pct > 10.0 {
        let (x, lo) = if f.fat_pct > 30.0 { (0.8, 0.6) } else { (0.9, 0.75) };
        out.push(m("fat", x, lo, 1.0, Extrapolation));
    }
    if f.milk {
        out.push(m("milk", 1.0, 0.9, 1.05, Extrapolation));
    }
    if f.eggs {
        out.push(m("eggs", 1.0, 0.9, 1.05, Extrapolation));
    }
    if f.spice_pct >= 0.5 {
        let x = interp(&[(0.5, 1.0), (1.0, 0.8), (3.0, 0.6)], f.spice_pct);
        let lo = interp(&[(0.5, 1.0), (1.0, 0.5), (3.0, 0.3)], f.spice_pct);
        let hi = interp(&[(0.5, 1.0), (1.0, 1.0), (3.0, 0.9)], f.spice_pct);
        out.push(m("spice", x, lo, hi, Extrapolation));
    }
    if f.cocoa_pct >= 3.0 {
        out.push(m("cocoa", 0.9, 0.75, 1.0, Extrapolation));
    }
    if f.whole_grain_pct > 0.0 {
        let w = (f.whole_grain_pct / 100.0).clamp(0.0, 1.0);
        let (x, hi, ev) = if sourdough {
            (1.0 + 0.3 * w, 1.0 + 0.6 * w, Measured)
        } else {
            (1.0 + 0.1 * w, 1.0 + 0.25 * w, Extrapolation)
        };
        out.push(m("whole_grain", x, 1.0, hi, ev));
    }
    if f.scalded_pct > 0.0 {
        out.push(m("scald", 1.0, 0.95, 1.1, Extrapolation));
    }
    let h = f.hydration_pct;
    if sourdough {
        let x = (1.0 + 0.005 * (h - 75.0)).clamp(0.85, 1.15);
        let hi = (1.0 + 0.015 * (h - 75.0)).clamp(0.6, 1.6);
        out.push(m("hydration", x, x.min(1.0).min(hi), x.max(1.0).max(hi), Extrapolation));
    } else {
        let x = (1.0 + 0.004 * (h - 65.0)).clamp(0.9, 1.1);
        let hi = (1.0 + 0.01 * (h - 65.0)).clamp(0.75, 1.25);
        out.push(m("hydration", x, x.min(1.0).min(hi), x.max(1.0).max(hi), Literature));
    }
    out
}

/// Yeast rate model at a given instant-yeast-equivalent dose.
pub struct YeastRate {
    /// Hours to the endpoint at 25 °C.
    pub tau25_h: f64,
}

impl YeastRate {
    pub fn new(idy_pct: f64, endpoint_factor: f64, rate_factor: f64) -> Self {
        Self {
            tau25_h: endpoint_factor * yeast::A_H * idy_pct.powf(-yeast::DOSE_EXP) / rate_factor,
        }
    }
}

impl RateModel for YeastRate {
    fn tau_h(&self, temp_c: f64) -> Option<f64> {
        let g = yeast::gamma(temp_c);
        (g > 0.0).then(|| self.tau25_h / g)
    }
}

/// Sourdough rate model: ln((R + ι)/ι) growth needed at rate μ_ref·M·γ, plus a starter lag.
pub struct SourdoughRate {
    pub growth: f64,
    pub mu25: f64,
    pub lag25_h: f64,
}

impl SourdoughRate {
    pub fn new(iota: f64, rise: f64, rate_factor: f64, lag25_h: f64) -> Self {
        Self { growth: (1.0 + rise / iota).ln(), mu25: sourdough::MU_REF * rate_factor, lag25_h }
    }
}

impl RateModel for SourdoughRate {
    fn tau_h(&self, temp_c: f64) -> Option<f64> {
        let g = sourdough::gamma(temp_c);
        (g > 0.0).then(|| (self.growth / self.mu25 + self.lag25_h) / g)
    }
}

/// Prefermented-flour fraction from a starter given as % of the added flour.
pub fn iota_from_starter(starter_pct: f64, starter_hydration_pct: f64) -> f64 {
    let starter_flour = starter_pct / (1.0 + starter_hydration_pct / 100.0);
    starter_flour / (100.0 + starter_flour)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Solve {
    /// Given the dose, how long to the endpoint.
    Time,
    /// Given the time, how much yeast or starter.
    Dose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterState {
    /// At or near its peak.
    Ripe,
    /// Taken from the fridge without a refresh.
    FromFridge,
}

/// The fermentation calculator's form, shared by the server and the browser island.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickFerment {
    pub solve: Solve,
    pub leavening: Leavening,
    /// Yeast as % of flour in its own form, or starter mass as % of the added flour.
    pub dose_pct: f64,
    pub starter_hydration_pct: f64,
    pub starter_state: StarterState,
    /// Total hours from mixing to the endpoint (`Solve::Dose`).
    pub target_h: f64,
    pub endpoint: Endpoint,
    /// Dough temperature right after mixing, °C.
    pub dough_c: f64,
    pub room_c: f64,
    pub dough_g: f64,
    pub container: Container,
    /// Hours at room temperature before the fridge (with `fridge_h`).
    pub warm_h: f64,
    /// Hours in the fridge (0 = no cold phase); then back at room temperature.
    pub fridge_h: f64,
    pub fridge_c: f64,
    pub dough: DoughFactors,
    /// The baker's calibration for this culture; `None` = model's reference culture.
    pub vigour: Option<Vigour>,
    /// Mean dough temperature of the calibration bakes (band widens away from it), °C.
    pub anchor_c: Option<f64>,
}

impl Default for QuickFerment {
    fn default() -> Self {
        Self {
            solve: Solve::Time,
            leavening: Leavening::Sourdough,
            dose_pct: 20.0,
            starter_hydration_pct: 100.0,
            starter_state: StarterState::Ripe,
            target_h: 8.0,
            endpoint: Endpoint::Rise50,
            dough_c: 24.0,
            room_c: 22.0,
            dough_g: 900.0,
            container: Container::Tub,
            warm_h: 0.0,
            fridge_h: 0.0,
            fridge_c: 4.0,
            dough: DoughFactors::default(),
            vigour: None,
            anchor_c: None,
        }
    }
}

/// Where the endpoint falls in the schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointPhase {
    /// At room temperature, before the fridge.
    BeforeFridge,
    Fridge,
    /// At room temperature: the only phase without a fridge, or after the fridge.
    Room,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum FermentNote {
    /// Prediction made mostly outside the validated temperatures or doses.
    Extrapolation,
    /// Above ~10 % sugar a regular yeast slows down: osmotolerant yeast is advised.
    UseOsmotolerant { sugar_pct: f64 },
    /// Under 45 minutes: outside the data, the lag of the yeast dominates.
    VeryShort,
    /// The endpoint is reached before the fridge: the dough would over-ferment in a retard.
    EndpointBeforeFridge,
    /// No dose in the searched range reaches the endpoint at the target time.
    NoDoseFits { too_slow: bool },
    /// Enriched dough or heavily yeasted preferment in the cold: visible rise often far less.
    RichDoughCold,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FermentResult {
    /// Hours from mixing to the endpoint, with its 80 % band.
    pub hours: Band,
    pub endpoint_phase: EndpointPhase,
    /// Dose found (`Solve::Dose`) or given, in the leavening's own form and unit.
    pub dose_pct: f64,
    /// Instant-yeast equivalent (yeast) or prefermented flour fraction ι (sourdough).
    pub idy_pct: Option<f64>,
    pub iota: Option<f64>,
    /// Progress at the end of each scheduled phase (1 = endpoint).
    pub schedule: ScheduleProgress,
    pub modifiers: Vec<Modifier>,
    /// Combined rate factor of the modifiers.
    pub rate_factor: f64,
    /// Log-scale standard deviation behind the band.
    pub sd: f64,
    /// Progress-weighted mean dough temperature, °C.
    pub mean_temp_c: f64,
    pub cooling_tau_h: f64,
    pub notes: Vec<FermentNote>,
}

impl QuickFerment {
    fn validate(&self) -> Result<(), EngineError> {
        check_range("dough_c", self.dough_c, 0.0, 45.0)?;
        check_range("room_c", self.room_c, 0.0, 45.0)?;
        check_range("fridge_c", self.fridge_c, -2.0, 12.0)?;
        check_range("dough_g", self.dough_g, 50.0, 20_000.0)?;
        check_range("warm_h", self.warm_h, 0.0, 72.0)?;
        check_range("fridge_h", self.fridge_h, 0.0, 120.0)?;
        check_range("starter_hydration_pct", self.starter_hydration_pct, 30.0, 300.0)?;
        let d = &self.dough;
        for (field, v, max) in [
            ("salt_pct", d.salt_pct, 5.0),
            ("sugar_pct", d.sugar_pct, 40.0),
            ("fat_pct", d.fat_pct, 100.0),
            ("hydration_pct", d.hydration_pct, 200.0),
            ("whole_grain_pct", d.whole_grain_pct, 100.0),
            ("spice_pct", d.spice_pct, 10.0),
            ("cocoa_pct", d.cocoa_pct, 30.0),
            ("scalded_pct", d.scalded_pct, 50.0),
        ] {
            check_range(field, v, 0.0, max)?;
        }
        let sourdough = self.leavening == Leavening::Sourdough;
        let ok = if sourdough {
            self.endpoint.sourdough_rise().is_some()
        } else {
            self.endpoint.yeast_factor().is_some()
        };
        if !ok {
            return Err(EngineError::Invalid {
                field: "endpoint".into(),
                reason: if sourdough {
                    "not a sourdough endpoint".into()
                } else {
                    "not a yeast endpoint".into()
                },
            });
        }
        match self.solve {
            Solve::Time if sourdough => check_range("dose_pct", self.dose_pct, 0.1, 200.0),
            Solve::Time => check_range("dose_pct", self.dose_pct, 0.001, 10.0),
            Solve::Dose => check_range("target_h", self.target_h, 0.5, 96.0),
        }
    }

    fn phases(&self, tau_c: f64, open_h: f64) -> Vec<Phase> {
        let mut phases = Vec::new();
        if self.fridge_h > 0.0 {
            if self.warm_h > 0.0 {
                phases.push(Phase::Environment {
                    env_c: self.room_c,
                    hours: self.warm_h,
                    time_constant_h: tau_c,
                });
            }
            phases.push(Phase::Environment {
                env_c: self.fridge_c,
                hours: self.fridge_h,
                time_constant_h: tau_c,
            });
            // Warming is a little faster than cooling (condensation): 0.85 of the cooling constant.
            phases.push(Phase::Environment {
                env_c: self.room_c,
                hours: open_h,
                time_constant_h: 0.85 * tau_c,
            });
        } else {
            phases.push(Phase::Environment {
                env_c: self.room_c,
                hours: self.warm_h.max(0.0) + open_h,
                time_constant_h: tau_c,
            });
        }
        phases
    }

    pub fn solve(&self) -> Result<FermentResult, EngineError> {
        self.validate()?;
        let sourdough = self.leavening == Leavening::Sourdough;
        let mods = modifiers(&self.dough, self.leavening);
        let mut rate_factor: f64 = mods.iter().map(|m| m.factor).product();
        let vigour =
            self.vigour.unwrap_or_else(|| Vigour::prior(if sourdough { 0.35 } else { 0.45 }));
        rate_factor *= vigour.factor();
        let tau_c = self.container.cooling_tau_h(self.dough_g);
        let fixed_h = if self.fridge_h > 0.0 { self.warm_h + self.fridge_h } else { 0.0 };
        let mut notes = Vec::new();
        let mut extra_sd: Vec<f64> = mods.iter().map(Modifier::sd).collect();

        let (idy_form, _, _) = self.leavening.idy_equivalent();
        let endpoint_factor = self.endpoint.yeast_factor().unwrap_or((1.0, 1.0, 1.0));
        if !sourdough && endpoint_factor.2 > endpoint_factor.1 {
            extra_sd.push((endpoint_factor.2 / endpoint_factor.1).ln() / 2.56);
        }
        let (_, eq_lo, eq_hi) = self.leavening.idy_equivalent();
        if eq_hi > eq_lo {
            extra_sd.push(DOSE_EXP_FOR_FORM * (eq_hi / eq_lo).ln() / 2.56);
        }
        let rise = self.endpoint.sourdough_rise().unwrap_or(0.5);
        let lag = if self.starter_state == StarterState::FromFridge {
            sourdough::FRIDGE_LAG_H
        } else {
            0.0
        };
        let hydration = self.starter_hydration_pct;
        let model_at = |dose: f64| -> BoxedRate {
            BoxedRate(if sourdough {
                Box::new(SourdoughRate::new(
                    iota_from_starter(dose, hydration),
                    rise,
                    rate_factor,
                    lag,
                ))
            } else {
                Box::new(YeastRate::new(dose * idy_form, endpoint_factor.0, rate_factor))
            })
        };

        let open = 96.0 + 24.0;
        let phases = self.phases(tau_c, open);
        let (dose, schedule) = match self.solve {
            Solve::Time => {
                (self.dose_pct, progress(&model_at(self.dose_pct), 1.0, self.dough_c, &phases)?)
            }
            Solve::Dose => {
                let (lo, hi) =
                    if sourdough { (0.2, 150.0) } else { (0.003 / idy_form, 6.0 / idy_form) };
                match dose_for_time(model_at, 1.0, self.dough_c, &phases, self.target_h, lo, hi)? {
                    Some(d) => (d, progress(&model_at(d), 1.0, self.dough_c, &phases)?),
                    None => {
                        // Even the largest dose ends after the target: the target is too short.
                        // Otherwise even the smallest dose ends before it: too long.
                        let fastest = progress(&model_at(hi), 1.0, self.dough_c, &phases)?;
                        let too_slow = fastest.endpoint_at_h.is_none_or(|h| h > self.target_h);
                        notes.push(FermentNote::NoDoseFits { too_slow });
                        let d = if too_slow { hi } else { lo };
                        (d, progress(&model_at(d), 1.0, self.dough_c, &phases)?)
                    }
                }
            }
        };
        let Some(hours) = schedule.endpoint_at_h else {
            return Err(EngineError::Invalid {
                field: "schedule".into(),
                reason: "the endpoint is not reached within 5 days at these temperatures".into(),
            });
        };
        let endpoint_phase = if self.fridge_h <= 0.0 || hours > fixed_h {
            EndpointPhase::Room
        } else if hours <= self.warm_h {
            EndpointPhase::BeforeFridge
        } else {
            EndpointPhase::Fridge
        };
        if endpoint_phase == EndpointPhase::BeforeFridge {
            notes.push(FermentNote::EndpointBeforeFridge);
        }

        // Uncertainty: temperature distance, domain penalties.
        let stats = schedule.temperature_stats;
        let anchor = self.anchor_c.unwrap_or(T_REF);
        let mut sd2 = vigour.log_variance
            + 0.2 * 0.2
            + (0.03 * (stats.mean_c - anchor).abs().min(10.0)).powi(2);
        sd2 += extra_sd.iter().map(|s| s * s).sum::<f64>();
        let mut penalty = |p: f64| sd2 += p * p;
        let mut extrapolated = false;
        let idy = (!sourdough).then_some(dose * idy_form);
        let iota = sourdough.then(|| iota_from_starter(dose, hydration));
        if sourdough {
            if stats.share_below_10 >= 0.5 {
                penalty(0.4);
                extrapolated = true;
            }
            if stats.share_above_30 >= 0.5 {
                penalty(0.3);
                extrapolated = true;
            }
            if let Some(i) = iota
                && !(sourdough::IOTA_DOMAIN.0..=sourdough::IOTA_DOMAIN.1).contains(&i)
            {
                penalty(0.25);
                extrapolated = true;
            }
            if self.starter_state == StarterState::FromFridge {
                penalty(0.2);
            }
        } else {
            if stats.share_below_8 >= 0.5 {
                penalty(0.5);
                extrapolated = true;
            }
            if stats.share_above_35 >= 0.5 {
                penalty(0.4);
                extrapolated = true;
            }
            if let Some(d) = idy
                && !(yeast::DOSE_DOMAIN.0..=yeast::DOSE_DOMAIN.1).contains(&d)
            {
                penalty(0.25);
                extrapolated = true;
            }
        }
        if hours < 0.75 {
            penalty(0.3);
            notes.push(FermentNote::VeryShort);
        }
        let enriched = self.dough.sugar_pct > 10.0 || self.dough.fat_pct > 20.0;
        if enriched {
            penalty(0.25);
        }
        if enriched && self.fridge_h > 0.0 {
            notes.push(FermentNote::RichDoughCold);
        }
        if matches!(
            self.leavening,
            Leavening::InstantYeast | Leavening::ActiveDryYeast | Leavening::FreshYeast
        ) && self.dough.sugar_pct > 10.0
        {
            notes.push(FermentNote::UseOsmotolerant { sugar_pct: self.dough.sugar_pct });
        }
        if extrapolated
            || mods
                .iter()
                .any(|m| m.evidence == Evidence::Extrapolation && (m.factor - 1.0).abs() > 0.02)
        {
            notes.insert(0, FermentNote::Extrapolation);
        }
        let sd = sd2.sqrt();
        Ok(FermentResult {
            hours: band80(hours, sd),
            endpoint_phase,
            dose_pct: dose,
            idy_pct: idy,
            iota,
            schedule,
            modifiers: mods,
            rate_factor,
            sd,
            mean_temp_c: stats.mean_c,
            cooling_tau_h: tau_c,
            notes,
        })
    }
}

/// Dose exponent used to turn the uncertainty of a yeast form's equivalence into time.
const DOSE_EXP_FOR_FORM: f64 = yeast::DOSE_EXP;

/// `dose_for_time` needs a concrete `RateModel` per dose.
struct BoxedRate(Box<dyn RateModel>);

impl RateModel for BoxedRate {
    fn tau_h(&self, temp_c: f64) -> Option<f64> {
        self.0.tau_h(temp_c)
    }
}

/// Starter refresh: seed:flour:water feeding, time to peak, and the grams of a build.
///
/// The same growth law as sourdough dough (`model-fermentation.json`, `starter_peak`): the starter
/// peaks when its culture is back at ripe density, K = 1 (range 0.7–1.5), so
/// τ = ln(1/ι) / (μ_ref · 1.18 · M_h · v · γ_s(T)) + lag, with ι the seed's share of the
/// flour, 1.18 the no-salt factor and M_h = 1 + 0.005 (H − 75) (0.85–1.15) for the starter's
/// own hydration H. It reproduces 6.6 h for a liquid 1:5:5 and 7.4 h for a stiff 1:5:2.75 at 24 °C.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickStarter {
    /// Given the feed, when does it peak (`Time`), or which feed peaks at `target_h` (`Dose`).
    pub solve: Solve,
    /// Fresh flour per part of seed starter.
    pub flour_parts: f64,
    /// Fresh water per part of seed starter (`Time`); in `Dose` mode the water is whatever brings
    /// the refreshed starter to `hydration_pct`.
    pub water_parts: f64,
    pub seed_hydration_pct: f64,
    /// Hydration of the refreshed starter in `Dose` mode, %.
    pub hydration_pct: f64,
    pub temp_c: f64,
    pub target_h: f64,
    pub seed_state: StarterState,
    /// Grams of refreshed starter wanted; 0 = only the timing.
    pub need_g: f64,
    pub vigour: Option<Vigour>,
}

impl Default for QuickStarter {
    fn default() -> Self {
        Self {
            solve: Solve::Time,
            flour_parts: 5.0,
            water_parts: 5.0,
            seed_hydration_pct: 100.0,
            hydration_pct: 100.0,
            temp_c: 24.0,
            target_h: 8.0,
            seed_state: StarterState::Ripe,
            need_g: 0.0,
            vigour: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StarterResult {
    pub hours: Band,
    pub flour_parts: f64,
    pub water_parts: f64,
    /// Hydration of the refreshed starter, %.
    pub hydration_pct: f64,
    /// Seed flour / all flour of the refreshed starter.
    pub iota: f64,
    /// Grams to weigh for `need_g` (seed, flour, water); `None` when `need_g` is 0.
    pub build: Option<(f64, f64, f64)>,
    pub notes: Vec<FermentNote>,
}

/// Spread of K_peak (0.7–1.5) on the log time scale, at a typical growth of ln(1/ι) ≈ 2.5.
const K_PEAK_SD: f64 = 0.30 / 2.5;

impl QuickStarter {
    fn hours(&self, flour: f64, water: f64, v: f64) -> Option<(f64, f64, f64)> {
        let hs = self.seed_hydration_pct / 100.0;
        let (seed_flour, seed_water) = (1.0 / (1.0 + hs), hs / (1.0 + hs));
        let iota = seed_flour / (seed_flour + flour);
        let hydration = (seed_water + water) / (seed_flour + flour) * 100.0;
        let m_h = (1.0 + 0.005 * (hydration - 75.0)).clamp(0.85, 1.15);
        let g = sourdough::gamma(self.temp_c);
        if g <= 0.0 {
            return None;
        }
        let lag =
            if self.seed_state == StarterState::FromFridge { sourdough::FRIDGE_LAG_H } else { 0.0 };
        let tau = (1.0 / iota).ln() / (sourdough::MU_REF * 1.18 * m_h * v * g) + lag / g;
        Some((tau, iota, hydration))
    }

    pub fn solve(&self) -> Result<StarterResult, EngineError> {
        check_range("temp_c", self.temp_c, 2.0, 38.0)?;
        check_range("seed_hydration_pct", self.seed_hydration_pct, 30.0, 300.0)?;
        check_range("need_g", self.need_g, 0.0, 100_000.0)?;
        let vigour = self.vigour.unwrap_or_else(|| Vigour::prior(0.35));
        let v = vigour.factor();
        let unreachable = || EngineError::Invalid {
            field: "temp_c".into(),
            reason: "a starter does not grow at this temperature".into(),
        };
        let mut notes = Vec::new();
        let (flour, water) = match self.solve {
            Solve::Time => {
                check_range("flour_parts", self.flour_parts, 0.2, 100.0)?;
                check_range("water_parts", self.water_parts, 0.0, 300.0)?;
                (self.flour_parts, self.water_parts)
            }
            Solve::Dose => {
                check_range("target_h", self.target_h, 1.0, 72.0)?;
                check_range("hydration_pct", self.hydration_pct, 30.0, 300.0)?;
                // Fresh water so that the whole starter is at `hydration_pct`.
                let hs = self.seed_hydration_pct / 100.0;
                let h = self.hydration_pct / 100.0;
                let water_for = |f: f64| ((h * (1.0 / (1.0 + hs) + f)) - hs / (1.0 + hs)).max(0.0);
                let time_for = |f: f64| self.hours(f, water_for(f), v).map(|x| x.0);
                let (mut lo, mut hi) = (0.2f64.ln(), 100f64.ln());
                let (t_lo, t_hi) = (
                    time_for(0.2).ok_or_else(unreachable)?,
                    time_for(100.0).ok_or_else(unreachable)?,
                );
                let f = if self.target_h <= t_lo {
                    notes.push(FermentNote::NoDoseFits { too_slow: true });
                    0.2
                } else if self.target_h >= t_hi {
                    notes.push(FermentNote::NoDoseFits { too_slow: false });
                    100.0
                } else {
                    for _ in 0..60 {
                        let mid = 0.5 * (lo + hi);
                        if time_for(mid.exp()).ok_or_else(unreachable)? < self.target_h {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    (0.5 * (lo + hi)).exp()
                };
                (f, water_for(f))
            }
        };
        let (hours, iota, hydration) = self.hours(flour, water, v).ok_or_else(unreachable)?;
        let mut sd2 = vigour.log_variance + 0.2 * 0.2 + K_PEAK_SD * K_PEAK_SD;
        sd2 += (0.03 * (self.temp_c - T_REF).abs().min(10.0)).powi(2);
        if self.seed_state == StarterState::FromFridge {
            sd2 += 0.2 * 0.2;
        }
        if self.temp_c < 10.0 || self.temp_c > 30.0 {
            sd2 += 0.3 * 0.3;
            notes.insert(0, FermentNote::Extrapolation);
        }
        let build = (self.need_g > 0.0).then(|| {
            let parts = 1.0 + flour + water;
            let seed = self.need_g / parts;
            (seed, seed * flour, seed * water)
        });
        Ok(StarterResult {
            hours: band80(hours, sd2.sqrt()),
            flour_parts: flour,
            water_parts: water,
            hydration_pct: hydration,
            iota,
            build,
            notes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    fn constant(form: &QuickFerment) -> QuickFerment {
        // Dough at room temperature from the start: isothermal.
        QuickFerment { dough_c: form.room_c, ..form.clone() }
    }

    #[test]
    fn gamma_is_one_at_25_and_zero_at_tmax() {
        close(yeast::gamma(25.0), 1.0, 1e-12);
        close(sourdough::gamma(25.0), 1.0, 1e-12);
        assert_eq!(sourdough::gamma(41.0), 0.0);
        assert!(sourdough::gamma(4.0) < 0.06 && sourdough::gamma(4.0) > 0.01);
        assert!(yeast::gamma(4.0) > sourdough::gamma(4.0));
    }

    #[test]
    fn sourdough_framework_table_7_1_lands_in_its_8_to_12_hour_window() {
        // Recently fed starter: 5 / 10 / 15 % at 30 / 25 / 20 °C, endpoint +50 %, 100 % starter.
        for (t, pct) in [(30.0, 5.0), (25.0, 10.0), (20.0, 15.0)] {
            let form = constant(&QuickFerment {
                dose_pct: pct,
                room_c: t,
                endpoint: Endpoint::Rise50,
                dough: DoughFactors { hydration_pct: 75.0, ..Default::default() },
                ..Default::default()
            });
            let r = form.solve().unwrap();
            assert!((7.5..=12.5).contains(&r.hours.mid_h), "{t} °C {pct} %: {} h", r.hours.mid_h);
        }
    }

    #[test]
    fn yeast_matches_the_dose_law_and_gets_slower_in_the_cold() {
        let base = constant(&QuickFerment {
            leavening: Leavening::InstantYeast,
            dose_pct: 1.0,
            room_c: 25.0,
            endpoint: Endpoint::Doubled,
            dough: DoughFactors { hydration_pct: 65.0, ..Default::default() },
            ..Default::default()
        });
        let r = base.solve().unwrap();
        close(r.hours.mid_h, yeast::A_H, 0.01);
        let quarter = QuickFerment { dose_pct: 0.25, ..base.clone() }.solve().unwrap();
        close(quarter.hours.mid_h / r.hours.mid_h, 4f64.powf(yeast::DOSE_EXP), 0.01);
        let cold = QuickFerment { room_c: 4.0, dough_c: 4.0, ..base.clone() }.solve().unwrap();
        assert!(cold.hours.mid_h > 10.0 * r.hours.mid_h);
        assert!(cold.notes.contains(&FermentNote::Extrapolation));
        // Fresh yeast at 3× the weight of instant is the same dough.
        let fresh = QuickFerment { leavening: Leavening::FreshYeast, dose_pct: 1.0 / 0.33, ..base }
            .solve()
            .unwrap();
        close(fresh.hours.mid_h, r.hours.mid_h, 0.01);
        assert!(fresh.hours.high_h > r.hours.high_h, "conversion uncertainty widens the band");
    }

    #[test]
    fn dose_mode_inverts_time_mode() {
        let form = QuickFerment {
            dose_pct: 12.0,
            room_c: 21.0,
            dough_c: 24.0,
            warm_h: 3.0,
            fridge_h: 12.0,
            endpoint: Endpoint::Rise75,
            ..Default::default()
        };
        let t = form.solve().unwrap();
        let d =
            QuickFerment { solve: Solve::Dose, target_h: t.hours.mid_h, ..form }.solve().unwrap();
        close(d.dose_pct, 12.0, 0.05);
    }

    #[test]
    fn retard_progress_includes_the_cooling_transient() {
        let form = QuickFerment {
            dose_pct: 10.0,
            room_c: 21.0,
            dough_c: 21.0,
            warm_h: 2.0,
            fridge_h: 12.0,
            ..Default::default()
        };
        let r = form.solve().unwrap();
        let fridge = &r.schedule.phases[1];
        let during = fridge.progress - r.schedule.phases[0].progress;
        assert!(during > 0.0);
        // A dough already at fridge temperature would progress far less in those 12 h.
        let cold_rate = 12.0
            / SourdoughRate::new(iota_from_starter(10.0, 100.0), 0.5, r.rate_factor, 0.0)
                .tau_h(4.0)
                .unwrap();
        assert!(during > 1.5 * cold_rate, "{during} vs {cold_rate}");
        assert_eq!(r.endpoint_phase, EndpointPhase::Room);
    }

    #[test]
    fn modifiers_and_notes_follow_the_dough() {
        let sweet = QuickFerment {
            leavening: Leavening::InstantYeast,
            dose_pct: 1.0,
            endpoint: Endpoint::Doubled,
            dough: DoughFactors {
                sugar_pct: 18.0,
                fat_pct: 25.0,
                eggs: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = sweet.solve().unwrap();
        assert!(r.rate_factor < 0.7);
        assert!(r.notes.contains(&FermentNote::UseOsmotolerant { sugar_pct: 18.0 }));
        let osmo = QuickFerment { leavening: Leavening::OsmotolerantYeast, ..sweet.clone() }
            .solve()
            .unwrap();
        assert!(osmo.hours.mid_h < r.hours.mid_h);
        assert!(matches!(
            QuickFerment { endpoint: Endpoint::Rise25, ..sweet }.solve(),
            Err(EngineError::Invalid { .. })
        ));
    }

    #[test]
    fn band_matches_the_model_examples() {
        // Uncalibrated sourdough at 25 °C: factor 1.68 either way (model JSON example).
        let form = constant(&QuickFerment {
            dose_pct: 10.0,
            room_c: 25.0,
            dough: DoughFactors { hydration_pct: 75.0, ..Default::default() },
            ..Default::default()
        });
        let r = form.solve().unwrap();
        close(r.hours.high_h / r.hours.mid_h, 1.68, 0.03);
    }

    #[test]
    fn container_constants_follow_the_scaling_rule() {
        close(Container::Banneton.cooling_tau_h(650.0), 2.8, 1e-9);
        assert!(Container::BareTray.cooling_tau_h(650.0) < Container::Tub.cooling_tau_h(650.0));
        assert!(Container::Tub.cooling_tau_h(2000.0) > Container::Tub.cooling_tau_h(500.0));
    }

    #[test]
    fn starter_peak_reproduces_the_model_reference_points() {
        let liquid = QuickStarter::default().solve().unwrap();
        close(liquid.hours.mid_h, 6.6, 0.05);
        close(liquid.hydration_pct, 100.0, 1e-9);
        let stiff =
            QuickStarter { water_parts: 2.75, seed_hydration_pct: 55.0, ..Default::default() }
                .solve()
                .unwrap();
        close(stiff.hours.mid_h, 7.4, 0.05);
        close(stiff.hydration_pct, 55.0, 0.01);
        // Colder and more diluted: later.
        let slow = QuickStarter {
            flour_parts: 10.0,
            water_parts: 10.0,
            temp_c: 20.0,
            ..Default::default()
        }
        .solve()
        .unwrap();
        assert!(slow.hours.mid_h > liquid.hours.mid_h * 1.5);
    }

    #[test]
    fn starter_ratio_for_a_time_inverts_and_builds_grams() {
        let q = QuickStarter {
            solve: Solve::Dose,
            target_h: 10.0,
            need_g: 150.0,
            ..Default::default()
        };
        let r = q.solve().unwrap();
        close(r.hours.mid_h, 10.0, 0.01);
        close(r.hydration_pct, 100.0, 1e-6);
        let (seed, flour, water) = r.build.unwrap();
        close(seed + flour + water, 150.0, 1e-9);
        close(flour / seed, r.flour_parts, 1e-9);
        let check = QuickStarter {
            flour_parts: r.flour_parts,
            water_parts: r.water_parts,
            ..Default::default()
        }
        .solve()
        .unwrap();
        close(check.hours.mid_h, 10.0, 0.01);
        // At 12 °C even the richest feed (1:0.2) needs about 5 h: 2 h is out of reach.
        let fast =
            QuickStarter { solve: Solve::Dose, target_h: 2.0, temp_c: 12.0, ..Default::default() }
                .solve()
                .unwrap();
        assert_eq!(fast.notes, vec![FermentNote::NoDoseFits { too_slow: true }]);
    }
}
