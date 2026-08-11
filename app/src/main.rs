mod domain;
mod repository;
mod rest;

use crate::rest::{router, server};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn,ff_codex=trace")),
        )
        .json()
        .init();

    info!("Iniciando a api de Final Fantasy.");

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("Variável DATABASE_URL não definida"))?;

    let banco = repository::game::GameRepository::new(
        PgPoolOptions::new()
            .connect(&database_url)
            .await
            .map_err(|e| {
                error!("Erro ao conectar com o banco de dados: {}", e);
                e
            })?,
    );

    let games = banco.get_all_games().await?;
    info!("Games: {:?}", games);

    let app = router();
    server(app).await?;

    Ok(())
}
