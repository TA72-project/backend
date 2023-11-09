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
    models::{Address, CenterRecord, ZoneRecord},
    pagination::{PaginatedResponse, PaginationParam},
    schema::{self, centers},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, zones),
    components(schemas(
        CenterRecord,
        Address,
        ZoneRecord,
        crate::pagination::PaginatedCenters,
        crate::pagination::PaginatedZones,
        JsonError
    )),
    security(
        ("token" = ["manager"])
    )
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/centers")
        .service(all)
        .service(get)
        .service(zones)
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

#[utoipa::path(
    context_path = "/centers",
    params(PaginationParam),
    responses(
        (status = 200, body = PaginatedZones),
        (status = 404, body = JsonError)
    ),
    tag = "centers"
)]
#[get("/{id}/zones")]
#[has_roles("Role::Manager", type = "Role")]
async fn zones(
    query: web::Query<PaginationParam>,
    id: web::Path<i64>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let res: Vec<ZoneRecord> = schema::zones::table
        .filter(schema::zones::id_center.eq(*id))
        .limit(query.limit().into())
        .offset(query.offset().into())
        .load(&mut pool.get()?)?;

    let total: i64 = schema::zones::table
        .filter(schema::zones::id_center.eq(*id))
        .count()
        .get_result(&mut pool.get()?)?;

    Ok(Json(
        PaginatedResponse::new(res, &query).total(total as u32),
    ))
}
