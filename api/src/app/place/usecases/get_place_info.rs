use axum::{
    extract::Path,
    Extension,
};

use crate::domain::entities::user::JwtPayload;

pub async fn execute(Extension(user): Extension<JwtPayload>, Path(id): Path<u32>) {
    println!("{:?}", user);
}
