use crate::data::todo;
use crate::handle_client_error;
use crate::handlers::errors::AppError;
use crate::handlers::functions::{next_page, previous_page};
use crate::handlers::helpers;
use crate::models::app::{AppState, CurrentUser, FlashStatus};
use crate::models::templates::{CreateTemplate, NavItem, TodosTemplate};
use crate::models::todo_form_model::{CreateTodoFormModel, MarkTodoAsDoneFormModel, TodoPageQuery};
use askama::Template;
use axum::extract::{Path, Query, State};
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
    Path(page): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;
    let user_id = current_user.user_id.unwrap();

    let page_size = 3;
    let total_todos = todo::get_total_todos(&app_state.connection_pool).await?;
    let total_pages = (total_todos + page_size - 1) / page_size;

    let todos = todo::get_all(&app_state.connection_pool, &user_id, &page_size, &page).await?;

    let html = TodosTemplate {
        title: "List",
        current_page: NavItem::Todos,
        is_authenticated: current_user.is_authenticated,
        flash_data,
        todos,
        current_page_number: page,
        total_pages,
        next_page,
        previous_page,
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

    handle_client_error!(result, &session, Redirect::to("/todos/1").into_response());

    session.insert("flash", "Todo created successfully").await?;
    session
        .insert("flash_status", FlashStatus::Success.to_string())
        .await?;
    Ok(Redirect::to("/todos/1").into_response())
}

pub async fn set_as_done_todo_handler(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Query(todo_page_query): Query<TodoPageQuery>,
    Form(todo_form): Form<MarkTodoAsDoneFormModel>,
) -> Result<Response, AppError> {
    todo::set_as_done(&app_state.connection_pool, &id, &todo_form.is_done).await?;

    let path = format!("/todos/{}", todo_page_query.page);

    Ok(Redirect::to(&path).into_response())
}

pub async fn delete_todo_handler(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Query(todo_page_query): Query<TodoPageQuery>,
) -> Result<Response, AppError> {
    todo::delete(&app_state.connection_pool, &id).await?;

    let path = format!("/todos/{}", todo_page_query.page);

    Ok(Redirect::to(&path).into_response())
}
