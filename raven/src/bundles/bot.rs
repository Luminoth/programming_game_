use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::actor::*;
use crate::bundles::agent::*;
use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::inventory::*;
use crate::components::physics::*;
use crate::components::spawnpoint::*;
use crate::components::weapon::*;
use crate::components::*;
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
    pub inventory: Inventory,
    pub equipped_weapon: EquippedWeapon,
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
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    position.extend(BOT_SORT),
                )),
                ..Default::default()
            },
            physical: Physical {
                mass: 75.0,
                max_force: 200.0,
                max_speed: 100.0,
                ..Default::default()
            },
            bounds: Bounds::Circle(Vec2::ZERO, BOT_RADIUS),
            agent: AgentBundle::default(),
            bot: Bot::new(color, health),
            inventory: Inventory::default(),
            equipped_weapon: EquippedWeapon::default(),
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: BOT_RADIUS * 2.0,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::RED,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Visibility { is_visible: false })
                .insert(SelectedBotVisual)
                .insert(Name::new("Selected"));

            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: BOT_RADIUS * 2.0,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::BLUE,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Visibility { is_visible: false })
                .insert(PossessedBotVisual)
                .insert(Name::new("Possessed"));

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
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ))
                .insert(Model)
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
