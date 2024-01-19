use crate::prelude::*;

#[system(for_each)]
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
    use empty::EmptyArchitect;

    use super::*;
    // use crate::prelude::*;

    #[test]
    fn test_movement() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();
        state.step(destination);

        assert_eq!(state.player_pos(), destination);
    }

    #[test]
    fn test_blocked() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();
        state.map.tiles[map_idx(0, 1)] = TileType::Wall;
        state.step(destination);

        assert_eq!(state.player_pos(), Point::zero());
    }

    #[test]
    fn test_fov() {
        let destination = Point::new(0, 1);
        let mut state = MovementSystemTest::new().setup();

        assert!(!state.player_fov().is_dirty);

        state.step(destination);

        assert!(state.player_fov().is_dirty);
    }

    struct MovementSystemTest {
        world: World,
        resources: Resources,
        map: Map,
        camera: Camera,
        cb: CommandBuffer,
        player: Entity,
    }

    impl MovementSystemTest {
        fn new() -> Self {
            let mut world = World::default();
            let resources = Resources::default();
            let map = Map::new();
            EmptyArchitect {}.build(&mut RandomNumberGenerator::new());
            let camera = Camera::new(Point::zero());
            let cb = CommandBuffer::new(&world);
            let player = spawn_player(&mut world, Point::zero());

            Self {
                world,
                resources,
                map,
                camera,
                cb,
                player,
            }
        }

        fn setup(mut self) -> Self {
            let mut player = self.world.entry_mut(self.player).unwrap();
            player.get_component_mut::<FieldOfView>().unwrap().is_dirty = false;

            self
        }

        fn step(&mut self, move_to: Point) {
            let wants_to_move_component = WantsToMove {
                entity: self.player,
                destination: move_to,
            };

            let wants_to_move = self.world.push(((), wants_to_move_component));
            self.cb.flush(&mut self.world, &mut self.resources);
            let mut subworld =
                unsafe { SubWorld::new_unchecked(&self.world, ComponentAccess::All, None) };

            movement(
                &wants_to_move,
                &wants_to_move_component,
                &mut self.map,
                &mut self.camera,
                &mut subworld,
                &mut self.cb,
            );

            self.cb.flush(&mut self.world, &mut self.resources);
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
