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
    schema::{managers, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, me, get, post, put, delete),
    components(schemas(
        ManagerRecord,
        Manager,
        User,
        NewManager,
        UpdateUser,
        NewManagerRecord,
        NewUser,
        crate::pagination::PaginatedManagers,
        JsonError
    )),
    security(
        ("token" = ["manager"])
    )
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/managers")
        .service(all)
        .service(me)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    context_path = "/managers",
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of managers", body = PaginatedManagers),
    ),
    tag = "managers"
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
    let pool = &mut pool.get()?;

    let req = managers::table
        .inner_join(users::table)
        .filter(users::fname.ilike(search.value()))
        .or_filter(users::lname.ilike(search.value()))
        .or_filter(users::mail.ilike(search.value()));

    let res: Vec<Manager> = req
        .clone()
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(pool)?;

    let total = req.count().get_result::<i64>(pool)? as u32;

    Ok(Json(PaginatedResponse::new(res, &pagination).total(total)))
}

#[utoipa::path(
    context_path = "/managers",
    responses(
        (status = 200, body = Manager),
    ),
    tag = "managers"
)]
#[get("/me")]
#[has_roles("Role::Manager", type = "Role")]
async fn me(pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let res: Manager = macros::get!(managers, pool, auth.id, users);

    Ok(Json(res))
}

#[utoipa::path(
    context_path = "/managers",
    responses(
        (status = 200, body = Manager),
        (status = 404, body = JsonError)
    ),
    tag = "managers"
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let res: Manager = macros::get!(managers, pool, *id, users);

    Ok(Json(res))
}

#[utoipa::path(
    path = "/managers",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "managers"
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_record: Json<NewManager>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    pool.get()?.build_transaction().run(|conn| {
        let NewManager { manager, user } = new_record.0;

        let id_user: i64 = insert_into(users::table)
            .values(user)
            .returning(users::id)
            .get_result(conn)?;

        insert_into(managers::table)
            .values(NewManagerRecord { id_user, ..manager })
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}

#[utoipa::path(
    path = "/managers",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "managers"
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_record: Json<UpdateUser>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    if auth.id != *id {
        return Err(ErrorForbidden("").into());
    }

    diesel::update(users::table)
        .set(&update_record.0)
        .filter(users::id.eq(auth.id_user))
        .execute(&mut pool.get()?)?;

    Ok(Json(()))
}

/// Delete manager
///
/// This will also delete the associated user
#[utoipa::path(
    context_path = "/managers",
    responses(
        (status = 200),
        (status = 404, body = JsonError),
    ),
    tag = "managers"
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let id_center: i64 = managers::table
        .select(managers::id_center)
        .get_result(&mut pool.get()?)?;

    if id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    pool.get()?.build_transaction().run(|conn| {
        let id_user: i64 = diesel::delete(managers::table)
            .filter(managers::id.eq(*id))
            .returning(managers::id_user)
            .get_result(conn)?;

        diesel::delete(users::table)
            .filter(users::id.eq(id_user))
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}
