use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::Mission;
use crate::schema::visits;

#[derive(Serialize, Queryable, Selectable, ToSchema)]
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

#[derive(Serialize, Queryable, Selectable, ToSchema)]
pub struct Visit {
    #[serde(flatten)]
    #[diesel(embed)]
    visit: VisitRecord,
    #[diesel(embed)]
    pub mission: Mission,
}

impl From<Visit> for icalendar::Event {
    fn from(value: Visit) -> Self {
        use icalendar::{Component, EventLike};

        icalendar::Event::new()
            .uid(&value.visit.id.to_string())
            .summary(&value.mission.mission_type.name)
            .description(&format!(
                "Patient: {} {}\n\nDescription: {}\n\n",
                value.mission.patient.user.fname,
                value.mission.patient.user.lname.to_uppercase(),
                value.mission.mission.desc.unwrap_or_default()
            ))
            .starts(value.visit.start)
            .ends(value.visit.end)
            .location(&value.mission.patient.address.to_string())
            .done()
    }
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
