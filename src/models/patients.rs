use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Address, User};
use crate::schema::patients;

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = patients)]
pub struct PatientRecord {
    id: i64,
    id_user: i64,
    id_address: i64,
}

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = patients)]
pub struct Patient {
    #[serde(flatten)]
    patient: PatientRecord,
    #[serde(flatten)]
    user: User,
    address: Address,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = patients)]
pub struct UpdatePatient {
    id_user: Option<i64>,
    id_address: Option<i64>,
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = patients)]
pub struct NewPatient {
    id_user: i64,
    id_address: i64,
}
