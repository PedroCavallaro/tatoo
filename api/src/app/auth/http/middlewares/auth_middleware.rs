use axum::{
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{app::auth::strategies::jwt::JwtStrategy, domain::entities::user::JwtPayload};

pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.uri().path().contains("auth") {
        let response = next.run(req).await;

        return Ok(response);
    }

    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if auth_header.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token: Vec<&str> = auth_header.unwrap().split(" ").collect();

    if token.len() != 2 || token[0] != "Bearer" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user = verify(token[1]);

    if user.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(user.unwrap());

    let response = next.run(req).await;

    Ok(response)
}

fn verify(token: &str) -> Option<JwtPayload> {
    let user = JwtStrategy::verify(token);

    user.ok()
}
