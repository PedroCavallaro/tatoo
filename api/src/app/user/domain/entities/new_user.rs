use diesel::prelude::*;

use crate::infra::db::schema::user;

#[derive(Debug, Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub email: String,
    pub sub: String,
    pub name: String,
    pub picture: String,
}
