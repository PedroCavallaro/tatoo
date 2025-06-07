use crate::{
    domain::error::ApiError,
    infra::db::{
        conn::get_connection,
        schema::place::{self},
    },
};

#[derive(Debug, Default)]
pub struct PlaceRepository {}

impl PlaceRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_place_by_id(&self, id: u32) -> Result<(), ApiError> {
        let con = get_connection()?;

        todo!()
    }
}
