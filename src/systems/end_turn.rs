use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(#[resource] turn: &mut TurnState, world: &SubWorld) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = turn.clone();
    let mut next_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(world).for_each(|hp| {
        if hp.current < 1 {
            next_state = TurnState::GameOver;
        }
    });

    *turn = next_state;
}
