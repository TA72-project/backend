use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::total;

use crate::{
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
    ))
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
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
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
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
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
        (status = 200, body = MissionRecord),
        (status = 400, body = JsonError)
    ),
    tag = "missions"
)]
#[post("")]
async fn post(new_record: Json<NewMission>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: MissionRecord = web::block(move || {
        insert_into(missions::table)
            .values(&new_record.0)
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    context_path = "/missions",
    responses(
        (status = 200, body = MissionRecord),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "missions"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_record: Json<UpdateMission>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let res: MissionRecord = web::block(move || {
        diesel::update(missions::table)
            .set(&update_record.0)
            .filter(missions::id.eq(*id))
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    context_path = "/missions",
    responses(
        (status = 200, body = MissionRecord, description = "The deleted mission"),
        (status = 404, body = JsonError)
    ),
    tag = "missions"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: MissionRecord = macros::delete!(missions, pool, *id);

    Ok(Json(res))
}
