use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MoveRandomly)]
pub fn move_random(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let draw_batch = &mut DrawBatch::new();
    draw_batch.target(1);
    let mut monsters = <(Entity, &Point, &MoveRandomly)>::query();

    monsters.iter(world).for_each(|(entity, pos, _)| {
        let mut rnd = RandomNumberGenerator::new();
        let destination = match rnd.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    })
}
