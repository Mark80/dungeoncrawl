use crate::prelude::*;
use std::ptr::addr_of_mut;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    WALL,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn index_from_coordinates(x: i32, y: i32) -> usize {
        ((SCREEN_WIDTH * y) + x) as usize
    }

    pub fn coordinates_from_index(index: usize) -> (i32, i32) {
        let y = index as i32 / SCREEN_WIDTH;
        let x = index as i32 % SCREEN_WIDTH;
        (x, y)
    }

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = Map::index_from_coordinates(x, y);
                let current_tile = self.tiles[index];
                match current_tile {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::WALL => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrac_tile_index_from_coordinate() {
        assert_eq!(Map::index_from_coordinates(0, 0), 0);
        assert_eq!(Map::index_from_coordinates(20, 0), 20);
        assert_eq!(Map::index_from_coordinates(0, 1), 80);
    }
    #[test]
    fn test_extrac_coordinates_from_index() {
        assert_eq!(Map::coordinates_from_index(0), (0, 0));
        assert_eq!(Map::coordinates_from_index(80), (0, 1));
        assert_eq!(Map::coordinates_from_index(85), (5, 1));
        assert_eq!(Map::coordinates_from_index(163), (3, 2));
    }
}
