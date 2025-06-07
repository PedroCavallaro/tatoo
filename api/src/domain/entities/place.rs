use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::Serialize;

use crate::infra::db::schema::place;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = place)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Place {
    pub id: i64,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub whatsapp_number: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
