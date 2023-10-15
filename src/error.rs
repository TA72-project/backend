use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize)]
struct JsonError {
    message: String,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    R2d2(r2d2::Error),
    Blocking(actix_web::error::BlockingError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Diesel(err) => err.fmt(f),
            Error::R2d2(err) => err.fmt(f),
            Error::Blocking(err) => err.fmt(f),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Diesel(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(JsonError {
            message: format!("{self}"),
        })
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
