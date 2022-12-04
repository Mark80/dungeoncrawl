use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
#[read_component(Player)]
pub fn end_turn(#[resource] turn: &mut TurnState, world: &SubWorld) {
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let current_state = turn.clone();
    let mut next_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let amulet_position = amulet.iter(world).nth(0).unwrap();

    player.iter(world).for_each(|(hp, pos)| {
        if hp.current < 1 {
            next_state = TurnState::GameOver;
        }
        if pos == amulet_position {
            next_state = TurnState::Victory;
        }
    });

    *turn = next_state;
}
