use todo::routes;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let app = routes::routes();
    axum::serve(listener, app).await.unwrap();
}