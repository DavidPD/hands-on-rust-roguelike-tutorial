use legion::world::SubWorld;

pub use crate::prelude::*;

#[system]
#[allow(clippy::too_many_arguments)] // Allowing for queries, this function will only be called by automation.
pub fn player_input(
    ecs: &mut SubWorld,
    players: &mut Query<(Entity, &Point, &Player)>,
    player_items: &mut Query<(Entity, &Item, &Carried)>,
    items_on_ground: &mut Query<(Entity, &Item, &Point)>,
    weapons: &mut Query<(Entity, &Carried, &Weapon)>,
    enemies: &mut Query<(Entity, &Point, &Enemy)>,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    let (player_entity, player_pos) = players
        .iter(ecs)
        .map(|(&entity, &pos, _player)| (entity, pos))
        .next()
        .unwrap();

    let mut player_items = player_items
        .iter(ecs)
        .filter(|(_entity, _item, carried)| carried.0 == player_entity)
        .map(|(entity, item, _carried)| (entity, item));

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Key1 => use_item(0, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key2 => use_item(1, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key3 => use_item(2, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key4 => use_item(3, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key5 => use_item(4, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key6 => use_item(5, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key7 => use_item(6, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key8 => use_item(7, &player_entity, &mut player_items, commands),
            VirtualKeyCode::Key9 => use_item(8, &player_entity, &mut player_items, commands),
            VirtualKeyCode::G => {
                for (&entity, &_item, &_pos) in items_on_ground
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                {
                    commands.remove_component::<Point>(entity);
                    commands.add_component(entity, Carried(player_entity));

                    if let Ok(e) = ecs.entry_ref(entity) {
                        if e.get_component::<Weapon>().is_ok() {
                            for (entity, _carried, _weapon) in
                                weapons.iter(ecs).filter(|(_, c, _)| c.0 == player_entity)
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
            .map(|(entity, pos, _player)| (*entity, *pos + delta))
            .next()
            .unwrap();

        if delta != Point::zero() {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == destination)
                .for_each(|(entity, _, _)| {
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

    fn use_item<'a, I>(
        n: usize,
        player_entity: &Entity,
        player_items: &mut I,
        commands: &mut CommandBuffer,
    ) -> Point
    where
        I: Iterator<Item = (&'a Entity, &'a Item)>,
    {
        println!("Using Item {}", n);

        let item_entity = player_items
            .enumerate()
            .filter(|(item_count, _)| *item_count == n) // This seems super fragile, but it's consistent with the display I guess
            .map(|(_, (entity, _))| entity)
            .next();

        if let Some(item_entity) = item_entity {
            println!("Pushing Command");
            commands.push((
                (),
                WantsToActivateItem {
                    used_by: *player_entity,
                    item: *item_entity,
                },
            ));
        }

        Point::zero()
    }
}
