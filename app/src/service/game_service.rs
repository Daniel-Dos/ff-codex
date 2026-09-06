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

    pub async fn games_by_title(&self, title: &str) -> Result<Vec<Game>, GameError> {
        self.db.games_by_title(title).await.map_err(GameError::from)
    }

    pub async fn games_by_release_year(&self, release_year: i32) -> Result<Vec<Game>, GameError> {
        self.db
            .games_by_release_year(release_year)
            .await
            .map_err(GameError::from)
    }

    pub async fn games_by_title_and_release_year(
        &self,
        title: &str,
        release_year: i32,
    ) -> Result<Vec<Game>, GameError> {
        self.db
            .games_by_title_and_release_year(title, release_year)
            .await
            .map_err(GameError::from)
    }

    pub async fn create_game(&self, title: &str, release_year: i32) -> Result<Game, GameError> {
        self.db
            .create_game(title, release_year)
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
