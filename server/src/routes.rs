use crate::utils;
use actix_web::{http, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Temparature {
    from_scale: String,
    to_scale: String,
    temperature: f64,
}

#[post("/api/v1/convert")]
pub async fn convert(form_data: web::Form<Temparature>) -> impl Responder {
    let temp = utils::convert_temp(
        form_data.temperature,
        &form_data.from_scale,
        &form_data.to_scale,
    );

    let result = match temp.is_nan() {
        true => format!("Conversion Failed"),
        false => format!(
            "{}° {}",
            utils::truncate_decimal_places(temp, 2),
            utils::capitalize_first_letter(&form_data.to_scale)
        ),
    };

    HttpResponse::Ok()
        .content_type(http::header::ContentType::plaintext())
        .body(result)
}
