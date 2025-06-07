use std::sync::Arc;

use axum::{routing::get, Router};

use crate::app::place::{
    infra::repositories::place_repository::PlaceRepository, usecases::get_place_info,
};

#[derive(Debug, Default)]
pub struct PlaceController {}

impl PlaceController {
    pub fn routes() -> Router {
        let repo: Arc<PlaceRepository> = Arc::new(PlaceRepository::new());

        Router::new()
            .route("/place/{id}", get(get_place_info::execute))
            .with_state(repo)
    }
}
