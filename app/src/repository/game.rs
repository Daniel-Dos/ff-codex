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

    pub async fn games_by_lancamento(&self, lancamento: i32) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as("select * from games where ano_lancamento = $1")
            .bind(lancamento)
            .fetch_all(&self.pool)
            .await?;
        Ok(games)
    }

    pub async fn games_by_titulo_and_lancamento(
        &self,
        titulo: &str,
        lancamento: i32,
    ) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as(
            "select * from games where titulo ilike '%' || $1 || '%' and ano_lancamento = $2",
        )
        .bind(titulo)
        .bind(lancamento)
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    pub async fn create_game(&self, titulo: &str, lancamento: i32) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_as(
            "insert into games (titulo, ano_lancamento) values ($1, $2) returning *",
        )
        .bind(titulo)
        .bind(lancamento)
        .fetch_one(&self.pool)
        .await?;

        Ok(game)
    }
}
