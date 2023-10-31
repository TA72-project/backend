use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use utoipa::ToSchema;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, ToSchema)]
pub struct JsonError {
    pub message: String,
}

impl JsonError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

/// A general wrapper around errors that could be produced by the different crates.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Errors from [diesel]
    Diesel(diesel::result::Error),
    /// Errors from [r2d2], this is also somewhat linked to [diesel]
    R2d2(r2d2::Error),
    /// Errors from [`actix_web::web::block`]
    Blocking(actix_web::error::BlockingError),
    /// Errors from [`jsonwebtoken`]
    JwtError(jsonwebtoken::errors::ErrorKind),

    /// The auth token has not been provided
    TokenNotProvided,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Diesel(err) => err.fmt(f),
            Error::R2d2(err) => err.fmt(f),
            Error::Blocking(err) => err.fmt(f),
            Error::JwtError(err) => std::fmt::Debug::fmt(&err, f),
            Error::TokenNotProvided => write!(f, "Token not provided"),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Diesel(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
            Error::TokenNotProvided | Error::JwtError(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            Error::Diesel(diesel::result::Error::NotFound)
            | Error::TokenNotProvided
            | Error::JwtError(_) => self.to_string(),
            #[cfg(debug_assertions)]
            _ => self.to_string(),
            #[cfg(not(debug_assertions))]
            _ => "Internal Server Error".into(),
        };

        HttpResponse::build(self.status_code()).json(JsonError::new(message))
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<r2d2::Error> for Error {
    fn from(value: r2d2::Error) -> Self {
        Self::R2d2(value)
    }
}

impl From<actix_web::error::BlockingError> for Error {
    fn from(value: actix_web::error::BlockingError) -> Self {
        Self::Blocking(value)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JwtError(value.into_kind())
    }
}
