use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_postgres_db(db_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    Ok(pool)
}