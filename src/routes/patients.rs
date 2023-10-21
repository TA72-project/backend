use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::{
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    schema::{addresses, patients, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete),
    components(schemas(
        Patient,
        PatientRecord,
        UpdatePatient,
        NewPatient,
        User,
        Address,
        crate::pagination::PaginatedPatients,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/patients")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    get,
    path = "/patients",
    responses(
        (status = 200, description = "Paginated list of patients", body = PaginatedPatients),
    ),
    params(
        PaginationParam
    ),
    tag = "patients"
)]
#[get("")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let res: Vec<Patient> = list!(patients, pool, query, users, addresses);

    let total = total!(patients, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total)))
}

#[utoipa::path(
    get,
    path = "/patients/{id}",
    responses(
        (status = 200, body = Patient),
        (status = NOT_FOUND, body = JsonError)
    ),
    params(
        ("id" = i64, Path, description = "Patient id")
    ),
    tag = "patients"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: Patient = macros::get!(patients, pool, *id, users, addresses);

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/patients",
    responses(
        (status = 200, body = PatientRecord),
        (status = 400)
    ),
    tag = "patients"
)]
#[post("")]
async fn post(new_record: Json<NewPatient>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: PatientRecord = web::block(move || {
        insert_into(patients::table)
            .values(&new_record.0)
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    put,
    path = "/patients/{id}",
    responses(
        (status = 200, body = PatientRecord),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the patient to update")
    ),
    tag = "patients"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_record: Json<UpdatePatient>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let res: PatientRecord = web::block(move || {
        diesel::update(patients::table)
            .set(&update_record.0)
            .filter(patients::id.eq(*id))
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/patients/{id}",
    responses(
        (status = 200, body = PatientRecord, description = "The deleted patient"),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the patient to delete")
    ),
    tag = "patients"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: PatientRecord = macros::delete!(patients, pool, *id);

    Ok(Json(res))
}
