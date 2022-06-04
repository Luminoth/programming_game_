use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::actor::*;
use crate::bundles::agent::*;
use crate::components::physics::*;
use crate::components::weapon::*;
use crate::components::world::*;
use crate::game::BOT_RADIUS;
use crate::BOT_SORT;

#[derive(Debug, Default, Bundle)]
pub struct BotBundle {
    #[bundle]
    pub actor: ActorBundle,

    pub physical: Physical,

    #[bundle]
    pub agent: AgentBundle,
}

impl BotBundle {
    pub fn spawn_at_spawnpoint(
        commands: &mut Commands,
        name: impl Into<String>,
        color: Color,
        spawnpoint: SpawnPointQueryItem,
    ) -> Entity {
        let position = spawnpoint
            .spawnpoint
            .get_spawn_position(spawnpoint.transform);
        Self::spawn_at_position(commands, name, color, position)
    }

    pub fn spawn_at_position(
        commands: &mut Commands,
        name: impl Into<String>,
        color: Color,
        position: Vec2,
    ) -> Entity {
        let name = name.into();
        info!("spawning bot '{}' at {}", name, position);

        let mut bundle = commands.spawn_bundle(BotBundle {
            actor: ActorBundle {
                name: Name::new(name),
                transform: TransformBundle::from_transform(Transform::from_translation(
                    position.extend(BOT_SORT),
                )),
                ..Default::default()
            },
            physical: Physical {
                mass: 75.0,
                ..Default::default()
            },
            agent: AgentBundle::default(),
        });

        bundle.insert(Blaster::default());

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: BOT_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
