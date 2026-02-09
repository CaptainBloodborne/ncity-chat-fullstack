use axum::{Json, extract::State};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

use crate::{adapters::api::{AUTH_TOKEN, app_state::AppState, user::user_payload::LoginUserPayload}, application::{AppError, AppResult}};

pub async fn login(
    State(app_state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginUserPayload>,
) -> AppResult<Json<Value>> {
    let login_response = app_state.use_cases
        .login_user(payload.into())
        .await
        .map_err(|_|AppError::LoginFail)?;

    let mut auth_cookie = Cookie::new(AUTH_TOKEN, login_response.token);
    auth_cookie.set_http_only(true);
    auth_cookie.set_path("/");

    cookies.add(auth_cookie);

    Ok(
        Json(
            json!(
                {
                    "result": {
                        "success": true
                    }
                }
            )
        )
    )
}

pub async fn logout(
    cookies: Cookies,
) -> AppResult<Json<Value>> {
    let mut auth_cookie = Cookie::from(AUTH_TOKEN);

    auth_cookie.set_path("/");

    cookies.remove(auth_cookie);

    Ok(
        Json(
            json!(
                {
                    "result": {
                        "success": true
                    }
                }
            )
        )
    )
}
