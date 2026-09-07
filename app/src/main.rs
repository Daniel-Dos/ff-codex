mod domain;
mod repository;
mod rest;
mod service;
mod util;

use crate::rest::{router, server};
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::rest::app_state::AppState;
use crate::service::characters_service::CharactersService;
use crate::service::game_service::GameService;
use crate::util::banner::print_banner;
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
    print_banner();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("Variável DATABASE_URL não definida"))?;

    let postgres_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .map_err(|e| {
            error!("Erro ao conectar com o banco de dados: {}", e);
            e
        })?;

    let banco_game = repository::game::GameRepository::new(postgres_pool.clone());

    let banco_character = repository::character::CharactersRepository::new(postgres_pool.clone());

    let game_service = GameService::new(banco_game);
    let characters_service = CharactersService::new(banco_character);
    let app_state = AppState {
        game_service,
        characters_service,
    };

    let app = router(app_state);
    server(app).await?;

    Ok(())
}
