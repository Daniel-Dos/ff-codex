use axum::Router;
use axum::routing::get;

use crate::rest::handler::{health, ready};
use crate::rest::handler::games_handler::get_games;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/ff-codex/games", get(get_games))
}
