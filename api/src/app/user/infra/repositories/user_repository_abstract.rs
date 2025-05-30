use crate::domain::{entities::user::User, error::ApiError};

pub trait UserRepositoryAbstract {
    fn get_user_by_email(&self, user_email: &String) -> Result<Option<User>, ApiError>;
    fn get_users(&self) -> Result<Vec<User>, ApiError>;
}
