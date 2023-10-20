//! Contains the applications routes.
//!
//! It is recommended that each model have its own submodule. Each submodule export a `routes`
//! function which returns an [actix_web::Scope] with the routes defined.

pub mod centers;
pub mod mission_types;
pub mod nurses;
pub mod skills;
pub mod version;
