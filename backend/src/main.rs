use axum::{routing::get, Router, response::{IntoResponse, Html}};
use std::net::SocketAddr;
use axum::extract::Path;

#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/", get(handler))
    .route("/hello/:name", get(handler_hello2));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello: {name}"))
}