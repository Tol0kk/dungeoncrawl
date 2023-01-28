use bracket_lib::terminal::{to_cp437, ColorPair, Point};

use crate::prelude::*;

pub fn spawner_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
