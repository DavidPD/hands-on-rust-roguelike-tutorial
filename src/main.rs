// #![warn(clippy::pedantic)]

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
    pub const MAX_FLOWMAP_DISTANCE: f32 = 1024.0;
    pub const UNREACHABLE: &f32 = &f32::MAX;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use std::collections::HashSet;

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

    let mut state = State::new();
    state.start();
    main_loop(context, state)
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
        let ecs = World::default();
        let resources = Resources::default();
        // These get recreated immediately on start, so they should probably just be optional.

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn start(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new().build(&mut rng);
        let mut ecs = World::default();
        let mut resources = Resources::default();

        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_player(&mut ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);

        State::spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        self.ecs = ecs;
        self.resources = resources;
    }

    pub fn spawn_level(
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let template = Templates::load();
        template.spawn_entities(ecs, rng, level, spawn_points);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_HUD);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
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
            self.start();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.start();
        }
    }

    fn advance_level(&mut self) {
        let player = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player);

        for item in <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carried)| carried.0 == player)
            .map(|(&e, _carried)| e)
        {
            entities_to_keep.insert(item);
        }

        let mut cb = CommandBuffer::new(&mut self.ecs);

        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut map_level: usize = 0;

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new().build(&mut rng);
        for (player, pos) in <(&mut Player, &mut Point)>::query().iter_mut(&mut self.ecs) {
            player.map_level += 1;
            map_level = player.map_level;
            pos.x = map_builder.player_start.x;
            pos.y = map_builder.player_start.y;
        }

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        State::spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
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
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
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
