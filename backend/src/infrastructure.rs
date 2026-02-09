mod app;
mod config;
mod db;

use std::sync::Arc;

use crate::{
    adapters::{api::app_state::AppState, crypto::argon::ArgonHasher, db::postgres::{PostgresChatRepo, PostgresUserRepo}},
    application::use_cases::UseCases
};

pub async fn init_app() -> anyhow::Result<()> {
    let server = app::Server::new("my_app".to_string())?;

    let db_pool = db::init_postgres_db(&server.config.database_url).await?;

    let postgres_user_repo = PostgresUserRepo::new(db_pool.clone());
    let postgres_chat_repo = PostgresChatRepo::new(db_pool);

    let argon_hasher = ArgonHasher::new();

    let use_cases = UseCases::new(Arc::new(postgres_user_repo), Arc::new(postgres_chat_repo), Arc::new(argon_hasher));

    let app_state = AppState {
        use_cases: Arc::new(use_cases),
    };

    server.start(app_state).await?;

    anyhow::Ok(())
}
