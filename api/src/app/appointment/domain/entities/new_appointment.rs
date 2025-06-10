use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infra::db::schema::appointment;

#[derive(Debug, Insertable)]
#[diesel(table_name = appointment)]
pub struct NewAppointment {
    pub date: NaiveDateTime,
    pub user_id: i64,
    pub place_id: i64,
}
