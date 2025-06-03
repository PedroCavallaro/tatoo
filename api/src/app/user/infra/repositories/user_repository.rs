use diesel::*;
use serde_derive::Deserialize;

use crate::{
    domain::{
        entities::user::{NewUser, User},
        error::ApiError,
    },
    infra::db::{
        conn::get_connection,
        schema::user::{self, dsl::*, table},
    },
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
    fn get_user_by_email(&self, user_email: &str) -> Result<Option<User>, ApiError> {
        let mut con = get_connection()?;

        let res = user
            .filter(email.eq(user_email))
            .first::<User>(&mut con)
            .optional();

        println!("{:?}", res);

        Ok(res.unwrap())
    }

    fn get_user_by_sub(&self, user_sub: &str) -> Result<Option<User>, ApiError> {
        let mut con = get_connection()?;

        let res = user
            .filter(email.eq(user_sub))
            .first::<User>(&mut con)
            .optional();

        println!("{:?}", res);

        Ok(res.unwrap())
    }

    fn get_users(&self) -> Result<Vec<User>, ApiError> {
        todo!()
    }

    fn create_user(&self, dto: NewUser) -> Result<User, ApiError> {
        let mut con = get_connection()?;

        let res = con.transaction::<User, diesel::result::Error, _>(|con| {
            diesel::insert_into(table).values(&dto).execute(con)?;

            let created_user = table
                .order(user::id.desc())
                .select(User::as_select())
                .first(con)?;

            Ok(created_user)
        });

        Ok(res.unwrap())
    }
}
