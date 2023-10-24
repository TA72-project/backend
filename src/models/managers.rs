use diesel::Queryable;
use serde::Serialize;
use utoipa::ToSchema;

use super::User;

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
