use crate::data::errors::DataError;
use sqlx::PgPool;

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
            e => DataError::Query(e)
        })?;

    Ok(())
}
