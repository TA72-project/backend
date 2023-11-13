use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::has_roles;
use diesel::{insert_into, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::{MissionType, NewMissionType, UpdateMissionType},
    pagination::{PaginatedResponse, PaginationParam},
    params::{SearchParam, SortParam},
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
    )),
    security(
        ("token" = ["manager"])
    )
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/mission_types")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    context_path = "/mission_types",
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of missions types", body = PaginatedMissionTypes),
    ),
    tag = "mission_types"
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
    let req = mission_types::table.filter(mission_types::name.ilike(search.value()));

    let res: Vec<MissionType> = req
        .clone()
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(&mut pool.get()?)?;

    let total = req.count().get_result::<i64>(&mut pool.get()?)? as u32;

    Ok(Json(PaginatedResponse::new(res, &pagination).total(total)))
}

#[utoipa::path(
    context_path = "/mission_types",
    responses(
        (status = 200, body = MissionType),
        (status = 404, body = JsonError)
    ),
    tag = "mission_types"
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: MissionType = macros::get!(mission_types, pool, *id);

    Ok(Json(res))
}

#[utoipa::path(
    context_path = "/mission_types",
    responses(
        (status = 200),
        (status = 400, body = JsonError)
    ),
    tag = "mission_types"
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_mission_type: Json<NewMissionType>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        insert_into(mission_types::table)
            .values(&new_mission_type.0)
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/mission_types",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "mission_types"
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_skill: Json<UpdateMissionType>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        diesel::update(mission_types::table)
            .set(&update_skill.0)
            .filter(mission_types::id.eq(*id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/mission_types",
    responses(
        (status = 200),
        (status = 404, body = JsonError)
    ),
    tag = "mission_types"
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(mission_types, pool, *id);

    Ok(Json(()))
}
