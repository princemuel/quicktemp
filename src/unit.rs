use core::str::FromStr;

use crate::error::TemperatureError;

/// Temperature units supported by the converter
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
}

impl TemperatureUnit {
    /// Returns the absolute zero value for this unit
    pub const fn min(self) -> f64 {
        match self {
            Self::Celsius => -273.15,
            Self::Fahrenheit => -459.67,
            Self::Kelvin => 0.0,
            Self::Rankine => 0.0,
            Self::Reaumur => -218.5199999999999,
        }
    }
}

impl FromStr for TemperatureUnit {
    type Err = TemperatureError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        if value.is_empty() {
            return Err(TemperatureError::MissingUnit);
        }

        match value.as_bytes() {
            [b'c' | b'C'] => Ok(Self::Celsius),
            [b'f' | b'F'] => Ok(Self::Fahrenheit),
            [b'k' | b'K'] => Ok(Self::Kelvin),
            [b'r' | b'R'] => Ok(Self::Rankine),
            [b'r' | b'R', b'e' | b'E'] => Ok(Self::Reaumur),
            _ => Err(TemperatureError::UnknownUnit),
        }
    }
}

// impl TryFrom<&str> for TemperatureUnit {
//     type Error = TemperatureError;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         value.parse()
//     }
// }

// impl AsRef<str> for TemperatureUnit {
//     fn as_ref(&self) -> &str {
//         match *self {
//             Self::Celsius => "c",
//             Self::Fahrenheit => "f",
//             Self::Kelvin => "k",
//             Self::Rankine => "r",
//             Self::Reaumur => "re",
//         }
//     }
// }
