use crate::prelude::*;

pub fn spawn_player(world: &mut World, player_position: Point) {
    world.push((
        Player,
        player_position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
