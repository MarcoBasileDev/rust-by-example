use tower_sessions::Session;
use crate::handlers::errors::AppError;
use crate::models::app::FlashData;

pub fn extract_error<F>(input: &str, mut f: F)
where
    F: FnMut(String, String),
{
    let lines = input.lines();

    lines.for_each(|line| {
        // email: invalid email
        if let Some((first, second)) = line.split_once(": ") {
            f(first.to_string(), second.to_string());
        };
    });
}

pub async fn template_flash(session: &Session) -> Result<FlashData, AppError> {
    let flash = session
        .remove::<String>("flash")
        .await?
        .unwrap_or("".to_string());

    let flash_status = session
        .remove::<String>("flash_status")
        .await?
        .unwrap_or("".to_string());

    Ok(FlashData { flash, flash_status })
}