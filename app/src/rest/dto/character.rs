use crate::domain::character::Character;
use crate::domain::characters_games::CharactersGames;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CharactersRequest {
    #[validate(length(
        min = 1,
        message = "O nome do personogem do jogo não pode ser vazio",
        code = "name_vazio"
    ))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersQuery {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersDetailResponse {
    pub id: i32,
    pub name: String,
    pub game_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersGamesDetailResponse {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub release_year: i32,
}

impl From<CharactersGames> for CharactersGamesDetailResponse {
    fn from(characters: CharactersGames) -> Self {
        Self {
            id: characters.character_id,
            name: characters.character_name,
            title: characters.title,
            release_year: characters.release_year,
        }
    }
}

impl From<Character> for CharactersResponse {
    fn from(character: Character) -> Self {
        Self {
            name: character.name,
        }
    }
}

impl From<Character> for CharactersDetailResponse {
    fn from(character: Character) -> Self {
        Self {
            id: character.id,
            name: character.name,
            game_id: character.game_id,
        }
    }
}
