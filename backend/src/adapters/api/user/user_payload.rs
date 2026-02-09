use secrecy::SecretString;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NewUserPayload {
    pub name: String,
    pub email: String,
    pub password: SecretString,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserPayload {
    pub email: String,
    pub password: SecretString,
}

#[derive(Debug, Deserialize)]
pub struct UserByIDPayload {
    pub id: String,
}


#[derive(Debug, Deserialize)]
pub struct DeleteUserByIDPayload {
    pub id: String,
}