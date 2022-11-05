use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let draw_batch = &mut DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(point) {
                let index = map_idx(x, y);
                let glyph = match map.tiles[index] {
                    TileType::Wall => to_cp437('.'),
                    TileType::Floor => to_cp437('#'),
                };
                draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Draw error");
}
