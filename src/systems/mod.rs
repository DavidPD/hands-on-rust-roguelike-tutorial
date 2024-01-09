pub use crate::prelude::*;

use self::{
    collisions::collisions_system, entity_render::entity_render_system,
    map_render::map_render_system,
};

mod collisions;
mod entity_render;
mod map_render;
mod player_input;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(collisions_system())
        .build()
}
