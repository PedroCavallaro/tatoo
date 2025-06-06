use std::net::SocketAddr;

use api::{
    app::{
        auth::http::{auth_controller::AuthController, middlewares::auth_middleware::AuthLayer}, place::http::place_controller::PlaceController,
    },
    infra::config::CONFIGS,
};
use axum::{ routing::get, Router};
use dotenv::dotenv;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = CONFIGS.port;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(AuthController::routes())
        .merge(PlaceController::routes())
        .layer(
            ServiceBuilder::new().layer(AuthLayer{})
        );

    let listener = tokio::net::TcpListener::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap();

    println!("Server running on http://localhost:{:?}", port);

    axum::serve(listener, app).await.unwrap();
}
