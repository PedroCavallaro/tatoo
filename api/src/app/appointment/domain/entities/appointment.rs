use chrono::*;
use diesel::prelude::*;
use serde_derive::Serialize;

use crate::infra::db::schema::appointment;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = appointment)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Appointment {
    pub place_id: i64,
    pub user_id: i64,
    pub date: NaiveDateTime,
}
