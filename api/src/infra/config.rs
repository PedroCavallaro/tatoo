use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{env, sync::LazyLock};

use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: Hmac<Sha256>,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET").unwrap();
        let jwt_key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

        Self {
            database_url: env::var("DATABASE_URL").unwrap(),
            jwt_secret: jwt_key,
            port: env::var("PORT").unwrap().parse::<u16>().unwrap(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub static CONFIGS: LazyLock<Config> = LazyLock::new(Config::new);
