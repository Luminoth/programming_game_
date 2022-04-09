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

        bundle.insert(Name::new("Obstacle"));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: radius,
            },
            transform: Transform::from_translation(position.extend(0.0)),
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
