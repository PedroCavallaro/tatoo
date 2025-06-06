use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::user::{JwtPayload, User},
    infra::config::CONFIGS,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    email: String,
    name: String,
    exp: usize,
}

pub struct JwtStrategy {}

impl JwtStrategy {
    pub fn generate_token(user: User) -> Result<String, Box<dyn std::error::Error>> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email,
            name: user.name,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(CONFIGS.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn verify(token: &str) -> Result<JwtPayload, Box<dyn std::error::Error>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(CONFIGS.jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;

        let claims = token_data.claims;

        Ok(JwtPayload {
            id: claims.sub.parse::<i64>()?,
            email: claims.email,
            name: claims.name,
        })
    }
}
