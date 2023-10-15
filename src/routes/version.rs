//! Contains the routes returning the API version.

use actix_web::{web, Responder, Scope};

/// API version related routes
pub fn routes() -> Scope {
    web::scope("")
        .route("", web::get().to(version))
        .route("/version", web::get().to(version))
}

async fn version() -> impl Responder {
    env!("CARGO_PKG_VERSION")
}
