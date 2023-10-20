use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::models::{NewMissionType, UpdateMissionType};
use crate::{
    database::DbPool,
    error::{JsonError, Result},
    models::MissionType,
    pagination::{PaginatedResponse, PaginationParam},
    schema::mission_types,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete),
    components(schemas(
        MissionType,
        UpdateMissionType,
        NewMissionType,
        crate::pagination::PaginatedMissionTypes,
        JsonError
    ))
)]
pub struct SkillDoc;

pub fn routes() -> Scope {
    web::scope("/mission_types")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    get,
    path = "/mission_types",
    responses(
        (status = 200, description = "Paginated list of missions types", body = PaginatedMissionTypes),
    ),
    params(
        PaginationParam
    ),
    tag = "mission_types"
)]
#[get("")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let res: Vec<MissionType> = list!(mission_types, pool, query);

    let total = total!(mission_types, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total)))
}

#[utoipa::path(
    get,
    path = "/mission_types/{id}",
    responses(
        (status = 200, body = MissionType),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i64, Path, description = "Missions types id")
    ),
    tag = "mission_types"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: MissionType = macros::get!(mission_types, pool, *id);

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/mission_types",
    responses(
        (status = 200, body = MissionType),
        (status = 400)
    ),
    tag = "mission_types"
)]
#[post("")]
async fn post(
    new_mission_type: Json<NewMissionType>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let res: MissionType = web::block(move || {
        insert_into(mission_types::table)
            .values(&new_mission_type.0)
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    put,
    path = "/mission_types/{id}",
    responses(
        (status = 200, body = MissionType),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the mission type to update")
    ),
    tag = "mission_types"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_skill: Json<UpdateMissionType>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let res: MissionType = web::block(move || {
        diesel::update(mission_types::table)
            .set(&update_skill.0)
            .filter(mission_types::id.eq(*id))
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/mission_types/{id}",
    responses(
        (status = 200, body = MissionType, description = "The deleted mission type"),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the mission type to delete")
    ),
    tag = "mission_types"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: MissionType = macros::delete!(mission_types, pool, *id);

    Ok(Json(res))
}
