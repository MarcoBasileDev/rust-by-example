use crate::data::errors::DataError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error),
}
