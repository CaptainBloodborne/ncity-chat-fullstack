use std::sync::Arc;

use axum::{
    Json, body::Body, extract::{Request, State}, http::{Method, Uri}, middleware::Next, response::{IntoResponse, Response}
};
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::{
    adapters::{
        api::{AUTH_TOKEN, app_state::AppState}, crypto::token::Token, ctx::{Ctx, CtxError, CtxResult}
    },
    application::{AppError, AppResult, dto::user::GetUserByIdDTO},
};

pub async fn require_auth(
    ctx: AppResult<Ctx>,
    req: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    println!(
        "->> {:<12} - middleware_require_auth - {ctx:?}",
        "MIDDLEWARE"
    );

    ctx?;

    Ok(next.run(req).await)
}

pub async fn context_resolver(
    State(app_state): State<AppState>,
    uri: Uri,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let result = resolve_context(app_state, &cookies, uri).await;

    if result.is_err() && !matches!(result, Err(CtxError::NoTokenInCookies)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result);

    next.run(req).await
}

async fn resolve_context(app_state: AppState, cookies: &Cookies, uri: Uri) -> CtxResult {
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxError::NoTokenInCookies)?;

    // Compute Result<Ctx>
    let token: Token = auth_token.parse().map_err(|_| CtxError::TokenBadFormat)?;

    let user_by_id_dto = GetUserByIdDTO {
        id: token.ident.clone(),
    };

    let user = app_state
        .use_cases
        .get_user_for_auth(user_by_id_dto)
        .await
        .map_err(|_| CtxError::UserNotFound)?;

    app_state
        .use_cases
        .validate_token(&token)
        .await
        .map_err(|_| CtxError::ValidationFail)?;

    if uri.path().starts_with("/api/admin") && user.role != "admin" {
        return Err(CtxError::UserNotAdmin);
    }

    Ok(Ctx::new(user.id, user.role))
}


pub async fn main_response_middleware(
    ctx: AppResult<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let request_uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Arc<AppError>>();
    
    let client_error = service_error.map(|e| e.get_client_and_status_code());

    let error_response = client_error
        .map(
            |(status_code, client_error)| {
                let error_response_body = json!({
                    "error": {
                        "type": client_error,
                        "req_uuid": request_uuid.to_string(),
                    }
                });

                println!(" -->> client_error_body: {error_response_body}");

                (status_code, Json(error_response_body)).into_response()
            });

    println!(" ->> app log - uri: {uri} - {request_uuid} - Error: {service_error:?} - method: {req_method:?} - user: {ctx:?}");

    error_response.unwrap_or(res)
}