use api::app::user::infra::repositories::user_repository::UserRepository;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let user = UserRepository::new();

    user.get_user().unwrap();
}
