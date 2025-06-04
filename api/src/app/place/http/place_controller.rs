use axum::{routing::get, Router};

use crate::app::place::usecases::get_place_info;

#[derive(Debug, Default)]
pub struct PlaceController {}

impl PlaceController {
    pub fn routes() -> Router {
        Router::new().route("/place/:id", get(get_place_info::execute))
    }
}
