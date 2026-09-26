use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct CharactersGames {
    pub character_id: i32,
    pub character_name: String,
    pub title: String,
    pub release_year: i32,
}
