use boltis_leaderboard::{MatchResult, PlayerScore};
use linera_sdk::views::{linera_views, MapView, RootView, ViewStorageContext};

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct BoltisLeaderboardState {
    pub players: MapView<String, PlayerScore>,
    pub matches: MapView<String, MatchResult>,
}
