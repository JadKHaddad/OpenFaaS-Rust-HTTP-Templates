use actix_web::{post, web, App, Error, HttpServer, Responder};
use std::str;

use handler;

#[post("/")]
async fn handler_service(body: web::Bytes) -> Result<impl Responder, Error> {
    let string_body = str::from_utf8(&body)?.to_owned();
    Ok(handler::handle(string_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(handler_service))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
