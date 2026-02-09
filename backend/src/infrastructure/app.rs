use axum::{Router, middleware, routing::post, serve};
use tower_cookies::CookieManagerLayer;

use crate::adapters::api::{app_state::AppState, chat::chat_controller::chat_router, login::login_controller, middlewares, user::user_controller::user_router};

pub struct Server {
    pub app_name: String,
    pub config: super::config::Config,
}

impl Server {
    pub fn new(app_name: String) -> anyhow::Result<Self> {
        let app_config = super::config::init_config()?;
        anyhow::Ok(Self { app_name, config: app_config })
    }

    pub async fn start(&self, app_state: AppState) -> anyhow::Result<()> {
        println!("Start {}", self.app_name);


        let router: Router = Router::new()
            .merge(user_router())
            .merge(chat_router())
            .route("/api/login", post(login_controller::login))
            .route("/api/logout", post(login_controller::logout))
            .layer(middleware::map_response(middlewares::main_response_middleware))
            .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::context_resolver))
            .layer(CookieManagerLayer::new())
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        serve(listener, router).await?;

        anyhow::Ok(())
    }
}