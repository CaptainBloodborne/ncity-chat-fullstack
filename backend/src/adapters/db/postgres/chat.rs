use async_trait::async_trait;
use sqlx::query_as;
use uuid::Uuid;
use std::str::FromStr;


use crate::{adapters::{api::chat::chat_controller::CreateChatPayload, db::postgres::PostgresChatRepo}, application::{AppResult, repositories::chat::ChatRepo}, domain::entities::chat::Chat};

#[derive(Debug, sqlx::FromRow)]
struct ChatDB{
    id: Uuid,
    name: String,
    users_count: i64,
    location: String,
    description: String,

}

impl From<ChatDB> for Chat {
    fn from(value: ChatDB) -> Self {
        Chat::new(Some(value.id), value.name, value.description, value.users_count as u64, value.location)
    }
}

#[async_trait]
impl ChatRepo for PostgresChatRepo {
    async fn add_chat(&self, payload: CreateChatPayload) -> AppResult<Chat> {
        let chat = sqlx::query_as!(
            ChatDB,
            r#"insert into chats(id,name,users_count,location,description)
            values ($1, $2, $3, $4, $5)
                returning id,name,users_count,location,description"#,
            Uuid::new_v4(),
            payload.name,
            payload.users_count,
            payload.location,
            payload.description,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(chat.into())
    }

    async fn get_chats(&self) -> AppResult<Vec<Chat>> {

        let chats_from_db = query_as!(
            ChatDB,
            r#"
            select *
            from chats
            "#
        ).fetch_all(&self.pool).await?;

        let chats = chats_from_db.into_iter().map(|c| c.into()).collect();

        
        Ok(chats)
    }

    async fn delete_chat_by_id(&self, id: String) -> AppResult<()> {
        let query = "DELETE FROM chats WHERE id = $1";

        let chat_id = Uuid::from_str(&id)?;

        let _res = sqlx::query(
            query
        ).bind(chat_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}