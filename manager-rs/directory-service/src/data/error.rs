use std::array::TryFromSliceError;

use sqlx::Error as SqlxError;
use thiserror::Error;

/// General error type for directory service failures
#[derive(Error, Debug)]
pub enum DirectoryError {
    #[error("Not found error -- {0}")]
    NotFoundError(String),

    #[error("Config error -- {0}")]
    ConfigError(String),

    #[error("Data error -- {0}")]
    DataError(String),

    #[error("Database error -- {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Conversion error -- {0}")]
    ConversionError(#[from] TryFromSliceError),

    #[error("Invalid access -- {0}")]
    InvalidAccess(String),
}
