use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::error::TemperatureError;
use crate::temp::Temp;
use crate::unit::TemperatureUnit;

#[wasm_bindgen]
pub fn convert(value: &str, unit: &str, target: &str) -> Result<f64, JsValue> {
    // Parse number
    let value: f64 = value
        .trim()
        .parse::<f64>()
        .map_err(|_| JsValue::from(TemperatureError::InvalidNumber))?;

    // Parse units
    let unit = TemperatureUnit::try_from(unit).map_err(JsValue::from)?;
    let target = TemperatureUnit::try_from(target).map_err(JsValue::from)?;

    // Validate and convert
    if value < unit.min() {
        return Err(JsValue::from(TemperatureError::BelowAbsoluteZero));
    }

    Temp::new(value, unit).convert(target).map_err(JsValue::from)
}
