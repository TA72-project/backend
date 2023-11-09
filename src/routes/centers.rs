use actix_web::{
    get,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::has_roles;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::{Address, CenterRecord},
    pagination::{PaginatedResponse, PaginationParam},
    schema::centers,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get),
    components(schemas(
        CenterRecord,
        Address,
        crate::pagination::PaginatedCenters,
        JsonError
    )),
    security(
        ("token" = ["manager"])
    )
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/centers").service(all).service(get)
}

#[utoipa::path(
    context_path = "/centers",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of centers", body = PaginatedCenters),
    ),
    tag = "centers"
)]
#[get("")]
#[has_roles("Role::Manager", type = "Role")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let res: Vec<CenterRecord> = list!(centers, pool, query);

    let total = total!(centers, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total as u32)))
}

#[utoipa::path(
    context_path = "/centers",
    responses(
        (status = 200, body = CenterRecord),
        (status = 404, body = JsonError)
    ),
    tag = "centers"
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: CenterRecord = macros::get!(centers, pool, *id);

    Ok(Json(res))
}
