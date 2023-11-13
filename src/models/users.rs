use std::option::Option;

use backend_derive::HasColumn;
use diesel::{ExpressionMethods, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    auth::Role,
    database::{crypt, gen_salt},
    schema::users,
};

#[derive(Clone, Serialize, Queryable, Identifiable, Selectable, HasColumn, ToSchema)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub fname: String,
    pub lname: String,
    mail: String,
    phone: Option<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    password: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct LoggedUser {
    #[serde(flatten)]
    pub user: User,
    pub role: Role,
    pub id_center: i64,
    pub id_zone: Option<i64>,
}

#[derive(Deserialize, ToSchema)]
pub struct NewUser {
    fname: String,
    lname: String,
    mail: String,
    phone: Option<String>,
    password: String,
}

/// Implements [`Insertable`] in such a way that the password is always and automatically hashed.
impl Insertable<users::table> for NewUser {
    type Values = <(
        Option<diesel::dsl::Eq<users::fname, String>>,
        Option<diesel::dsl::Eq<users::lname, String>>,
        Option<diesel::dsl::Eq<users::mail, String>>,
        Option<diesel::dsl::Eq<users::phone, String>>,
        Option<
            diesel::dsl::Eq<
                users::password,
                crypt::HelperType<String, gen_salt::HelperType<String>>,
            >,
        >,
    ) as Insertable<users::table>>::Values;

    fn values(self) -> Self::Values {
        (
            Some(users::fname.eq(self.fname)),
            Some(users::lname.eq(self.lname)),
            Some(users::mail.eq(self.mail)),
            self.phone.map(|x| users::phone.eq(x)),
            Some(users::password.eq(crypt(self.password, gen_salt(String::from("bf"))))),
        )
            .values()
    }
}

#[derive(Deserialize, ToSchema)]
pub struct LoginUser {
    pub mail: String,
    pub password: String,
}
