use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(world: &SubWorld, commands: &mut CommandBuffer) {
    let mut player_position = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(world).for_each(|p| player_position = *p);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    enemies
        .iter(world)
        .filter(|(_, e_p)| **e_p == player_position)
        .for_each(|(entity, _)| commands.remove(*entity))
}
