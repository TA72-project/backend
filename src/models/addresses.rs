use diesel::{Identifiable, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::addresses;

#[derive(Clone, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = addresses)]
pub struct Address {
    id: i64,
    /// Street number
    number: Option<i32>,
    street_name: String,
    postcode: String,
    city_name: String,
    /// Address complement
    complement: Option<String>,
}
