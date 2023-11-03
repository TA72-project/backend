use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::User;
use crate::schema::managers;

#[derive(Serialize, Queryable, ToSchema)]
pub struct ManagerRecord {
    id: i64,
    id_user: i64,
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
    pub id_user: i64,
}
