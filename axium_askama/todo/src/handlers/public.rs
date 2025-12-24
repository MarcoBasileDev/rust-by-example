use crate::models::templates::{HomeTemplate, NavItem};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use crate::handlers::errors::AppError;

pub async fn home() -> Result<Response, AppError> {
    let html = HomeTemplate {
        title: "Home",
        current_page: NavItem::Home,
    }
    .render()?;
    Ok(Html(html).into_response())
}
