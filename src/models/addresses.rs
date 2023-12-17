use std::fmt::Display;

use backend_derive::HasColumn;
use diesel::{prelude::AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::addresses;

#[derive(Clone, Serialize, Queryable, Identifiable, Selectable, HasColumn, ToSchema)]
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
    id_zone: i64,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}, {} {}",
            self.complement
                .as_ref()
                .map_or(String::new(), |c| format!("{}, ", c)),
            self.number.map_or(String::new(), |n| n.to_string()),
            self.street_name,
            self.postcode,
            self.city_name,
        )
    }
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = addresses)]
pub struct NewAddress {
    /// Street number
    number: Option<i32>,
    street_name: String,
    postcode: String,
    city_name: String,
    /// Address complement
    complement: Option<String>,
    id_zone: i64,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = addresses)]
pub struct UpdateAddress {
    /// Street number
    number: Option<Option<i32>>,
    street_name: Option<String>,
    postcode: Option<String>,
    city_name: Option<String>,
    /// Address complement
    complement: Option<Option<String>>,
    id_zone: Option<i64>,
}
