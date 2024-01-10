pub use crate::prelude::*;

use self::{
    end_turn::*, entity_render::*, hud::*, map_render::*, movement::*, random_move::*, tooltips::*,
};

mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .add_system(hud_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .add_system(hud_system())
        .build()
}
