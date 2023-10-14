use actix_web::{get, web, Responder, Scope};

pub fn routes() -> Scope {
    web::scope("/skills").service(get)
}

#[get("/{id}")]
async fn get(id: web::Path<u64>) -> impl Responder {
    format!("{}", id)
}
