use utoipa::OpenApi;

use crate::routes::*;

#[derive(utoipa::OpenApi)]
#[openapi(servers(
    (url = "http://localhost:8000/api", description = "Dev server"),
))]
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

    doc
}
