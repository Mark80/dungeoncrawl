use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player, destination) = players
            .iter(world)
            .find_map(|(entity, position)| Some((*entity, *position + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(world)
                .filter(|(entity, enemy_position)| destination == **enemy_position)
                .for_each(|(enemy, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantToAttack {
                            attacker: player,
                            victim: *enemy,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player,
                        destination,
                    },
                ));
            }
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
