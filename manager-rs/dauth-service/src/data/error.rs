use std::{error::Error, fmt};

/// General error type for dAuth service failures
#[derive(Debug)]
pub enum DauthError {
    NotFound(String),
    ClientFailure(String),
    _DatabaseFailure(String),
}

impl Error for DauthError {}

impl fmt::Display for DauthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DauthError::NotFound(content) => write!(f, "Auth vector not found: {}", content),
            DauthError::ClientFailure(content) => {
                write!(f, "Failure with client stub: {}", content)
            }
            DauthError::_DatabaseFailure(content) => write!(f, "Failure with database: {}", content),
        }
    }
}
