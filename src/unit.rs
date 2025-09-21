use crate::error::TemperatureError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankin,
    Reaumur,
}
impl TemperatureUnit {
    pub const fn min(self) -> f64 {
        match self {
            Self::Celsius => -273.15,
            Self::Fahrenheit => -459.67,
            Self::Kelvin => 0.0,
            Self::Rankin => 0.0,
            Self::Reaumur => -218.52,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Celsius => "C",
            Self::Fahrenheit => "F",
            Self::Kelvin => "K",
            Self::Rankin => "R",
            Self::Reaumur => "Re",
        }
    }
}
impl TryFrom<&str> for TemperatureUnit {
    type Error = TemperatureError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "c" => Ok(Self::Celsius),
            "f" => Ok(Self::Fahrenheit),
            "k" => Ok(Self::Kelvin),
            "r" => Ok(Self::Rankin),
            "re" => Ok(Self::Reaumur),
            "" => Err(TemperatureError::MissingUnit),
            _ => Err(TemperatureError::UnknownUnit),
        }
    }
}
impl From<TemperatureUnit> for &str {
    fn from(value: TemperatureUnit) -> Self { value.as_str() }
}
