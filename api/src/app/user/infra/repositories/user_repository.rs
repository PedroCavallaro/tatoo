use diesel::*;

use crate::{
    domain::entities::user::User,
    infra::db::{conn::get_pool, schema::user::dsl::*},
};

pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_user(&self) {
        let mut con = get_pool().get().unwrap();

        let res = user.load::<User>(&mut con);

        println!("{:?}", res);
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}
