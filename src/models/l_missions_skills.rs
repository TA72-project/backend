use diesel::{Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::l_missions_skills;

#[derive(Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = l_missions_skills)]
#[diesel(primary_key(id_visit, id_nurse))]
pub struct LMissionSkill {
    id_mission_type: i64,
    id_skill: i64,
    preferred: Option<bool>,
}
