//! Contains the applications routes.
//!
//! It is recommended that each model have its own submodule. Each submodule export a `routes`
//! function which returns an [actix_web::Scope] with the routes defined.

pub mod auth;
pub mod centers;
pub mod managers;
pub mod mission_types;
pub mod missions;
pub mod nurses;
pub mod patients;
pub mod skills;
pub mod version;
pub mod visits;
pub mod zones;

