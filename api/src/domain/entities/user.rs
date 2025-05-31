use chrono::*;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
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

#[derive(Debug, Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub email: String,
    pub sub: String,
    pub name: String,
    pub picture: String,
}
