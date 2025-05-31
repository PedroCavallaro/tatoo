use axum::{routing::post, Json, Router};

use crate::{
    app::{
        auth::usecases::login_usecase::LoginUseCase,
        user::infra::repositories::user_repository::UserRepository,
    },
    common::usecase::UseCase,
};

pub struct AuthController {
    login_usecase: LoginUseCase,
}

impl AuthController {
    pub fn new() -> Self {
        let repo = Box::new(UserRepository::new());

        Self {
            login_usecase: LoginUseCase::new(repo),
        }
    }

    pub fn routes(&self) -> Router {
        Router::new().route("/login", post(self.login_usecase.execute));
        todo!()
    }
}

impl Default for AuthController {
    fn default() -> Self {
        Self::new()
    }
}
