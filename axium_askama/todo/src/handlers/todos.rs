use crate::handlers::errors::AppError;
use crate::models::templates::{CreateTemplate, NavItem, TodosTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn create_todo() -> Result<Response, AppError> {
    let html = CreateTemplate {
        title: "Create",
        current_page: NavItem::Create,
    }
    .render()?;

    Ok(Html(html).into_response())
}

pub async fn todos() -> Result<Response, AppError> {
    let html = TodosTemplate {
        title: "List",
        current_page: NavItem::Todos,
    }
    .render()?;

    Ok(Html(html).into_response())
}
