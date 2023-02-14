use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(BigFieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                if let Ok(big_fov) = entry.get_component::<BigFieldOfView>() {
                    commands.add_component(want_move.entity, big_fov.clone_dirty());
                }

                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles
                        .iter()
                        .for_each(|pos| map.revealed_tiles[map_idx(pos.x, pos.y)] = true);
                    if let Ok(big_fov) = entry.get_component::<BigFieldOfView>() {
                        big_fov
                            .visible_tiles
                            .iter()
                            .for_each(|pos| map.far_revealed_tiles[map_idx(pos.x, pos.y)] = true);
                    }
                }
            }
        };
    }
    commands.remove(*entity)
}
