use crate::models::templates::{LoginTemplate, NavItem, SignupTemplate};
use crate::models::user_form_model::UserFormModel;
use askama::Template;
use axum::Form;
use axum::response::{Html, IntoResponse, Redirect};

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

pub async fn post_signup_handler(Form(user_form): Form<UserFormModel>) -> impl IntoResponse {
    tracing::info!(
        "Email is {} and the password is {}",
        user_form.email,
        user_form.password
    );
    Redirect::to("/").into_response()
}
