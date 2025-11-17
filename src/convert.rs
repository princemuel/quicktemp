use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::error::TemperatureError;
use crate::temp::Temp;

#[wasm_bindgen]
pub fn convert(value: &str, unit: &str, target: &str) -> Result<f64, JsValue> {
    // Parse number
    let value = value
        .trim()
        .parse()
        .map_err(|_| JsValue::from(TemperatureError::InvalidNumber))?;

    // Parse units
    let unit = unit.parse().map_err(JsValue::from)?;
    let target = target.parse().map_err(JsValue::from)?;

    // Validate and convert
    let temp = Temp::new(value, unit).map_err(JsValue::from)?;
    temp.convert(target).map_err(JsValue::from)
}
