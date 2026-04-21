use wasm_bindgen::prelude::wasm_bindgen;

use crate::error::TempError;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum TempUnit {
    Celsius = "°C",
    Fahrenheit = "°F",
    Kelvin = "°K",
    Rankine = "°R",
    Reaumur = "°Re",
}
impl TempUnit {
    pub(crate) const fn min(self) -> f64 {
        match self {
            Self::Celsius => -273.15,
            Self::Fahrenheit => -459.67,
            Self::Kelvin | Self::Rankine | Self::__Invalid => 0.0,
            Self::Reaumur => -218.519_999_999_999_9,
        }
    }
}

impl core::str::FromStr for TempUnit {
    type Err = TempError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match *value.trim().as_bytes() {
            [b'c' | b'C'] => Ok(Self::Celsius),
            [b'f' | b'F'] => Ok(Self::Fahrenheit),
            [b'k' | b'K'] => Ok(Self::Kelvin),
            [b'r' | b'R'] => Ok(Self::Rankine),
            [b'r' | b'R', b'e' | b'E'] => Ok(Self::Reaumur),
            [] => Err(TempError::MissingUnit),
            _ => Err(TempError::UnknownUnit),
        }
    }
}
