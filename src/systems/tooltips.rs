use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(world: &SubWorld, #[resource] mouse_position: &Point, #[resource] camera: &Camera) {
    let mut entities = <(Entity, &Point, &Name)>::query();
    let draw_batch = &mut DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_position + offset;

    entities.iter(world).for_each(|(entity, position, name)| {
        if *position == map_pos {
            let screen_position = *mouse_position * 4;
            let display =
                if let Ok(health) = world.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_position, &display);
        }
    });
    draw_batch.submit(10100).expect("Batch error");
}
