use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{LoginTemplate, SignupTemplate};
use askama::Template;
pub async fn login_handler() -> impl IntoResponse {
    let html = LoginTemplate { title: "Log in" }.render().unwrap();
    Html(html)
}

pub async fn signup_handler() -> impl IntoResponse {
    let html = SignupTemplate { title: "Sign up" }.render().unwrap();
    Html(html)
}