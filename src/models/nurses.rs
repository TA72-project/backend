use backend_derive::HasColumn;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::*;
use crate::schema::nurses;

#[derive(
    Clone, Copy, Identifiable, Selectable, Serialize, Queryable, Associations, HasColumn, ToSchema,
)]
#[diesel(table_name = nurses)]
#[diesel(belongs_to(Address, foreign_key = id_address))]
#[diesel(belongs_to(User, foreign_key = id_user))]
pub struct NurseRecord {
    /// Nurse ID
    pub id: i64,
    /// Minutes of working time per week
    minutes_per_week: i32,
    id_user: i64,
    id_address: i64,
}

#[derive(Serialize, Selectable, Queryable, ToSchema)]
pub struct Nurse {
    #[serde(flatten)]
    #[diesel(embed)]
    pub nurse: NurseRecord,
    #[serde(flatten)]
    #[diesel(embed)]
    pub user: User,
    #[diesel(embed)]
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
pub struct UpdateNurseRecord {
    minutes_per_week: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateNurse {
    #[serde(flatten)]
    pub nurse: UpdateNurseRecord,
    #[serde(flatten)]
    pub user: UpdateUser,
    pub address: UpdateAddress,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = nurses)]
pub struct NewNurseRecord {
    /// Minutes of working time per week
    pub minutes_per_week: i32,
    #[serde(skip_deserializing)]
    pub id_user: i64,
    #[serde(skip_deserializing)]
    pub id_address: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct NewNurse {
    #[serde(flatten)]
    pub nurse: NewNurseRecord,
    #[serde(flatten)]
    pub user: NewUser,
    pub address: NewAddress,
}
