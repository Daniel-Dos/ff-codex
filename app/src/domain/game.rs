use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Game {
    id: i32,
    titulo: String,
    ano_lancamento: i32,
}
