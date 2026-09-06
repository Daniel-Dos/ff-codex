use crate::rest::AppError;
use crate::rest::app_state::AppState;
use crate::rest::dto::character::{CharactersDetailResponse, CharactersQuery, CharactersRequest, CharactersResponse};
use crate::service::characters_service::CharacterError;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use tracing::info;
use validator::Validate;

pub async fn create_characters(
    State(state): State<AppState>,
    Path(game_id): Path<i32>,
    Json(payload): Json<CharactersRequest>,
) -> Result<(StatusCode, Json<CharactersRequest>), AppError> {
    info!(
        "Criando personagem com o nome: {} para o jogo com id: {}.",
        payload.name, game_id
    );

    payload.validate()?;

    let character_id = state
        .characters_service
        .create_character(&payload.name, game_id)
        .await
        .map(|character| character.id)
        .map_err(crate::rest::handler::characters_handler::map_service_error)?;

    info!(
        "O personagem {} foi cadastrado com sucesso, e o seu id é: {}",
        payload.name, character_id
    );

    Ok((StatusCode::CREATED, Json(payload)))
}

pub async fn character(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<CharactersDetailResponse>, AppError> {
    info!("Buscando personagem com o id: {}", id);

    if id <= 0 {
        return Err(AppError::BadRequest(
            "O id do personagem deve ser maior que zero.".to_string()
        ));
    }

    let character = state
        .characters_service
        .character_by_id(id)
        .await
        .map_err(|_| AppError::NotFound(format!("Personagem com id {} não encontrado", id)))?;

    Ok(Json(CharactersDetailResponse::from(character)))
}
pub async fn list_characters(
    State(state): State<AppState>,
    Query(params): Query<CharactersQuery>,
) -> Result<Json<Vec<CharactersResponse>>, AppError> {
    let name = params
        .name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    match name {
        Some(n) => characters_by_name(state, n).await,
        None => list_all(state).await,
    }
}

fn map_service_error(e: crate::service::characters_service::CharacterError) -> AppError {
    match e {
        CharacterError::Internal(sqlx_err) => AppError::from(sqlx_err),
        other => AppError::Internal(anyhow::anyhow!(
            "Erro ao processar operação de characters: {}",
            other
        )),
    }
}

async fn characters_by_name(
    state: AppState,
    name: &str,
) -> Result<Json<Vec<CharactersResponse>>, AppError> {
    info!("Buscando personagens com o nome: {}", name);

    let characters = state
        .characters_service
        .find_character_by_name(name)
        .await
        .map_err(map_service_error)?;

    Ok(Json(
        characters
            .into_iter()
            .map(CharactersResponse::from)
            .collect(),
    ))
}

async fn list_all(state: AppState) -> Result<Json<Vec<CharactersResponse>>, AppError> {
    info!("Obtendo todos os characters!");

    let characters = state
        .characters_service
        .all_characters()
        .await
        .map_err(map_service_error)?;

    Ok(Json(
        characters
            .into_iter()
            .map(CharactersResponse::from)
            .collect(),
    ))
}
