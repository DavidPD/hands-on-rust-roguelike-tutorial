pub use crate::prelude::*;

dry_mods::mods! {
    mod use end_turn;
    mod use entity_render;
    mod use hud;
    mod use map_render;
    mod use movement;
    mod use player_input;
    mod use random_move;
    mod use tooltips;
    mod use combat;
    mod use chasing;
    mod use fov;
}

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(fov_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(fov_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .add_system(chasing_system())
        .flush()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(fov_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(end_turn_system())
        .build()
}
