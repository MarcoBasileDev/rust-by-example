use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{CreateTemplate, NavItem, TodosTemplate};
use askama::Template;

pub async fn create_todo() -> impl IntoResponse {
    let html = CreateTemplate  { title: "Create", current_page: NavItem::Create, }.render().unwrap();
    Html(html)
}

pub async fn todos() -> impl IntoResponse {
    let html = TodosTemplate { title: "List", current_page: NavItem::Todos, }.render().unwrap();
    Html(html)
}