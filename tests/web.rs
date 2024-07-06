//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate quicktemp;
use quicktemp::{Temperature, Unit};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn celsius_to_fahrenheit() {
    let temp =
        Temperature { degree: 0.0, from: Unit::Celsius, to: Unit::Fahrenheit };
    assert_eq!(temp.convert(), 32.0);

    let temp = Temperature {
        degree: 100.0,
        from: Unit::Celsius,
        to: Unit::Fahrenheit,
    };
    assert_eq!(temp.convert(), 212.0);
}

#[wasm_bindgen_test]
fn fahrenheit_to_celsius() {
    let temp =
        Temperature { degree: 32.0, from: Unit::Fahrenheit, to: Unit::Celsius };
    assert_eq!(temp.convert(), 0.0);

    let temp = Temperature {
        degree: 212.0,
        from: Unit::Fahrenheit,
        to: Unit::Celsius,
    };
    assert_eq!(temp.convert(), 100.0);
}

#[wasm_bindgen_test]
fn celsius_to_kelvin() {
    let temp =
        Temperature { degree: 0.0, from: Unit::Celsius, to: Unit::Kelvin };
    assert_eq!(temp.convert(), 273.15);

    let temp =
        Temperature { degree: 100.0, from: Unit::Celsius, to: Unit::Kelvin };
    assert_eq!(temp.convert(), 373.15);
}

#[wasm_bindgen_test]
fn kelvin_to_celsius() {
    let temp =
        Temperature { degree: 273.15, from: Unit::Kelvin, to: Unit::Celsius };
    assert_eq!(temp.convert(), 0.0);

    let temp =
        Temperature { degree: 373.15, from: Unit::Kelvin, to: Unit::Celsius };
    assert_eq!(temp.convert(), 100.0);
}

#[wasm_bindgen_test]
fn fahrenheit_to_kelvin() {
    let temp =
        Temperature { degree: 32.0, from: Unit::Fahrenheit, to: Unit::Kelvin };
    assert_eq!(temp.convert(), 273.15);

    let temp =
        Temperature { degree: 212.0, from: Unit::Fahrenheit, to: Unit::Kelvin };
    assert_eq!(temp.convert(), 373.15);
}

#[wasm_bindgen_test]
fn kelvin_to_fahrenheit() {
    let temp = Temperature {
        degree: 273.15,
        from: Unit::Kelvin,
        to: Unit::Fahrenheit,
    };
    assert_eq!(temp.convert(), 32.0);

    let temp = Temperature {
        degree: 373.15,
        from: Unit::Kelvin,
        to: Unit::Fahrenheit,
    };
    assert_eq!(temp.convert(), 212.0);
}

#[wasm_bindgen_test]
fn nan_value() {
    let mut temp = Temperature::new();
    temp.set_degree(f64::NAN);
    assert_eq!(temp.degree(), 0.0);
    assert_eq!(temp.convert(), 32.0);
}
