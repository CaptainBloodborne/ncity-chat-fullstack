use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub users_count: u64,
    pub location: String,
}

impl Chat {
    pub fn new(
        id: Option<Uuid>,
        name: String,
        description: String,
        users_count: u64,
        location: String,
    ) -> Self {
        let id = match id  {
            Some(id) => id,
            None => {
                Uuid::new_v4()
            }
        };

        Self{ id, name, description, users_count, location }
    }
    
}
