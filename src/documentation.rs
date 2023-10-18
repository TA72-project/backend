use utoipa::OpenApi;

use crate::routes::*;

#[derive(utoipa::OpenApi)]
#[openapi(servers(
    (url = "http://localhost:8000/api", description = "Dev server"),
))]
pub struct ApiDoc;

pub fn doc() -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();

    doc.merge(skills::SkillDoc::openapi());
    doc.merge(centers::CenterDoc::openapi());

    doc
}
