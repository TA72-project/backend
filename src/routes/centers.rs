use actix_web::{
    get,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::has_roles;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::{Address, CenterRecord, ZoneRecord},
    pagination::{PaginatedResponse, PaginationParam},
    params::{SearchParam, SortParam},
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
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of centers", body = PaginatedCenters),
    ),
    tag = "centers"
)]
#[get("")]
#[has_roles("Role::Manager", type = "Role")]
async fn all(
    pagination: web::Query<PaginationParam>,
    search: web::Query<SearchParam>,
    sort: web::Query<SortParam>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let req = centers::table
        .filter(centers::name.ilike(search.value()))
        .or_filter(centers::desc.ilike(search.value()));

    let res: Vec<CenterRecord> = req
        .clone()
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(&mut pool.get()?)?;

    let total = req.count().get_result::<i64>(&mut pool.get()?)?;

    Ok(Json(
        PaginatedResponse::new(res, &pagination).total(total as u32),
    ))
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
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, body = PaginatedZones),
        (status = 404, body = JsonError)
    ),
    tag = "centers"
)]
#[get("/{id}/zones")]
#[has_roles("Role::Manager", type = "Role")]
async fn zones(
    pagination: web::Query<PaginationParam>,
    search: web::Query<SearchParam>,
    sort: web::Query<SortParam>,
    id: web::Path<i64>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let req = schema::zones::table
        .filter(schema::zones::id_center.eq(*id))
        .filter(schema::zones::name.ilike(search.value()));

    let res: Vec<ZoneRecord> = req
        .clone()
        .order(sort.raw_sql())
        .limit(pagination.limit().into())
        .offset(pagination.offset().into())
        .load(&mut pool.get()?)?;

    let total: i64 = req.count().get_result(&mut pool.get()?)?;

    Ok(Json(
        PaginatedResponse::new(res, &pagination).total(total as u32),
    ))
}
