use serde_derive::Deserialize;

use crate::{common::usecase::UseCase, domain::error::ApiError};

pub struct LoginUseCase {}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub email: String,
}

impl UseCase<(), LoginDTO> for LoginUseCase {
    fn execute(_: LoginDTO) -> Result<(), ApiError> {
        todo!()
    }
}
