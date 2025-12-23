use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthFormModel {
    pub email: String,
    pub password: String,
}
