use chrono::NaiveTime;
use diesel::{Associations, Queryable};
use serde::Serialize;
use utoipa::ToSchema;

use super::Address;
use crate::schema::centers;

#[derive(Clone, Serialize, Queryable, Associations, ToSchema)]
#[diesel(table_name = centers)]
#[diesel(belongs_to(Address, foreign_key = id_address))]
pub struct Center {
    id: i64,
    name: String,
    desc: Option<String>,
    /// The time the center starts working
    workday_start: NaiveTime,
    /// The time the center stops working
    workday_end: NaiveTime,
    /// The maximum range the center can operate at
    range_km: i16,
    id_address: i64,
}

#[derive(Clone, Serialize, Queryable, ToSchema)]
pub struct CenterWithAddress {
    #[serde(flatten)]
    center: Center,
    address: Address,
}
