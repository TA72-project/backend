use backend_derive::HasColumn;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Address, NewAddress, NewUser, UpdateAddress, UpdateUser, User};
use crate::schema::patients;

#[derive(Serialize, Queryable, Selectable, HasColumn, ToSchema)]
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

#[derive(Deserialize, ToSchema)]
pub struct UpdatePatient {
    #[serde(flatten)]
    pub user: UpdateUser,
    pub address: UpdateAddress,
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = patients)]
pub struct NewPatientRecord {
    pub id_user: i64,
    pub id_address: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct NewPatient {
    #[serde(flatten)]
    pub user: NewUser,
    pub address: NewAddress,
}
