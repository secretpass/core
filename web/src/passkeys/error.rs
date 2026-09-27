use std::fmt;

#[derive(Debug)]
pub enum PasskeyError {
    InvalidChallenge,
    OriginMismatch { expected: String, got: String },
    InvalidOperationType,
    RpIdHashMismatch,
    InvalidSignature(String),
    UserHandleMismatch,
    SerializationError(serde_json::Error),
    Base64Error(base64::DecodeError),
    InternalError(String),
}

impl fmt::Display for PasskeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChallenge => write!(f, "Invalid challenge"),
            Self::OriginMismatch { expected, got } => {
                write!(f, "Origin mismatch: expected {}, got {}", expected, got)
            }
            Self::InvalidOperationType => write!(f, "Invalid operation type"),
            Self::RpIdHashMismatch => write!(f, "RP ID Hash mismatch"),
            Self::InvalidSignature(e) => write!(f, "Invalid signature: {}", e),
            Self::UserHandleMismatch => write!(f, "User Handle mismatch"),
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Self::Base64Error(e) => write!(f, "Base64 decode error: {}", e),
            Self::InternalError(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl std::error::Error for PasskeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SerializationError(e) => Some(e),
            Self::Base64Error(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for PasskeyError {
    fn from(err: serde_json::Error) -> Self {
        PasskeyError::SerializationError(err)
    }
}

impl From<base64::DecodeError> for PasskeyError {
    fn from(err: base64::DecodeError) -> Self {
        PasskeyError::Base64Error(err)
    }
}

pub type Result<T> = std::result::Result<T, PasskeyError>;
