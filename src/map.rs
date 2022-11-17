use crate::prelude::*;
use std::os::unix::raw::ino_t;
use std::path::is_separator;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let position = self.index_to_point2d(idx);

        self.valid_exit(position, Point::new(-1, 0))
            .iter()
            .for_each(|idx| {
                exits.push((*idx, 1.0));
            });

        self.valid_exit(position, Point::new(1, 0))
            .iter()
            .for_each(|idx| {
                exits.push((*idx, 1.0));
            });

        self.valid_exit(position, Point::new(0, -1))
            .iter()
            .for_each(|idx| {
                exits.push((*idx, 1.0));
            });

        self.valid_exit(position, Point::new(0, 1))
            .iter()
            .for_each(|idx| {
                exits.push((*idx, 1.0));
            });

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl Map {
    pub fn valid_exit(&self, position: Point, delta: Point) -> Option<usize> {
        let destination = position + delta;
        if self.in_bounds(destination) && self.can_enter_in_tile(destination) {
            Some(self.point2d_to_index(destination))
        } else {
            None
        }
    }

    pub fn try_index(&self, p: Point) -> Option<usize> {
        if self.in_bounds(p) {
            Some(map_idx(p.x, p.y))
        } else {
            None
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < SCREEN_WIDTH && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_in_tile(&self, point: Point) -> bool {
        let index = map_idx(point.x, point.y);
        self.in_bounds(point) && self.tiles[index] != TileType::Wall
    }

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrac_tile_index_from_coordinate() {
        assert_eq!(Map::map_idx(Point::new(0, 0)), 0);
        assert_eq!(Map::map_idx(Point::new(20, 0)), 20);
        assert_eq!(Map::map_idx(Point::new(0, 1)), 80);
    }
    #[test]
    fn test_extrac_coordinates_from_index() {
        assert_eq!(Map::coordinates_from_index(0), Point::new(0, 0));
        assert_eq!(Map::coordinates_from_index(80), Point::new(0, 1));
        assert_eq!(Map::coordinates_from_index(85), Point::new(5, 1));
        assert_eq!(Map::coordinates_from_index(163), Point::new(3, 2));
    }

    #[test]
    fn test_can_enter_in_tile() {
        let mut map = Map::new();
        map.tiles[0] = TileType::Wall;
        assert_eq!(map.can_enter_in_tile(Point::new(0, 0)), false);
        assert_eq!(map.can_enter_in_tile(Point::new(1, 0)), true);
    }
}
