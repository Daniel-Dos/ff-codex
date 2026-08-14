use crate::domain::game::Game;
use sqlx::PgPool;

#[derive(Clone)]
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

    pub async fn games_by_titulo(&self, titulo: &str) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as("select * from games where titulo ilike '%' || $1 || '%'")
            .bind(titulo)
            .fetch_all(&self.pool)
            .await?;

        Ok(games)
    }
}
