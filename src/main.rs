mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod system;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::system::*;
}

use prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let rnd = &mut RandomNumberGenerator::new();
        let builder = MapBuilder::new(rnd);
        let mut world = World::default();
        spawn_player(&mut world, builder.player_start_point);
        builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_enemy(&mut world, rnd, pos));
        let camera = Camera::new(builder.player_start_point);
        let mut resources = Resources::default();
        resources.insert(builder.map);
        resources.insert(camera);

        Self {
            ecs: world,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
