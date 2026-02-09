use dotenvy::dotenv;

use anyhow::Ok;
use ncity_chat_network::infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    infrastructure::init_app().await?;

    Ok(())
}

