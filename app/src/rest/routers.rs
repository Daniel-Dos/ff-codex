use axum::Router;
use axum::routing::get;

use crate::rest::handler::games_handler::{create_games, list_games};
use crate::rest::handler::{health, ready};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/ff-codex/games", get(list_games).post(create_games))
}
