use crate::domain::{entities::user::User, error::ApiError};

pub trait UserRepositoryAbstract {
    fn get_user(&self) -> Result<Vec<User>, ApiError>;
}
