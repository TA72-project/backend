use actix_web::{
    cookie::Cookie,
    get,
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponseBuilder, Responder, Scope,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::{
    auth::{self, Auth},
    database::{crypt, DbPool},
    error::{JsonError, Result},
    models::LoginUser,
    schema::{managers, nurses, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(paths(login, logout), components(schemas(LoginUser, JsonError)))]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("auth").service(login).service(logout)
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200),
        (status = 401),
    ),
    tag = "auth",
    security(())
)]
#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, user: Json<LoginUser>) -> Result<impl Responder> {
    let user = user.into_inner();

    let id_user: i64 = users::table
        .select(users::id)
        .filter(users::mail.eq(user.mail))
        .filter(users::password.eq(crypt(user.password, users::password)))
        .first(&mut pool.get()?)?;

    let nurse: Option<i64> = nurses::table
        .find(id_user)
        .select(nurses::id)
        .first(&mut pool.get()?)
        .optional()?;

    let id = if let Some(id_nurse) = nurse {
        id_nurse
    } else {
        managers::table
            .find(id_user)
            .select(managers::id)
            .first::<i64>(&mut pool.get()?)?
    };

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .cookie(Auth::new(id, auth::Role::Nurse).try_into()?)
        .finish())
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
        .cookie(Cookie::new(auth::COOKIE_TOKEN_NAME, ""))
        .finish())
}
