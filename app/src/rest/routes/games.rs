use crate::rest::app_state::AppState;
use crate::rest::handler::games_handler::{create_games, delete_game, game, list_games};
use axum::Router;
use axum::routing::{delete, get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/games", get(list_games).post(create_games))
        .route("/games/{id}", delete(delete_game).get(game))
}
