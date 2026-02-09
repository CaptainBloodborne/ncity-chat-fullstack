use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::{AppResult, dto::user::{CreateNewUserDTO, DeleteUserDTO}, repositories::user::UserRepository},
    domain::entities::user::User,
};

use super::PostgresUserRepo;

#[derive(sqlx::FromRow, Debug)]
struct UserDB {
    id: uuid::Uuid,
    name: String,
    email: String,
    role: String,
    password_hash: String,
    token_salt: Option<String>,
}

impl From<UserDB> for User {
    fn from(value: UserDB) -> Self {
        let token_hash = value.token_salt.unwrap_or(String::from("token")); // FIXME Handle cases when no token in db

        User::new(value.id, value.name, value.email, value.role, value.password_hash, token_hash)
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepo {
    async fn create_user(&self, user_dto: CreateNewUserDTO) -> AppResult<User> {
        let token: Option<String> = None;

        let user = sqlx::query_as!(
            UserDB,
            r#"insert into users(id,name,email,role,password_hash,token_salt)
            values ($1, $2, $3, $4, $5, $6)
                returning id,name,email,role,password_hash,token_salt"#,
            Uuid::new_v4(),
            user_dto.name,
            user_dto.email,
            "user".to_string(),
            user_dto.password,
            token,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user.into())
    }

    async fn get_user_by_id(
        &self,
        user_dto: crate::application::dto::user::GetUserByIdDTO,
    ) -> AppResult<User> {
        let user_id = uuid::Uuid::from_str(&user_dto.id)?;

        let user = sqlx::query_as!(
            UserDB,
            r#"select *
            from users
            where id = $1"#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user.into())
    }

    async fn get_user_by_email(
        &self,
        user_dto: crate::application::dto::user::GetUserByEmailDTO,
    ) -> AppResult<User> {
        let user = sqlx::query_as!(
            UserDB,
            r#"select *
            from users
            where email = $1"#,
            user_dto.email,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user.into())
    }

    async fn get_users(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            UserDB,
            r#"select *
            from users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

    let users_results = users.into_iter().map(|u| u.into()).collect();

    Ok(users_results)
    }

    async fn delete_user_by_id(&self, user_dto: DeleteUserDTO) -> AppResult<()> {
        let query = "DELETE FROM users WHERE id = $1";

        let user_id = Uuid::from_str(&user_dto.id)?;

        sqlx::query(
            query
        ).bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
