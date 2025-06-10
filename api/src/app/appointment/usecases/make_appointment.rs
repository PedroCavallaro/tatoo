use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::{
    app::{
        appointment::{
            domain::entities::appointment::Appointment,
            infra::repositories::appointment_repository::AppointmentRepository,
        },
        auth::domain::entities::jwt_payload::JwtPayload,
    },
    domain::error::ApiError,
};

pub async fn execute(
    Extension(user): Extension<JwtPayload>,
    State(repository): State<Arc<AppointmentRepository>>,
) -> Result<Json<Vec<Appointment>>, ApiError> {
    let appointments = repository.get_user_appointments(user.id);

    match appointments {
        Ok(val) => Ok(Json(val)),
        Err(_) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "Erro ao buscar agendamentos",
        )),
    }
}
