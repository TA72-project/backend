use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

use crate::schema::skills;

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = skills)]
pub struct Skill {
    id: i64,
    name: String,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = skills)]
pub struct UpdateSkill {
    name: String,
}

#[derive(Deserialize, Insertable, ToSchema, IntoParams)]
#[diesel(table_name = skills)]
pub struct NewSkill {
    name: String,
}
