use std::{error::Error, fmt};

/// General error type for dAuth service failures
#[derive(Debug)]
pub enum DauthError {
    NotFoundError(String),
    ClientError(String),
    _DatabaseError(String),
    ConfigError(String),
    DataError(String),
}

impl Error for DauthError {}

impl fmt::Display for DauthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DauthError::NotFoundError(content) => write!(f, "Not found error -- {}", content),
            DauthError::ClientError(content) => {
                write!(f, "Client error -- {}", content)
            }
            DauthError::_DatabaseError(content) => write!(f, "Database error -- {}", content),
            DauthError::ConfigError(content) => write!(f, "Config error -- {}", content),
            DauthError::DataError(content) => write!(f, "Data error -- {}", content),
        }
    }
}
