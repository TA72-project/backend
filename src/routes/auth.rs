use std::sync::Arc;

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
    paths(login, info, logout),
    components(schemas(LoginUser, Role, LoggedUser, JsonError))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("auth")
        .service(login)
        .service(info)
        .service(logout)
}

mod helper {
    use super::*;

    pub struct UserInfo {
        pub id: i64,
        pub id_center: i64,
        pub id_zone: Option<i64>,
        pub role: Role,
    }

    pub fn get_user_info(user: &User, pool: Arc<DbPool>) -> Result<UserInfo> {
        let nurse: Option<(i64, i64, i64)> = nurses::table
            .inner_join(addresses::table.inner_join(zones::table))
            .filter(nurses::id_user.eq(user.id))
            .select((nurses::id, zones::id_center, addresses::id_zone))
            .first(&mut pool.get()?)
            .optional()?;

        Ok(if let Some((id, id_center, id_zone)) = nurse {
            UserInfo {
                id,
                id_center,
                id_zone: Some(id_zone),
                role: Role::Nurse,
            }
        } else {
            let (id, id_center) = managers::table
                .filter(managers::id_user.eq(user.id))
                .select((managers::id, managers::id_center))
                .first::<(i64, i64)>(&mut pool.get()?)?;

            UserInfo {
                id,
                id_center,
                id_zone: None,
                role: Role::Manager,
            }
        })
    }
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200, body = LoggedUser),
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

    let infos = helper::get_user_info(&user, pool.into_inner())?;
    let logged_user = LoggedUser {
        user,
        role: infos.role,
        id_zone: infos.id_zone,
        id_center: infos.id_center,
    };
    let auth = Auth::new(infos.id, &logged_user);

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .cookie(auth.try_into()?)
        .json(logged_user))
}

/// info
///
/// This route return the same information as `login`. It is meant for when you want general user
/// information and are already logged in. You can get the complete information of a nurse or
/// manager with their respective `me` routes.
#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200, body = LoggedUser),
        (status = 401),
    ),
    tag = "auth",
    security(
        ("token" = [])
    )
)]
#[get("/info")]
pub async fn info(pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let user: User = match auth.role {
        Role::Manager => users::table
            .inner_join(managers::table)
            .filter(managers::id.eq(auth.id))
            .select(users::all_columns)
            .first(&mut pool.get()?)?,
        Role::Nurse => users::table
            .inner_join(nurses::table)
            .filter(nurses::id.eq(auth.id))
            .select(users::all_columns)
            .first(&mut pool.get()?)?,
    };

    let infos = helper::get_user_info(&user, pool.into_inner())?;
    let logged_user = LoggedUser {
        user,
        role: infos.role,
        id_zone: infos.id_zone,
        id_center: infos.id_center,
    };

    Ok(Json(logged_user))
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
