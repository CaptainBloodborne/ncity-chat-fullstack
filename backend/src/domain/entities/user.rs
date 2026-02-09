use secrecy::SecretString;
use uuid::Uuid;

use crate::domain::entities::{chat::{Chat}, errors::{DomainError, DomainResult}};

pub enum UserRole {
    Admin,
    User,
}
pub struct User {
    id: Uuid,
    role: UserRole,
    name: String,
    email: String,
    password_hash: String,
    token_hash: String,
}

impl User {
    pub fn new(
        id: Uuid, 
        name: String,
         email: String,
         role: String,
         password_hash: String, 
         token_hash: String //FIXME: 
    ) -> Self {
        let role = match role.as_str() {
            "admin" => UserRole::Admin,
            _ => UserRole::User,

        };

        Self {id, role, name, email, password_hash, token_hash }
    }

    pub fn get_id(&self) -> &Uuid {
        return &self.id;
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_email(&self) -> &str {
        return &self.email;
    }

    pub fn get_role(&self) -> &UserRole {
        &self.role
    }

    pub fn get_password_hash(&self) -> SecretString {
        return SecretString::from(self.password_hash.clone());
    }

    pub fn get_token_hash(&self) -> SecretString {
        return SecretString::from(self.token_hash.clone());
    }

    pub fn change_role_to_admin(&mut self) {
        self.role = UserRole::Admin;
    }

    pub fn change_role_to_user(&mut self) {
        self.role = UserRole::User;
    }

    pub fn add_chat(
        &self, 
        name: String, 
        description: String,
        users_count: u64,
        location: String,
    ) -> DomainResult<Chat> {
        if let UserRole::User = self.role {
            return Err(DomainError::OperationNotPermitted);
        }

        Ok(Chat::new(None, name, description, users_count, location))
    }

    pub fn validate_email(&self) -> DomainResult<()>{
        if !self.email.contains('@') {
            return Err(DomainError::EmailValidationFailed);
        }

        Ok(())
    }
}
