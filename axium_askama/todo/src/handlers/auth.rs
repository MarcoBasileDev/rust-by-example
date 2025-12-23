use crate::models::templates::{LoginTemplate, NavItem, SignupTemplate};
use crate::models::user_form_model::AuthFormModel;
use askama::Template;
use axum::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use validator::Validate;
use crate::handlers::helpers;

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
        email: "",
        email_error: "",
        password_error: "",
    }
    .render()
    .unwrap();
    Html(html)
}

pub async fn post_signup_handler(Form(user_form): Form<AuthFormModel>) -> impl IntoResponse {
    match user_form.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(errs) => {
            let errs = errs.to_string();

            let mut email_error = String::new();
            let mut password_error = String::new();

            helpers::extract_error(&errs, |field, message| {
                if field == "email" {
                    email_error = message;
                } else if field == "password" {
                    password_error = message
                }
            });

            let html_string = SignupTemplate {
                title: "Sign up",
                current_page: NavItem::Signup,
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
            }
                .render()
                .unwrap();

            let response = Html(html_string).into_response();

            (StatusCode::BAD_REQUEST, response).into_response()
        }
    }
}

