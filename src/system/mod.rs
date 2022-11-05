mod collision_detection;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collision_detection::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::render_entity_system())
        .add_system(random_move::move_random_system())
        .build()
}
