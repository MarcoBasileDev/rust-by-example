use crate::data::errors::DataError;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
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
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(e) => {
            if e.constraint() == Some("users_email_key") {
                DataError::FailedQuery("Email already exists".to_string())
            } else {
                DataError::Internal(e.to_string())
            }
        }
        e => DataError::Query(e),
    })?;

    Ok(())
}

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<i32, DataError> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => DataError::FailedQuery("Invalid credentials".to_string()),
        e => DataError::Query(e),
    })?;

    let hashed_password = String::from_utf8(user.password_hash)?;
    let is_password_valid = bcrypt::verify(password, &hashed_password)?;

    if !is_password_valid {
        Err(DataError::FailedQuery("Invalid credentials".to_string()))
    } else {
        Ok(user.id)
    }
}
