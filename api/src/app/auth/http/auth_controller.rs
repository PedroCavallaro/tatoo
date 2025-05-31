use axum::{routing::post, Router};

use crate::{
    app::{
        auth::usecases::login_usecase::LoginUseCase,
        user::infra::repositories::user_repository::UserRepository,
    },
    common::usecase::UseCase,
    domain::entities::user::User,
};

use super::dto::login_dto::LoginDTO;

pub struct AuthController {
    login_usecase: Box<dyn UseCase<User, LoginDTO>>,
}

impl AuthController {
    pub fn new() -> Self {
        let repo = Box::new(UserRepository::new());

        Self {
            login_usecase: Box::new(LoginUseCase::new(repo)),
        }
    }

    pub fn routes(&self) -> Router {
        // Router::new().route("/login", pos;
        todo!()
    }
}

impl Default for AuthController {
    fn default() -> Self {
        Self::new()
    }
}
