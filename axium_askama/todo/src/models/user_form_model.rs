use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)*\.[A-Za-z]{2,}$").unwrap()
});

#[derive(Deserialize, Validate)]
pub struct AuthFormModel {
    #[validate(regex(path = "EMAIL_REGEX", message = "Email is invalid"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 16,
        message = "Password must be between 8 and 16 characters"
    ))]
    pub password: String,
}
