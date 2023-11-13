//! Contains the application models
//!
//! Each model usually have a normal, updating and new version of the model.

mod addresses;
mod availabilities;
mod centers;
mod has_column;
mod l_missions_skills;
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
mod zones;

pub use addresses::*;
pub use availabilities::*;
pub use centers::*;
pub use has_column::*;
pub use l_missions_skills::*;
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
pub use zones::*;

pub fn has_column(col: &str) -> bool {
    Address::has_column(col)
        || Availability::has_column(col)
        || CenterRecord::has_column(col)
        || LMissionSkill::has_column(col)
        || LNurseSkill::has_column(col)
        || LVisitNurse::has_column(col)
        || ManagerRecord::has_column(col)
        || MissionType::has_column(col)
        || MissionRecord::has_column(col)
        || NurseRecord::has_column(col)
        || PatientRecord::has_column(col)
        || Skill::has_column(col)
        || User::has_column(col)
        || VisitRecord::has_column(col)
        || ZoneRecord::has_column(col)
}
