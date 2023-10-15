use std::fmt::Display;

use actix_web::ResponseError;

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    DieselError(diesel::result::Error),
    R2d2Error(r2d2::Error),
    BlockingError(actix_web::error::BlockingError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DieselError(err) => err.fmt(f),
            Error::R2d2Error(err) => err.fmt(f),
            Error::BlockingError(err) => err.fmt(f),
        }
    }
}

impl ResponseError for Error {}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::DieselError(value)
    }
}

impl From<r2d2::Error> for Error {
    fn from(value: r2d2::Error) -> Self {
        Self::R2d2Error(value)
    }
}

impl From<actix_web::error::BlockingError> for Error {
    fn from(value: actix_web::error::BlockingError) -> Self {
        Self::BlockingError(value)
    }
}
