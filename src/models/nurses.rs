use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::*;
use crate::schema::nurses;

#[derive(Clone, Copy, Identifiable, Selectable, Serialize, Queryable, Associations, ToSchema)]
#[diesel(table_name = nurses)]
#[diesel(belongs_to(Address, foreign_key = id_address))]
#[diesel(belongs_to(User, foreign_key = id_user))]
pub struct NurseRecord {
    pub id: i64,
    /// Minutes of working time per week
    minutes_per_week: i32,
    id_user: i64,
    id_address: i64,
}

#[derive(Serialize, Queryable, ToSchema)]
pub struct Nurse {
    #[serde(flatten)]
    pub nurse: NurseRecord,
    #[serde(flatten)]
    pub user: User,
    address: Address,
}

#[derive(Serialize, Queryable, ToSchema)]
pub struct SkilledNurse {
    #[serde(flatten)]
    pub nurse: Nurse,
    pub skills: Vec<Skill>,
}

impl From<(Vec<(LNurseSkill, Skill)>, Nurse)> for SkilledNurse {
    fn from(value: (Vec<(LNurseSkill, Skill)>, Nurse)) -> Self {
        let (skills, nurse) = value;

        Self {
            nurse,
            skills: skills.into_iter().map(|(_, skill)| skill).collect(),
        }
    }
}

#[derive(Clone, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = nurses)]
pub struct UpdateNurse {
    minutes_per_week: Option<i32>,
}

#[derive(Clone, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = nurses)]
pub struct NewNurse {
    minutes_per_week: i32,
}
