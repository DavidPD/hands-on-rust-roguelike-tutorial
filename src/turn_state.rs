use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
