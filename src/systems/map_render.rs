use crate::prelude::*;
#[allow(clippy::borrowed_box)]

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(BigFieldOfView)]
#[read_component(Light)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut fov_light = <&FieldOfView>::query().filter(component::<Light>());
    let mut big_fov = <&BigFieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    let player_fov = fov.iter(ecs).next().unwrap();
    let player_big_fov = big_fov.iter(ecs).next().unwrap();
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            if map.in_bound(pt) && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]
            {
                let light_far_visible = |light_fov : &FieldOfView| {
                    light_fov.visible_tiles.contains(&pt)
                        && player_big_fov.visible_tiles.contains(&pt)
                };
                // let light_close_visible = |light_fov| {
                    // light_fov.visible_tiles.contains(&pt) && player_fov.visible_tiles.contains(&pt)
                // };
                let tint = if fov_light.iter(ecs).any(light_far_visible) {
                    LIGHTYELLOW
                // } else if fov_light.iter(ecs).any(light_close_visible) {
                    // LIGHTYELLOW
                } else if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARKGRAY
                };
                let glyph = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch Error")
}
