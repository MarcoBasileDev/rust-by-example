use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{CreateTemplate, TodosTemplate};
use askama::Template;

pub async fn create_todo() -> impl IntoResponse {
    let html = CreateTemplate  { title: "Create" }.render().unwrap();
    Html(html)
}

pub async fn todos() -> impl IntoResponse {
    let html = TodosTemplate { title: "List" }.render().unwrap();
    Html(html)
}