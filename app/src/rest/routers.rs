use crate::rest::app_state::AppState;
use crate::rest::handler::characters_handler::{character, create_characters, list_characters};
use crate::rest::handler::games_handler::{create_games, delete_game, game, list_games};
use crate::rest::handler::{health, ready};
use axum::Router;
use axum::routing::{delete, get, post};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/ff-codex/api",
            Router::new()
                .route("/games", get(list_games).post(create_games))
                .route("/games/{id}", delete(delete_game).get(game))
                .route("/characters/{id}", get(character))
                .route("/characters", get(list_characters))
                .route("/games/{game_id}/characters", post(create_characters))
                .with_state(state),
        )
        .route("/health", get(health))
        .route("/ready", get(ready))
}
