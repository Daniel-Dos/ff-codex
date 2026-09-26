use crate::domain::character::Character;
use crate::domain::characters_games::CharactersGames;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CharactersRepository {
    pool: PgPool,
}

impl CharactersRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_character(
        &self,
        name: &str,
        game_id: i32,
    ) -> Result<Character, sqlx::Error> {
        let character = sqlx::query_as!(
            Character,
            "insert into characters (name, game_id) values ($1, $2) returning *",
            name,
            game_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character)
    }

    pub async fn find_character_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Character>, sqlx::Error> {
        let character = sqlx::query_as!(
            Character,
            "select * from characters where name ilike '%' || $1 || '%'",
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(character)
    }

    pub async fn all_characters(&self) -> Result<Vec<Character>, sqlx::Error> {
        let characters = sqlx::query_as!(Character, "select * from characters")
            .fetch_all(&self.pool)
            .await?;

        Ok(characters)
    }

    pub async fn characters_by_id(&self, id: i32) -> Result<Character, sqlx::Error> {
        let character = sqlx::query_as!(Character, "select * from characters where id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(character)
    }

    pub async fn all_characters_by_id_game(
        &self,
        game_id: i32,
    ) -> Result<Vec<CharactersGames>, sqlx::Error> {
        let characters_games = sqlx::query_as!(CharactersGames,"select c.id AS character_id, c.name AS character_name, g.title AS title, g.release_year AS release_year from characters c  INNER JOIN games g on c.game_id = g.id where g.id = $1", game_id
 ).fetch_all(&self.pool).await?;

        Ok(characters_games)
    }
}
