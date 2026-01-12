use boltis_game_engine::GameStateWire;
use linera_sdk::views::{linera_views, MapView, RootView, ViewStorageContext};

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct BoltisGameEngineState {
    pub games: MapView<String, GameStateWire>,
}
