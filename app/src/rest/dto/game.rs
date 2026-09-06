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
    pub title: String,
    #[validate(range(
        min = 1,
        message = "O ano de lançamento do jogo deve ser maior que 0",
        code = "ano_invalido"
    ))]
    pub release_year: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesQuery {
    pub title: Option<String>,
    pub release_year: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesResponse {
    pub title: String,
    pub release_year: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameDetailResponse {
    pub id: i32,
    pub title: String,
    pub release_year: i32,
}

impl From<Game> for GamesResponse {
    fn from(game: Game) -> Self {
        Self {
            title: game.title,
            release_year: game.release_year,
        }
    }
}

impl From<Game> for GameDetailResponse {
    fn from(game: Game) -> Self {
        Self {
            id: game.id,
            title: game.title,
            release_year: game.release_year,
        }
    }
}
