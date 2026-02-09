use async_trait::async_trait;

use crate::adapters::crypto::errors::CryptoResult;

#[async_trait]
pub trait Hasher: Send + Sync {
    async fn hash(&self, content: String) -> CryptoResult<String>;
    async fn validate(&self, content: String, hash: String ) -> CryptoResult<()>;
}