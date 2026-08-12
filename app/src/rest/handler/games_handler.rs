use crate::rest::AppError;
use crate::rest::dto::game::{GamesRequest, GamesResponse};
use axum::Json;
use tracing::info;

pub async fn list_games() -> Result<Json<Vec<GamesResponse>>, AppError> {
    info!("Obtendo todos os games!");

    let game = GamesResponse {
        titulo: String::from("Final Fantasy VII"),
        ano_lancamento: 1997,
    };

    Ok(Json(vec![game]))
}

pub async fn create_games(payload: Json<GamesRequest>) -> Result<Json<GamesResponse>, AppError> {
    info!("Cadastrando um novo game: {}", payload.titulo);

    let novo_game = GamesResponse {
        titulo: payload.titulo.clone(),
        ano_lancamento: payload.ano_lancamento,
    };

    Ok(Json(novo_game))
}
