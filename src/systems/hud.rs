use macroquad::{
    prelude::{Color, WHITE, BLACK},
    text::{Font, get_text_center, draw_text, draw_text_ex, TextParams}, window::{screen_width, screen_height}, shapes::{draw_circle, draw_rectangle_lines},
};

use crate::{prelude::*, stage};

#[system]

// #[system(for_each)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld, #[resource] font: &Font) {
// pub fn hud(ecs: &World) {
    // let mut health_querry = <&Health>::query().filter(component::<Player>());
    // let player_health = health_querry.iter(ecs).next().unwrap();

    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("egui ❤ macroquad")
            .show(egui_ctx, |ui| {
                ui.label("Test");
            });
    });
    
    egui_macroquad::draw();

    // stage::print_color_centered_utils(100., "test", WHITE, *font);
    
    // draw_rectangle_lines(20., 100., 1000., 50., 10., WHITE);
    // draw_circle(20., 100., 20., WHITE);



    // draw_batch.target(2);
    // draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    // draw_batch.bar_horizontal(
    //     Point::zero(),
    //     SCREEN_WIDTH,
    //     player_health.current,
    //     player_health.max,
    //     ColorPair::new(RED, BLACK),
    // );
    // draw_batch.print_color_centered(
    //     0,
    //     format!("Health: {} / {} ", player_health.current, player_health.max),
    //     ColorPair::new(WHITE, RED),
    // );

    // let player = <(Entity, &Player)>::query()
    //     .iter(ecs)
    //     .map(|(entity, _)| *entity)
    //     .next()
    //     .unwrap();
    // let mut item_query = <(&Name, &Carried)>::query().filter(component::<Item>());
    // let mut y = 3;
    // item_query
    //     .iter(ecs)
    //     .filter(|(_, carried)| carried.0 == (player))
    //     .for_each(|(name, _)| {

    //         draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));

    //         y += 1;
    //     });
    // if y > 3 {

    //     draw_batch.print_color(
    //         Point::new(3, 2),
    //         "Items carried",
    //         ColorPair::new(YELLOW, BLACK),

    //     );
    // }

    // draw_batch.submit(10000).expect("Batch error")
}
