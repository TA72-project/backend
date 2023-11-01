use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::has_roles;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::total;

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    schema::{addresses, mission_types, missions, patients, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete),
    components(schemas(
        Mission,
        MissionRecord,
        UpdateMission,
        NewMission,
        crate::pagination::PaginatedMissions,
        JsonError
    )),
    security(
        ("token" = ["manager"])
    )
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/missions")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    context_path = "/missions",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of missions", body = PaginatedMissions),
    ),
    tag = "missions"
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

    let res: Vec<Mission> = actix_web::web::block(move || {
        missions::table
            .inner_join(mission_types::table)
            .inner_join(
                patients::table
                    .inner_join(users::table)
                    .inner_join(addresses::table),
            )
            .offset(query.offset().into())
            .limit(query.limit().into())
            .load(&mut pool.get().unwrap())
    })
    .await??;

    let total = total!(missions, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total)))
}

#[utoipa::path(
    context_path = "/missions",
    responses(
        (status = 200, body = Mission),
        (status = 404, body = JsonError)
    ),
    tag = "missions"
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: Mission = actix_web::web::block(move || {
        missions::table
            .inner_join(mission_types::table)
            .inner_join(
                patients::table
                    .inner_join(users::table)
                    .inner_join(addresses::table),
            )
            .filter(missions::id.eq(*id))
            .first(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/missions",
    responses(
        (status = 200),
        (status = 400, body = JsonError)
    ),
    tag = "missions"
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_record: Json<NewMission>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        insert_into(missions::table)
            .values(&new_record.0)
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/missions",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "missions"
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_record: Json<UpdateMission>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        diesel::update(missions::table)
            .set(&update_record.0)
            .filter(missions::id.eq(*id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/missions",
    responses(
        (status = 200),
        (status = 404, body = JsonError)
    ),
    tag = "missions"
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(missions, pool, *id);

    Ok(Json(()))
}
