use crate::{
    app::{
        auth::http::dto::login_dto::LoginDTO,
        user::infra::repositories::user_repository_abstract::UserRepositoryAbstract,
    },
    common::usecase::UseCase,
    domain::{entities::user::User, error::ApiError},
};

pub struct LoginUseCase {
    pub user_repository: Box<dyn UserRepositoryAbstract>,
}

impl LoginUseCase {
    pub fn new(user_repository: Box<dyn UserRepositoryAbstract>) -> Self {
        Self { user_repository }
    }

    fn create_user(&self, _: LoginDTO) {}
}

impl UseCase<User, LoginDTO> for LoginUseCase {
    fn execute(&self, dto: LoginDTO) -> Result<User, ApiError> {
        let user = self.user_repository.get_user_by_email(&dto.email);

        if user.is_err() {
            return Err(ApiError::new(404, "User not found"));
        }

        if let Ok(Some(_user)) = user {
            return Ok(_user);
        }

        self.create_user(dto);

        Err(ApiError::new(404, "User not found"))
    }
}
