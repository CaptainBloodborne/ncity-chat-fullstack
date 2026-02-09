use async_trait::async_trait;

use crate::{application::{AppResult, dto::user::{CreateNewUserDTO, DeleteUserDTO, GetUserByIdDTO, GetUserByEmailDTO}}, domain::entities::user::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user_dto: CreateNewUserDTO) -> AppResult<User>;

    async fn get_user_by_email(&self, user_dto: GetUserByEmailDTO) -> AppResult<User>;
    
    async fn get_user_by_id(&self, user_dto: GetUserByIdDTO) -> AppResult<User>;

    async fn get_users(&self) -> AppResult<Vec<User>>;

    async fn delete_user_by_id(&self, user_dto: DeleteUserDTO) -> AppResult<()>;

}