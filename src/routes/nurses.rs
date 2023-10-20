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
    schema::{addresses, nurses, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete),
    components(schemas(
        Nurse,
        NurseRecord,
        UpdateNurse,
        NewNurse,
        User,
        crate::pagination::PaginatedNurses,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/nurses")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    get,
    path = "/nurses",
    responses(
        (status = 200, description = "Paginated list of nurses", body = PaginatedNurses),
    ),
    params(
        PaginationParam
    ),
    tag = "nurses"
)]
#[get("")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let res: Vec<Nurse> = web::block(move || {
        nurses::table
            .inner_join(users::table)
            .inner_join(addresses::table)
            .offset(query.offset().into())
            .limit(query.limit().into())
            .load(&mut pool.get().unwrap())
    })
    .await??;

    let total = total!(nurses, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total as u32)))
}

#[utoipa::path(
    get,
    path = "/nurses/{id}",
    responses(
        (status = 200, body = Nurse),
        (status = NOT_FOUND, body = JsonError)
    ),
    params(
        ("id" = i64, Path, description = "Nurse id")
    ),
    tag = "nurses"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: Nurse = web::block(move || {
        nurses::table
            .inner_join(users::table)
            .inner_join(addresses::table)
            .filter(nurses::id.eq(*id))
            .first(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/nurses",
    responses(
        (status = 200, body = NurseRecord),
        (status = 400)
    ),
    tag = "nurses"
)]
#[post("")]
async fn post(new_record: web::Json<NewNurse>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: NurseRecord = web::block(move || {
        insert_into(nurses::table)
            .values(&new_record.0)
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    put,
    path = "/nurses/{id}",
    responses(
        (status = 200, body = NurseRecord),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the nurse to update")
    ),
    tag = "nurses"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_record: web::Json<UpdateNurse>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let res: NurseRecord = web::block(move || {
        diesel::update(nurses::table)
            .set(&update_record.0)
            .filter(nurses::id.eq(*id))
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/nurses/{id}",
    responses(
        (status = 200, body = NurseRecord, description = "The deleted nurse"),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the nurse to delete")
    ),
    tag = "nurses"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: NurseRecord = macros::delete!(nurses, pool, *id);

    Ok(Json(res))
}