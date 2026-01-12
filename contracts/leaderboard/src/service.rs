#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::BoltisLeaderboardState;
use boltis_leaderboard::{Query, QueryResponse};
use linera_sdk::{
    linera_base_types::WithServiceAbi,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};
use std::sync::Arc;

pub struct BoltisLeaderboardService {
    state: Arc<BoltisLeaderboardState>,
}

linera_sdk::service!(BoltisLeaderboardService);

impl WithServiceAbi for BoltisLeaderboardService {
    type Abi = boltis_leaderboard::BoltisLeaderboardAbi;
}

impl Service for BoltisLeaderboardService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = BoltisLeaderboardState::load(ViewStorageContext::from(runtime.root_view_storage_context()))
            .await
            .expect("Failed to load state");
        BoltisLeaderboardService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, query: Query) -> QueryResponse {
        match query {
            Query::GetPlayer { wallet_address } => {
                let player = self.state
                    .players
                    .get(&wallet_address)
                    .await
                    .expect("Failed to get player");
                QueryResponse::Player(player)
            }
            Query::GetTopPlayers { limit } => {
                let mut players = Vec::new();
                
                self.state
                    .players
                    .for_each_index_value(|_key, player| {
                        players.push(player.into_owned());
                        Ok(())
                    })
                    .await
                    .expect("Failed to iterate players");

                // Sort by total XP descending
                players.sort_by(|a, b| b.total_xp.cmp(&a.total_xp));
                
                // Take top N
                let result = players.into_iter().take(limit as usize).collect();
                QueryResponse::Players(result)
            }
            Query::GetMatch { match_id } => {
                let match_result = self.state
                    .matches
                    .get(&match_id)
                    .await
                    .expect("Failed to get match");
                QueryResponse::Match(match_result)
            }
            Query::GetPlayerMatches { wallet_address, limit } => {
                let mut matches = Vec::new();
                
                self.state
                    .matches
                    .for_each_index_value(|_key, match_result| {
                        if match_result.wallet_address == wallet_address {
                            matches.push(match_result.into_owned());
                        }
                        Ok(())
                    })
                    .await
                    .expect("Failed to iterate matches");

                // Sort by timestamp descending (most recent first)
                matches.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                
                // Take top N
                let result = matches.into_iter().take(limit as usize).collect();
                QueryResponse::Matches(result)
            }
            Query::GetTotalPlayers => {
                let count = *self.state.total_players.get();
                QueryResponse::Count(count)
            }
            Query::GetTotalMatches => {
                let count = *self.state.total_matches.get();
                QueryResponse::Count(count)
            }
        }
    }
}
