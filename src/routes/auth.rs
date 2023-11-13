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
    models::{LoggedUser, LoginUser, User},
    schema::{addresses, managers, nurses, users, zones},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(login, logout),
    components(schemas(LoginUser, Role, LoggedUser, JsonError))
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

    let nurse: Option<(i64, i64, i64)> = nurses::table
        .inner_join(addresses::table.inner_join(zones::table))
        .filter(nurses::id_user.eq(user.id))
        .select((nurses::id, zones::id_center, addresses::id_zone))
        .first(&mut pool.get()?)
        .optional()?;

    let (ids, role) = if let Some((id, id_center, id_zone)) = nurse {
        ((id, id_center, Some(id_zone)), Role::Nurse)
    } else {
        let (id, id_center) = managers::table
            .filter(managers::id_user.eq(user.id))
            .select((managers::id, managers::id_center))
            .first::<(i64, i64)>(&mut pool.get()?)?;

        ((id, id_center, None), Role::Manager)
    };

    let auth = Auth::new(ids, role);
    let logged_user = LoggedUser {
        user,
        role,
        id_center: ids.1,
        id_zone: ids.2,
    };

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .cookie(auth.try_into()?)
        .json(logged_user))
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
