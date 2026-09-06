use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub release_year: i32,
}
