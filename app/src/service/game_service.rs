use crate::domain::game::Game;
use crate::repository::game::GameRepository;
use thiserror::Error;

#[derive(Clone)]
pub struct GameService {
    db: GameRepository,
}

#[derive(Error, Debug)]
pub enum GameError {
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

    pub async fn games_by_titulo(&self, titulo: &str) -> Result<Vec<Game>, GameError> {
        self.db
            .games_by_titulo(titulo)
            .await
            .map_err(GameError::from)
    }

    pub async fn games_by_lancamento(&self, lancamento: i32) -> Result<Vec<Game>, GameError> {
        self.db
            .games_by_lancamento(lancamento)
            .await
            .map_err(GameError::from)
    }

    pub async fn games_by_titulo_and_lancamento(
        &self,
        titulo: &str,
        lancamento: i32,
    ) -> Result<Vec<Game>, GameError> {
        self.db
            .games_by_titulo_and_lancamento(titulo, lancamento)
            .await
            .map_err(GameError::from)
    }

    pub async fn create_game(&self, titulo: &str, ano_lancamento: i32) -> Result<Game, GameError> {
        self.db
            .create_game(titulo, ano_lancamento)
            .await
            .map_err(GameError::from)
    }

    pub async fn delete_game_by_id(&self, id: i32) -> Result<(), GameError> {
        let rows_affected = self.db.delete_game(id).await.map_err(GameError::from)?;

        if rows_affected == 0 {
            return Err(GameError::NotFound);
        }

        Ok(())
    }

    pub async fn game_by_id(&self, id: i32) -> Result<Game, GameError> {
        self.db.games_by_id(id).await.map_err(GameError::from)
    }
}
