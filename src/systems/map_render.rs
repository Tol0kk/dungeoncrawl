use macroquad::prelude::vec2;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad_tiled::TileSet;

use crate::prelude::*;

#[allow(clippy::borrowed_box)]
#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(BigFieldOfView)]
#[read_component(Light)]
#[write_component(Render)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &mut Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
    #[resource] tileset : &TileSet
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let mut fov_light = <&FieldOfView>::query().filter(component::<Light>());
    let mut big_fov = <&BigFieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).next().unwrap();

    let player_big_fov = big_fov.iter(ecs).next().unwrap();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            if map.in_bound(pt) && (player_fov.visible_tiles.contains(&pt)  || map.revealed_tiles[idx] || map.far_revealed_tiles[idx])
            {


                let light_far_visible = |light_fov: &FieldOfView| {
                    light_fov.visible_tiles.contains(&pt)
                        && player_big_fov.visible_tiles.contains(&pt)
                };

                let tint = if fov_light.iter(ecs).any(light_far_visible) {
                    map.revealed_tiles[map_idx(pt.x, pt.y)] = true;
                    macroquad::prelude::YELLOW
                    
                } else {
                    if !map.revealed_tiles[idx] {
                        continue;
                    }
                    if player_fov.visible_tiles.contains(&pt) {
                    macroquad::prelude::WHITE
                } else {
                    macroquad::prelude::DARKGRAY
                }};

                let glyph = theme.tile_to_render(map.tiles[idx]);
                let pt = (pt - offset) * 32;

                draw_texture_ex(
                    tileset.texture,
                    pt.x as f32,
                    (pt.y) as f32,
                    tint,
                    DrawTextureParams {
                        dest_size: Some(vec2(32., 32.)),
                        source: Some(glyph.into()),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
