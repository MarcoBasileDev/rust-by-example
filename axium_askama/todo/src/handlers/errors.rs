use thiserror::Error;
use crate::data::errors::DataError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error)
}