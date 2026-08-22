use crate::rest::AppError;
use crate::rest::app_state::AppState;
use crate::rest::dto::game::{GameDetailResponse, GamesQuery, GamesRequest, GamesResponse};
use crate::service::game_service::GameError;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use tracing::{info, warn};
use validator::Validate;

pub async fn game(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<GameDetailResponse>, AppError> {
    info!("Buscando game com id: {}", id);
    if id <= 0 {
        return Err(AppError::BadRequest(
            "O id do game não pode ser vazio ou menor que 1".to_string(),
        ));
    }

    let game = state
        .game_service
        .game_by_id(id)
        .await
        .map_err(|_| AppError::NotFound(format!("Game com id {} não encontrado", id)))?;

    Ok(Json(GameDetailResponse::from(game)))
}

pub async fn list_games(
    State(state): State<AppState>,
    Query(params): Query<GamesQuery>,
) -> Result<Json<Vec<GamesResponse>>, AppError> {
    let titulo = params
        .titulo
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty());

    let lancamento = params.lancamento;

    match (titulo, lancamento) {
        (Some(t), Some(a)) => games_by_titulo_and_lancamento(state, t, a).await,
        (Some(t), None) => games_by_titulo(state, t).await,
        (None, Some(a)) => games_by_lancamento(state, a).await,
        (None, None) => list_all(state).await,
    }
}

fn map_service_error(e: crate::service::game_service::GameError) -> AppError {
    AppError::Internal(anyhow::anyhow!(
        "Erro ao processar operação de games: {}",
        e
    ))
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
        .map_err(map_service_error)?;

    Ok(Json(games.into_iter().map(GamesResponse::from).collect()))
}

async fn games_by_lancamento(
    state: AppState,
    lancamento: i32,
) -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!("Buscando games com Ano lancamento: {}", lancamento);

    let games = state
        .game_service
        .games_by_lancamento(lancamento)
        .await
        .map_err(map_service_error)?;

    Ok(Json(games.into_iter().map(GamesResponse::from).collect()))
}

async fn games_by_titulo_and_lancamento(
    state: AppState,
    titulo: &str,
    lancamento: i32,
) -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!(
        "Buscando games com o titulo: {} e ano de lançamento: {}",
        titulo, lancamento
    );

    let games = state
        .game_service
        .games_by_titulo_and_lancamento(titulo, lancamento)
        .await
        .map_err(map_service_error)?;
    Ok(Json(games.into_iter().map(GamesResponse::from).collect()))
}

async fn list_all(state: AppState) -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!("Obtendo todos os games!");

    let games = state
        .game_service
        .all_games()
        .await
        .map_err(map_service_error)?;

    Ok(Json(games.into_iter().map(GamesResponse::from).collect()))
}

pub async fn create_games(
    State(state): State<AppState>,
    Json(payload): Json<GamesRequest>,
) -> Result<(StatusCode, Json<GamesRequest>), AppError> {
    info!("Cadastrando um novo game: {}", payload.titulo);

    payload.validate()?;

    let game_id = state
        .game_service
        .create_game(&payload.titulo, payload.ano_lancamento)
        .await
        .map(|game| game.id)
        .map_err(map_service_error)?;

    info!(
        "O game {} foi cadastrado com sucesso, e o seu id é: {}",
        payload.titulo, game_id
    );

    Ok((StatusCode::CREATED, Json(payload)))
}

pub async fn delete_game(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, String), AppError> {
    warn!("Deletando o game com id: {}", id);

    state
        .game_service
        .delete_game_by_id(id)
        .await
        .map_err(|e| match e {
            GameError::NotFound => {
                AppError::NotFound(format!("Game com id {} não encontrado para deleção", id))
            }
            _ => map_service_error(e),
        })?;

    Ok((
        StatusCode::OK,
        format!("Game com id {} deletado com sucesso!", id),
    ))
}
