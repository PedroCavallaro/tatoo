use diesel::prelude::*;

use crate::{
    app::place::{
        domain::entities::place::Place, http::dto::get_place_paginated_dto::GetPlacePaginatedDTO,
    },
    domain::error::ApiError,
    helpers::pagination::get_limit_offset,
    infra::db::{
        conn::get_connection,
        schema::place::{dsl::*, id},
    },
};

#[derive(Debug, Default)]
pub struct PlaceRepository {}

impl PlaceRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_place_by_id(&self, place_id: i64) -> Result<Option<Place>, ApiError> {
        let mut con = get_connection()?;

        let found_place = place
            .filter(id.eq(place_id))
            .first::<Place>(&mut con)
            .optional();

        match found_place {
            Ok(val) => Ok(Some(val.unwrap())),
            Err(_) => Ok(None),
        }
    }

    pub fn get_places_list(
        &self,
        pagination: Option<&GetPlacePaginatedDTO>,
    ) -> Result<Vec<Place>, ApiError> {
        let pagination = match pagination {
            Some(val) => val,
            None => &GetPlacePaginatedDTO {
                q: "".to_string(),
                page: 1,
                limit: 10,
                order: "ASC".to_string(),
            },
        };

        let mut con = get_connection()?;

        let (limit, offset) = get_limit_offset(pagination.page, pagination.limit);

        let places = place
            .select(Place::as_select())
            .offset(offset as i64)
            .limit(limit as i64)
            .load::<Place>(&mut con);

        match places {
            Ok(val) => Ok(val),
            Err(_) => Ok(vec![]),
        }
    }
}
