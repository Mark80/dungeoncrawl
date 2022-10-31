use crate::prelude::*;
const NUM_ROOMS: usize = 10;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start_point: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start_point: Point::zero(),
        };

        mb.fill(TileType::WALL);
        mb.build_random_rooms(rng);
        mb.player_start_point = mb.rooms[0].center();
        mb.build_corridors(rng);
        //mb.link_rooms();
        mb
    }

    fn link_rooms(&mut self) {
        for i in 0..NUM_ROOMS - 1 {
            self.apply_vertical_tunnel(self.rooms[i].y1, self.rooms[i + 1].y1, self.rooms[i].x1);
            self.apply_horizontal_tunnel(
                self.rooms[i].x1,
                self.rooms[i + 1].x1,
                self.rooms[i + 1].y1,
            );
        }
    }

    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    pub fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(5, 10),
                rng.range(5, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let index = Map::index_from_coordinates(p);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
}
