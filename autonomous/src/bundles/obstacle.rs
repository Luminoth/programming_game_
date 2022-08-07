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
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            name: Name::new("Obstacle"),
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
    pub spatial: SpatialBundle,
}

impl WallBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, from: Vec2, to: Vec2) -> Entity {
        info!("spawning wall at {} from {} to {}", position, from, to);

        let wall = Wall::new(from, to);
        let wall_normal = wall.normal();

        let mut bundle = commands.spawn_bundle(WallBundle {
            wall,
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
        });
        bundle.insert(Name::new("Wall"));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(from, to),
                    DrawMode::Stroke(StrokeMode {
                        color: Color::DARK_GRAY,
                        options: StrokeOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        // wall normal
        /*bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(from, from + wall_normal * 50.0),
                    DrawMode::Stroke(StrokeMode {
                        color: Color::PINK,
                        options: StrokeOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Wall Normal"))
                .insert(WallDebug);
        });*/

        bundle.id()
    }
}
