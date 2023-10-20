use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::schema::mission_types;

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = mission_types)]
pub struct MissionType {
    id: i64,
    name: String,
    people_required: i16,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = mission_types)]
pub struct UpdateMissionType {
    name: Option<String>,
    people_required: Option<i16>,
}

#[derive(Deserialize, Insertable, ToSchema, IntoParams)]
#[diesel(table_name = mission_types)]
pub struct NewMissionType {
    name: String,
    people_required: Option<i16>,
}
