use crate::data::errors::DataError;
use crate::data::user;
use crate::handlers::errors::AppError;
use crate::handlers::helpers;
use crate::models::app::{AppState, CurrentUser, FlashStatus};
use crate::models::templates::{LoginTemplate, NavItem, SignupTemplate};
use crate::models::user_form_model::AuthFormModel;
use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Extension, Form};
use tower_sessions::Session;
use validator::Validate;

pub async fn login_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;

    let html = LoginTemplate {
        title: "Log in",
        current_page: NavItem::Login,
        email: "",
        email_error: "",
        password_error: "",
        is_authenticated: current_user.is_authenticated,
        flash_data,
    }
    .render()?;
    Ok(Html(html).into_response())
}

pub async fn signup_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;

    let html = SignupTemplate {
        title: "Sign up",
        current_page: NavItem::Signup,
        email: "",
        email_error: "",
        password_error: "",
        is_authenticated: current_user.is_authenticated,
        flash_data,
    }
    .render()?;
    Ok(Html(html).into_response())
}

pub async fn post_signup_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
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
                    helpers::set_flash(&session, msg, FlashStatus::Error.to_string()).await?;
                    return Ok(Redirect::to("/signup").into_response());
                } else {
                    Err(e)?
                }
            }

            helpers::set_flash(
                &session,
                "Account created successfully!".to_string(),
                FlashStatus::Success.to_string(),
            )
            .await?;
            Ok(Redirect::to("/login").into_response())
        }
        Err(errs) => {
            let (email_error, password_error) = auth_validation_errors(errs);
            let flash_data = helpers::get_flash(&session).await?;

            let html_string = SignupTemplate {
                title: "Sign up",
                current_page: NavItem::Signup,
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
                is_authenticated: current_user.is_authenticated,
                flash_data,
            }
            .render()?;

            let response = Html(html_string).into_response();

            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }
}

pub async fn post_login_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(app_state): State<AppState>,
    session: Session,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError> {
    match user_form.validate() {
        Ok(_) => {
            let user_id = user::authenticate_user(
                &app_state.connection_pool,
                &user_form.email,
                &user_form.password,
            )
            .await;
            match user_id {
                Ok(user_id) => {
                    session.insert("authenticated_user_id", user_id).await?;
                    Ok(Redirect::to("/todos").into_response())
                }
                Err(err) => {
                    if let DataError::FailedQuery(e) = err {
                        helpers::set_flash(&session, e, FlashStatus::Error.to_string()).await?;

                        Ok(Redirect::to("/login").into_response())
                    } else {
                        Err(err)?
                    }
                }
            }
        }
        Err(errs) => {
            let (email_error, password_error) = auth_validation_errors(errs);
            let flash_data = helpers::get_flash(&session).await?;

            let html_string = LoginTemplate {
                title: "Log in",
                current_page: NavItem::Signup,
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
                is_authenticated: current_user.is_authenticated,
                flash_data,
            }
            .render()?;

            let response = Html(html_string).into_response();

            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }
}

pub async fn logout_handler(session: Session) -> Result<Response, AppError> {
    session.remove::<i32>("authenticated_user_id").await?;

    Ok(Redirect::to("/").into_response())
}

fn auth_validation_errors(errs: validator::ValidationErrors) -> (String, String) {
    let errs = errs.to_string();

    let mut email_error = String::new();
    let mut password_error = String::new();

    helpers::extract_error(&errs, |field, message| {
        if field == "email" {
            email_error = message;
        } else if field == "password" {
            password_error = message;
        }
    });

    (email_error, password_error)
}
