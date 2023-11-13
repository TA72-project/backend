use backend_derive::HasColumn;
use chrono::NaiveTime;
use diesel::Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Serialize, Queryable, HasColumn, ToSchema)]
#[diesel(table_name = centers)]
pub struct CenterRecord {
    id: i64,
    name: String,
    desc: Option<String>,
    /// The time the center starts working
    workday_start: NaiveTime,
    /// The time the center stops working
    workday_end: NaiveTime,
}
