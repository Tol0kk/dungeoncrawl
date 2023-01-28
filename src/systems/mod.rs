pub use crate::prelude::*;


mod map_render;
mod player_inputs;
mod entity_render;
mod collisions;
mod random_move;
mod end_turn;
mod movements;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_inputs::player_inputs_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movements::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_moves_system())
        .flush()
        .add_system(movements::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()

}