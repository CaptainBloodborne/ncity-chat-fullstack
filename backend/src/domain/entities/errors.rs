#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Email has invalid format")]
    EmailValidationFailed,

    #[error("Operation not permitted")]
    OperationNotPermitted
}

pub type DomainResult<T> = Result<T, DomainError>;