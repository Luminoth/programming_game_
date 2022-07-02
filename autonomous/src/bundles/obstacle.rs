use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::actor::*;
use crate::components::obstacle::*;

use super::actor::ActorBundle;

#[derive(Debug, Default, Bundle)]
pub struct ObstacleBundle {
    pub obstacle: Obstacle,
}

impl ObstacleBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, radius: f32) -> Entity {
        let size = radius * 2.0;

        info!("spawning obstacle of size {} at {}", size, position);

        let mut bundle = commands.spawn_bundle(ObstacleBundle {
            obstacle: Obstacle::default(),
        });

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: radius,
            },
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            name: Name::new("Obstacle"),
            ..Default::default()
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::RegularPolygon {
                        sides: 4,
                        feature: shapes::RegularPolygonFeature::SideLength(size),
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::GRAY,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        // debug bounding volume
        /*bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: radius,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::PINK,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Bounding Volume"))
                .insert(ObstacleDebug);
        });*/

        bundle.id()
    }
}

#[derive(Debug, Default, Bundle)]
pub struct WallBundle {
    pub wall: Wall,

    #[bundle]
    pub transform: TransformBundle,
}

impl WallBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, extents: Vec2, facing: Vec2) -> Entity {
        info!(
            "spawning wall of size {} at {} facing {}",
            extents, position, facing
        );

        let mut bundle = commands.spawn_bundle(WallBundle {
            wall: Wall { extents, facing },
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
        });
        bundle.insert(Name::new("Wall"));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::DARK_GRAY,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
