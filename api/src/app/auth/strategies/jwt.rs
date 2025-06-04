use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::{domain::entities::user::User, infra::config::CONFIGS};

pub struct JwtStrategy {}

impl JwtStrategy {
    pub fn generate_token(user: User) -> Result<String, Box<dyn std::error::Error>> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(CONFIGS.jwt_secret.as_bytes())?;

        let mut claims = BTreeMap::new();

        let sub = &user.id.to_string();

        claims.insert("email", &user.email);
        claims.insert("name", &user.name);
        claims.insert("sub", sub);

        let token = claims.sign_with_key(&key)?;

        Ok(token)
    }
}
