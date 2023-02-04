use crate::{
    map_builder::{
        prefab::apply_prefab,
        themes::{DungeonTheme, ForestTheme},
    },
    prelude::*,
};

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub lantern_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => {
                println!("Generate with CellularAutomataArchitect");
                Box::new(automata::CellularAutomataArchitect {})
            }
            1 => {
                println!("Generate with DrunkardWalkArchitect");
                Box::new(drunkard::DrunkardWalkArchitect {})
            }
            _ => {
                println!("Generate with RoomsArchitect");
                Box::new(rooms::RoomsArchitect {})
            }
        };

        //let mut architect = empty::EmptyArchitect{};
        let mut mb = architect.new(rng);

        apply_prefab(&mut mb, rng);

        mb.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };

        println!("{} lantern have been generated", mb.lantern_spawns.len());
        println!("{} monster have been generated", mb.monster_spawns.len()); // counting method not accurate with spawn_entity method
        println!(
            "The amulet of Yala has spawn at {},{}",
            mb.amulet_start.x, mb.amulet_start.y
        );
        mb
    }
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile)
    }
    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }
    fn spawn_monster(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
    fn spawn_lantern(&self, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_LANTERN: usize = 20;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..NUM_LANTERN {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
    fn complete_map_border(&self) {
        self.map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, _)| {
                let pt = self.map.index_to_point2d(*idx);
                pt.x == 0 || pt.y == 0 || pt.x == SCREEN_WIDTH || pt.y == SCREEN_HEIGHT
            })
            .for_each(|(_, mut t)| t = &TileType::Wall)
    }
}
