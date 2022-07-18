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

    #[error("Conversion error -- {0}")]
    AuthConversionError(#[from] auth_vector::types::AuthVectorConversionError),

    #[error("Invalid message error -- {0}")]
    InvalidMessageError(String),

    #[error("Invalid UTF8 error -- {0}")]
    InvalidUtf8Error(#[from] std::str::Utf8Error),

    #[error("Error while verifying message -- {0}")]
    SigningError(#[from] ed25519_dalek::SignatureError),

    #[error("Error while decoding message -- {0}")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Error while encoding message -- {0}")]
    EncodeError(#[from] prost::EncodeError),

    #[error("Error while generating shamir share")]
    ShamirShareError(),

    #[error("Tonic transport error -- {0}")]
    TransportError(#[from] tonic::transport::Error),

    #[error("Tonic status -- {0}")]
    StatusError(#[from] tonic::Status),

    #[error("Received incorrect key type for 4G/5G context -- {0}")]
    KeyTypeError(String),

    #[error("Unable to make filesystem write {0}")]
    IoError(#[from] std::io::Error),
}
