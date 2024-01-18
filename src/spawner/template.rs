use crate::prelude::*;
use std::{collections::HashSet, fs::File};

use ron::de::from_reader;
use serde::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed to load templates file");
        from_reader(file).expect("Unable to deserialize templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        for template in self.entities.iter().filter(|e| e.levels.contains(&level)) {
            for _ in 0..template.frequency {
                available_entities.push(template);
            }
        }

        let mut commands = CommandBuffer::new(ecs);
        for &spawn in spawn_points.iter() {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(spawn, entity, &mut commands);
            }
        }
        commands.flush(ecs);
    }

    fn spawn_entity(&self, spawn: Point, template: &Template, commands: &mut CommandBuffer) {
        let entity = commands.push((
            spawn,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer);
                commands.add_component(entity, Health::new(template.hp.unwrap()));
            }
            EntityType::Item => commands.add_component(entity, Item),
        }

        if let Some(effects) = &template.provides {
            for (provides, n) in effects.iter() {
                match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap),
                    _ => panic!("Error, cannot provide component {}", provides),
                }
            }
        }

        if let Some(damage) = template.base_damage {
            commands.add_component(entity, Damage(damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon);
            }
        }
    }
}
