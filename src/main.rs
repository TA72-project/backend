use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use utoipa_redoc::{Redoc, Servable};

mod database;
mod documentation;
mod error;
mod models;
mod pagination;
mod routes;
mod schema;

use env_logger::Env;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = database::create_pool();

    database::run_migrations(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("\"%r\" -> %s in %D ms"))
            .wrap(NormalizePath::trim())
            .service(Redoc::with_url("/doc", documentation::doc()))
            .service(
                web::scope("/api")
                    .service(skills::routes())
                    .service(version::routes()),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
