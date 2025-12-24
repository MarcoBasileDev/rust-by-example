use crate::data::errors::DataError;
use crate::data::user;
use crate::handlers::errors::AppError;
use crate::handlers::helpers;
use crate::models::app::AppState;
use crate::models::templates::{LoginTemplate, NavItem, SignupTemplate};
use crate::models::user_form_model::AuthFormModel;
use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use validator::Validate;

pub async fn login_handler() -> Result<Response, AppError> {
    let html = LoginTemplate {
        title: "Log in",
        current_page: NavItem::Login,
    }
    .render()?;
    Ok(Html(html).into_response())
}

pub async fn signup_handler() -> Result<Response, AppError> {
    let html = SignupTemplate {
        title: "Sign up",
        current_page: NavItem::Signup,
        email: "",
        email_error: "",
        password_error: "",
    }
    .render()?;
    Ok(Html(html).into_response())
}

pub async fn post_signup_handler(
    State(app_state): State<AppState>,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError> {
    match user_form.validate() {
        Ok(_) => {
            let result = user::create_user(
                &app_state.connection_pool,
                &user_form.email,
                &user_form.password,
            )
            .await;

            if let Err(e) = result {
                if let DataError::FailedQuery(msg) = e {
                    tracing::error!("Failed to sign up: {}", msg);

                    return Ok(Redirect::to("/signup").into_response());
                } else {
                    Err(e)?
                }
            }

            Ok(Redirect::to("/login").into_response())
        }
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
            .render()?;

            let response = Html(html_string).into_response();

            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }
}
