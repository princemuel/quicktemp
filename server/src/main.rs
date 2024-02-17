mod routes;
mod utils;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

const MAX_CACHE_RESOURCE_TIME: usize = 60 * 60 * 2;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", utils::get_env_value("0.0.0.0", "127.0.0.1"));
    HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allowed_origin(&get_env_value(
        //         "https://quicktemp.vercel.app",
        //         "http://127.0.0.1:3000",
        //     ))
        //     .allowed_origin_fn(|origin, _req_head| {
        //         origin.as_bytes().ends_with(b":quicktemp.vercel.app")
        //     })
        //     .allowed_methods(vec!["POST"])
        //     .allowed_headers(vec![http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(MAX_CACHE_RESOURCE_TIME);

        let cors = Cors::permissive()
            .allowed_methods(vec!["POST"])
            .max_age(MAX_CACHE_RESOURCE_TIME);

        App::new().wrap(cors).service(routes::convert)
    })
    .bind((utils::get_env_value("0.0.0.0", "127.0.0.1"), 8080))?
    .run()
    .await
}
