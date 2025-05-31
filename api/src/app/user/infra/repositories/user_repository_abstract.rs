use crate::domain::{
    entities::user::{NewUser, User},
    error::ApiError,
};

pub trait UserRepositoryAbstract {
    fn create_user(&self, dto: NewUser) -> Result<User, ApiError>;
    fn get_user_by_email(&self, user_email: &str) -> Result<Option<User>, ApiError>;
    fn get_users(&self) -> Result<Vec<User>, ApiError>;
}
