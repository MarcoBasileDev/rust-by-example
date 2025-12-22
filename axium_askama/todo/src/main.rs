use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let app = Router::new().route("/", get(home));

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> impl IntoResponse {
    "Axum server"
}