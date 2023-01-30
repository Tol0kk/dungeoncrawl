use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(BigFieldOfView)]
#[read_component(Player)]
#[read_component(Light)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut big_fov = <&BigFieldOfView>::query().filter(component::<Player>());
    let mut lights_fov = <(&Point, &FieldOfView)>::query().filter(component::<Light>());
    let mut renderables = <(&Point, &Render)>::query();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = fov.iter(ecs).next().unwrap();
    let player_big_fov = big_fov.iter(ecs).next().unwrap();
    renderables
        .iter(ecs)
        .filter(|(pos, _)| {
            player_fov.visible_tiles.contains(&pos)
                || (player_big_fov.visible_tiles.contains(&pos) && {
                    lights_fov
                        .iter(ecs)
                        .any(|(pt,_)| *pt == **pos )
                })
        })
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
            //println!("{:?}",pos)
        });
    draw_batch.submit(5000).expect("Batch Error");
}
