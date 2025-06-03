use std::sync::Arc;

use axum::{routing::post, Router};

use crate::app::{
    auth::usecases::login_usecase, user::infra::repositories::user_repository::UserRepository,
};

pub struct AuthController {}

impl AuthController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn routes() -> Router {
        let repo = Arc::new(UserRepository::new());

        Router::new()
            .route("/login", post(login_usecase::execute))
            .with_state(repo)
    }
}

impl Default for AuthController {
    fn default() -> Self {
        Self::new()
    }
}
