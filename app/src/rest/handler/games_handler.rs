use crate::rest::AppError;
use crate::rest::app_state::AppState;
use crate::rest::dto::game::{GamesRequest, GamesResponse};
use axum::Json;
use axum::extract::State;
use tracing::info;

pub async fn list_games(
    State(state): State<AppState>,
) -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!("Obtendo todos os games!");

    let games = state
        .game_service
        .all_games()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro ao obter os games: {}", e)))?;

    let game_response: Vec<GamesResponse> = games.into_iter().map(GamesResponse::from).collect();

    Ok(Json(game_response))
}

pub async fn create_games(payload: Json<GamesRequest>) -> Result<Json<GamesResponse>, AppError> {
    info!("Cadastrando um novo game: {}", payload.titulo);

    let novo_game = GamesResponse {
        titulo: payload.titulo.clone(),
        ano_lancamento: payload.ano_lancamento,
    };

    Ok(Json(novo_game))
}
