use diesel::prelude::*;

use crate::{
    app::appointment::domain::entities::{
        appointment::Appointment, new_appointment::NewAppointment,
    },
    domain::error::ApiError,
    infra::db::{
        conn::get_connection,
        schema::appointment::{dsl::*, table, user_id},
    },
};

#[derive(Default, Debug)]
pub struct AppointmentRepository {}

impl AppointmentRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_appointment(&self, dto: NewAppointment) -> Result<(), ApiError> {
        let mut conn = get_connection()?;

        let created_appointment = diesel::insert_into(table).values(dto).execute(&mut conn);

        Ok(())
    }

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
