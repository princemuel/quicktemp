use actix_cors::Cors;
use actix_web::{http, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

const ORIGIN_URL: &str = "https://quicktemp.vercel.app";
const MAX_CACHE_RESOURCE_TIME: usize = 60 * 60 * 1;

#[derive(Deserialize)]
struct FormData {
    from_scale: String,
    to_scale: String,
    temperature: f64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(ORIGIN_URL)
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b":quicktemp.vercel.app")
            })
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(MAX_CACHE_RESOURCE_TIME);

        App::new().wrap(cors).service(convert)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/api/v1/convert")]
async fn convert(form_data: web::Form<FormData>) -> impl Responder {
    let temp: f64 = convert_temp(
        form_data.temperature,
        &form_data.from_scale,
        &form_data.to_scale,
    );

    let result: String;

    if temp.is_nan() {
        result = format!("Conversion Failed");
    } else {
        result = format!(
            "{}° {}",
            truncate_decimal_places(temp, 2),
            capitalize_first_letter(&form_data.to_scale)
        );
    }

    HttpResponse::Ok()
        .content_type(http::header::ContentType::plaintext())
        .body(result)
}

fn convert_temp(temparature: f64, from_scale: &str, to_scale: &str) -> f64 {
    match from_scale {
        "celsius" => match to_scale {
            "celsius" => temparature,
            "fahrenheit" => (temparature * 9.0 / 5.0) + 32.0,
            "kelvin" => temparature + 273.15,
            _ => std::f64::NAN,
        },
        "fahrenheit" => match to_scale {
            "celsius" => (temparature - 32.0) * 5.0 / 9.0,
            "fahrenheit" => temparature,
            "kelvin" => (temparature - 32.0) * 5.0 / 9.0 + 273.15,
            _ => std::f64::NAN,
        },
        "kelvin" => match to_scale {
            "celsius" => temparature - 273.15,
            "fahrenheit" => (temparature - 273.15) * 9.0 / 5.0 + 32.0,
            "kelvin" => temparature,
            _ => std::f64::NAN,
        },
        _ => std::f64::NAN,
    }
}

fn capitalize_first_letter(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let rest_of_string: String = input.chars().skip(1).collect();
        return first_char.to_uppercase().to_string() + &rest_of_string;
    }
    input.to_owned()
}

fn truncate_decimal_places(value: f64, places: usize) -> f64 {
    let multiplier = 10u64.pow(places as u32) as f64;
    (value * multiplier).round() / multiplier
}
