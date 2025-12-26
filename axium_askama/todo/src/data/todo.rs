use super::errors::DataError;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(sqlx::FromRow, serde::Deserialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
}

pub async fn create(pool: &PgPool, task: &str, author_id: &i32) -> Result<(), DataError> {
    sqlx::query!(
        r#"INSERT INTO todos (task, author_id) VALUES ($1, $2)"#,
        task,
        author_id
    )
    .execute(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(e) => {
            if e.constraint() == Some("todo_task_key") {
                DataError::FailedQuery("This task already exists.".to_string())
            } else {
                DataError::Internal(e.to_string())
            }
        }
        e => DataError::Query(e),
    })?;

    Ok(())
}

pub async fn get_all(pool: &PgPool, author_id: &i32) -> Result<Vec<Todo>, DataError> {
    let todos: Vec<Todo> = sqlx::query_as(
        "SELECT id, task, is_done, created_at
        FROM todos WHERE author_id = $1
        ORDER BY todos.created_at DESC",
    )
    .bind(author_id)
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn set_as_done(pool: &PgPool, todo_id: &i32, is_done: &bool) -> Result<(), DataError> {
    sqlx::query!(
        r#"UPDATE todos SET is_done = $1 WHERE id = $2"#,
        is_done,
        todo_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, todo_id: &i32) -> Result<(), DataError> {
    sqlx::query!(r#"DELETE FROM todos WHERE id = $1"#, todo_id)
        .execute(pool)
        .await?;

    Ok(())
}
