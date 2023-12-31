use std::io;

use actix_web::{
    middleware::{Compress, Logger, NormalizePath},
    web::{self, JsonConfig, QueryConfig, ServiceConfig},
    App, HttpResponse, HttpServer,
};
use actix_web_grants::GrantsMiddleware;
use backend::*;
use env_logger::Env;
use error::JsonError;
use routes::*;
use utoipa_redoc::{Redoc, Servable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    if let Err(e) = backend::auth::initialize_jwt() {
        return Err(io::Error::new(io::ErrorKind::Other, e));
    }

    let pool = database::create_pool();

    database::run_migrations(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to run migrations");

    HttpServer::new(move || {
        let app = App::new()
            .configure(json_config)
            .configure(query_config)
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("\"%r\" -> %s in %D ms"))
            .wrap(NormalizePath::trim())
            .wrap(Compress::default())
            .wrap(GrantsMiddleware::with_extractor(
                backend::auth::extract_permissions,
            ));

        #[cfg(feature = "cors")]
        let app = app.wrap(actix_cors::Cors::permissive());

        app.service(Redoc::with_url("/doc", documentation::doc()))
            .service(
                web::scope("/api")
                    .service(skills::routes())
                    .service(centers::routes())
                    .service(mission_types::routes())
                    .service(nurses::routes())
                    .service(patients::routes())
                    .service(missions::routes())
                    .service(visits::routes())
                    .service(managers::routes())
                    .service(zones::routes())
                    .service(routes::auth::routes())
                    .service(version::routes()),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

/// Configures the [Json](actix_web::web::Json) extractor error response to be JSON.
fn json_config(app: &mut ServiceConfig) {
    let json_config = JsonConfig::default().error_handler(|err, _| {
        actix_web::error::InternalError::from_response(
            "",
            HttpResponse::BadRequest()
                .content_type("Content-Type: application/json")
                .body(serde_json::to_string(&JsonError::new(err.to_string())).unwrap()),
        )
        .into()
    });

    app.app_data(json_config);
}

/// Configures the [Query](actix_web::web::Query) extractor error response to be JSON.
fn query_config(app: &mut ServiceConfig) {
    let query_config = QueryConfig::default().error_handler(|err, _| {
        actix_web::error::InternalError::from_response(
            "",
            HttpResponse::BadRequest()
                .content_type("Content-Type: application/json")
                .body(serde_json::to_string(&JsonError::new(err.to_string())).unwrap()),
        )
        .into()
    });

    app.app_data(query_config);
}
