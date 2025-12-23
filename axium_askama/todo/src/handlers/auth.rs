use crate::models::templates::{LoginTemplate, NavItem, SignupTemplate};
use crate::models::user_form_model::AuthFormModel;
use askama::Template;
use axum::Form;
use axum::response::{Html, IntoResponse, Redirect};
use validator::Validate;

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

pub async fn post_signup_handler(Form(user_form): Form<AuthFormModel>) -> impl IntoResponse {
    match user_form.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(errs) => {
            let errs = errs.to_string();
            tracing::error!("{}", errs);

            Redirect::to("/").into_response()
        }
    }
}
