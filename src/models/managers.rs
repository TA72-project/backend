use backend_derive::HasColumn;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{NewUser, User};
use crate::schema::managers;

#[derive(Serialize, Queryable, HasColumn, ToSchema)]
pub struct ManagerRecord {
    id: i64,
    id_user: i64,
    id_center: i64,
}

#[derive(Serialize, Queryable, ToSchema)]
pub struct Manager {
    #[serde(flatten)]
    manager: ManagerRecord,
    #[serde(flatten)]
    user: User,
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = managers)]
pub struct NewManagerRecord {
    #[serde(skip_deserializing)]
    pub id_user: i64,
    pub id_center: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct NewManager {
    #[serde(flatten)]
    pub manager: NewManagerRecord,
    #[serde(flatten)]
    pub user: NewUser,
}
