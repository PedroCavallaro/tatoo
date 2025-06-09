use crate::{
    app::user::domain::entities::{new_user::NewUser, user::User},
    domain::error::ApiError,
};

pub trait UserRepositoryAbstract {
    fn create_user(&self, dto: NewUser) -> Result<User, ApiError>;
    fn get_user_by_sub(&self, sub: &str) -> Result<Option<User>, ApiError>;
    fn get_user_by_email(&self, user_email: &str) -> Result<Option<User>, ApiError>;
    fn get_users(&self) -> Result<Vec<User>, ApiError>;
}
