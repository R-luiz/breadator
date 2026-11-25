//! Water temperature for a desired dough temperature.
//!
//! Two methods:
//! - **Classic** (base temperature, "température de base"): water = n × desired − Σ other
//!   temperatures − friction factor, with n the number of temperature terms (flour, room, water,
//!   and the preferment when there is one). Every term weighs the same, whatever its mass and heat
//!   capacity, so its friction factor is an empirical correction: for a mixing temperature rise
//!   ΔT, FF ≈ n × ΔT.
//! - **Heat balance**: masses × specific heats, with the mixing temperature rise and optional ice
//!   (latent heat of fusion 334 J/g).
//!
//! Specific heats follow Choi & Okos (1986), *Effects of temperature and composition on the
//! thermal properties of foods* (`choi-okos-1986`): the specific heat of a food is the mass-weighted
//! sum of its components' specific heats, each a quadratic function of temperature.

use serde::{Deserialize, Serialize};

use crate::check_range;
use crate::error::{EngineError, Warning};
use crate::ingredients::IngredientKind;

/// Latent heat of fusion of ice, J/g.
pub const ICE_LATENT_HEAT: f64 = 334.0;
/// Above this, water poured onto yeast damages it.
const YEAST_SAFE_WATER_C: f64 = 40.0;

/// Mass fractions of a food's components (Choi & Okos 1986 categories).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Proximate {
    pub water: f64,
    pub protein: f64,
    pub fat: f64,
    pub carbohydrate: f64,
    pub fiber: f64,
    pub ash: f64,
}

/// Specific heat (J/g·K) at `t_c` from composition, Choi & Okos (1986) above 0 °C.
pub fn specific_heat(p: &Proximate, t_c: f64) -> f64 {
    let t = t_c;
    let water = 4.1762 - 9.0864e-5 * t + 5.4731e-6 * t * t;
    let protein = 2.0082 + 1.2089e-3 * t - 1.3129e-6 * t * t;
    let fat = 1.9842 + 1.4733e-3 * t - 4.8008e-6 * t * t;
    let carbohydrate = 1.5488 + 1.9625e-3 * t - 5.9399e-6 * t * t;
    let fiber = 1.8459 + 1.8306e-3 * t - 4.6509e-6 * t * t;
    let ash = 1.0926 + 1.8896e-3 * t - 3.6817e-6 * t * t;
    p.water * water
        + p.protein * protein
        + p.fat * fat
        + p.carbohydrate * carbohydrate
        + p.fiber * fiber
        + p.ash * ash
}

/// Typical composition of a wheat flour at 14 % moisture (the trade reference moisture) from its
/// protein content; fibre and ash grow with the extraction rate (`whole` = whole grain).
pub fn flour_proximate(protein_pct: f64, whole: bool) -> Proximate {
    let (fat, fiber, ash) = if whole { (0.025, 0.11, 0.016) } else { (0.015, 0.03, 0.006) };
    let water = 0.14;
    let protein = protein_pct / 100.0;
    Proximate {
        water,
        protein,
        fat,
        fiber,
        ash,
        carbohydrate: (1.0 - water - protein - fat - fiber - ash).max(0.0),
    }
}

/// Typical composition of liquid and enriching ingredients (USDA FoodData Central, SR Legacy).
/// Inclusions and other ingredients have no single composition: they are treated as dry plant
/// matter (carbohydrate), close enough for their small share of a dough's heat capacity.
pub fn ingredient_proximate(kind: IngredientKind) -> Proximate {
    let p = |water, protein, fat, carbohydrate, ash| Proximate {
        water,
        protein,
        fat,
        carbohydrate,
        fiber: 0.0,
        ash,
    };
    match kind {
        IngredientKind::Water => p(1.0, 0.0, 0.0, 0.0, 0.0),
        IngredientKind::Milk => p(0.88, 0.032, 0.033, 0.048, 0.007),
        IngredientKind::Cream => p(0.58, 0.028, 0.36, 0.027, 0.005),
        IngredientKind::Yogurt => p(0.88, 0.035, 0.033, 0.047, 0.007),
        IngredientKind::Beer => p(0.92, 0.005, 0.0, 0.036, 0.002),
        IngredientKind::Egg => p(0.76, 0.126, 0.095, 0.007, 0.011),
        IngredientKind::EggYolk => p(0.52, 0.159, 0.265, 0.036, 0.017),
        IngredientKind::EggWhite => p(0.876, 0.109, 0.002, 0.007, 0.006),
        IngredientKind::Butter => p(0.16, 0.009, 0.81, 0.001, 0.021),
        IngredientKind::Oil => p(0.0, 0.0, 1.0, 0.0, 0.0),
        IngredientKind::Sugar => p(0.0, 0.0, 0.0, 1.0, 0.0),
        IngredientKind::Honey => p(0.17, 0.003, 0.0, 0.824, 0.002),
        IngredientKind::Salt => p(0.0, 0.0, 0.0, 0.0, 1.0),
        IngredientKind::FreshYeast => p(0.69, 0.084, 0.019, 0.183, 0.018),
        IngredientKind::ActiveDryYeast
        | IngredientKind::InstantYeast
        | IngredientKind::OsmotolerantYeast => p(0.05, 0.40, 0.076, 0.41, 0.064),
        IngredientKind::Inclusion | IngredientKind::Other => p(0.0, 0.0, 0.0, 1.0, 0.0),
    }
}

/// One ingredient (or premixed stage) entering the mixer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub grams: f64,
    pub temp_c: f64,
    /// J/(g·K); see [`specific_heat`].
    pub specific_heat: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum WaterTempInput {
    Classic {
        desired_dough_c: f64,
        flour_c: f64,
        room_c: f64,
        /// Preferment or levain temperature: adds a fourth term.
        #[serde(default)]
        preferment_c: Option<f64>,
        /// Friction factor on the classic scale (°C summed over the terms).
        friction_factor_c: f64,
    },
    HeatBalance {
        desired_dough_c: f64,
        /// Water of the final mix, g.
        water_g: f64,
        /// Everything else in the final mix (flours, preferments, milk, eggs, butter…).
        components: Vec<Component>,
        /// Temperature rise produced by mixing, °C.
        mixing_rise_c: f64,
        /// Coldest water available; when the required water is colder, part of it becomes ice.
        #[serde(default)]
        tap_c: Option<f64>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WaterTempResult {
    /// Temperature the water must have (may be below 0: ice is then required).
    pub water_c: f64,
    /// With ice: ice at 0 °C and water at `tap_c` adding up to the mix water.
    pub ice_g: Option<f64>,
    pub tap_water_g: Option<f64>,
    /// Dough temperature expected after mixing with the proposed water/ice.
    pub expected_dough_c: f64,
    pub warnings: Vec<Warning>,
}

pub fn water_temperature(input: &WaterTempInput) -> Result<WaterTempResult, EngineError> {
    match input {
        WaterTempInput::Classic {
            desired_dough_c,
            flour_c,
            room_c,
            preferment_c,
            friction_factor_c,
        } => {
            check_range("desired_dough_c", *desired_dough_c, -10.0, 60.0)?;
            check_range("flour_c", *flour_c, -30.0, 60.0)?;
            check_range("room_c", *room_c, -10.0, 60.0)?;
            check_range("friction_factor_c", *friction_factor_c, -20.0, 100.0)?;
            let mut n = 3.0;
            let mut others = flour_c + room_c;
            if let Some(p) = preferment_c {
                check_range("preferment_c", *p, -10.0, 60.0)?;
                n += 1.0;
                others += p;
            }
            let water_c = n * desired_dough_c - others - friction_factor_c;
            let mut warnings = Vec::new();
            push_water_warnings(water_c, &mut warnings);
            Ok(WaterTempResult {
                water_c,
                ice_g: None,
                tap_water_g: None,
                expected_dough_c: *desired_dough_c,
                warnings,
            })
        }
        WaterTempInput::HeatBalance {
            desired_dough_c,
            water_g,
            components,
            mixing_rise_c,
            tap_c,
        } => heat_balance(*desired_dough_c, *water_g, components, *mixing_rise_c, *tap_c),
    }
}

fn heat_balance(
    desired_dough_c: f64,
    water_g: f64,
    components: &[Component],
    mixing_rise_c: f64,
    tap_c: Option<f64>,
) -> Result<WaterTempResult, EngineError> {
    check_range("desired_dough_c", desired_dough_c, -10.0, 60.0)?;
    check_range("water_g", water_g, 1e-6, 1.0e6)?;
    check_range("mixing_rise_c", mixing_rise_c, -10.0, 30.0)?;
    // A dough has a handful of components; the list can come from anyone (the public API).
    if components.len() > 32 {
        return Err(EngineError::Invalid {
            field: "components".into(),
            reason: "more than 32 entries".into(),
        });
    }
    for (i, c) in components.iter().enumerate() {
        check_range(&format!("components[{i}].grams"), c.grams, 0.0, 1.0e6)?;
        check_range(&format!("components[{i}].temp_c"), c.temp_c, -30.0, 100.0)?;
        check_range(&format!("components[{i}].specific_heat"), c.specific_heat, 0.1, 5.0)?;
    }
    let cw = 4.18;
    let t_pre = desired_dough_c - mixing_rise_c; // dough temperature before mixing heat
    // Σ m·c·(T − T_pre) of the non-water components.
    let others: f64 =
        components.iter().map(|c| c.grams * c.specific_heat * (c.temp_c - t_pre)).sum();
    let water_c = t_pre - others / (water_g * cw);

    let mut warnings = Vec::new();
    let Some(tap) = tap_c.filter(|&tap| water_c < tap) else {
        push_water_warnings(water_c, &mut warnings);
        return Ok(WaterTempResult {
            water_c,
            ice_g: None,
            tap_water_g: None,
            expected_dough_c: desired_dough_c,
            warnings,
        });
    };
    check_range("tap_c", tap, 0.0, 40.0)?;
    // Ice at 0 °C melts (L) then warms to T_pre; tap water cools from `tap` to T_pre:
    // others + (m_w − m_i)·c_w·(tap − T_pre) − m_i·(L + c_w·T_pre) = 0
    let ice_g = (others + water_g * cw * (tap - t_pre)) / (ICE_LATENT_HEAT + cw * tap);
    if ice_g <= water_g {
        return Ok(WaterTempResult {
            water_c,
            ice_g: Some(ice_g.max(0.0)),
            tap_water_g: Some(water_g - ice_g.max(0.0)),
            expected_dough_c: desired_dough_c,
            warnings,
        });
    }
    // All the water as ice is not enough: report the coldest reachable dough.
    let heat: f64 = components.iter().map(|c| c.grams * c.specific_heat).sum();
    let weighted: f64 = components.iter().map(|c| c.grams * c.specific_heat * c.temp_c).sum();
    let best_pre = (weighted - water_g * ICE_LATENT_HEAT) / (heat + water_g * cw);
    let best_dough_c = best_pre + mixing_rise_c;
    warnings.push(Warning::TargetUnreachable { best_dough_c });
    Ok(WaterTempResult {
        water_c,
        ice_g: Some(water_g),
        tap_water_g: Some(0.0),
        expected_dough_c: best_dough_c,
        warnings,
    })
}

fn push_water_warnings(water_c: f64, warnings: &mut Vec<Warning>) {
    if water_c < 0.5 {
        warnings.push(Warning::IceNeeded { water_c });
    } else if water_c > YEAST_SAFE_WATER_C {
        warnings.push(Warning::WaterHotForYeast { water_c });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickMethod {
    Balance,
    Classic,
}

/// The water-temperature calculator's simple form: flour, an optional preferment, mixing rise.
/// Shared by the server (no-JavaScript form) and the browser island.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickTemperature {
    pub method: QuickMethod,
    pub desired: f64,
    pub flour_c: f64,
    pub room_c: f64,
    pub flour_g: f64,
    pub water_g: f64,
    /// Levain or preferment mass, g (0 = none).
    pub pref_g: f64,
    pub pref_c: f64,
    pub pref_hyd: f64,
    /// Mixing temperature rise (heat balance), °C.
    pub rise: f64,
    /// Friction factor (classic rule), °C.
    pub friction: f64,
    pub tap_c: Option<f64>,
}

impl Default for QuickTemperature {
    fn default() -> Self {
        Self {
            method: QuickMethod::Balance,
            desired: 24.0,
            flour_c: 20.0,
            room_c: 20.0,
            flour_g: 500.0,
            water_g: 350.0,
            pref_g: 0.0,
            pref_c: 20.0,
            pref_hyd: 100.0,
            rise: 0.0,
            friction: 0.0,
            tap_c: None,
        }
    }
}

impl QuickTemperature {
    pub fn to_input(&self) -> WaterTempInput {
        if self.method == QuickMethod::Classic {
            return WaterTempInput::Classic {
                desired_dough_c: self.desired,
                flour_c: self.flour_c,
                room_c: self.room_c,
                preferment_c: (self.pref_g > 0.0).then_some(self.pref_c),
                friction_factor_c: self.friction,
            };
        }
        let cp_flour = specific_heat(&flour_proximate(11.0, false), self.flour_c);
        let mut components = vec![Component {
            id: "flour".into(),
            grams: self.flour_g,
            temp_c: self.flour_c,
            specific_heat: cp_flour,
        }];
        if self.pref_g > 0.0 {
            // A preferment is flour and water at its hydration.
            let h = self.pref_hyd.max(0.0) / 100.0;
            let cp = (cp_flour
                + specific_heat(&ingredient_proximate(IngredientKind::Water), self.pref_c) * h)
                / (1.0 + h);
            components.push(Component {
                id: "preferment".into(),
                grams: self.pref_g,
                temp_c: self.pref_c,
                specific_heat: cp,
            });
        }
        WaterTempInput::HeatBalance {
            desired_dough_c: self.desired,
            water_g: self.water_g,
            components,
            mixing_rise_c: self.rise,
            tap_c: self.tap_c,
        }
    }
}
