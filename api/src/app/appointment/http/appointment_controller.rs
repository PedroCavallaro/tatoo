use axum::{routing::post, Router};

use crate::app::appointment::usecases::make_appointment;

#[derive(Debug, Default)]
pub struct PlaceController {}

impl PlaceController {
    pub fn routes() -> Router {
        Router::new().route("/appointment", post(make_appointment::execute))
    }
}
