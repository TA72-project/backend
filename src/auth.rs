use std::{borrow::Cow, env, future::Ready};

use actix_web::{cookie::SameSite, dev::ServiceRequest, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    error::{Error, Result},
    models::LoggedUser,
};

type Hours = i64;

/// Time the token is valid for
pub static TOKEN_VALIDITY: Hours = 4;
/// The name of the cookie that contains the token
pub static COOKIE_TOKEN_NAME: &str = "token";
/// The passphrase used to encode and decode the JWT
static JWT_SECRET: OnceCell<String> = OnceCell::new();
/// The JWT encoding key
static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    EncodingKey::from_secret(
        JWT_SECRET
            .get()
            .expect("JWT_SECRET is not initialized")
            .as_bytes(),
    )
});
/// The JWT decoding key
static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret(
        JWT_SECRET
            .get()
            .expect("JWT_SECRET is not initialized")
            .as_bytes(),
    )
});

/// Gets the `JWT_SECRET` env variable and register it for JWT authentication.
///
/// This function should be called only once at the start of the program.
pub fn initialize_jwt() -> std::result::Result<(), String> {
    let Ok(var) = env::var("JWT_SECRET") else {
        return Err("JWT_SECRET env variable not set".into());
    };

    if var.is_empty() {
        return Err("JWT_SECRET should not be empty".into());
    }

    if JWT_SECRET.set(var).is_err() {
        return Err("JWT_SECRET was already set".into());
    }

    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub enum Role {
    Manager,
    Nurse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    exp: u64,
    iat: u64,
    /// User's id, this references `nurses` or `managers` table depending on its role
    pub id: i64,
    /// The real user ID, references `users` table
    pub id_user: i64,
    pub id_center: i64,
    pub id_zone: Option<i64>,
    pub role: Role,
}

impl Auth {
    /// Creates an auth structure from id and a logged user.
    ///
    /// `id` may reference a nurse or a manager depending on the role in `user`
    pub fn new(id: i64, user: &LoggedUser) -> Self {
        let now = Utc::now();
        let LoggedUser {
            role,
            user,
            id_center,
            id_zone,
            ..
        } = user;

        Self {
            exp: (now + Duration::hours(TOKEN_VALIDITY)).timestamp() as u64,
            iat: now.timestamp() as u64,
            id,
            id_user: user.id,
            id_center: *id_center,
            id_zone: *id_zone,
            role: *role,
        }
    }

    /// Builds the base of an authentication cookie.
    pub fn build_cookie<'c, V>(value: V) -> actix_web::cookie::CookieBuilder<'c>
    where
        V: Into<Cow<'c, str>>,
    {
        actix_web::cookie::Cookie::build(COOKIE_TOKEN_NAME, value)
            .path("/")
            .same_site(SameSite::Strict)
            .secure(true)
            .http_only(true)
    }
}

impl TryFrom<Auth> for actix_web::cookie::Cookie<'_> {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(value: Auth) -> std::result::Result<Self, Self::Error> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &value, &ENCODING_KEY)?;

        Ok(Auth::build_cookie(token)
            .max_age(actix_web::cookie::time::Duration::hours(TOKEN_VALIDITY))
            .finish())
    }
}

impl FromRequest for Auth {
    type Error = Error;

    type Future = Ready<Result<Auth>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(get_token(req))
    }
}

/// Extracts the roles from the token.
///
/// See [actix_web_grants].
pub async fn extract_permissions(
    req: &ServiceRequest,
) -> std::result::Result<Vec<Role>, actix_web::Error> {
    match get_token(req.request()) {
        Ok(auth) => Ok(vec![auth.role]),
        Err(_) => Ok(Vec::new()),
    }
}

/// Get the token from a request if it exists and decode it.
fn get_token(req: &HttpRequest) -> Result<Auth> {
    let Some(received_token) = req.cookie(COOKIE_TOKEN_NAME) else {
        return Err(Error::TokenNotProvided);
    };

    // Require the `exp` field to be present in the JWT.
    // Other fields are checked only if present.
    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp"]);

    let decoded_token = decode::<Auth>(received_token.value(), &DECODING_KEY, &validation);

    match decoded_token {
        Ok(t) => Ok(t.claims),
        Err(err) => Err(err.into()),
    }
}
