use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    app::place::{
        domain::entities::place::Place, http::dto::get_place_paginated_dto::GetPlacePaginatedDTO, infra::repositories::place_repository::PlaceRepository
    }, domain::error::ApiError, helpers::pagination::PaginationDTO
};

pub async fn execute(
    Query(dto): Query<GetPlacePaginatedDTO>,
    State(place_repository): State<Arc<PlaceRepository>>,
) -> Result<Json<PaginationDTO<Vec<Place>>>, ApiError> {
    let places = place_repository.get_places_list(Some(&dto));

    match places {
        Ok(val) => Ok(Json(PaginationDTO {
            data: val,
            page: dto.page as i64,
            limit: dto.limit as i64,
        })),
        Err(_) => Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "Erro ao buscar estabelecimentos",
        )),
    }
}
