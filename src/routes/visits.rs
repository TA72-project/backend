use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::{has_any_role, has_roles};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use macros::total;

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    params::SortParam,
    schema::{self, addresses, l_visits_nurses, mission_types, missions, patients, users, visits},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, nurses, post, post_visit_nurse, put_report, put, delete),
    components(schemas(
        Visit,
        VisitRecord,
        UpdateVisit,
        UpdateLVisitNurse,
        NewVisit,
        crate::pagination::PaginatedVisits,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/visits")
        .service(all)
        .service(get)
        .service(nurses)
        .service(post)
        .service(post_visit_nurse)
        .service(put_report)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    context_path = "/visits",
    params(PaginationParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of visits", body = PaginatedVisits),
    ),
    tag = "visits",
    security(
        ("token" = ["manager"])
    )
)]
#[get("")]
#[has_roles("Role::Manager", type = "Role")]
async fn all(
    query: web::Query<PaginationParam>,
    sort: web::Query<SortParam>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let res: Vec<Visit> = actix_web::web::block(move || {
        visits::table
            .inner_join(
                missions::table.inner_join(mission_types::table).inner_join(
                    patients::table
                        .inner_join(users::table)
                        .inner_join(addresses::table),
                ),
            )
            .order(sort.raw_sql())
            .offset(query.offset().into())
            .limit(query.limit().into())
            .load(&mut pool.get().unwrap())
    })
    .await??;

    let total = total!(visits, p2);

    Ok(Json(PaginatedResponse::new(res, &q2).total(total)))
}

#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200, body = Visit),
        (status = 404, body = JsonError)
    ),
    tag = "visits",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[get("/{id}")]
#[has_any_role("Role::Manager", "Role::Nurse", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: Visit = actix_web::web::block(move || {
        visits::table
            .inner_join(
                missions::table.inner_join(mission_types::table).inner_join(
                    patients::table
                        .inner_join(users::table)
                        .inner_join(addresses::table),
                ),
            )
            .filter(visits::id.eq(*id))
            .first(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(res))
}

#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200, body = PaginatedNurses),
        (status = 404, body = JsonError)
    ),
    tag = "visits",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[get("/{id}/nurses")]
#[has_any_role("Role::Manager", "Role::Nurse", type = "Role")]
async fn nurses(
    id: web::Path<i64>,
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let res: Vec<Nurse> = schema::nurses::table
        .inner_join(schema::users::table)
        .inner_join(schema::addresses::table)
        .inner_join(schema::l_visits_nurses::table)
        .filter(schema::l_visits_nurses::id_visit.eq(*id))
        .limit(query.limit().into())
        .offset(query.offset().into())
        .select(Nurse::as_select())
        .load(&mut pool.get()?)?;

    let total: i64 = schema::l_visits_nurses::table
        .filter(schema::l_visits_nurses::id_visit.eq(*id))
        .count()
        .get_result(&mut pool.get()?)?;

    Ok(Json(
        PaginatedResponse::new(res, &query).total(total as u32),
    ))
}

#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200),
        (status = 400, body = JsonError)
    ),
    tag = "visits",
    security(
        ("token" = ["manager"])
    )
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_record: Json<NewVisit>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        insert_into(visits::table)
            .values(&new_record.0)
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

/// Associate nurse & visit
///
/// Associates the given nurse with the given visit.
#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200),
    ),
    tag = "visits",
    security(
        ("token" = ["manager"])
    )
)]
#[post("/{id_visit}/nurses/{id_nurse}")]
#[has_roles("Role::Manager", type = "Role")]
async fn post_visit_nurse(
    ids: web::Path<(i64, i64)>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        insert_into(l_visits_nurses::table)
            .values(&NewLVisitNurse {
                id_visit: ids.0,
                id_nurse: ids.1,
            })
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

/// Create report
///
/// Create or modify a report for the current nurse and the given visit.
#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "visits",
    security(
        ("token" = ["nurse"])
    )
)]
#[put("/{id}/report")]
#[has_roles("Role::Nurse", type = "Role")]
async fn put_report(
    id: web::Path<i64>,
    update_record: Json<UpdateLVisitNurse>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    let rows = web::block(move || {
        diesel::update(l_visits_nurses::table)
            .set(&update_record.0)
            .filter(l_visits_nurses::id_visit.eq(*id))
            .filter(l_visits_nurses::id_nurse.eq(auth.id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    if rows == 0 {
        Err(diesel::result::Error::NotFound.into())
    } else {
        Ok(Json(()))
    }
}

#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "visits",
    security(
        ("token" = ["manager"])
    )
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_record: Json<UpdateVisit>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        diesel::update(visits::table)
            .set(&update_record.0)
            .filter(visits::id.eq(*id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/visits",
    responses(
        (status = 200),
        (status = 404, body = JsonError)
    ),
    tag = "visits",
    security(
        ("token" = ["manager"])
    )
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(visits, pool, *id);

    Ok(Json(()))
}
