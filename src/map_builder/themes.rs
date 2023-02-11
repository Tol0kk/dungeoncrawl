use crate::prelude::*;

// TODO: theme is add to MapBuilder but is use only one time. search for an alternative

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn build() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}
impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn build() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
        }
    }
}