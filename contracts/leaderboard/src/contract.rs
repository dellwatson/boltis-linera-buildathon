#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::BoltisLeaderboardState;
use boltis_leaderboard::{Operation, PlayerScore};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

pub struct BoltisLeaderboardContract {
    state: BoltisLeaderboardState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(BoltisLeaderboardContract);

impl WithContractAbi for BoltisLeaderboardContract {
    type Abi = boltis_leaderboard::BoltisLeaderboardAbi;
}

impl Contract for BoltisLeaderboardContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = BoltisLeaderboardState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        BoltisLeaderboardContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::UpdatePlayerScore {
                wallet_address,
                player_name,
                xp_change,
                won,
                rank,
            } => {
                self.update_player_score(wallet_address, player_name, xp_change, won, rank)
                    .await;
            }
            Operation::RecordMatch {
                match_id,
                wallet_address,
                player_name,
                rank,
                xp_earned,
                cards_remaining,
                timestamp,
            } => {
                self.record_match(
                    match_id,
                    wallet_address,
                    player_name,
                    rank,
                    xp_earned,
                    cards_remaining,
                    timestamp,
                )
                .await;
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        panic!("Messages not supported");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl BoltisLeaderboardContract {
    async fn update_player_score(
        &mut self,
        wallet_address: String,
        player_name: String,
        xp_change: i64,
        won: bool,
        rank: i32,
    ) {
        let mut player = self
            .state
            .players
            .get(&wallet_address)
            .await
            .expect("Failed to get player")
            .unwrap_or_else(|| PlayerScore {
                wallet_address: wallet_address.clone(),
                player_name: player_name.clone(),
                total_xp: 0,
                level: 1,
                wins: 0,
                losses: 0,
                total_matches: 0,
                best_rank: 4,
                win_rate: 0.0,
            });

        // Update stats
        player.total_xp = (player.total_xp + xp_change).max(0);
        player.level = (player.total_xp / 1000) as i32 + 1;
        player.total_matches += 1;

        if won {
            player.wins += 1;
        } else {
            player.losses += 1;
        }

        if rank < player.best_rank {
            player.best_rank = rank;
        }

        player.win_rate = if player.total_matches > 0 {
            (player.wins as f64 / player.total_matches as f64) * 100.0
        } else {
            0.0
        };

        self.state
            .players
            .insert(&wallet_address, player)
            .expect("Failed to insert player");
    }

    async fn record_match(
        &mut self,
        match_id: String,
        wallet_address: String,
        player_name: String,
        rank: i32,
        xp_earned: i64,
        cards_remaining: i32,
        timestamp: u64,
    ) {
        let match_result = boltis_leaderboard::MatchResult {
            match_id: match_id.clone(),
            wallet_address,
            player_name,
            rank,
            xp_earned,
            cards_remaining,
            timestamp,
        };

        self.state
            .matches
            .insert(&match_id, match_result)
            .expect("Failed to insert match");
    }
}
