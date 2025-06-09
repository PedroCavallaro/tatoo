use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    app::place::{
        http::dto::get_place_paginated_dto::GetPlacePaginatedDTO,
        infra::repositories::place_repository::PlaceRepository,
    },
    domain::{entities::place::Place, error::ApiError},
};

pub async fn get_places_list(
    Query(dto): Query<GetPlacePaginatedDTO>,
    State(place_repository): State<Arc<PlaceRepository>>,
) -> Result<Vec<Place>, ApiError> {
    let places = place_repository.get_places_list(Some(dto));

    match places {
        Ok(val) => Ok(val),
        Err(_) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "Erro ao buscar estabelecimentos",
        )),
    }
}
