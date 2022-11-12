use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MoveRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn move_random(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let draw_batch = &mut DrawBatch::new();
    draw_batch.target(1);
    let mut monsters = <(Entity, &Point, &MoveRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    monsters.iter(world).for_each(|(entity, pos, _)| {
        let mut rnd = RandomNumberGenerator::new();
        let destination = match rnd.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        let mut attacked = false;
        positions
            .iter(world)
            .filter(|(_, target_position, _)| **target_position == destination)
            .for_each(|(victim, _, _)| {
                if world
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },
                    ));
                    attacked = true;
                }
            });
        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    })
}
