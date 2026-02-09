use serde::Serialize;
use uuid::Uuid;

use crate::domain::entities::chat::Chat;

#[derive(Debug, Serialize)]
pub struct ChatPresenter {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub users_count: u64,
    pub location: String,
}

impl From<Chat> for ChatPresenter {
    fn from(value: Chat) -> Self {
        Self { 
            id: value.id, 
            name: value.name, 
            description: value.description, 
            users_count: value.users_count, 
            location: value.location,
         }
    }
}