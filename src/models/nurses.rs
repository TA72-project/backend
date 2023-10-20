use diesel::{AsChangeset, Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Address, User};
use crate::schema::nurses;

#[derive(Clone, Serialize, Queryable, Associations, ToSchema)]
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

#[derive(Clone, Serialize, Queryable, ToSchema)]
pub struct Nurse {
    #[serde(flatten)]
    nurse: NurseRecord,
    #[serde(flatten)]
    user: User,
    address: Address,
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
