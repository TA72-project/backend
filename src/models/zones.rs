use backend_derive::HasColumn;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::zones;

#[derive(Serialize, Queryable, HasColumn, ToSchema)]
pub struct ZoneRecord {
    id: i64,
    name: String,
    pub id_center: i64,
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = zones)]
pub struct NewZone {
    pub name: String,
    #[serde(skip_deserializing)]
    pub id_center: i64,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = zones)]
pub struct UpdateZone {
    name: Option<String>,
    pub id_center: Option<i64>,
}
