use chrono::*;
use diesel::prelude::*;
use serde_derive::Serialize;

use crate::infra::db::schema::user;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub picture: String,
    pub sub: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
