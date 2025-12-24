use sqlx::PgPool;
use crate::data::errors::DataError;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<(), DataError> {
    let hashed_psw = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    let bytea_hash = hashed_psw.as_bytes();

    // It's possible to execute the query like this
    // let query = r#"INSERT INTO users (email, password_hash) VALUES ($1, $2)"#;
    // sqlx::query(&query).bind(email).bind(bytea_hash).execute(pool).await?;

    // Or like this
    sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        email,
        bytea_hash
    )
    .execute(pool)
    .await.map_err(|e| {
        match e {
            sqlx::Error::Database(e) => {
                if e.constraint() == Some("users_email_key") {
                    DataError::FailedQuery("Email already exists".to_string())
                } else {
                    DataError::Internal(e.to_string())
                }
            },
            e => DataError::Query(e),
        }
    })?;

    Ok(())
}
