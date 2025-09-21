use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::error::TemperatureError;
use crate::temp::TemperatureBuilder;
use crate::unit::TemperatureUnit;

#[wasm_bindgen]
pub fn convert(value: &str, unit: &str, target: &str) -> Result<f64, JsValue> {
    // Parse numeric input
    let value: f64 = value
        .trim()
        .parse()
        .map_err(|_| to_value(&TemperatureError::InvalidNumber).unwrap())?;

    // Parse units
    let unit = TemperatureUnit::try_from(unit).map_err(|e| to_value(&e).unwrap())?;
    let target = TemperatureUnit::try_from(target).map_err(|e| to_value(&e).unwrap())?;

    let temp = TemperatureBuilder::new()
        .value(value)
        .unit(unit)
        .build()
        .map_err(|e| to_value(&e).unwrap())?;

    // Convert and return
    temp.convert(target).map_err(|e| to_value(&e).unwrap())
}
