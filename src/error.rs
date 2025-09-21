#[derive(Debug, serde::Serialize)]
pub enum TemperatureError {
    InvalidNumber,
    MissingInput,
    BelowAbsoluteZero,
    UnknownUnit,
    MissingUnit,
    Overflow,
}

impl core::fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidNumber => write!(f, "Invalid number: could not parse input."),
            Self::MissingInput => write!(f, "Missing input: no temperature value provided."),
            Self::BelowAbsoluteZero => write!(f, "Temperature below absolute zero is invalid."),
            Self::UnknownUnit => write!(f, "Unknown unit: use °C, °F, or K."),
            Self::MissingUnit => write!(f, "Unit missing: specify °C, °F, or K."),
            Self::Overflow => write!(f, "Number overflow: input too large."),
        }
    }
}

impl core::error::Error for TemperatureError {}

pub type TempResult<T> = core::result::Result<T, TemperatureError>;
