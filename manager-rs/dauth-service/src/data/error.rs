use std::array::TryFromSliceError;

use thiserror::Error;

/// General error type for dAuth service failures
#[derive(Error, Debug)]
pub enum DauthError {
    #[error("Not found error -- {0}")]
    NotFoundError(String),

    #[error("Client error -- {0}")]
    ClientError(String),

    #[error("Config error -- {0}")]
    ConfigError(String),

    #[error("Data error -- {0}")]
    DataError(String),

    #[error("Conversion error -- {0}")]
    ConversionError(#[from] TryFromSliceError),

    #[error("Invalid message error -- {0}")]
    InvalidMessageError(String),

    #[error("Invalid UTF8 error -- {0}")]
    InvalidUtf8Error(#[from] std::str::Utf8Error),
}
