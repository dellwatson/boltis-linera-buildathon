#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use card_game::{CardGameAbi, Operation, PlayerState};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::CardGameState;

pub struct CardGameContract {
    state: CardGameState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(CardGameContract);

impl WithContractAbi for CardGameContract {
    type Abi = CardGameAbi;
}

impl Contract for CardGameContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = CardGameState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        CardGameContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // Validate that the application parameters were configured correctly.
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::UpdatePlayer {
                player_id,
                cards,
                move_action,
            } => {
                let mut state = self
                    .state
                    .players
                    .get(&player_id)
                    .await
                    .expect("Failed to read players")
                    .unwrap_or(PlayerState {
                        player_id: player_id.clone(),
                        cards: Vec::new(),
                        last_move: None,
                    });

                state.cards = cards;
                state.last_move = move_action;

                self.state
                    .players
                    .insert(&player_id, state)
                    .expect("Failed to write player state");
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
