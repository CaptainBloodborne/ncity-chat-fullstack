use async_trait::async_trait;

use crate::{adapters::api::chat::chat_controller::CreateChatPayload, application::AppResult, domain::entities::chat::Chat};

#[async_trait]
pub trait ChatRepo: Send + Sync {
    async fn add_chat(&self, payload: CreateChatPayload) -> AppResult<Chat>;
    async fn get_chats(&self) -> AppResult<Vec<Chat>>;
    async fn delete_chat_by_id(&self, id: String) -> AppResult<()>;
}