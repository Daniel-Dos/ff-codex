use crate::domain::game::Game;
use sqlx::PgPool;

pub struct GameRepository {
    pool: PgPool,
}

impl GameRepository {
    pub fn new(pool: PgPool) -> GameRepository {
        Self { pool }
    }

    pub async fn all_games(&self) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as("select * from games")
            .fetch_all(&self.pool)
            .await?;

        Ok(games)
    }
}
