use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
pub fn chasing(ecs: &SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).next().unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];

    let djikstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        MAX_FLOWMAP_DISTANCE,
    );

    for (entity, pos, _, fov) in movers.iter(ecs) {
        if !fov.visible_tiles.contains(player_pos) {
            continue;
        }
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&djikstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;

            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        println!("Attacking player?");
                        let victim = *victim;
                        let attacker = *entity;
                        commands.push(((), WantsToAttack { attacker, victim }));
                        attacked = true;
                    }
                });

            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::DerefMut;

    use self::empty::EmptyArchitect;

    use super::*;

    fn build_schedule() -> Schedule {
        Schedule::builder().add_system(chasing_system()).build()
    }

    #[test]
    fn test_chasing() {
        let mut state = StateFixture::default().with_schedule(build_schedule());
        let mut enemy_fov = FieldOfView::new(6);
        enemy_fov.visible_tiles.insert(Point::zero());

        let enemy = state
            .world
            .push((Enemy, ChasingPlayer, enemy_fov, Point::new(2, 0)));
        let expected_destination = Point::new(1, 0);

        state.step();

        println!("{:?}", state.world);

        let WantsToMove {
            entity,
            destination,
        } = <&WantsToMove>::query()
            .iter(&state.world)
            .next()
            .expect("Expected enemy to move");

        assert_eq!(&enemy, entity);
        assert_eq!(destination, &expected_destination);
    }

    struct StateFixture {
        step_schedule: Schedule,
        world: World,
        resources: Resources,
        player: Option<Entity>,
    }

    impl StateFixture {
        pub fn new(schedule: Schedule) -> Self {
            Self {
                step_schedule: schedule,
                world: World::default(),
                resources: Resources::default(),
                player: None,
            }
        }

        pub fn with_player(mut self) -> Self {
            self.player = Some(spawn_player(&mut self.world, Point::zero()));
            self
        }

        pub fn with_map(mut self, map: Map) -> Self {
            self.resources.insert(map);
            self
        }

        pub fn with_camera(mut self, camera: Camera) -> Self {
            self.resources.insert(camera);
            self
        }

        pub fn with_schedule(mut self, schedule: Schedule) -> Self {
            self.step_schedule = schedule;
            self
        }

        pub fn step(&mut self) {
            self.step_schedule
                .execute(&mut self.world, &mut self.resources);
        }

        pub fn update<T, F>(&mut self, update: &mut F)
        where
            T: Resource,
            F: Fn(&mut T),
        {
            update(self.resources.get_mut::<T>().unwrap().deref_mut());
        }
    }

    impl Default for StateFixture {
        fn default() -> Self {
            let map_builder = EmptyArchitect {}.build(&mut RandomNumberGenerator::new());
            let camera = Camera::new(Point::zero());

            Self::new(Schedule::builder().build())
                .with_map(map_builder.map)
                .with_camera(camera)
                .with_player()
        }
    }
}
