use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{HomeTemplate, NavItem};
use askama::Template;

pub async fn home() -> impl IntoResponse {
    let html = HomeTemplate {  title: "Home", current_page: NavItem::Home }.render().unwrap();
    Html(html)
}