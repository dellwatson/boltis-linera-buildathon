#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::BoltisGameEngineState;
use boltis_game_engine::GameStateWire;
use linera_sdk::{
    linera_base_types::WithServiceAbi,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};
use std::sync::Arc;

pub struct BoltisGameEngineService {
    state: Arc<BoltisGameEngineState>,
}

linera_sdk::service!(BoltisGameEngineService);

impl WithServiceAbi for BoltisGameEngineService {
    type Abi = boltis_game_engine::BoltisGameEngineAbi;
}

impl Service for BoltisGameEngineService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = BoltisGameEngineState::load(ViewStorageContext::from(runtime.root_view_storage_context()))
            .await
            .expect("Failed to load state");
        BoltisGameEngineService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, game_id: String) -> Option<GameStateWire> {
        self.state
            .games
            .get(&game_id)
            .await
            .expect("Failed to get game")
    }
}
