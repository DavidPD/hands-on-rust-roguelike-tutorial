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

    fn init_test() -> (World, Resources, Map, Camera, CommandBuffer) {
        let world = World::default();
        let resources = Resources::default();
        let map = Map::new();
        EmptyArchitect {}.build(&mut RandomNumberGenerator::new());
        let camera = Camera::new(Point::zero());
        let mut command_buffer = CommandBuffer::new(&world);

        (world, resources, map, camera, command_buffer)
    }

    #[test]
    fn test_movement() {
        let (mut world, mut resources, mut map, mut camera, mut cb) = init_test();
        let player = spawn_player(&mut world, Point::zero());
        let wants_to_move_component = WantsToMove {
            entity: player,
            destination: Point::new(0, 1),
        };
        let wants_to_move = world.push(((), wants_to_move_component));

        cb.flush(&mut world, &mut resources);
        let mut subworld = unsafe { SubWorld::new_unchecked(&world, ComponentAccess::All, None) };

        movement(
            &wants_to_move,
            &wants_to_move_component,
            &mut map,
            &mut camera,
            &mut subworld,
            &mut cb,
        );

        cb.flush(&mut world, &mut resources);

        assert_eq!(
            world
                .entry(player)
                .unwrap()
                .get_component::<Point>()
                .unwrap(),
            &Point::new(0, 1)
        )
    }
}
