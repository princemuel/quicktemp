use std::f64;

pub fn convert_temp(t: f64, from: &str, to: &str) -> f64 {
    match from {
        "celsius" => match to {
            "celsius" => t,
            "fahrenheit" => (t * 9.0 / 5.0) + 32.0,
            "kelvin" => t + 273.15,
            _ => f64::NAN,
        },
        "fahrenheit" => match to {
            "celsius" => (t - 32.0) * 5.0 / 9.0,
            "fahrenheit" => t,
            "kelvin" => (t - 32.0) * 5.0 / 9.0 + 273.15,
            _ => f64::NAN,
        },
        "kelvin" => match to {
            "celsius" => t - 273.15,
            "fahrenheit" => (t - 273.15) * 9.0 / 5.0 + 32.0,
            "kelvin" => t,
            _ => f64::NAN,
        },
        _ => f64::NAN,
    }
}

pub fn capitalize_first_letter(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let rest_of_string: String = input.chars().skip(1).collect();
        return first_char.to_uppercase().to_string() + &rest_of_string;
    }
    input.to_owned()
}

pub fn truncate_decimal_places(value: f64, places: usize) -> f64 {
    let multiplier = 10_i8.pow(places as u32) as f64;
    (value * multiplier).round() / multiplier
}

pub fn get_env_value(prod_value: &str, dev_value: &str) -> String {
    match std::env::var("DEBUG") {
        Ok(value) if value == "true" => dev_value.to_string(),
        _ => prod_value.to_string(),
    }
}
