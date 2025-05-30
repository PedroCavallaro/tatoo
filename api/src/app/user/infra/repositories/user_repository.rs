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
    fn get_user_by_email(&self, user_email: &String) -> Result<Option<User>, ApiError> {
        let mut con = get_connection()?;

        let res = user
            .filter(email.eq(user_email))
            .first::<User>(&mut con)
            .optional();

        println!("{:?}", res);

        Ok(res.unwrap())
    }

    fn get_users(&self) -> Result<Vec<User>, ApiError> {
        todo!()
    }
}
