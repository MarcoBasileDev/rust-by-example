use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_todo))
        .route("/todos", get(todos))
        .route("/login", get(login_handler));

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> impl IntoResponse {
    let html = HomeTemplate {
        title: "Todo App | Home",
        }
        .render()
        .unwrap();

    Html(html)
}

async fn create_todo() -> impl IntoResponse {
    let html = CreateTemplate  {
        title: "Todo App | Create",
        }
        .render()
        .unwrap();

    Html(html)
}

async fn todos() -> impl IntoResponse {
    let html = TodosTemplate {
        title: "Todo App | List",
    }
        .render()
        .unwrap();

    Html(html)
}

async fn login_handler() -> impl IntoResponse {
    let html = LoginTemplate {
        title: "Todo App | Login",
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

#[derive(Template)]
#[template(path = "pages/create.html")]
struct CreateTemplate {
    title: &'static str,
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
struct LoginTemplate {
    title: &'static str,
}

