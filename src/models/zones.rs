use backend_derive::HasColumn;
use diesel::Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Queryable, HasColumn, ToSchema)]
pub struct ZoneRecord {
    id: i64,
    name: String,
    id_center: i64,
}
