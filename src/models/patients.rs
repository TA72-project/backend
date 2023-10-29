use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Address, User};
use crate::schema::patients;

#[derive(Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = patients)]
pub struct PatientRecord {
    id: i64,
    id_user: i64,
    id_address: i64,
}

#[derive(Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = patients)]
pub struct Patient {
    #[serde(flatten)]
    #[diesel(embed)]
    patient: PatientRecord,
    #[serde(flatten)]
    #[diesel(embed)]
    pub user: User,
    #[diesel(embed)]
    pub address: Address,
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
