use actix_web::Responder;

pub fn handle(body: String) -> impl Responder {
    format!("body: {}", body)
}
