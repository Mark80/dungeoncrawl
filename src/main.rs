extern crate core;

mod camera;
mod components;
mod entity_spawner;
mod map;
mod map_builder;
mod systems;
mod turn_state;

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
    pub use crate::entity_spawner::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let rnd = &mut RandomNumberGenerator::new();
        let builder = MapBuilder::new(rnd);
        let mut world = World::default();
        spawn_player(&mut world, builder.player_start_point);
        spawn_amulet_of_yala(&mut world, builder.amulet_position);

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
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs: world,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK, "you have found the amulet");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala is yours!!!!");
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "You can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.ecs = World::default();
            self.resources = Resources::default();
            let mut rng = RandomNumberGenerator::new();
            let map_builder = MapBuilder::new(&mut rng);
            spawn_player(&mut self.ecs, map_builder.player_start_point);
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_position);
            map_builder
                .rooms
                .iter()
                .skip(1)
                .map(|r| r.center())
                .for_each(|pos| spawn_enemy(&mut self.ecs, &mut rng, pos));
            self.resources.insert(map_builder.map);
            self.resources
                .insert(Camera::new(map_builder.player_start_point));
            self.resources.insert(TurnState::AwaitingInput);
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.ecs = World::default();
            self.resources = Resources::default();
            let mut rng = RandomNumberGenerator::new();
            let map_builder = MapBuilder::new(&mut rng);
            spawn_player(&mut self.ecs, map_builder.player_start_point);
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_position);
            map_builder
                .rooms
                .iter()
                .skip(1)
                .map(|r| r.center())
                .for_each(|pos| spawn_enemy(&mut self.ecs, &mut rng, pos));
            self.resources.insert(map_builder.map);
            self.resources
                .insert(Camera::new(map_builder.player_start_point));
            self.resources.insert(TurnState::AwaitingInput);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }
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
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
