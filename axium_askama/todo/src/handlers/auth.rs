use crate::models::templates::{LoginTemplate, NavItem, SignupTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
pub async fn login_handler() -> impl IntoResponse {
    let html = LoginTemplate {
        title: "Log in",
        current_page: NavItem::Login,
    }
    .render()
    .unwrap();
    Html(html)
}

pub async fn signup_handler() -> impl IntoResponse {
    let html = SignupTemplate {
        title: "Sign up",
        current_page: NavItem::Signup,
    }
    .render()
    .unwrap();
    Html(html)
}
