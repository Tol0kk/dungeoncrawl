use macroquad::{
    prelude::{vec2, RED, WHITE},
    texture::{draw_texture_ex, DrawTextureParams},
};
use macroquad_tiled::TileSet;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(BigFieldOfView)]
#[read_component(Player)]
#[read_component(Light)]
#[read_component(Health)]
#[write_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera, #[resource] tileset: &TileSet) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut big_fov = <&BigFieldOfView>::query().filter(component::<Player>());
    let mut lights_fov = <(&Point, &FieldOfView)>::query().filter(component::<Light>());
    let mut renderables = <(&Point, &Render, Entity)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = fov.iter(ecs).next().unwrap();
    let player_big_fov = big_fov.iter(ecs).next().unwrap();

    renderables
        .iter(ecs)
        .filter(|(pos, _, _)| {
            let light_far_visible = |(_, light_fov): (_, &FieldOfView)| {
                light_fov.visible_tiles.contains(&pos)
                    && player_big_fov.visible_tiles.contains(&pos)
            };
            player_fov.visible_tiles.contains(pos)
                || (player_big_fov.visible_tiles.contains(pos) && {
                    lights_fov.iter(ecs).any(light_far_visible)
                })
        })
        .for_each(|(pos, render, entity)| {
            // let tint = if let Ok(health) = ecs
            //     .entry_ref(*entity)
            //     .unwrap()
            //     .get_component::<Health>()
            // {
            //     if health.is_damaged() {
            //         RED
            //     } else {
            //         WHITE
            //     }
            // } else {
            //     WHITE
            // };
            let tint = ecs
                .entry_ref(*entity)
                .unwrap()
                .get_component::<Health>()
                .ok()
                .and_then(|h| h.is_damaged().then_some(RED))
                .unwrap_or(WHITE);

            let pt = (*pos - offset) * 32;

            draw_texture_ex(
                tileset.texture,
                pt.x as f32,
                pt.y as f32,
                tint,
                DrawTextureParams {
                    dest_size: Some(vec2(32., 32.)),
                    source: Some(render.glyph.into()),
                    ..Default::default()
                },
            );
        });


}
