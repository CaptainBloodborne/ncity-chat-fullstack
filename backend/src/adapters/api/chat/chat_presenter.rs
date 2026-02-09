use serde::Serialize;
use uuid::Uuid;

use crate::domain::entities::chat::Chat;

#[derive(Debug, Serialize)]
pub struct ChatPresenter {
    id: Uuid,
    name: String,
    description: String,
    users_count: u64,
    location: String,
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