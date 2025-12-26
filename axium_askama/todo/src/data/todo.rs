use crate::data::errors::DataError;
use sqlx::PgPool;

pub async fn create(pool: &PgPool, task: &str, author_id: &i32) -> Result<(), DataError> {
    sqlx::query!(
        r#"INSERT INTO todos (task, author_id) VALUES ($1, $2)"#,
        task,
        author_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
