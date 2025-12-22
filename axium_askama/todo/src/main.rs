use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let app = Router::new().route("/", get(home));

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> impl IntoResponse {
    let html = HomeTemplate{
        title: "Todo App | Home",
        }
        .render()
        .unwrap();

    Html(html)
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {
    title: &'static str,
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
struct TodosTemplate {
    title: &'static str,
}