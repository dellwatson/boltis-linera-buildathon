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

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct CardWire {
    pub id: String,
    pub element: ElementType,
    #[graphql(name = "type")]
    pub card_type: CardType,
    pub value: Option<i32>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum MoveKind {
    Draw,
    PlayCard,
    SelectColor,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct MoveActionInput {
    pub kind: MoveKind,
    pub card_id: Option<String>,
    pub element: Option<ElementType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct PlayerState {
    pub player_id: String,
    pub cards: Vec<CardWire>,
    pub last_move: Option<MoveActionInput>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    UpdatePlayer {
        player_id: String,
        cards: Vec<CardWire>,
        move_action: Option<MoveActionInput>,
    },
}

pub struct CardGameAbi;

impl ContractAbi for CardGameAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for CardGameAbi {
    type Query = Request;
    type QueryResponse = Response;
}
