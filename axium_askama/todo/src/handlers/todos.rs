use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;
use crate::models::templates::{CreateTemplate, NavItem, TodosTemplate};
use askama::Template;
use axum::Extension;
use axum::response::{Html, IntoResponse, Response};

pub async fn create_todo(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let html = CreateTemplate {
        title: "Create",
        current_page: NavItem::Create,
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html).into_response())
}

pub async fn todos(Extension(current_user): Extension<CurrentUser>) -> Result<Response, AppError> {
    let html = TodosTemplate {
        title: "List",
        current_page: NavItem::Todos,
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html).into_response())
}
