use card_game::PlayerState;
use linera_sdk::views::{linera_views, MapView, RootView, ViewStorageContext};

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct CardGameState {
    pub players: MapView<String, PlayerState>,
}
