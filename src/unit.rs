use crate::error::TemperatureError;

#[derive(Clone, Copy)]
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
}
impl TryFrom<&str> for TemperatureUnit {
    type Error = TemperatureError;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let v = v.trim();
        if v.eq_ignore_ascii_case("c") {
            Ok(Self::Celsius)
        } else if v.eq_ignore_ascii_case("f") {
            Ok(Self::Fahrenheit)
        } else if v.eq_ignore_ascii_case("k") {
            Ok(Self::Kelvin)
        } else if v.eq_ignore_ascii_case("r") {
            Ok(Self::Rankin)
        } else if v.eq_ignore_ascii_case("re") {
            Ok(Self::Reaumur)
        } else if v.is_empty() {
            Err(TemperatureError::MissingUnit)
        } else {
            Err(TemperatureError::UnknownUnit)
        }
    }
}
