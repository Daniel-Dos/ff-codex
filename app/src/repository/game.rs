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
        let games = sqlx::query_file_as!(Game, "sql/games/all_games.sql")
            .fetch_all(&self.pool)
            .await?;

        Ok(games)
    }

    pub async fn games_by_title(&self, title: &str) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_file_as!(Game, "sql/games/find_games_by_title.sql", title)
            .fetch_all(&self.pool)
            .await?;

        Ok(games)
    }

    pub async fn games_by_release_year(&self, release_year: i32) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_file_as!(
            Game,
            "sql/games/find_games_by_release_year.sql",
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
        let games = sqlx::query_file_as!(
            Game,
            "sql/games/find_games_by_title_and_release_year.sql",
            title,
            release_year
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    pub async fn create_game(&self, title: &str, release_year: i32) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_file_as!(Game, "sql/games/create_game.sql", title, release_year,)
            .fetch_one(&self.pool)
            .await?;

        Ok(game)
    }

    pub async fn delete_game(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query_file!("sql/games/delete_game.sql", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn games_by_id(&self, id: i32) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_file_as!(Game, "sql/games/find_games_by_id.sql", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(game)
    }
}
