use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Game {
    // `id` ainda não é exposto na API (GamesResponse não o mapeia); o campo
    // é populado pelo FromRow e será lido quando o GET passar a incluir o id.
    #[expect(dead_code, reason = "id ainda não exposto na API")]
    pub id: i32,
    pub titulo: String,
    pub ano_lancamento: i32,
}
