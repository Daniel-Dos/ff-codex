use sqlx::FromRow;

#[allow(dead_code)]
#[derive(FromRow, Debug)]
pub struct Game {
    id: i32,
    titulo: String,
    ano_lancamento: i32,
}
