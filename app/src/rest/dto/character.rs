use crate::domain::character::Character;
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
