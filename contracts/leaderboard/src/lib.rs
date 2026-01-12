use linera_sdk::linera_base_types::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScore {
    pub wallet_address: String,
    pub player_name: String,
    pub total_xp: i64,
    pub level: i32,
    pub wins: i32,
    pub losses: i32,
    pub total_matches: i32,
    pub best_rank: i32,
    pub win_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchResult {
    pub match_id: String,
    pub wallet_address: String,
    pub player_name: String,
    pub rank: i32,
    pub xp_earned: i64,
    pub cards_remaining: i32,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    UpdatePlayerScore {
        wallet_address: String,
        player_name: String,
        xp_change: i64,
        won: bool,
        rank: i32,
    },
    RecordMatch {
        match_id: String,
        wallet_address: String,
        player_name: String,
        rank: i32,
        xp_earned: i64,
        cards_remaining: i32,
        timestamp: u64,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    GetPlayer { wallet_address: String },
    GetTopPlayers { limit: i32 },
    GetMatch { match_id: String },
    GetPlayerMatches { wallet_address: String, limit: i32 },
    GetTotalPlayers,
    GetTotalMatches,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum QueryResponse {
    Player(Option<PlayerScore>),
    Players(Vec<PlayerScore>),
    Match(Option<MatchResult>),
    Matches(Vec<MatchResult>),
    Count(u64),
}

pub struct BoltisLeaderboardAbi;

impl ContractAbi for BoltisLeaderboardAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for BoltisLeaderboardAbi {
    type Query = Query;
    type QueryResponse = QueryResponse;
}
