use crate::domain::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesRequest {
    pub titulo: String,
    pub ano_lancamento: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesQuery {
    pub titulo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesResponse {
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
