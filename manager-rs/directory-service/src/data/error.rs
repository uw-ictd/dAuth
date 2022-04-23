use sqlx::Error as SqlxError;
use thiserror::Error;

/// General error type for directory service failures
#[derive(Error, Debug)]
pub enum DirectoryError {
    #[error("Database error -- {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Invalid access -- {0}")]
    InvalidAccess(String),

    #[error("Config error -- {0}")]
    ConfigError(String),
}
