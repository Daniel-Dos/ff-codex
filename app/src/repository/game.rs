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

    pub async fn games_by_title(&self, title: &str) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            "select * from games where title ilike '%' || $1 || '%'",
            title
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    pub async fn games_by_release_year(&self, release_year: i32) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            "select * from games where release_year = $1",
            release_year
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    pub async fn games_by_title_and_release_year(
        &self,
        title: &str,
        release_year: i32,
    ) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            "select * from games where title ilike '%' || $1 || '%' and release_year = $2",
            title,
            release_year
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    pub async fn create_game(&self, title: &str, release_year: i32) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_as!(
            Game,
            "insert into games (title, release_year) values ($1, $2) returning *",
            title,
            release_year,
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
