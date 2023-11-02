use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::schema::mission_types;

#[derive(Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = mission_types)]
pub struct MissionType {
    id: i64,
    pub name: String,
    /// Number of people required for this kind of mission
    people_required: i16,
    /// Mission type duration in minutes
    minutes_duration: i32,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = mission_types)]
pub struct UpdateMissionType {
    name: Option<String>,
    /// Number of people required for this kind of mission
    people_required: Option<i16>,
    /// Mission type duration in minutes
    minutes_duration: Option<i32>,
}

#[derive(Deserialize, Insertable, ToSchema, IntoParams)]
#[diesel(table_name = mission_types)]
pub struct NewMissionType {
    name: String,
    /// Number of people required for this kind of mission, `1` by default.
    people_required: Option<i16>,
    /// Mission type duration in minutes
    minutes_duration: i32,
}
