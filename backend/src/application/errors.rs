use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{adapters::{api::errors::ClientError, crypto::errors::CryptoError, ctx::CtxError}, domain::entities::errors::DomainError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    //Login
    #[error("Login failed")]
    LoginFail,

    // User
    #[error("Can not find user by id")]
    UserNotFoundByID,

    // Database Error
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    // Crypto Error
    #[error("Crypto operation failed")]
    Crypt(#[from] CryptoError),

    // Context Error
    #[error("Context error")]
    Context(#[from] CtxError),

    // Domain
    #[error("Domain error")]
    Domain(#[from] DomainError),

    // UUID
    #[error("UUID is not valid")]
    UUID(#[from] uuid::Error),

    #[error("Internal server error")]
    Internal,
    
}

impl AppError {
    pub fn get_client_and_status_code(&self) -> (StatusCode, ClientError) {

        match self {
            AppError::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            AppError::Context(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            AppError::UserNotFoundByID => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(Arc::new(self));

        response
        
    }
}
