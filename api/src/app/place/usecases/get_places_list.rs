use axum::extract::State;

pub async fn get_places_list(
    State(place_repository): State<Arc<PlaceRepository>>,
    ) {}
