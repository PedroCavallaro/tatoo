use std::sync::Arc;

use axum::{
    extract::State,
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse},
    Json,
};
use cookie::{Cookie, SameSite};

use crate::{
    app::{
        auth::{
            http::dto::login_dto::LoginDTO,
            strategies::jwt::JwtStrategy,
        },
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

pub fn get_response(user: User) -> Result<impl IntoResponse, ApiError> {
    let token = JwtStrategy::generate_token(user)?;

    let cookie = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let headers = AppendHeaders([(SET_COOKIE, cookie.to_string())]);

    Ok(headers)
}

pub async fn execute(
    State(user_repository): State<Arc<UserRepository>>,
    Json(dto): Json<LoginDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let user = user_repository.get_user_by_sub(&dto.sub);

    if user.is_err() {
        return Err(ApiError::new(404, "User not found"));
    }

    if let Ok(Some(_user)) = user {
        let res = get_response(_user);

        return Ok(res);
    }

    let created_user = create_user(user_repository, dto)?;

    let res = get_response(created_user);

    Ok(res)
}
