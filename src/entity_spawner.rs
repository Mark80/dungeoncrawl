use crate::prelude::*;

const MAX_HEALTH: i32 = 1;

pub fn spawn_player(world: &mut World, player_position: Point) {
    world.push((
        Player,
        player_position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: MAX_HEALTH,
            max: MAX_HEALTH,
        },
    ));
}

pub fn spawn_enemy(world: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => spawn_goblin(),
        _ => spawn_orc(),
    };
    world.push((
        Enemy,
        //MoveRandomly,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Name(name),
        Health {
            current: hp,
            max: hp,
        },
        ChasingPlayer {},
    ));
}

fn spawn_goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn spawn_orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
