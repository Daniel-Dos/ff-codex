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
        let games = sqlx::query_as!(Game, "select * from games")
            .fetch_all(&self.pool)
            .await?;

        Ok(games)
    }

    pub async fn games_by_titulo(&self, titulo: &str) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            "select * from games where titulo ilike '%' || $1 || '%'",
            titulo
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    pub async fn games_by_lancamento(&self, lancamento: i32) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            "select * from games where ano_lancamento = $1",
            lancamento
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    pub async fn games_by_titulo_and_lancamento(
        &self,
        titulo: &str,
        lancamento: i32,
    ) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            "select * from games where titulo ilike '%' || $1 || '%' and ano_lancamento = $2",
            titulo,
            lancamento
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    pub async fn create_game(&self, titulo: &str, lancamento: i32) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_as!(
            Game,
            "insert into games (titulo, ano_lancamento) values ($1, $2) returning *",
            titulo,
            lancamento
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(game)
    }

    pub async fn delete_game(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("delete from games where id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn games_by_id(&self, id: i32) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_as!(Game, "select * from games where id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(game)
    }
}
