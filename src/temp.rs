use crate::error::TemperatureError;
use crate::unit::TemperatureUnit;

pub struct Temp {
    value: f64,
    unit: TemperatureUnit,
}

impl Temp {
    /// Creates a new temperature, validating it's not below absolute zero
    pub const fn new(value: f64, unit: TemperatureUnit) -> Result<Self, TemperatureError> {
        if value < unit.min() {
            return Err(TemperatureError::BelowAbsoluteZero);
        }

        Ok(Self { value, unit })
    }

    pub const fn convert(&self, target: TemperatureUnit) -> Result<f64, TemperatureError> {
        use TemperatureUnit::*;

        let kelvin = match self.unit {
            Celsius => self.value + 273.15,
            Fahrenheit => (self.value + 459.67) * 5.0 / 9.0,
            Kelvin => self.value,
            Rankine => self.value * 5.0 / 9.0,
            Reaumur => self.value * 5.0 / 4.0 + 273.15,
        };
        let result = match target {
            Celsius => kelvin - 273.15,
            Fahrenheit => kelvin * 9.0 / 5.0 - 459.67,
            Kelvin => kelvin,
            Rankine => kelvin * 9.0 / 5.0,
            Reaumur => (kelvin - 273.15) * 4.0 / 5.0,
        };

        if result.is_finite() {
            // let multiplier = 10f64.powi(2);
            let multiplier = 100.0;
            Ok((result * multiplier).round() / multiplier)
        } else {
            Err(TemperatureError::Overflow)
        }
    }

    // #[inline]
    // const fn to_kelvin(&self) -> f64 {

    // }

    // #[inline]
    // const fn from_kelvin(kelvin: f64, target: TemperatureUnit) -> f64 {
    //     use TemperatureUnit::*;

    // }
}
