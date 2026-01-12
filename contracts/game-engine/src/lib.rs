use async_graphql::{Enum, GraphQLMutationRoot, InputObject, Request, Response, SimpleObject};
use linera_sdk::linera_base_types::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ElementType {
    Fire,
    Water,
    Plant,
    Thunder,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CardType {
    Number,
    Skip,
    Reverse,
    Stack,
    Void,
    Bomb,
    Strike,
    Mirror,
    Locked,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject, SimpleObject)]
pub struct CardWire {
    pub id: String,
    pub element: ElementType,
    #[graphql(name = "type")]
    pub card_type: CardType,
    pub value: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject, SimpleObject)]
pub struct PlayerWire {
    pub id: String,
    pub name: String,
    pub cards: Vec<CardWire>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct GameStateWire {
    pub game_id: String,
    pub players: Vec<PlayerWire>,
    pub current_player_index: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct MoveActionInput {
    pub player_id: String,
    pub kind: String,
    pub card_id: Option<String>,
    pub element: Option<ElementType>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    InitializeGame {
        game_id: String,
        players: Vec<PlayerWire>,
        starting_player_index: i32,
    },
    ApplyMove {
        game_id: String,
        move_action: MoveActionInput,
        updated_state: GameStateWire,
    },
}

pub struct BoltisGameEngineAbi;

impl ContractAbi for BoltisGameEngineAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for BoltisGameEngineAbi {
    type Query = Request;
    type QueryResponse = Response;
}
