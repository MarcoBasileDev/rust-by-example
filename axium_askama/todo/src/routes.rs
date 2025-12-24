use crate::handlers::auth::{login_handler, post_login_handler, post_signup_handler, signup_handler};
use crate::handlers::public::home;
use crate::handlers::todos::{create_todo, todos};
use crate::models::app::AppState;
use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use axum::routing::get;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn routes(app_state: AppState) -> Router {
    let server_dir = ServeDir::new("static");

    Router::new()
        .route("/", get(home))
        .route("/create", get(create_todo))
        .route("/todos", get(todos))
        .route("/login", get(login_handler).post(post_login_handler))
        .route("/signup", get(signup_handler).post(post_signup_handler))
        .nest_service("/static", server_dir)
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        )
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
