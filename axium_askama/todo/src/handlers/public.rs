use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;
use crate::models::templates::{HomeTemplate, NavItem, PageNotFoundTemplate};
use askama::Template;
use axum::Extension;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub async fn home(Extension(current_user): Extension<CurrentUser>) -> Result<Response, AppError> {
    let html = HomeTemplate {
        title: "Home",
        current_page: NavItem::Home,
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;
    Ok(Html(html).into_response())
}

pub async fn page_not_found_handler() -> Result<Response, AppError> {
    let html = PageNotFoundTemplate {
        title: "404 Not Found",
    }
    .render()?;

    Ok((StatusCode::NOT_FOUND, Html(html)).into_response())
}
