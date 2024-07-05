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
    degree: f64,
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
            self.degree, unit_from_str, unit_to_str
        )
    }
}

#[wasm_bindgen]
impl Temperature {
    pub fn new() -> Temperature {
        utils::set_panic_hook();

        let degree = 100.0;
        let from_degree = Unit::Celsius;
        let to_degree = Unit::Fahrenheit;

        Temperature { degree, from: from_degree, to: to_degree }
    }

    pub fn convert(&self) -> f64 {
        log!(
            "Converting temperature: degree={}, from={:?}, to={:?}",
            self.degree,
            self.from,
            self.to
        );

        let converted = match (self.from, self.to) {
            (Unit::Celsius, Unit::Fahrenheit) => self.degree * 9.0 / 5.0 + 32.0,
            (Unit::Fahrenheit, Unit::Celsius) => {
                (self.degree - 32.0) * 5.0 / 9.0
            },
            (Unit::Celsius, Unit::Kelvin) => self.degree + 273.15,
            (Unit::Kelvin, Unit::Celsius) => self.degree - 273.15,
            (Unit::Fahrenheit, Unit::Kelvin) => {
                (self.degree - 32.0) * 5.0 / 9.0 + 273.15
            },
            (Unit::Kelvin, Unit::Fahrenheit) => {
                (self.degree - 273.15) * 9.0 / 5.0 + 32.0
            },
            _ => self.degree, // No conversion needed if units are the same
        };

        let multiplier = 10_i8.pow(2) as f64;
        (converted * multiplier).round() / multiplier
    }

    pub fn get_degree(&self) -> f64 {
        self.degree
    }
    /// Set the temparature's degree
    pub fn set_degree(&mut self, degree: f64) {
        self.degree = degree
    }

    pub fn get_from_unit(&self) -> Unit {
        self.from
    }
    /// Set the temparature's from unit
    pub fn set_from_unit(&mut self, unit: Unit) {
        self.from = unit
    }

    pub fn get_to_unit(&self) -> Unit {
        self.to
    }
    /// Set the temparature's to unit
    pub fn set_to_unit(&mut self, unit: Unit) {
        self.to = unit
    }
}
