use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;
use crate::models::templates::{HomeTemplate, NavItem};
use askama::Template;
use axum::Extension;
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
