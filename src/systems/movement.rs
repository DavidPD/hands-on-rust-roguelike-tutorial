use crate::prelude::*;

#[system(for_each)]
// Components still have to be declared here because they're not referenced from a query. :(
// Furthermore they can't be included as optional components because they're not on `entity`
// Ask me how I know 😅
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    for pos in fov.visible_tiles.iter() {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    }
                }
            }
        }

        commands.remove(*entity);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use empty::EmptyArchitect;

    #[test]
    fn test_movement() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();
        state.step(state.player, destination);

        assert_eq!(state.player_pos(), destination);
    }

    #[test]
    fn test_blocked() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();
        state.resources.get_mut::<Map>().unwrap().tiles[map_idx(0, 1)] = TileType::Wall;
        state.step(state.player, destination);

        assert_eq!(state.player_pos(), Point::zero());
    }

    #[test]
    fn test_fov() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();

        assert!(!state.player_fov().is_dirty);

        state.step(state.player, destination);

        assert!(state.player_fov().is_dirty);
    }

    #[test]
    fn test_camera_moves_for_player() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();

        let camera_start_top_y = state.resources.get::<Camera>().unwrap().top_y;

        state.step(state.player, destination);

        assert_ne!(
            state.resources.get::<Camera>().unwrap().top_y,
            camera_start_top_y
        );
    }

    #[test]
    fn test_camera_still_for_enemy() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();

        let camera_start_top_y = state.resources.get::<Camera>().unwrap().top_y;

        let enemy = state
            .world
            .push((Enemy, FieldOfView::new(6), Point::zero()));

        state.step(enemy, destination);

        assert_eq!(
            state.resources.get::<Camera>().unwrap().top_y,
            camera_start_top_y
        );
        assert_eq!(
            state
                .world
                .entry(enemy)
                .unwrap()
                .get_component::<Point>()
                .unwrap(),
            &destination
        ); // just to be sure the enemy moves
    }

    #[test]
    fn test_revealed_tiles() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();

        let mut player = state.world.entry_mut(state.player).unwrap();
        player
            .get_component_mut::<FieldOfView>()
            .unwrap()
            .visible_tiles
            .insert(Point::zero());

        let num_tiles = |state: &MovementSystemTest| {
            state
                .resources
                .get::<Map>()
                .unwrap()
                .revealed_tiles
                .iter()
                .filter(|&&t| t)
                .count()
        };

        assert_eq!(num_tiles(&state), 0);

        state.step(state.player, destination);

        assert_eq!(num_tiles(&state), 1);
    }

    struct MovementSystemTest {
        world: World,
        resources: Resources,
        player: Entity,
    }

    impl MovementSystemTest {
        fn new() -> Self {
            let mut world = World::default();
            let mut resources = Resources::default();
            let map_builder = EmptyArchitect {}.build(&mut RandomNumberGenerator::new());
            let camera = Camera::new(Point::zero());
            let player = spawn_player(&mut world, Point::zero());

            resources.insert(map_builder.map);
            resources.insert(camera);

            Self {
                world,
                resources,
                player,
            }
        }

        fn setup(mut self) -> Self {
            let mut player = self.world.entry_mut(self.player).unwrap();
            player.get_component_mut::<FieldOfView>().unwrap().is_dirty = false;

            self
        }

        fn step(&mut self, entity: Entity, move_to: Point) {
            let wants_to_move_component = WantsToMove {
                entity,
                destination: move_to,
            };

            self.world.push(((), wants_to_move_component));

            let mut system = movement_system();
            system.run(&mut self.world, &mut self.resources);

            system
                .command_buffer_mut(self.world.id())
                .unwrap()
                .flush(&mut self.world, &mut self.resources);
        }

        fn player_pos(&mut self) -> Point {
            *self
                .world
                .entry(self.player)
                .unwrap()
                .get_component::<Point>()
                .unwrap()
        }

        fn player_fov(&mut self) -> FieldOfView {
            self.world
                .entry(self.player)
                .unwrap()
                .get_component::<FieldOfView>()
                .unwrap()
                .clone()
        }
    }
}
