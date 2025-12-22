use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{HomeTemplate};
use askama::Template;

pub async fn home() -> impl IntoResponse {
    let html = HomeTemplate {  title: "Home" }.render().unwrap();
    Html(html)
}