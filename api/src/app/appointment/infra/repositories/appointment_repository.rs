use diesel::prelude::*;

use crate::{
    app::appointment::domain::entities::appointment::Appointment,
    domain::error::ApiError,
    infra::db::{
        conn::get_connection,
        schema::appointment::{dsl::*, user_id},
    },
};

pub struct AppointmentRepository {}

impl AppointmentRepository {
    pub fn get_user_appointments(&self, id: i64) -> Result<Vec<Appointment>, ApiError> {
        let mut conn = get_connection()?;

        let user_appointments = appointment
            .select(Appointment::as_select())
            .filter(user_id.eq(id))
            .load(&mut conn);

        match user_appointments {
            Ok(val) => Ok(val),
            Err(_) => Ok(vec![]),
        }
    }
}
