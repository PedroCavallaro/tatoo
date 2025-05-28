use chrono::*;
use diesel::{prelude::Queryable, Selectable};
use serde_derive::Serialize;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::infra::db::schema::user)]
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
