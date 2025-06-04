use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    app::{
        auth::{http::dto::{login_dto::LoginDTO, login_response_dto::LoginResponseDTO}, strategies::jwt::JwtStrategy},
        user::infra::repositories::{
            user_repository::UserRepository, user_repository_abstract::UserRepositoryAbstract,
        },
    },
    domain::{
        entities::user::{NewUser, User},
        error::ApiError,
    },
};

fn create_user(user_repository: Arc<UserRepository>, dto: LoginDTO) -> Result<User, ApiError> {
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
) -> Result<Json<LoginResponseDTO>, ApiError> {
    let user = user_repository.get_user_by_sub(&dto.sub);

    if user.is_err() {
        return Err(ApiError::new(404, "User not found"));
    }

    if let Ok(Some(_user)) = user {
        let token = JwtStrategy::generate_token(_user)?;

        return Ok(Json(LoginResponseDTO {token}));
    }

    let created_user = create_user(user_repository, dto)?;

    let token = JwtStrategy::generate_token(created_user)?;

    Ok(Json(LoginResponseDTO {token}))
}
