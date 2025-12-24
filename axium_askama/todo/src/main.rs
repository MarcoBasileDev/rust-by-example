use todo::models::app::AppState;
use todo::{init, routes};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind address");

    init::logging();

    let pg_pool = init::database_connection().await;

    let session_layer = init::session(pg_pool.clone()).await;

    let app_state = AppState {
        connection_pool: pg_pool,
    };

    tracing::info!("Server is starting...");
    tracing::info!("Listening on: {}", listener.local_addr().unwrap());

    let app = routes::routes(app_state).layer(session_layer);
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");
}
