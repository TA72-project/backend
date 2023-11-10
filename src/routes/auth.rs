use actix_web::{
    cookie, get,
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponseBuilder, Responder, Scope,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::{
    auth::{Auth, Role},
    database::{crypt, DbPool},
    error::{JsonError, Result},
    models::{LoginUser, RoledUser, User},
    schema::{managers, nurses, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(login, logout),
    components(schemas(LoginUser, Role, RoledUser, JsonError))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("auth").service(login).service(logout)
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200, body = RoledUser),
        (status = 401),
    ),
    tag = "auth",
    security(())
)]
#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, user: Json<LoginUser>) -> Result<impl Responder> {
    let user = user.into_inner();

    let user: User = users::table
        .filter(users::mail.eq(user.mail))
        .filter(users::password.eq(crypt(user.password, users::password)))
        .first(&mut pool.get()?)?;

    let nurse: Option<i64> = nurses::table
        .filter(nurses::id_user.eq(user.id))
        .select(nurses::id)
        .first(&mut pool.get()?)
        .optional()?;

    let (id, role) = if let Some(id_nurse) = nurse {
        (id_nurse, Role::Nurse)
    } else {
        (
            managers::table
                .filter(managers::id_user.eq(user.id))
                .select(managers::id)
                .first::<i64>(&mut pool.get()?)?,
            Role::Manager,
        )
    };

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .cookie(Auth::new(id, role).try_into()?)
        .json(RoledUser { user, role }))
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200),
        (status = 401),
    ),
    tag = "auth",
    security(
        ("token" = [])
    )
)]
#[get("/logout")]
pub async fn logout(_: Auth) -> Result<impl Responder> {
    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .cookie(
            Auth::build_cookie("")
                .expires(cookie::time::OffsetDateTime::UNIX_EPOCH)
                .finish(),
        )
        .finish())
}
