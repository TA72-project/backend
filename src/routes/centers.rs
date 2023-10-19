use actix_web::{
    get,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
pub struct CenterDoc;

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

    let res: Vec<CenterWithAddress> = web::block(move || {
        centers::table
            .inner_join(addresses::table)
            .offset(query.offset().into())
            .limit(query.limit().into())
            .load(&mut pool.get().unwrap())
    })
    .await??;

    let total = web::block(move || {
        centers::table
            .count()
            .get_result::<i64>(&mut p2.get().unwrap())
    })
    .await??;

    Ok(Json(PaginatedResponse::new(res, &q2).total(total as u32)))
}

#[utoipa::path(
    get,
    path = "/centers/{id}",
    responses(
        (status = 200, body = CenterWithAddress),
        (status = NOT_FOUND, body = JsonError)
    ),
    params(
        ("id" = i64, Path, description = "Center id")
    ),
    tag = "centers"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: CenterWithAddress = web::block(move || {
        centers::table
            .inner_join(addresses::table)
            .filter(centers::id.eq(*id))
            .first(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}
