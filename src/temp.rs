use crate::error::{TempResult, TemperatureError};
use crate::unit::TemperatureUnit;

pub struct Temp {
    value: f64,
    unit:  TemperatureUnit,
}

impl Temp {
    pub fn convert(&self, target: TemperatureUnit) -> TempResult<f64> {
        // Convert unit to Kelvin
        let kelvin = match self.unit {
            TemperatureUnit::Celsius => self.value + 273.15,
            TemperatureUnit::Fahrenheit => (self.value + 459.67) * (5.0 / 9.0),
            TemperatureUnit::Kelvin => self.value,
            TemperatureUnit::Rankin => self.value * (5.0 / 9.0),
            TemperatureUnit::Reaumur => self.value * (5.0 / 4.0) + 273.15,
        };

        // Convert Kelvin to target
        let result = match target {
            TemperatureUnit::Celsius => kelvin - 273.15,
            TemperatureUnit::Fahrenheit => (kelvin * 9.0 / 5.0) - 459.67,
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

#[derive(Default, Debug)]
pub struct TemperatureBuilder {
    value: Option<f64>,
    unit:  Option<TemperatureUnit>,
}

impl TemperatureBuilder {
    pub fn new() -> Self { Self::default() }

    /// Set the temparature's value
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Set the temparature's target unit
    pub fn unit(mut self, unit: TemperatureUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn build(self) -> TempResult<Temp> {
        let value = self.value.ok_or(TemperatureError::MissingInput)?;
        let unit = self.unit.ok_or(TemperatureError::MissingUnit)?;

        if value < unit.min() {
            return Err(TemperatureError::BelowAbsoluteZero);
        }

        Ok(Temp { value, unit })
    }
}
