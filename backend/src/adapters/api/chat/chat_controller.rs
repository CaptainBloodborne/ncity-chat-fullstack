use axum::{Json, Router, extract::State, middleware, routing::{delete, get, post}};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{adapters::api::{app_state::AppState, chat::chat_presenter::ChatPresenter, middlewares}, application::AppResult};

pub fn chat_router() -> Router<AppState> {
    let router = Router::new()
        .route("/api/admin/chat", post(add_new_chat))
        .route("/api/admin/chat", delete(delete_chat))
        .route("/api/chats", get(get_chats))
        .route_layer(middleware::from_fn(middlewares::require_auth));

    router
}

#[derive(Debug, Deserialize)]
pub struct CreateChatPayload {
    pub name: String,
    pub users_count: i64,
    pub location: String,
    pub description: String,
}

async fn add_new_chat(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateChatPayload>
) -> AppResult<Json<ChatPresenter>> {
    let chat = app_state.use_cases.add_new_chat(payload).await?;


    Ok(
        Json(
            chat
        )
    )
}

// #[cfg_attr(axum_macros::debug_handler, debug_handler)]
async fn get_chats(
    State(app_state): State<AppState>
) -> AppResult<Json<Vec<ChatPresenter>>> {
    let chats = app_state
        .use_cases
        .get_chats()
        .await?;

    let response = chats
        .into_iter()
        .map(|c|c.into())
        .collect();

    Ok(Json(response))
}

#[derive(Deserialize)]
pub struct DeleteChatPayload {
    id: String,
}

async fn delete_chat(
    State(app_state): State<AppState>,
    Json(payload): Json<DeleteChatPayload>
) -> AppResult<Json<Value>> {

    app_state.use_cases.delete_chat_by_id(payload.id).await?;

    Ok(
        Json(
            json!(
                {
                    "message": "chat deleted",
                    "status": "ok",
                }
            )
        )
    )
}