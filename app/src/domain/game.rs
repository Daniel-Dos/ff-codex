use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Game {
    pub id: i32,
    pub titulo: String,
    pub ano_lancamento: i32,
}
