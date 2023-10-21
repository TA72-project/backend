//! Contains the application models
//!
//! Each model usually have a normal, updating and new version of the model.

mod addresses;
mod centers;
mod mission_types;
mod nurses;
mod patients;
mod skills;
mod users;

pub use addresses::*;
pub use centers::*;
pub use mission_types::*;
pub use nurses::*;
pub use patients::*;
pub use skills::*;
pub use users::*;
