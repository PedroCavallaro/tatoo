use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{env, sync::LazyLock};

use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").unwrap(),
            jwt_secret: env::var("JWT_SECRET").unwrap(),
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
