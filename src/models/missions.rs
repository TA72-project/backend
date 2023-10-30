use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{MissionType, Patient};
use crate::schema::missions;

#[derive(Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = missions)]
pub struct MissionRecord {
    id: i64,
    /// Mission description
    pub desc: Option<String>,
    /// Start of the time window the mission should be fulfilled in
    start: NaiveDateTime,
    /// End of the time window the mission should be fulfilled in
    end: NaiveDateTime,
    /// Number of days after which the mission should be done again.
    ///
    /// If `null` the mission is not reccurent.
    recurrence_days: Option<i16>,
    /// Number of people required to execute this mission
    people_required: i16,
    /// ID of the type of mission
    id_mission_type: i64,
    /// ID of the patient related to this mission
    id_patient: i64,
}

#[derive(Serialize, Queryable, Selectable, ToSchema)]
pub struct Mission {
    #[serde(flatten)]
    #[diesel(embed)]
    pub mission: MissionRecord,
    #[diesel(embed)]
    pub mission_type: MissionType,
    #[diesel(embed)]
    pub patient: Patient,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = missions)]
pub struct UpdateMission {
    /// Mission description
    desc: Option<Option<String>>,
    /// Start of the time window the mission should be fulfilled in
    start: Option<NaiveDateTime>,
    /// End of the time window the mission should be fulfilled in
    end: Option<NaiveDateTime>,
    /// Number of days after which the mission should be done again.
    ///
    /// If `null` the mission is not reccurent.
    recurrence_days: Option<Option<i16>>,
    /// Number of people required to execute this mission
    people_required: Option<i16>,
    /// ID of the type of mission
    id_mission_type: Option<i64>,
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = missions)]
pub struct NewMission {
    /// Mission description
    desc: Option<String>,
    /// Start of the time window the mission should be fulfilled in
    start: NaiveDateTime,
    /// End of the time window the mission should be fulfilled in
    end: NaiveDateTime,
    /// Number of days after which the mission should be done again.
    ///
    /// If `null` the mission is not reccurent.
    recurrence_days: Option<i16>,
    /// Number of people required to execute this mission, defaults to `1`
    people_required: Option<i16>,
    /// ID of the type of mission
    id_mission_type: i64,
    /// ID of the patient related to this mission
    id_patient: i64,
}
