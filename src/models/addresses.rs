use std::fmt::Display;

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
