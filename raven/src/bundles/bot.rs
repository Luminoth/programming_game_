use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::actor::*;
use crate::bundles::agent::*;
use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::physics::*;
use crate::components::weapon::*;
use crate::components::world::*;
use crate::game::{BOT_RADIUS, BOT_SORT};

#[derive(Debug, Bundle)]
pub struct BotBundle {
    #[bundle]
    pub actor: ActorBundle,

    pub physical: Physical,
    pub bounds: Bounds,

    #[bundle]
    pub agent: AgentBundle,

    pub bot: Bot,
}

impl BotBundle {
    pub fn spawn_at_spawnpoint(
        commands: &mut Commands,
        name: impl Into<String>,
        color: Color,
        health: usize,
        spawnpoint: SpawnPointQueryItem,
    ) -> Entity {
        let position = spawnpoint
            .spawnpoint
            .get_spawn_position(spawnpoint.transform);

        Self::spawn_at_position(commands, name, color, health, position)
    }

    pub fn spawn_at_position(
        commands: &mut Commands,
        name: impl Into<String>,
        color: Color,
        health: usize,
        position: Vec2,
    ) -> Entity {
        let name = name.into();
        info!(
            "spawning bot '{}' at {} with {} health",
            name, position, health
        );

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
            bounds: Bounds::Circle(Vec2::ZERO, BOT_RADIUS),
            agent: AgentBundle::default(),
            bot: Bot::new(color, health),
        });

        bundle.insert(Weapon::Blaster);

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
