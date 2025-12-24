use crate::data::errors::DataError;
use crate::models::templates;
use askama::Template;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error),

    #[error("Failer loading session")]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, response) = match self {
            AppError::Database(e) => server_error(e.to_string()),
            AppError::Template(e) => server_error(e.to_string()),
            AppError::Session(e) => server_error(e.to_string()),
        };

        (status, response).into_response()
    }
}

fn server_error(e: String) -> (StatusCode, Response<Body>) {
    tracing::error!("Server error: {}", e);

    let html = templates::ServerErrorTemplate {
        title: "Server Error",
    }
    .render()
    .unwrap();
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(html).into_response(),
    )
}
