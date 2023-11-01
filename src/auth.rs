use std::future::Ready;

use actix_web::{dev::ServiceRequest, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

type Hours = i64;

pub static TOKEN_VALIDITY: Hours = 4;
pub static COOKIE_TOKEN_NAME: &str = "token";
pub static JWT_SECRET: &str = "TODO SECRET";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Role {
    Manager,
    Nurse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    exp: u64,
    iat: u64,
    /// User's id, may reference `nurses` or `managers` table.
    pub id: i64,
    pub role: Role,
}

impl Auth {
    pub fn new(id: i64, role: Role) -> Self {
        let now = Utc::now();
        Self {
            exp: (now + Duration::hours(TOKEN_VALIDITY)).timestamp() as u64,
            iat: now.timestamp() as u64,
            id,
            role,
        }
    }
}

impl TryFrom<Auth> for actix_web::cookie::Cookie<'_> {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(value: Auth) -> std::result::Result<Self, Self::Error> {
        use actix_web::cookie;

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &value,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )?;

        Ok(cookie::Cookie::build(COOKIE_TOKEN_NAME, token)
            .max_age(cookie::time::Duration::hours(TOKEN_VALIDITY))
            .secure(true)
            .http_only(true)
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

/// See [actix_web_grants]
pub async fn extract_permissions(
    req: &ServiceRequest,
) -> std::result::Result<Vec<Role>, actix_web::Error> {
    match get_token(req.request()) {
        Ok(auth) => Ok(vec![auth.role]),
        Err(_) => Ok(Vec::new()),
    }
}

fn get_token(req: &HttpRequest) -> Result<Auth> {
    let Some(received_token) = req.cookie(COOKIE_TOKEN_NAME) else {
        return Err(Error::TokenNotProvided);
    };

    // Require the `exp` field to be present in the JWT.
    // Other fields are checked only if present.
    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp"]);

    let decoded_token = decode::<Auth>(
        received_token.value(),
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    );

    match decoded_token {
        Ok(t) => Ok(t.claims),
        Err(err) => Err(err.into()),
    }
}
