use bracket_lib::terminal::{to_cp437, ColorPair, Point};

use crate::prelude::*;

use self::template::Templates;
mod template;

pub fn spawner_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(6),
        BigFieldOfView::new(80),
        Damage(1),
    ));
}

#[must_use]
pub fn spawn_level(
    ecs: &World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) -> CommandBuffer {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points)
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_lantern(ecs: &mut World, pos: Point) {
    ecs.push((
        Decor,
        Light {
            color: ColorPair::new(LIGHT_YELLOW, BLACK),
        },
        pos,
        Render {
            color: ColorPair::new(YELLOW, BLACK),
            glyph: to_cp437('s'),
        },
        Name("Lantern".to_string()),
        FieldOfView::new(2),
    ));
}
