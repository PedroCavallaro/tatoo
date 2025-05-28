use std::{env, sync::OnceLock};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub static POOL: OnceLock<MysqlPool> = OnceLock::new();

pub fn get_pool() -> &'static MysqlPool {
    POOL.get_or_init(|| {
        let url = env::var("DATABASE_URL").unwrap();

        let manager = ConnectionManager::<MysqlConnection>::new(url);

        Pool::builder().build(manager).unwrap()
    })
}
