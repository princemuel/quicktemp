mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro target_scale provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Temperature {
    source_scale: Scale,
    target_scale: Scale,
    value: f64,
    is_valid: bool,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = match self.source_scale {
            Scale::Celsius => "Celsius",
            Scale::Fahrenheit => "Fahrenheit",
            Scale::Kelvin => "Kelvin",
        };

        let target = match self.target_scale {
            Scale::Celsius => "Celsius",
            Scale::Fahrenheit => "Fahrenheit",
            Scale::Kelvin => "Kelvin",
        };

        write!(f, "Temperature: {:.2} {} -> {}", self.value, source, target)
    }
}
impl Default for Temperature {
    fn default() -> Self {
        Self::new()
            .value(100.0)
            .source_scale(Scale::Celsius)
            .target_scale(Scale::Fahrenheit)
    }
}

#[wasm_bindgen]
impl Temperature {
    pub fn new() -> Self {
        utils::set_panic_hook();

        Self {
            value: 100.0,
            source_scale: Scale::Celsius,
            target_scale: Scale::Fahrenheit,
            is_valid: true,
        }
    }

    pub fn build(mut self) -> Result<Temperature, String> {
        if self.value.is_nan() {
            Err("Could not create temperature. Temperature must have:
            1) A valid source scale type
            2) A valid target scale type
            3) A valid temparature degree value"
                .to_string())
        } else {
            self.is_valid = true;
            Ok(self)
        }
    }

    pub fn convert(&self) -> f64 {
        log!(
            "Converting temperature: value={}, source_scale={:?}, target_scale={:?}",
            self.value,
            self.source_scale,
            self.target_scale
        );

        let converted = match (self.source_scale, self.target_scale) {
            (Scale::Celsius, Scale::Fahrenheit) => {
                self.value * 9.0 / 5.0 + 32.0
            },
            (Scale::Fahrenheit, Scale::Celsius) => {
                (self.value - 32.0) * 5.0 / 9.0
            },
            (Scale::Celsius, Scale::Kelvin) => self.value + 273.15,
            (Scale::Kelvin, Scale::Celsius) => self.value - 273.15,
            (Scale::Fahrenheit, Scale::Kelvin) => {
                (self.value - 32.0) * 5.0 / 9.0 + 273.15
            },
            (Scale::Kelvin, Scale::Fahrenheit) => {
                (self.value - 273.15) * 9.0 / 5.0 + 32.0
            },
            _ => self.value, // No conversion needed if scales are the same
        };

        let multiplier = 10_i8.pow(2) as f64;
        (converted * multiplier).round() / multiplier
    }

    /// Set the temparature's value
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self.is_valid = false;
        self
    }

    /// Set the temparature's source scale
    pub fn source_scale(mut self, value: Scale) -> Self {
        self.source_scale = value;
        self.is_valid = false;
        self
    }

    /// Set the temparature's target scale
    pub fn target_scale(mut self, value: Scale) -> Self {
        self.target_scale = value;
        self.is_valid = false;
        self
    }
}
