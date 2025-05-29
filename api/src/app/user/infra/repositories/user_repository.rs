use diesel::*;
use serde_derive::Deserialize;

use crate::{
    domain::{entities::user::User, error::ApiError},
    infra::db::{conn::get_connection, schema::user::dsl::*},
};

use super::user_repository_abstract::UserRepositoryAbstract;

#[derive(Default, Debug, Deserialize)]
pub struct UserRepository {}
impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserRepositoryAbstract for UserRepository {
    fn get_user(&self) -> Result<Vec<User>, ApiError> {
        let mut con = get_connection()?;

        let res = user.load::<User>(&mut con);

        println!("{:?}", res);

        Ok(res.unwrap())
    }
}
