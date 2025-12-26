use crate::data::todo;
use crate::handle_client_error;
use crate::handlers::errors::AppError;
use crate::handlers::helpers;
use crate::models::app::{AppState, CurrentUser, FlashStatus};
use crate::models::templates::{CreateTemplate, NavItem, TodosTemplate};
use crate::models::todo_form_model::CreateTodoFormModel;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Extension, Form};
use tower_sessions::Session;

pub async fn create_todo(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let html = CreateTemplate {
        title: "Create",
        current_page: NavItem::Create,
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html).into_response())
}

pub async fn todos(
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    State(app_state): State<AppState>,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;
    let user_id = current_user.user_id.unwrap();

    let todos = todo::get_all(&app_state.connection_pool, &user_id).await?;

    let html = TodosTemplate {
        title: "List",
        current_page: NavItem::Todos,
        is_authenticated: current_user.is_authenticated,
        flash_data,
        todos,
    }
    .render()?;

    Ok(Html(html).into_response())
}

pub async fn post_create_todo_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
    State(app_state): State<AppState>,
    Form(create_todo_form): Form<CreateTodoFormModel>,
) -> Result<Response, AppError> {
    let user_id = current_user.user_id.unwrap();

    let result = todo::create(&app_state.connection_pool, &create_todo_form.task, &user_id).await;

    handle_client_error!(result, &session, Redirect::to("/todos").into_response());

    session.insert("flash", "Todo created successfully").await?;
    session
        .insert("flash_status", FlashStatus::Success.to_string())
        .await?;
    Ok(Redirect::to("/todos").into_response())
}
