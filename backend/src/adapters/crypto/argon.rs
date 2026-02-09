use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use async_trait::async_trait;

use crate::{adapters::crypto::errors::{CryptoError, CryptoResult}, application::{repositories::hash::Hasher}};

pub struct ArgonHasher {}

impl ArgonHasher {
    pub fn new() -> Self {

        Self { }
    }
}

#[async_trait]
impl Hasher for ArgonHasher {
    async fn hash(&self, content: String) -> CryptoResult<String> {
        let result: CryptoResult<String> = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);

            let hasher = Argon2::default();

            let password_hash = hasher
                .hash_password(content.as_bytes(), &salt)
                .map_err(|e| CryptoError::HashingFailed(e.to_string()))?
                .to_string();

            Ok(password_hash)

        }).await?;

        result
    }

    async fn validate(&self, content: String, hash: String) -> CryptoResult<()> {
        let result: CryptoResult<()> = tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash)
                .map_err(|e| CryptoError::ValidationFailed(e.to_string()))?;

            let hasher = Argon2::default();

            hasher
                .verify_password(content.as_bytes(), &parsed_hash)
                .map_err(|_| CryptoError::ValidationFailed("content not match".to_string()))?;

            Ok(())
        })
        .await?;

        result
    }
}
