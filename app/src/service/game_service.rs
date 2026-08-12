use crate::domain::game::Game;
use crate::repository::game::GameRepository;
use thiserror::Error;

#[derive(Clone)]
pub struct GameService {
    db: GameRepository,
}

#[derive(Error, Debug)]
pub enum GameError {
    // Variante reservada para uso futuro (ex.: GET /games/{id}); ainda não construída.
    #[expect(dead_code, reason = "NotFound será usado quando houver busca por id")]
    #[error("Game not found")]
    NotFound,
    #[error("internal database error: {0}")]
    Internal(#[from] sqlx::Error),
}

impl GameService {
    pub fn new(db: GameRepository) -> Self {
        Self { db }
    }

    pub async fn all_games(&self) -> Result<Vec<Game>, GameError> {
        self.db.all_games().await.map_err(GameError::from)
    }
}
