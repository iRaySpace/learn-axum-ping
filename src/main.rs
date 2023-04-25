use axum::{routing::get, Router};
use serde::Serialize;
use axum::response::IntoResponse;

#[derive(Serialize)]
struct Result {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(ping));
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> impl IntoResponse {
    let result = Result {
        message: "pong".to_string(),
    };
    axum::Json(result)
}
