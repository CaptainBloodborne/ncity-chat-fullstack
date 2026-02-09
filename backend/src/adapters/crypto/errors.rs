use tokio::task::JoinError;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    // Argon2
    #[error("Can not match encrypted data: {0}")]
    ValidationFailed(String),

    #[error("Can not encrypt data: {0}")]
    HashingFailed(String),

    // Token
    #[error("Can not match tokens")]
    TokenNotMatch,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Token is in wrong format")]
    TokenExpNotIso,

    #[error("Failed to decode identification part")]
    TokenCannotDecodeIdent,

    #[error("Failed to decode expiration part")]
    TokenCannotDecodeExp,

    #[error("Failed to decode sign part")]
    TokenCannotDecodeSign,

    #[error("Token has wrong format")]
    TokenInvalid,

    // Async Execution
    #[error("Failed to execute async task")]
    AsyncExecutionFailed(#[from] JoinError)
}

pub type CryptoResult<T> = Result<T, CryptoError>;