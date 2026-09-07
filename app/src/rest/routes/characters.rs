use crate::rest::app_state::AppState;
use crate::rest::handler::characters_handler::{character, create_characters, list_characters};
use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/characters/{id}", get(character))
        .route("/characters", get(list_characters))
        .route("/games/{game_id}/characters", post(create_characters))
}
