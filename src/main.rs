#![allow(clippy::uninlined_format_args)]
mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
// mod tiled;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    //pub use crate::player::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}
use macroquad::{
    prelude::get_last_key_pressed,
    text::load_ttf_font,
    window::Conf,
};
use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);

        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&ecs, &mut rng, 0, &map_builder.monster_spawns).flush(&mut ecs, &mut resources);
        map_builder
            .lantern_spawns
            .iter()
            .for_each(|pos| spawn_lantern(&mut ecs, *pos));
        spawner_player(&mut ecs, map_builder.player_start);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimend, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Yout town is saved, and you can return to your normal life",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press I to play again");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawner_player(&mut self.ecs, map_builder.player_start);
        //spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&self.ecs, &mut rng, 0, &map_builder.monster_spawns)
            .flush(&mut self.ecs, &mut self.resources);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();
        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(e, _)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });
        let mut cb = CommandBuffer::new(&self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs, &mut self.resources);
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        spawn_level(
            &self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        )
        .flush(&mut self.ecs, &mut self.resources);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
    fn state_tick(&mut self) {
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            // TurnState::GameOver => self.game_over(ctx),
            // TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
            _ => todo!(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        self.state_tick();
        render_draw_buffer(ctx).expect("Render error");
    }
}

// fn main() -> BError {
//     let context = BTermBuilder::new()
//         .with_title("DungeonCrawler")
//         .with_fps_cap(30.0)
//         .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
//         .with_tile_dimensions(32, 32)
//         .with_resource_path("resources/")
//         .with_font("dungeonfont.png", 32, 32)
//         .with_font("terminal8x8.png", 8, 8)
//         .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
//         .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
//         .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png")
//         .build()?;
//     main_loop(context, State::new())
// }

mod stage {
    use super::*;
    use macroquad::{
        prelude::{
            get_last_key_pressed, is_key_released, load_string, Color, KeyCode, DARKGRAY,
        },
        text::{draw_text_ex, get_text_center, Font, TextParams},
        texture::{load_texture, FilterMode},
        window::{clear_background, next_frame, screen_width},
    };
    use macroquad_tiled::Map;

    pub fn print_color_centered_utils(y: f32, text: &str, color: Color, font: Option<Font>) {
        let center = get_text_center(text, font, 20, 1.0, 0.);
        draw_text_ex(
            text,
            screen_width() / 2. - center.x,
            y - center.y * 2.,
            TextParams {
                font: font.unwrap_or(TextParams::default().font),
                color,
                ..Default::default()
            },
        );
    }

    // TODO Move TileSet inside legion ressource and remove tiled_map. only tileset is usefull
    pub(crate) async fn init_tilemap() -> Map {
        let tileset = load_texture("resources/dungeonfont.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let tiled_map_json = load_string("resources/map.json").await.unwrap();

        macroquad_tiled::load_map(&tiled_map_json, &[("dungeonfont.png", tileset)], &[]).unwrap()
    }

    pub(crate) async fn main_loop(state: &mut State) {
        clear_background(DARKGRAY);

        if let Some(key) = get_last_key_pressed() {
            state.resources.insert(Some(key));
        } else if let Some(mut key) = state.resources.get_mut::<Option<KeyCode>>() {
            if key
                .as_mut()
                .map(|key| is_key_released(*key))
                .unwrap_or_default()
            {
                *key = None;
            }
        }

        state.state_tick();
        // draw_text(
        //     "abcsdqsds",
        //     screen_width()/2.,
        //     screen_height()/2.,
        //     50.,
        //     BLACK,
        // );

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "ROGUE".to_owned(),
        // platform: Platform {
        //     linux_backend: LinuxBackend::WaylandOnly,
        //     ..Default::default()
        // },
        high_dpi: true,
        window_resizable: false,
        window_width: DISPLAY_WIDTH * 32,
        window_height: DISPLAY_HEIGHT * 32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut tilesets = stage::init_tilemap().await.tilesets;
    let tileset = tilesets.remove("dungeonfont").unwrap();
    let font = load_ttf_font("resources/font.ttf").await.unwrap();

    let mut state = State::new();
    state.resources.insert(tileset);
    state.resources.insert(get_last_key_pressed());
    state.resources.insert(font);
    loop {
        stage::main_loop(&mut state).await;
    }
}
