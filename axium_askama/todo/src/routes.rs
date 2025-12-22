use crate::handlers::auth::{login_handler, signup_handler};
use crate::handlers::public::home;
use crate::handlers::todos::{create_todo, todos};
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_todo))
        .route("/todos", get(todos))
        .route("/login", get(login_handler))
        .route("/signup", get(signup_handler))
        .nest_service("/static", server_dir);

    app
}
