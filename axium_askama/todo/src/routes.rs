use crate::handlers::auth::{
    login_handler, logout_handler, post_login_handler, post_signup_handler, signup_handler,
};
use crate::handlers::public::{home, page_not_found_handler};
use crate::handlers::todos::{create_todo, todos};
use crate::middlewares::{auth_required, authenticate, redirect_auth_user};
use crate::models::app::AppState;
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Router, middleware};
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn routes(app_state: AppState) -> Router {
    let server_dir = ServeDir::new("static");

    Router::new()
        .route("/", get(home))
        .merge(auth_routes())
        .nest_service("/static", server_dir)
        .merge(protected_routes())
        .fallback(page_not_found_handler)
        .layer(middleware::from_fn(authenticate))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        )
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_handler).post(post_login_handler))
        .route("/signup", get(signup_handler).post(post_signup_handler))
        .layer(middleware::from_fn(redirect_auth_user))
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/create", get(create_todo))
        .route("/todos", get(todos))
        .route("/logout", post(logout_handler))
        .route_layer(middleware::from_fn(auth_required))
}

fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "Request stated: method {} path {}",
        request.method(),
        request.uri().path()
    );
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "Response generated: status {} in {:?}",
        response.status(),
        latency
    );
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("Request failed: {:?} after {:?}", error, latency);
}
