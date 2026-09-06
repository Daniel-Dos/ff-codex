use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub game_id: i32,
}
