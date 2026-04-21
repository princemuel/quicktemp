use wasm_bindgen::prelude::wasm_bindgen;

use crate::error::TempError;
use crate::unit::TempUnit;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Temp {
    degree: f64,
    unit: TempUnit,
}

#[wasm_bindgen]
impl Temp {
    pub fn convert(value: &str, unit: &str, target: &str) -> Result<Temp, TempError> {
        let temp = (value.trim().parse()?, unit.parse()?).try_into()?;
        let unit = target.parse()?;
        let degree = Degree::try_from((&temp, unit))?.into();
        Ok(Self { degree, unit })
    }

    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn degree(&self) -> f64 { self.degree }

    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> TempUnit { self.unit }
}

impl TryFrom<(f64, TempUnit)> for Temp {
    type Error = TempError;

    fn try_from((degree, unit): (f64, TempUnit)) -> Result<Self, Self::Error> {
        if degree < unit.min() {
            return Err(TempError::BelowAbsoluteZero);
        }
        Ok(Self { degree, unit })
    }
}

struct Degree(f64);
impl TryFrom<(&Temp, TempUnit)> for Degree {
    type Error = TempError;

    fn try_from((temp, target): (&Temp, TempUnit)) -> Result<Self, Self::Error> {
        use TempUnit::{Celsius, Fahrenheit, Kelvin, Rankine, Reaumur};

        let kelvin = match temp.unit {
            Celsius => temp.degree + 273.15,
            Fahrenheit => (temp.degree + 459.67) * 5.0 / 9.0,
            Kelvin => temp.degree,
            Rankine => temp.degree * 5.0 / 9.0,
            Reaumur => temp.degree * 5.0 / 4.0 + 273.15,
            TempUnit::__Invalid => 0.0,
        };

        let result = match target {
            Celsius => kelvin - 273.15,
            Fahrenheit => kelvin * 9.0 / 5.0 - 459.67,
            Kelvin => kelvin,
            Rankine => kelvin * 9.0 / 5.0,
            Reaumur => (kelvin - 273.15) * 4.0 / 5.0,
            TempUnit::__Invalid => 0.0,
        };

        if result.is_finite() {
            Ok(Self((result * 100.0).round() / 100.0))
        } else {
            Err(TempError::Overflow)
        }
    }
}

impl From<Degree> for f64 {
    fn from(c: Degree) -> Self { c.0 }
}
