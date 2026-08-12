use crate::service::game_service::GameService;

#[derive(Clone)]
pub struct AppState {
    pub game_service: GameService,
}
