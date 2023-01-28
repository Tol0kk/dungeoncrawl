pub use crate::prelude::*;

mod map_render;
mod player_inputs;
mod entity_render;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_inputs::player_inputs_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}
