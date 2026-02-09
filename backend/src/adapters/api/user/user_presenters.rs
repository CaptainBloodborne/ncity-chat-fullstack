use serde::Serialize;

use crate::application::dto::user::ResponseUserDTO;

#[derive(Debug, Serialize)]
pub struct UserPresenter {
    id: String,
    email: String,
    display_name: String,
    role: String,
}

impl From<ResponseUserDTO> for UserPresenter {
    fn from(value: ResponseUserDTO) -> Self {
        Self {
            id: value.id,
            email: value.email,
            display_name: value.name,
            role: value.role,
        }
    }
}
