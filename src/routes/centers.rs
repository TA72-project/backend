use actix_web::{
    get,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::{
    database::DbPool,
    error::{JsonError, Result},
    models::{Address, Center, CenterWithAddress},
    pagination::{PaginatedResponse, PaginationParam},
    schema::{addresses, centers},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get),
    components(schemas(
        CenterWithAddress,
        Center,
        Address,
        crate::pagination::PaginatedCenters,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/centers").service(all).service(get)
}

#[utoipa::path(
    get,
    path = "/centers",
    responses(
        (status = 200, description = "Paginated list of centers", body = PaginatedCenters),
    ),
    params(
        PaginationParam
    ),
    tag = "centers"
)]
#[get("")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let res: Vec<CenterWithAddress> = list!(centers, pool, query, addresses);

    let total = total!(centers, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total as u32)))
}

#[utoipa::path(
    get,
    path = "/centers/{id}",
    responses(
        (status = 200, body = CenterWithAddress),
        (status = 404, body = JsonError)
    ),
    params(
        ("id" = i64, Path, description = "Center id")
    ),
    tag = "centers"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: CenterWithAddress = macros::get!(centers, pool, *id, addresses);

    Ok(Json(res))
}
