use actix_web::{
    delete,
    error::ErrorForbidden,
    get, post, put,
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
    schema::{addresses, patients, users, zones},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete),
    components(schemas(
        Patient,
        PatientRecord,
        UpdatePatient,
        UpdateAddress,
        NewPatientRecord,
        NewPatient,
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
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of patients", body = PaginatedPatients),
    ),
    tag = "patients"
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
    let res: Vec<Patient> = patients::table
        .inner_join(users::table)
        .inner_join(addresses::table)
        .filter(users::fname.ilike(search.value()))
        .or_filter(users::lname.ilike(search.value()))
        .or_filter(users::mail.ilike(search.value()))
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(&mut pool.get()?)?;

    let total = patients::table
        .inner_join(users::table)
        .filter(users::fname.ilike(search.value()))
        .or_filter(users::lname.ilike(search.value()))
        .or_filter(users::mail.ilike(search.value()))
        .count()
        .get_result::<i64>(&mut pool.get()?)? as u32;

    Ok(Json(PaginatedResponse::new(res, &pagination).total(total)))
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
            .values(user)
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
    auth: Auth,
) -> Result<impl Responder> {
    let (id_center, id_user, id_address): (i64, i64, i64) = patients::table
        .inner_join(addresses::table.inner_join(zones::table))
        .filter(patients::id.eq(*id))
        .select((zones::id_center, patients::id_user, patients::id_address))
        .first(&mut pool.get()?)?;

    if id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    pool.get()?.build_transaction().run(|conn| {
        diesel::update(users::table)
            .set(&update_record.user)
            .filter(users::id.eq(id_user))
            .execute(conn)?;

        diesel::update(addresses::table)
            .set(&update_record.address)
            .filter(addresses::id.eq(id_address))
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}

/// Delete patient
///
/// This will also delete the associated user and address.
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
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let id_center: i64 = patients::table
        .inner_join(addresses::table.inner_join(zones::table))
        .select(zones::id_center)
        .get_result(&mut pool.get()?)?;

    if id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    pool.get()?.build_transaction().run(|conn| {
        let (id_user, id_address): (i64, i64) = diesel::delete(patients::table)
            .filter(patients::id.eq(*id))
            .returning((patients::id_user, patients::id_address))
            .get_result(conn)?;

        diesel::delete(users::table)
            .filter(users::id.eq(id_user))
            .execute(conn)?;

        diesel::delete(addresses::table)
            .filter(addresses::id.eq(id_address))
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}
