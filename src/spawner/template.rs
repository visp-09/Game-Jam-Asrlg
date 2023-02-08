use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub ripple_ai: Option<bool>,
    pub detonator_ai: Option<bool>,
    pub hp: Option<f32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("data/template.ron").expect("Unable to open data/template.ron");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        commands: &mut CommandBuffer,
        rng: &mut RandomNumberGenerator,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities.iter().for_each(|t| {
            for _ in 0..t.frequency {
                available_entities.push(t);
            }
        });

        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, commands, rng);
            }
        });
    }

    pub fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut CommandBuffer,
        rng: &mut RandomNumberGenerator,
    ) {
        let entity = commands.push((
            Enemy,
            pt.clone(),
            Render {
                color: ColorPair::new(GREEN, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));
        match template.entity_type {
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
                if let Some(ai) = template.ripple_ai {
                    if ai {
                        commands.add_component(
                            entity,
                            RippleAI {
                                max_radius: rng.range(3, 12),
                                current_radius: 1,
                                timestamp: 0,
                                interval: rng.range(12, 45),
                            },
                        );
                    }
                }
                if let Some(ai) = template.detonator_ai {
                    if ai {
                        commands.add_component(
                            entity,
                            DetonatorAI {
                                tile_length: rng.range(1, 4),
                                dimensions: Point::new(4, 4),
                                interval: rng.range(12, 45),
                            },
                        );
                    }
                }
            }
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
}
