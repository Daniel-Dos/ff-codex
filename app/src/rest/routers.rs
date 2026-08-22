use crate::rest::app_state::AppState;
use crate::rest::handler::games_handler::{create_games, delete_game, game, list_games};
use crate::rest::handler::{health, ready};
use axum::Router;
use axum::routing::{delete, get};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/ff-codex/games", get(list_games).post(create_games))
        .route("/ff-codex/games/{id}", delete(delete_game).get(game))
        .with_state(state)
}
