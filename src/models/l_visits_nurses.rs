use backend_derive::HasColumn;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::l_visits_nurses;

#[derive(Serialize, Queryable, HasColumn, ToSchema)]
#[diesel(table_name = l_visits_nurses)]
#[diesel(primary_key(id_visit, id_nurse))]
pub struct LVisitNurse {
    id_visit: i64,
    id_nurse: i64,
    report: Option<String>,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = l_visits_nurses)]
#[diesel(primary_key(id_visit, id_nurse))]
pub struct UpdateLVisitNurse {
    report: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = l_visits_nurses)]
#[diesel(primary_key(id_visit, id_nurse))]
pub struct NewLVisitNurse {
    pub id_visit: i64,
    pub id_nurse: i64,
}
