use api::app::user::infra::repositories::{
    user_repository::UserRepository, user_repository_abstract::UserRepositoryAbstract,
};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let user = UserRepository::new();

    user.get_user().unwrap();
}
