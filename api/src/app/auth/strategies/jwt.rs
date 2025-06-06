use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use std::collections::BTreeMap;

use crate::{
    domain::entities::user::{JwtPayload, User},
    infra::config::CONFIGS,
};

pub struct JwtStrategy {}

impl JwtStrategy {
    pub fn generate_token(user: User) -> Result<String, Box<dyn std::error::Error>> {
        let mut claims = BTreeMap::new();

        let sub = &user.id.to_string();

        claims.insert("email", &user.email);
        claims.insert("name", &user.name);
        claims.insert("sub", sub);

        let token = claims.sign_with_key(&CONFIGS.jwt_secret)?;

        Ok(token)
    }

    pub fn verify(token: &str) -> Result<JwtPayload, Box<dyn std::error::Error>> {
        let res: Token<Header, BTreeMap<String, String>, _> =
            token.verify_with_key(&CONFIGS.jwt_secret)?;

        let claims = res.claims();

        let user = JwtPayload {
            id: claims["id"].parse::<i64>()?,
            email: String::from(&claims["email"]),
            name: String::from(&claims["name"]),
        };

        Ok(user)
    }
}
