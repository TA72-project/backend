use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::routes::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    servers(
        (url = "http://localhost:8000/api", description = "Dev server"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub fn doc() -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();

    doc.merge(skills::Doc::openapi());
    doc.merge(centers::Doc::openapi());
    doc.merge(mission_types::Doc::openapi());
    doc.merge(nurses::Doc::openapi());
    doc.merge(patients::Doc::openapi());
    doc.merge(missions::Doc::openapi());
    doc.merge(visits::Doc::openapi());
    doc.merge(managers::Doc::openapi());
    doc.merge(auth::Doc::openapi());
    doc.merge(zones::Doc::openapi());

    SecurityAddon.modify(&mut doc);

    doc
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // we can unwrap safely since there already is components registered.
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new(
                    crate::auth::COOKIE_TOKEN_NAME,
                ))),
            )
        }
    }
}
