use legion::world::SubWorld;

pub use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .map(|(&entity, &pos)| (entity, pos))
                    .next()
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query();

                for (&entity, &_item, &_pos) in items
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                {
                    commands.remove_component::<Point>(entity);
                    commands.add_component(entity, Carried(player));

                    if let Ok(e) = ecs.entry_ref(entity) {
                        if e.get_component::<Weapon>().is_ok() {
                            for (entity, _carried, _weapon) in
                                <(Entity, &Carried, &Weapon)>::query()
                                    .iter(ecs)
                                    .filter(|(_, c, _)| c.0 == player)
                            {
                                commands.remove(*entity);
                            }
                        }
                    }
                }

                Point::zero()
            }
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .map(|(entity, pos)| (*entity, *pos + delta))
            .next()
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        if delta != Point::zero() {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }

    fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
        println!("Using Item {}", n);
        let player_entity = <(Entity, &Player)>::query()
            .iter(ecs)
            .map(|(&entity, _)| entity)
            .next()
            .unwrap();

        let item_entity = <(Entity, &Item, &Carried)>::query()
            .iter(ecs)
            .filter(|(_, _, carried)| carried.0 == player_entity)
            .enumerate()
            .filter(|(item_count, _)| *item_count == n) // This seems super fragile, but it's consistent with the display I guess
            .map(|(_, (&entity, _, _))| entity)
            .next();

        if let Some(item_entity) = item_entity {
            println!("Pushing Command");
            commands.push((
                (),
                WantsToActivateItem {
                    used_by: player_entity,
                    item: item_entity,
                },
            ));
        }

        Point::zero()
    }
}
