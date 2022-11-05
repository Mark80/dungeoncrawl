use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn render_entity(world: &SubWorld, #[resource] camera: &Camera) {
    let draw_batch = &mut DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(& Point, & Render)>::query().iter(world).for_each(|(pos, render)| { draw_batch.set(
        *pos - offset, render.color, render.glyph
    ); };
}
