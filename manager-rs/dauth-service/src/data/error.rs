use std::array::TryFromSliceError;

use sqlx::Error as SqlxError;
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

    #[error("Database error -- {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Conversion error -- {0}")]
    ConversionError(#[from] TryFromSliceError),

    #[error("Invalid message error -- {0}")]
    InvalidMessageError(String),

    #[error("Invalid UTF8 error -- {0}")]
    InvalidUtf8Error(#[from] std::str::Utf8Error),

    #[error("Error while verifying message -- {0}")]
    SigningError(#[from] ed25519_dalek::SignatureError),

    #[error("Error while decoding message -- {0}")]
    DecodeError(#[from] prost::DecodeError),
}
