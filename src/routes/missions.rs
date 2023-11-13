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
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    params::{SearchParam, SortParam},
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
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of missions", body = PaginatedMissions),
    ),
    tag = "missions"
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
    let res: Vec<Mission> = missions::table
        .inner_join(mission_types::table)
        .inner_join(
            patients::table
                .inner_join(users::table)
                .inner_join(addresses::table),
        )
        .filter(missions::desc.ilike(search.value()))
        .or_filter(mission_types::name.ilike(search.value()))
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(&mut pool.get()?)?;

    let total = missions::table
        .inner_join(mission_types::table)
        .filter(missions::desc.ilike(search.value()))
        .or_filter(mission_types::name.ilike(search.value()))
        .count()
        .get_result::<i64>(&mut pool.get()?)? as u32;

    Ok(Json(PaginatedResponse::new(res, &pagination).total(total)))
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
