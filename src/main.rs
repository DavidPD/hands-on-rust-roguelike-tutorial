mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::*;
    pub use legion::world::*;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const TILE_SIZE: i32 = 32;
    pub const LAYER_MAP: usize = 0;
    pub const LAYER_ENTITIES: usize = 1;
    pub const LAYER_HUD: usize = 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

fn main() -> BError {
    let font = "dungeonfont.png";
    let terminal_font = "terminal8x8.png";

    let context = BTermBuilder::new()
        .with_title("Rusty Rogue")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_SIZE, TILE_SIZE)
        .with_resource_path("resources")
        .with_font(font, TILE_SIZE, TILE_SIZE)
        .with_font(terminal_font, 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font)
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, terminal_font)
        .build()?;

    main_loop(context, State::new())
}

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        let mut ecs = World::default();
        let mut resources = Resources::default();

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        spawn_player(&mut ecs, map_builder.player_start);

        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.clear_console(ctx);

        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = *self.resources.get::<TurnState>().unwrap();

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
        }

        render_draw_buffer(ctx).expect("Render Error");
    }
}

impl State {
    fn clear_console(&self, ctx: &mut BTerm) {
        let num_layers = 2;
        for i in 0..=num_layers {
            ctx.set_active_console(i);
            ctx.cls()
        }
        ctx.set_active_console(LAYER_MAP);
    }
}
