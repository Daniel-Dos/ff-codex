use crate::domain::character::Character;
use crate::repository::character::CharactersRepository;
use crate::service::game_service::GameError;
use thiserror::Error;

#[derive(Clone)]
pub struct CharactersService {
    db: CharactersRepository,
}

#[derive(Error, Debug)]
pub enum CharacterError {
    #[error("Character not found")]
    NotFound,
    #[error("internal database error: {0}")]
    Internal(#[from] sqlx::Error),
}

impl CharactersService {
    pub fn new(db: CharactersRepository) -> Self {
        Self { db }
    }

    pub async fn create_character(
        &self,
        name: &str,
        game_id: i32,
    ) -> Result<Character, CharacterError> {
        self.db
            .create_character(name, game_id)
            .await
            .map_err(CharacterError::from)
    }

    pub async fn find_character_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Character>, CharacterError> {
        self.db
            .find_character_by_name(name)
            .await
            .map_err(CharacterError::from)
    }

    pub async fn all_characters(&self) -> Result<Vec<Character>, CharacterError> {
        let characters = self
            .db
            .all_characters()
            .await
            .map_err(CharacterError::from)?;
        Ok(characters)
    }

    pub async fn character_by_id(&self, id: i32) -> Result<Character, GameError> {
        self.db.characters_by_id(id).await.map_err(GameError::from)
    }
}
