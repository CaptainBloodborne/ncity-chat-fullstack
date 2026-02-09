use std::sync::Arc;

use secrecy::ExposeSecret;

use crate::{
    adapters::{api::chat::{chat_controller::CreateChatPayload, chat_presenter::ChatPresenter}, crypto::token::{Token, generate_token, validate_token}},
    application::{
        AppError, AppResult,
        dto::user::{
            CreateNewUserDTO, DeleteUserDTO, GetUserByEmailDTO, GetUserByIdDTO, LoginResponseDTO, LoginUserDTO, ResponseAuthUserDTO, ResponseUserDTO
        },
        repositories::{chat::ChatRepo, hash::Hasher, user::UserRepository},
    }, domain::entities::chat::Chat,
};

pub struct UseCases {
    user_repo: Arc<dyn UserRepository>,
    chat_repo: Arc<dyn ChatRepo>,
    hasher: Arc<dyn Hasher>,
}

impl UseCases {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        chat_repo: Arc<dyn ChatRepo>,
        hasher: Arc<dyn Hasher>,
    ) -> Self {
        Self {
            user_repo,
            chat_repo,
            hasher,
        }
    }

    pub async fn create_user(&self, user_dto: CreateNewUserDTO) -> AppResult<ResponseUserDTO> {
        let password_hash = self.hasher.hash(user_dto.password.to_owned()).await?;

        let user_dto_with_password_hash = user_dto.with_password_hash(password_hash);

        let result = self
            .user_repo
            .create_user(user_dto_with_password_hash)
            .await;

        let user = match result {
            Ok(user) => {
                if let Err(e) = user.validate_email() {
                    println!("failed to create new user: {}", e);

                    return Err(AppError::Domain(e));
                }

                user
            }
            Err(e) => return Err(e),
        };

        let response_dto = user.into();
        Ok(response_dto)
    }

    pub async fn get_user_by_id(&self, user_dto: GetUserByIdDTO) -> AppResult<ResponseUserDTO> {
        let user = self.user_repo.get_user_by_id(user_dto).await?;

        let response_dto = user.into();

        Ok(response_dto)
    }

    pub async fn get_user_for_auth(
        &self,
        user_dto: GetUserByIdDTO,
    ) -> AppResult<ResponseAuthUserDTO> {
        let user = self.user_repo.get_user_by_id(user_dto).await?;

        let response_dto = user.into();

        Ok(response_dto)
    }

    pub async fn delete_user_by_id(&self, user_dto: DeleteUserDTO) -> AppResult<()> {
        self.user_repo.delete_user_by_id(user_dto).await?;

        Ok(())
    }

    pub async fn get_users(&self) -> AppResult<Vec<ResponseUserDTO>> {
        let users = self.user_repo.get_users().await?;

        let response_dto_vec = users.into_iter().map(|u| u.into()).collect();

        Ok(response_dto_vec)
    }

    pub async fn login_user(&self, user_dto: LoginUserDTO) -> AppResult<LoginResponseDTO> {
        let user_email = user_dto.email.clone();
        let user_by_email_dto = GetUserByEmailDTO { email: user_email };

        let user = self.user_repo.get_user_by_email(user_by_email_dto).await?;

        let password_hash = user.get_password_hash();

        self.hasher
            .validate(user_dto.password, password_hash.expose_secret().to_owned())
            .await?;

        let hasher = self.hasher.clone();

        let token = generate_token(hasher, format!("{}", user.get_id()).as_str()).await?;

        Ok(LoginResponseDTO::new(token.to_string(), user.get_id().to_string()))
    }

    pub async fn validate_token(&self, token: &Token) -> AppResult<()> {
        let hasher = self.hasher.clone();

        validate_token(hasher, token).await?;

        Ok(())
    }

    pub async fn add_new_chat(&self, payload: CreateChatPayload) -> AppResult<ChatPresenter> {
        let chat = self.chat_repo.add_chat(payload).await?;

        let chat_present = ChatPresenter{
            id: chat.id,
            name: chat.name,
            description: chat.description,
            users_count: chat.users_count,
            location: chat.location,
        };

        Ok(chat_present)
    }

    pub async fn get_chats(&self) -> AppResult<Vec<Chat>> {
        let chats = self.chat_repo.get_chats().await?;


        Ok(chats)
    }

    pub async fn get_chats_filtered(&self) -> AppResult<Vec<Chat>> {
        todo!()
    }

    pub async fn delete_chat_by_id(&self, id: String) -> AppResult<()> {

        self.chat_repo.delete_chat_by_id(id).await?;

        Ok(())
    }
}
