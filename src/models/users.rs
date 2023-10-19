use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::users;

#[derive(Clone, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = users)]
#[diesel(belongs_to(Center, foreign_key = id_center))]
pub struct User {
    id: i64,
    fname: String,
    lname: String,
    mail: String,
    phone: Option<String>,
    #[serde(skip)]
    password: Option<String>,
    #[serde(skip)]
    token: Option<String>,
    #[serde(skip)]
    token_gentime: Option<NaiveDateTime>,
    id_center: i64,
}
