use crate::rest::AppError;
use crate::rest::app_state::AppState;
use crate::rest::dto::game::{GamesQuery, GamesRequest, GamesResponse};
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use tracing::info;

pub async fn list_games(
    State(state): State<AppState>,
    Query(params): Query<GamesQuery>,
) -> Result<Json<Vec<GamesResponse>>, AppError> {
    let titulo = params
        .titulo
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty());

    match titulo {
        Some(t) => games_by_titulo(state, t).await,
        None => list_all(state).await,
    }
}

async fn games_by_titulo(
    state: AppState,
    titulo: &str,
) -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!("Buscando games com título: {}", titulo);

    let games = state
        .game_service
        .games_by_titulo(titulo)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro ao obter os games: {}", e)))?;

    Ok(Json(games.into_iter().map(GamesResponse::from).collect()))
}

async fn list_all(state: AppState) -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!("Obtendo todos os games!");

    let games = state
        .game_service
        .all_games()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro ao obter os games: {}", e)))?;

    Ok(Json(games.into_iter().map(GamesResponse::from).collect()))
}

pub async fn create_games(State(state): State<AppState>, payload: Json<GamesRequest>) -> Result<(StatusCode, Json<GamesRequest>), AppError> {
    info!("Cadastrando um novo game: {}", payload.titulo);

    let game_id = state.game_service
        .create_game(&payload.titulo, payload.ano_lancamento)
        .await
        .map(|game| {
            game.id
        })
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro ao criar o game: {}", e)))?;

    info!("O game {} foi cadastrado com sucesso, e o seu id é: {}", payload.titulo, game_id);

    Ok((StatusCode::CREATED, payload))
}
