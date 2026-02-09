use axum::extract::FromRequestParts;

use crate::application::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: String,
    user_role: String,
}

impl Ctx {
    pub fn new(user_id: String, user_role: String ) -> Self {
        Self {user_id, user_role }
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_user_role(&self) -> &str {
        &self.user_role
    }

}


impl<S:Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = AppError;

    async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            _state: &S,
        ) -> AppResult<Self> {
            parts
            .extensions
            .get::<CtxResult>()
            .ok_or(CtxError::CtxNotInRequest)?
            .clone()
            .map_err(|e| AppError::Context(e))

    }

}

pub type CtxResult = Result<Ctx, CtxError>;


#[derive(Debug, Clone, thiserror::Error)]
pub enum CtxError {
    #[error("Context not in request extensions")]
    CtxNotInRequest,

    #[error("Token not in coolies")]
    NoTokenInCookies,
    
    #[error("Can not parse token")]
    TokenBadFormat,

    #[error("User not found")]
    UserNotFound,

    #[error("Failed to validate token")]
    ValidationFail,

    #[error("User has no admin privileges")]
    UserNotAdmin,
}