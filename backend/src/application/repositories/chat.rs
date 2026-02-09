use async_trait::async_trait;

use crate::{application::AppResult, domain::entities::chat::Chat};

#[async_trait]
pub trait ChatRepo: Send + Sync {
    async fn get_chats(&self) -> AppResult<Vec<Chat>>;
    async fn delete_chat_by_id(&self, id: String) -> AppResult<()>;
}