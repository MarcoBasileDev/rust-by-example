use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_todo))
        .route("/todos", get(todos))
        .route("/login", get(login_handler))
        .route("/signup", get(signup_handler))
        .nest_service("/static", server_dir);

    axum::serve(listener, app).await.unwrap();
}
struct HtmlTemplate<T>(T);

impl<T: askama::Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {  title: "Home" })
}

async fn create_todo() -> impl IntoResponse {
    HtmlTemplate(CreateTemplate  { title: "Create" })
}

async fn todos() -> impl IntoResponse {
    HtmlTemplate(TodosTemplate { title: "List" })
}

async fn login_handler() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate { title: "Log in" })
}

async fn signup_handler() -> impl IntoResponse {
    HtmlTemplate(SignupTemplate { title: "Sign up" })
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
struct TodosTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "pages/create.html")]
struct CreateTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
struct LoginTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "pages/sign-up.html")]
struct SignupTemplate<'a> {
    title: &'a str,
}

