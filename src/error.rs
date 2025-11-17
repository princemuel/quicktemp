use wasm_bindgen::{JsError, JsValue};

#[derive(Debug)]
pub enum TemperatureError {
    BelowAbsoluteZero,
    InvalidNumber,
    MissingUnit,
    Overflow,
    UnknownUnit,
}

impl core::fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::BelowAbsoluteZero => "Temperature below absolute zero is invalid",
            Self::InvalidNumber => "Invalid number",
            Self::MissingUnit => "Unit missing",
            Self::Overflow => "Number overflow",
            Self::UnknownUnit => "Unknown unit",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for TemperatureError {}

impl From<TemperatureError> for JsValue {
    fn from(err: TemperatureError) -> JsValue {
        JsError::new(&err.to_string()).into()
    }
}
