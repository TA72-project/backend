use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::has_roles;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::{
    auth::{Auth, Role},
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
        NewPatientRecord,
        User,
        Address,
        crate::pagination::PaginatedPatients,
        JsonError
    )),
    security(
        ("token" = ["manager"])
    )
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
    context_path = "/patients",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of patients", body = PaginatedPatients),
    ),
    tag = "patients"
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

    let res: Vec<Patient> = list!(patients, pool, query, users, addresses);

    let total = total!(patients, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total)))
}

#[utoipa::path(
    context_path = "/patients",
    responses(
        (status = 200, body = Patient),
        (status = 404, body = JsonError)
    ),
    tag = "patients"
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: Patient = macros::get!(patients, pool, *id, users, addresses);

    Ok(Json(res))
}

#[utoipa::path(
    path = "/patients",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "patients"
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_record: Json<NewPatient>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    pool.get()?.build_transaction().run(|conn| {
        let NewPatient { user, address } = new_record.0;

        let id_user: i64 = insert_into(users::table)
            .values(&user)
            .returning(users::id)
            .get_result(conn)?;

        let id_address: i64 = insert_into(addresses::table)
            .values(&address)
            .returning(addresses::id)
            .get_result(conn)?;

        insert_into(patients::table)
            .values(NewPatientRecord {
                id_user,
                id_address,
            })
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/patients",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "patients"
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_record: Json<UpdatePatient>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        diesel::update(patients::table)
            .set(&update_record.0)
            .filter(patients::id.eq(*id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/patients",
    responses(
        (status = 200),
        (status = 404, body = JsonError),
    ),
    tag = "patients"
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(patients, pool, *id);

    Ok(Json(()))
}
