use macroquad::{
    prelude::{BLACK, RED, WHITE},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_text_ex, Font, TextParams},
    window::{screen_width},
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
    let mut health_querry = <&Health>::query().filter(component::<Player>());
    let player_health = health_querry.iter(ecs).next().unwrap();

    // egui_macroquad::ui(|egui_ctx| {
    //     egui::Window::new("egui ❤ macroquad")
    //         .show(egui_ctx, |ui| {
    //             ui.label("Test");
    //         });
    // });

    // egui_macroquad::draw();

    draw_rectangle(0., 0., screen_width(), 30., BLACK);
    draw_rectangle(
        0.,
        0.,
        screen_width() * player_health.current as f32 / player_health.max as f32,
        30.,
        RED,
    );
    draw_rectangle_lines(0., 0., screen_width(), 30., 2., WHITE);
    stage::print_color_centered_utils(
        0.,
        format!("Healt: {} / {}", player_health.current, player_health.max).as_str(),
        WHITE,
        None,
    );

    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _)| *entity)
        .next()
        .unwrap();
    let mut item_query = <(&Name, &Carried)>::query().filter(component::<Item>());
    let mut y = 80;
    item_query
        .iter(ecs)
        .filter(|(_, carried)| carried.0 == (player))
        .for_each(|(name, _)| {
            draw_text_ex(
                &format!("{} : {}", y - 20, &name.0),
                40.,
                y as f32,
                TextParams {
                    ..Default::default()
                },
            );

            y += 10;
        });
    if y > 80 {
        draw_text_ex(
            "Item carried",
            40.,
            80.,
            TextParams {
                ..Default::default()
            },
        );
    }

    // draw_batch.submit(10000).expect("Batch error")
}
