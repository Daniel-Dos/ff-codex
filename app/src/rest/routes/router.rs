use crate::rest::app_state::AppState;
use crate::rest::handler::health;
use crate::rest::routes::{characters, games};
use axum::Router;
use axum::routing::get;

pub fn router(state: AppState) -> Router {
    Router::new().route("/health", get(health)).nest(
        "/ff-codex/api/v1",
        Router::new()
            .merge(games::router())
            .merge(characters::router())
            .with_state(state),
    )
}
