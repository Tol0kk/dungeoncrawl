use crate::prelude::*;

use super::MyTile;

// TODO: theme is add to MapBuilder but is use only one time. search for an alternative

pub struct DungeonTheme {
}

impl DungeonTheme {
    pub fn build() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> MyTile {
        match tile_type {
            TileType::Floor => (15,3).into(),
            TileType::Wall => (4,3).into(),
            TileType::Exit => (15,4).into(),
        }
    }
}

pub struct ForestTheme {
    // tile_set: macroquad_tiled::Map,
}

impl ForestTheme {
    pub fn build() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> MyTile {
        match tile_type {
            TileType::Floor => (12,4).into(),
            TileType::Wall => (1,2).into(),
            TileType::Exit => (15,4).into(),
        }
    }
}

