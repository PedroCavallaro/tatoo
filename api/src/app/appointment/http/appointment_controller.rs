use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::app::appointment::{
    infra::repositories::appointment_repository::AppointmentRepository,
    usecases::{get_user_appointments, make_appointment},
};

#[derive(Debug, Default)]
pub struct PlaceController {}

impl PlaceController {
    pub fn routes() -> Router {
        let repo: Arc<AppointmentRepository> = Arc::new(AppointmentRepository::new());

        Router::new()
            .route("/appointment", get(get_user_appointments::execute))
            .route("/appointment", post(make_appointment::execute))
            .with_state(repo)
    }
}
