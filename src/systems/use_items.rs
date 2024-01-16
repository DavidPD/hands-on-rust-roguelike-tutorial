use crate::prelude::*;

#[system]
#[read_component(WantsToActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)] // !
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    for (entity, activate) in <(Entity, &WantsToActivateItem)>::query().iter(ecs) {
        let item = ecs.entry_ref(activate.item).unwrap();

        if let Ok(healing) = item.get_component::<ProvidesHealing>() {
            healing_to_apply.push((activate.used_by, healing.amount));
        }

        if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
            for tile in map.revealed_tiles.iter_mut() {
                *tile = true;
            }
        }

        commands.remove(activate.item);
        commands.remove(*entity);
    }

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                println!("Healing");
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}
