use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::Mission;
use crate::schema::visits;

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = visits)]
pub struct VisitRecord {
    id: i64,
    /// Date and time the visit begins
    start: NaiveDateTime,
    /// Date and time the visit ends
    end: NaiveDateTime,
    /// ID of the associated mission
    id_mission: i64,
}

#[derive(Serialize, Queryable, ToSchema)]
pub struct Visit {
    #[serde(flatten)]
    visit: VisitRecord,
    mission: Mission,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = visits)]
pub struct UpdateVisit {
    /// Date and time the visit begins
    start: Option<NaiveDateTime>,
    /// Date and time the visit ends
    end: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = visits)]
pub struct NewVisit {
    /// Date and time the visit begins
    start: NaiveDateTime,
    /// Date and time the visit ends
    end: NaiveDateTime,
    /// ID of the associated mission
    id_mission: i64,
}
