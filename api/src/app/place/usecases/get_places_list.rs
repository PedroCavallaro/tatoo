use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode, Json,
};

use crate::{
    app::place::{
        http::dto::get_place_paginated_dto::GetPlacePaginatedDTO,
        infra::repositories::place_repository::PlaceRepository,
    },
    domain::{entities::place::Place, error::ApiError}, helpers::pagination::PaginationDTO,
};

pub async fn execute(
    Query(dto): Query<GetPlacePaginatedDTO>,
    State(place_repository): State<Arc<PlaceRepository>>,
) -> Result<
                Json<PaginationDTO<Place>>, ApiError> {
    let places = place_repository.get_places_list(Some(dto));


    match places {
        Ok(val) => Ok(val),
        Err(_) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "Erro ao buscar estabelecimentos",
        )),
    }
}
