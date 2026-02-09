use axum::{
    Json, Router,
    extract::State,
    middleware,
    routing::{delete, get, post},
};
use serde_json::{Value, json};

use crate::{
    adapters::{
        api::{
            app_state::AppState,
            middlewares,
            user::{
                user_payload::{DeleteUserByIDPayload, UserByIDPayload},
                user_presenters::UserPresenter,
            },
        },
        ctx::Ctx,
    },
    application::{AppError, AppResult, dto::user::GetUserByIdDTO},
};

use super::user_payload::NewUserPayload;

pub fn user_router() -> Router<AppState> {
    let router = Router::new()
        .route("/api/admin/user", delete(delete_user))
        .route("/api/user", get(get_user))
        .route("/api/user-by-id", post(get_user_by_id))
        .route("/api/users", get(get_users))
        .route_layer(middleware::from_fn(middlewares::require_auth))
        .route("/api/user", post(add_new_user));

    router
}

async fn add_new_user(
    State(app_state): State<AppState>,
    Json(payload): Json<NewUserPayload>,
) -> AppResult<Json<UserPresenter>> {
    let user_response = app_state.use_cases.create_user(payload.into()).await?;

    Ok(Json(user_response.into()))
}

async fn get_user(State(app_state): State<AppState>, ctx: Ctx) -> AppResult<Json<UserPresenter>> {
    let user_id = ctx.get_user_id().to_owned();

    let user_dto = GetUserByIdDTO { id: user_id };

    let user_response = app_state
        .use_cases
        .get_user_by_id(user_dto)
        .await
        .map_err(|_| AppError::UserNotFoundByID)?;

    Ok(Json(user_response.into()))
}

async fn get_user_by_id(
    State(app_state): State<AppState>,
    Json(payload): Json<UserByIDPayload>,
) -> AppResult<Json<UserPresenter>> {
    let user_response = app_state
        .use_cases
        .get_user_by_id(payload.into())
        .await
        .map_err(|_| AppError::UserNotFoundByID)?;

    Ok(Json(user_response.into()))
}

async fn get_users(State(app_state): State<AppState>) -> AppResult<Json<Vec<UserPresenter>>> {
    let users = app_state.use_cases.get_users().await?;

    let response = users.into_iter().map(|u| u.into()).collect();

    Ok(Json(response))
}

async fn delete_user(
    State(app_state): State<AppState>,
    Json(payload): Json<DeleteUserByIDPayload>,
) -> AppResult<Json<Value>> {
    app_state.use_cases.delete_user_by_id(payload.into()).await?;

    Ok(Json(json!(
        {
            "message": "user deleted",
            "status": "ok",
        }
    )))
}
