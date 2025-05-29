use diesel::*;

use crate::{
    domain::{entities::user::User, error::ApiError},
    infra::db::{conn::get_connection, schema::user::dsl::*},
};

#[derive(Default)]
pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_user(&self) -> Result<Vec<User>, ApiError> {
        let mut con = get_connection()?;

        let res = user.load::<User>(&mut con);

        println!("{:?}", res);

        Ok(res.unwrap())
    }
}
