use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn: &mut TurnState) {
    let next_state = match turn {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *turn = next_state;
}
