use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::skills;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = skills)]
pub struct Skill {
    id: i64,
    name: String,
}

#[derive(Debug, Clone, Deserialize, AsChangeset)]
#[diesel(table_name = skills)]
pub struct UpdateSkill {
    name: String,
}

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = skills)]
pub struct NewSkill {
    name: String,
}
