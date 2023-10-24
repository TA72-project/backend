//! Contains the application models
//!
//! Each model usually have a normal, updating and new version of the model.

mod addresses;
mod centers;
mod l_visits_nurses;
mod mission_types;
mod missions;
mod nurses;
mod patients;
mod skills;
mod users;
mod visits;

pub use addresses::*;
pub use centers::*;
pub use l_visits_nurses::*;
pub use mission_types::*;
pub use missions::*;
pub use nurses::*;
pub use patients::*;
pub use skills::*;
pub use users::*;
pub use visits::*;
