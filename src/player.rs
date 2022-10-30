use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let position = self.position;
        ctx.set(position.x, position.y, WHITE, BLACK, to_cp437('@'));
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(m) = ctx.key {
            let delta = match m {
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Left => Point::new(-1, 0),
                _ => Point::zero(),
            };
            let new_position = self.position + delta;
            if map.can_enter_in_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
