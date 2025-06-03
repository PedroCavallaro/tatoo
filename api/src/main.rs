use std::{env, net::SocketAddr};

use api::app::auth::http::auth_controller::AuthController;
use axum::{routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3000);

    let app = Router::new().route("/", get(|| async { "Hello, World!" })).merge(AuthController::routes());

    let listener = tokio::net::TcpListener::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap();

    println!("Server running on http://localhost:{:?}", port);

    axum::serve(listener, app).await.unwrap();
}
