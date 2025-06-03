use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    app::{
        auth::http::dto::login_dto::LoginDTO,
        user::infra::repositories::{
            user_repository::UserRepository, user_repository_abstract::UserRepositoryAbstract,
        },
    },  domain::{
        entities::user::{NewUser, User},
        error::ApiError,
    }
};


fn create_user(
    user_repository: Arc<UserRepository>,
    dto: LoginDTO,
) -> Result<User, ApiError> {
    let new_user = NewUser {
        email: dto.email,
        name: dto.name,
        sub: dto.sub,
        picture: dto.picture,
    };

    let res = user_repository.create_user(new_user);

    Ok(res.unwrap())
}

pub async fn execute(
    State(user_repository): State<Arc<UserRepository>>,
    Json(dto): Json<LoginDTO>,
) -> Result<Json<User>, ApiError> {
    let user = user_repository.get_user_by_email(&dto.email);

    if user.is_err() {
        return Err(ApiError::new(404, "User not found"));
    }

    if let Ok(Some(_user)) = user {
        return Ok(Json(_user));
    }

    let created_user = create_user(user_repository, dto)?;

    Ok(Json(created_user))
}
