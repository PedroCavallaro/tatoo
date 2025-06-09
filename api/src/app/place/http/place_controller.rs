use std::sync::Arc;

use axum::{routing::get, Router};

use crate::app::place::{
    infra::repositories::place_repository::PlaceRepository,
    usecases::{get_place_info, get_places_list},
};

#[derive(Debug, Default)]
pub struct PlaceController {}

impl PlaceController {
    pub fn routes() -> Router {
        let repo: Arc<PlaceRepository> = Arc::new(PlaceRepository::new());

        Router::new()
            .route("/place/{id}", get(get_place_info::execute))
            .route("/place/list", get(get_places_list::execute))
            .with_state(repo)
    }
}
