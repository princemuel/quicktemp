use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello_world).service(convert_temp))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/api/v1/convert")]
async fn convert_temp() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
