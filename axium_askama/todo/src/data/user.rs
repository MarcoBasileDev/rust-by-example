use sqlx::PgPool;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    .await?;

    Ok(())
}
