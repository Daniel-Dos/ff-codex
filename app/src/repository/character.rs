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
        let character = sqlx::query_file_as!(
            Character,
            "sql/characters/insert_characters.sql",
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
        let character =
            sqlx::query_file_as!(Character, "sql/characters/find_character_by_name.sql", name)
                .fetch_optional(&self.pool)
                .await?;

        Ok(character)
    }

    pub async fn all_characters(&self) -> Result<Vec<Character>, sqlx::Error> {
        let characters = sqlx::query_file_as!(Character, "sql/characters/find_all_characters.sql")
            .fetch_all(&self.pool)
            .await?;

        Ok(characters)
    }

    pub async fn characters_by_id(&self, id: i32) -> Result<Character, sqlx::Error> {
        let character =
            sqlx::query_file_as!(Character, "sql/characters/find_character_by_id.sql", id)
                .fetch_one(&self.pool)
                .await?;

        Ok(character)
    }

    pub async fn all_characters_by_id_game(
        &self,
        game_id: i32,
    ) -> Result<Vec<CharactersGames>, sqlx::Error> {
        let characters_games = sqlx::query_file_as!(
            CharactersGames,
            "sql/characters/characters_games_join.sql",
            game_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(characters_games)
    }
}
