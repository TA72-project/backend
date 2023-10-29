//! Contains the application models
//!
//! Each model usually have a normal, updating and new version of the model.

mod addresses;
mod availabilities;
mod centers;
mod l_nurses_skills;
mod l_visits_nurses;
mod managers;
mod mission_types;
mod missions;
mod nurses;
mod patients;
mod skills;
mod users;
mod visits;

pub use addresses::*;
pub use availabilities::*;
pub use centers::*;
pub use l_nurses_skills::*;
pub use l_visits_nurses::*;
pub use managers::*;
pub use mission_types::*;
pub use missions::*;
pub use nurses::*;
pub use patients::*;
pub use skills::*;
pub use users::*;
pub use visits::*;
