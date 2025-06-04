use std::{io::Error, sync::OnceLock};

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};

use crate::infra::config::CONFIGS;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

type MysqlPoolConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub static POOL: OnceLock<MysqlPool> = OnceLock::new();

fn get_pool() -> &'static MysqlPool {
    POOL.get_or_init(|| {
        let url = &CONFIGS.database_url;

        let manager = ConnectionManager::<MysqlConnection>::new(url);

        Pool::builder().build(manager).unwrap()
    })
}

pub fn get_connection() -> Result<MysqlPoolConnection, Error> {
    let pool = get_pool();

    let con = pool.get();

    match con {
        Ok(con) => Ok(con),
        Err(_) => Err(Error::other("pool error")),
    }
}
