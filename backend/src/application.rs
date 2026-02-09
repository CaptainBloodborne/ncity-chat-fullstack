pub mod use_cases;
pub mod repositories;
pub mod dto;
pub mod errors;

pub use self::errors::AppError;

pub type AppResult<T, E = AppError> = Result<T, E>;