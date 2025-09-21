use crate::error::TemperatureError;
use crate::unit::TemperatureUnit;

pub struct Temp {
    value: f64,
    unit: TemperatureUnit,
}

impl Temp {
    pub fn new(value: f64, unit: TemperatureUnit) -> Self {
        Self { value, unit }
    }

    pub fn convert(&self, target: TemperatureUnit) -> Result<f64, TemperatureError> {
        let kelvin = match self.unit {
            TemperatureUnit::Celsius => self.value + 273.15,
            TemperatureUnit::Fahrenheit => (self.value + 459.67) * 5.0 / 9.0,
            TemperatureUnit::Kelvin => self.value,
            TemperatureUnit::Rankin => self.value * 5.0 / 9.0,
            TemperatureUnit::Reaumur => self.value * 5.0 / 4.0 + 273.15,
        };

        let result = match target {
            TemperatureUnit::Celsius => kelvin - 273.15,
            TemperatureUnit::Fahrenheit => kelvin * 9.0 / 5.0 - 459.67,
            TemperatureUnit::Kelvin => kelvin,
            TemperatureUnit::Rankin => kelvin * 9.0 / 5.0,
            TemperatureUnit::Reaumur => (kelvin - 273.15) * 4.0 / 5.0,
        };

        if result.is_finite() {
            Ok((result * 100.0).round() / 100.0) // round to 2 decimals
        } else {
            Err(TemperatureError::Overflow)
        }
    }
}
