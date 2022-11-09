mod collision_detection;
mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::render_entity_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collision_detection::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::render_entity_system())
        .add_system(end_turn::end_turn_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::move_random_system())
        .flush()
        .add_system(collision_detection::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::render_entity_system())
        .add_system(end_turn::end_turn_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}