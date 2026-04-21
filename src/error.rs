use wasm_bindgen::JsError;

#[derive(Debug, thiserror::Error)]
pub enum TempError {
    #[error("invalid temperature. value below absolute zero")]
    BelowAbsoluteZero,
    #[error("invalid temperature. value is not a number")]
    InvalidNumber(#[from] core::num::ParseFloatError),
    #[error("temperature unit is missing")]
    MissingUnit,
    #[error("invalid temperature. value is overflowed")]
    Overflow,
    #[error("invalid temperature. unknown temperature unit")]
    UnknownUnit,
}

impl From<TempError> for wasm_bindgen::JsValue {
    fn from(e: TempError) -> Self { JsError::new(e.to_string().as_str()).into() }
}
