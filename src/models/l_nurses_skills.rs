use diesel::{Associations, Identifiable, Queryable, Selectable};

use super::*;
use crate::schema::l_nurses_skills;

#[derive(Identifiable, Selectable, Queryable, Associations)]
#[diesel(table_name = l_nurses_skills)]
#[diesel(primary_key(id_nurse, id_skill))]
#[diesel(belongs_to(NurseRecord, foreign_key = id_nurse))]
#[diesel(belongs_to(Skill, foreign_key = id_skill))]
pub struct LNurseSkill {
    id_nurse: i64,
    id_skill: i64,
}