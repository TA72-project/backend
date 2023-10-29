use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::availabilities;

#[derive(Serialize, Selectable, Queryable, ToSchema)]
#[diesel(table_name = availabilities)]
pub struct Availability {
    id: i64,
    start: NaiveDateTime,
    end: NaiveDateTime,
    recurrent: bool,
    id_nurse: i64,
}
