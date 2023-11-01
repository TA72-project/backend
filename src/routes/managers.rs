use actix_web::{
    delete, get,
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
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    schema::{managers, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, delete),
    components(schemas(
        ManagerRecord,
        Manager,
        User,
        crate::pagination::PaginatedManagers,
        JsonError
    )),
    security(
        ("token" = ["manager"])
    )
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/managers")
        .service(all)
        .service(get)
        .service(delete)
}

#[utoipa::path(
    context_path = "/managers",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of managers", body = PaginatedManagers),
    ),
    tag = "managers"
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

    let res: Vec<Manager> = list!(managers, pool, query, users);

    let total = total!(managers, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total)))
}

#[utoipa::path(
    context_path = "/managers",
    responses(
        (status = 200, body = Manager),
        (status = 404, body = JsonError)
    ),
    tag = "managers"
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: Manager = macros::get!(managers, pool, *id, users);

    Ok(Json(res))
}

#[utoipa::path(
    context_path = "/managers",
    responses(
        (status = 200),
        (status = 404, body = JsonError),
    ),
    tag = "managers"
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(managers, pool, *id);

    Ok(Json(()))
}
