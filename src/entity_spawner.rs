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

pub fn spawn_enemy(world: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    world.push((
        Enemy,
        MoveRandomly,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ));
}
