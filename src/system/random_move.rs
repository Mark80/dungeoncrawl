use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MoveRandomly)]
pub fn move_random(world: &mut SubWorld, #[resource] map: &Map) {
    let draw_batch = &mut DrawBatch::new();
    draw_batch.target(1);

    let mut enemies = <(&mut Point, &MoveRandomly)>::query();

    enemies.iter_mut(world).for_each(|(pos, _)| {
        let mut rnd = RandomNumberGenerator::new();
        let destination = match rnd.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        if map.can_enter_in_tile(destination) {
            *pos = destination;
        }
    })
}
