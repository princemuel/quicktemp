mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[wasm_bindgen]
struct Temperature {
    from: Unit,
    to: Unit,
    value: f64,
}
impl Default for Temperature {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_from_str = match self.from {
            Unit::Celsius => "Celsius",
            Unit::Fahrenheit => "Fahrenheit",
            Unit::Kelvin => "Kelvin",
        };

        let unit_to_str = match self.to {
            Unit::Celsius => "Celsius",
            Unit::Fahrenheit => "Fahrenheit",
            Unit::Kelvin => "Kelvin",
        };

        write!(
            f,
            "Temperature: {:.2} {} -> {}",
            self.value, unit_from_str, unit_to_str
        )
    }
}

#[wasm_bindgen]
impl Temperature {
    pub fn new() -> Temperature {
        utils::set_panic_hook();

        let degree = 100.0;
        let from_value = Unit::Celsius;
        let to_value = Unit::Fahrenheit;

        Temperature { value: degree, from: from_value, to: to_value }
    }

    pub fn convert(&self) -> f64 {
        log!(
            "Converting temperature: value={}, from={:?}, to={:?}",
            self.value,
            self.from,
            self.to
        );

        match (self.from, self.to) {
            (Unit::Celsius, Unit::Fahrenheit) => self.value * 9.0 / 5.0 + 32.0,
            (Unit::Fahrenheit, Unit::Celsius) => {
                (self.value - 32.0) * 5.0 / 9.0
            },
            (Unit::Celsius, Unit::Kelvin) => self.value + 273.15,
            (Unit::Kelvin, Unit::Celsius) => self.value - 273.15,
            (Unit::Fahrenheit, Unit::Kelvin) => {
                (self.value - 32.0) * 5.0 / 9.0 + 273.15
            },
            (Unit::Kelvin, Unit::Fahrenheit) => {
                (self.value - 273.15) * 9.0 / 5.0 + 32.0
            },
            _ => self.value, // No conversion needed if units are the same
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
    /// Set the temparature's degree
    pub fn set_value(&mut self, degree: f64) {
        self.value = degree
    }

    pub fn from(&self) -> Unit {
        self.from
    }
    /// Set the temparature's from unit
    pub fn set_from(&mut self, unit: Unit) {
        self.from = unit
    }

    pub fn to(&self) -> Unit {
        self.to
    }
    /// Set the temparature's to unit
    pub fn set_to(&mut self, unit: Unit) {
        self.to = unit
    }
}
