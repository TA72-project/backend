use actix_web::{
    delete,
    error::ErrorForbidden,
    get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::{has_any_role, has_roles};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::{NewZone, UpdateZone, ZoneRecord},
    schema::zones,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get, post, put, delete),
    components(schemas(ZoneRecord, NewZone, UpdateZone, JsonError))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/skills")
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    context_path = "/zones",
    responses(
        (status = 200, body = ZoneRecord),
        (status = 404, body = JsonError),
    ),
    tag = "zones",
    security(
        ("token" = ["manager"])
    )
)]
#[get("/{id}")]
#[has_any_role["Role::Manager", "Role::Nurse", type = "Role"]]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let zone: ZoneRecord = macros::get!(zones, pool, *id);

    if zone.id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    Ok(Json(zone))
}

#[utoipa::path(
    context_path = "/zones",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "zones",
    security(
        ("token" = ["manager"])
    )
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_zone: web::Json<NewZone>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        insert_into(zones::table)
            .values(&NewZone {
                id_center: auth.id_center,
                ..new_zone.0
            })
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/zones",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "zones",
    security(
        ("token" = ["manager"])
    )
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_zone: web::Json<UpdateZone>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    let p2 = pool.clone();
    let id = *id;

    let zone: ZoneRecord = macros::get!(zones, p2, id);

    if zone.id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    diesel::update(zones::table)
        .set(&update_zone.0)
        .filter(zones::id.eq(id))
        .execute(&mut pool.get()?)?;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/zones",
    responses(
        (status = 200),
        (status = 404, body = JsonError),
    ),
    tag = "zones",
    security(
        ("token" = ["manager"])
    )
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let p2 = pool.clone();
    let id = *id;

    let zone: ZoneRecord = macros::get!(zones, p2, id);

    if zone.id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    macros::delete!(zones, pool, id);

    Ok(Json(()))
}
