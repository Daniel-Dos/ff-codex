use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesRequest {
    pub titulo: String,
    pub ano_lancamento: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesResponse {
    pub titulo: String,
    pub ano_lancamento: i32,
}
