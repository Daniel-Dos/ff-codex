use crate::domain::game::Game;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct GamesRequest {
    #[validate(length(
        min = 1,
        message = "O título do jogo não pode ser vazio",
        code = "titulo_vazio"
    ))]
    pub titulo: String,
    #[validate(range(
        min = 1,
        message = "O ano de lançamento do jogo deve ser maior que 0",
        code = "ano_invalido"
    ))]
    pub ano_lancamento: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesQuery {
    pub titulo: Option<String>,
    pub lancamento: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesResponse {
    pub titulo: String,
    pub ano_lancamento: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameDetailResponse {
    pub id: i32,
    pub titulo: String,
    pub ano_lancamento: i32,
}

impl From<Game> for GamesResponse {
    fn from(game: Game) -> Self {
        Self {
            titulo: game.titulo,
            ano_lancamento: game.ano_lancamento,
        }
    }
}

impl From<Game> for GameDetailResponse {
    fn from(game: Game) -> Self {
        Self {
            id: game.id,
            titulo: game.titulo,
            ano_lancamento: game.ano_lancamento,
        }
    }
}
