use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub connection_pool: PgPool,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub is_authenticated: bool,
    pub user_id: Option<i32>,
}

pub struct FlashData {
    pub flash: String,
    pub flash_status: String,
}

pub enum FlashStatus {
    Error,
    Info,
    Success,
}