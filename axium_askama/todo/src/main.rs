use todo::{init, routes};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind address");

    init::logging();

    tracing::info!("Listening on: {}", listener.local_addr().unwrap());

    let app = routes::routes();
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");
}
