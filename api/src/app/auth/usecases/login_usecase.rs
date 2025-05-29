use crate::{
    app::{
        auth::http::dto::login_dto::LoginDTO,
        user::infra::repositories::user_repository_abstract::UserRepositoryAbstract,
    },
    common::usecase::UseCase,
    domain::error::ApiError,
};

pub struct LoginUseCase {
    pub user_repository: Box<dyn UserRepositoryAbstract>,
}

impl LoginUseCase {
    pub fn new(user_repository: Box<dyn UserRepositoryAbstract>) -> Self {
        Self { user_repository }
    }
}

impl UseCase<(), LoginDTO> for LoginUseCase {
    fn execute(&self, _: LoginDTO) -> Result<(), ApiError> {
        self.user_repository.get_user().unwrap();
        todo!()
    }
}
