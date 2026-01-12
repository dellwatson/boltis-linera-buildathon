#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use boltis_game_engine::{BoltisGameEngineAbi, GameStateWire, Operation};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::BoltisGameEngineState;

pub struct BoltisGameEngineContract {
    state: BoltisGameEngineState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(BoltisGameEngineContract);

impl WithContractAbi for BoltisGameEngineContract {
    type Abi = BoltisGameEngineAbi;
}

impl Contract for BoltisGameEngineContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = BoltisGameEngineState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        BoltisGameEngineContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // Validate that the application parameters were configured correctly.
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::InitializeGame {
                game_id,
                players,
                starting_player_index,
            } => {
                let game = GameStateWire {
                    game_id: game_id.clone(),
                    players,
                    current_player_index: starting_player_index,
                };

                self.state
                    .games
                    .insert(&game_id, game)
                    .expect("Failed to write game state");
            }
            Operation::ApplyMove {
                game_id,
                move_action: _move_action,
                updated_state,
            } => {
                // For now, trust the client-side GameEngine to compute updated_state.
                // This keeps the contract as a scaffold that can be made authoritative later.
                self.state
                    .games
                    .insert(&game_id, updated_state)
                    .expect("Failed to write game state");
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state
            .save()
            .await
            .expect("Failed to save state");
    }
}
