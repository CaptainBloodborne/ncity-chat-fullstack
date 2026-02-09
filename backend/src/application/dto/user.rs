use secrecy::{ExposeSecret, SecretString};

use crate::{adapters::api::user::user_payload::{DeleteUserByIDPayload, LoginUserPayload, NewUserPayload, UserByIDPayload}, domain::entities::user::{User, UserRole}};

pub struct CreateNewUserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateNewUserDTO {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password,
        }
    }

    pub fn with_password_hash(self, password_hash: String) -> Self {
        Self { name: self.name, email: self.email, password: password_hash }
    }
}

impl From<NewUserPayload> for CreateNewUserDTO {
    fn from(value: NewUserPayload) -> Self {
        Self { name: value.name, email: value.email, password: value.password.expose_secret().to_string() }
    }
}

pub struct LoginUserDTO {
    pub email: String,
    pub password: String,
}

impl LoginUserDTO {
    pub fn new(email: String, password: String) -> Self {
        Self {
            email,
            password,
        }
    }

    pub fn with_password_hash(self, password_hash: String) -> Self {
        Self {email: self.email, password: password_hash }
    }
}

impl From<LoginUserPayload> for LoginUserDTO {
    fn from(value: LoginUserPayload) -> Self {
        Self { email: value.email, password: value.password.expose_secret().to_string() }
    }
}

pub struct ResponseUserDTO {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

impl From<User> for ResponseUserDTO {
    fn from(value: User) -> Self {
        let role = value.get_role();

        let role_string = {
            match role {
                UserRole::Admin => "admin".to_string(),
                UserRole::User => "user".to_string(),
            }
        };

        Self { 
            id: value.get_id().to_string(), 
            name: value.get_name().to_string(), 
            email: value.get_email().to_string(), 
            role: role_string,
         }
    }
}


impl ResponseUserDTO {
    pub fn new(id: String, name: String, email: String, role: String) -> Self {
        Self { id, name, email, role }
    }
}

pub struct ResponseAuthUserDTO {
    pub id: String,
    pub role: String,
    pub token_hash: SecretString,
}

impl From<User> for ResponseAuthUserDTO {
    fn from(value: User) -> Self {
        let role = match value.get_role() {
            UserRole::Admin => "admin".to_string(),
            UserRole::User => "user".to_string(),
        }; 

        Self { 
            id: value.get_id().to_string(), 
            role,
            token_hash: value.get_password_hash()
         }
    }
}


impl ResponseAuthUserDTO {
    pub fn new(id: String, role: String, token_hash: SecretString) -> Self {
        Self { id, role,   token_hash}
    }
}

pub struct LoginResponseDTO {
    pub token: String,
    pub user_id: String,
}

impl LoginResponseDTO  {
    pub fn new(token: String, user_id: String ) -> Self {
        Self { token: token, user_id }
    }
}


pub struct GetUserByIdDTO {
    pub id: String,
}

impl GetUserByIdDTO {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl From<UserByIDPayload> for GetUserByIdDTO {
    fn from(value: UserByIDPayload) -> Self {
        Self { id: value.id }
    }
}

pub struct GetUserByEmailDTO {
    pub email: String,
}

impl GetUserByEmailDTO {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

pub struct UpdateUserDTO {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl UpdateUserDTO {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

pub struct DeleteUserDTO {
    pub id: String,
}

impl DeleteUserDTO {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl From<DeleteUserByIDPayload> for DeleteUserDTO {
    fn from(value: DeleteUserByIDPayload) -> Self {
        DeleteUserDTO { id: value.id }
    }
}
