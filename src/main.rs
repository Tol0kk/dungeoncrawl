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
    prelude::{get_last_key_pressed, KeyCode},
    text::load_ttf_font,
    window::Conf,
};
use macroquad_tiled::TileSet;
use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new(tileset: TileSet, font: macroquad::text::Font) -> Self {
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
        resources.insert(get_last_key_pressed());
        resources.insert(tileset);
        resources.insert(font);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
    fn game_over(&mut self) {
        use macroquad::prelude::{GREEN, RED, WHITE, YELLOW};
        stage::print_color_centered_utils(20., "Your quest has ended.", RED, None);
        stage::print_color_centered_utils(
            40.,
            "Slain by a monster, your hero's journey has come to a premature end.",
            WHITE,
            None,
        );
        stage::print_color_centered_utils(
            60.,
            "The Amulet of Yala remains unclaimend, and your home town is not saved.",
            WHITE,
            None,
        );
        stage::print_color_centered_utils(
            80.,
            "Don't worry, you can always try again with a new hero.",
            YELLOW,
            None,
        );
        stage::print_color_centered_utils(100., "Press 1 to play again.", GREEN, None);
        if let Some(KeyCode::R) = get_last_key_pressed() {
            self.reset_game_state();
        }
    }
    fn victory(&mut self) {
        use macroquad::prelude::{GREEN, WHITE};
        stage::print_color_centered_utils(20., "You have won.", GREEN, None);
        stage::print_color_centered_utils(
            40.,
            "You put on the Amulet of Yala and feel its power course through your veins",
            WHITE,
            None,
        );
        stage::print_color_centered_utils(
            60.,
            "Yout town is saved, and you can return to your normal life",
            WHITE,
            None,
        );
        stage::print_color_centered_utils(80., "Press I to play again", GREEN, None);
        if let Some(KeyCode::R) = get_last_key_pressed() {
            self.reset_game_state();
        }
    }
    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        self.resources.remove::<Map>();
        self.resources.remove::<Camera>();
        self.resources.remove::<TurnState>();
        self.resources.remove::<Box<dyn MapTheme>>();
        self.resources.remove::<KeyCode>();

        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&self.ecs, &mut rng, 0, &map_builder.monster_spawns)
            .flush(&mut self.ecs, &mut self.resources);
        map_builder
            .lantern_spawns
            .iter()
            .for_each(|pos| spawn_lantern(&mut self.ecs, *pos));
        spawner_player(&mut self.ecs, map_builder.player_start);

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
        self.resources.insert(get_last_key_pressed());
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
            TurnState::GameOver => self.game_over(),
            TurnState::Victory => self.victory(),
            TurnState::NextLevel => self.advance_level(),
            _ => todo!(),
        }
    }
}

mod stage {
    use super::*;
    use macroquad::{
        prelude::{get_last_key_pressed, is_key_released, load_string, Color, KeyCode, DARKGRAY},
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

    let mut state = State::new(tileset, font);

    loop {
        stage::main_loop(&mut state).await;
    }
}
