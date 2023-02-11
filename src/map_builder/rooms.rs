use super::MapArchitect;
use crate::prelude::*;

const NULM_ROOMS: usize = 20;
const MIN_ROOM_SIZE: i32 = 3;
const MAX_ROOM_SIZE: i32 = 10;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            lantern_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::build(),
        };
        mb.fill(TileType::Wall);
        self.build_random_rooms(rng, &mut mb.map, &mut mb.rooms);
        self.build_corridors(rng, &mut mb.map, &mut mb.rooms);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        mb.lantern_spawns = mb.spawn_lantern(rng);
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center())
        }
        mb
    }
}

impl RoomsArchitect {
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map, rooms: &mut Vec<Rect>) {
        while rooms.len() < NULM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - MAX_ROOM_SIZE - 1),
                rng.range(1, SCREEN_HEIGHT - MAX_ROOM_SIZE - 1),
                rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
                rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
            );
            let mut overlap = false;
            for r in rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 1 && p.x < SCREEN_WIDTH - 1 && p.y > 1 && p.y < SCREEN_HEIGHT - 1 {
                        let idx = map_idx(p.x, p.y);
                        map.tiles[idx] = TileType::Floor;
                    }
                });
                rooms.push(room)
            }
        }
    }
    fn build_corridors(&mut self,rng: &mut RandomNumberGenerator, map: &mut Map, rooms: &mut Vec<Rect>) {
        fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
            use std::cmp::{max, min};
            for y in min(y1, y2)..=max(y1, y2) {
                if let Some(idx) = map.try_idx(Point::new(x, y)) {
                    map.tiles[idx] = TileType::Floor
                }
            }
        }

        fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
            use std::cmp::{max, min};
            for x in min(x1, x2)..=max(x1, x2) {
                if let Some(idx) = map.try_idx(Point::new(x, y)) {
                    map.tiles[idx] = TileType::Floor
                }
            }
        }

        let mut rooms = rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                apply_horizontal_tunnel(map, prev.x, new.x, prev.y);
                apply_vertical_tunnel(map, prev.y, new.y, new.x);
            } else {
                apply_vertical_tunnel(map,prev.y, new.y, prev.x);
                apply_horizontal_tunnel(map,prev.x, new.x, new.y);
            }
        }
    }
}
