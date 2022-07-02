use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::actor::*;
use crate::components::agent::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::vehicle::*;

use super::actor::ActorBundle;

pub const VEHICLE_RADIUS: f32 = 10.0;

#[derive(Debug, Default, Bundle)]
pub struct VehicleBundle {
    pub physical: Physical,
    pub agent: Agent,
    pub obstacle: Obstacle,
    pub obstacle_avoidance: ObstacleAvoidance,
    pub wall_avoidance: WallAvoidance,
    pub vehicle: Vehicle,
}

impl VehicleBundle {
    pub fn spawn(
        commands: &mut Commands,
        position: Vec2,
        mass: f32,
        max_speed: f32,
        max_force: f32,
        max_turn_rate: f32,
        name: impl Into<String>,
        color: Color,
    ) -> Entity {
        let name = name.into();

        info!("spawning vehicle {} ({:?}) at {}", name, color, position);

        let obstacle_avoidance_box_length = 0.0;

        let mut bundle = commands.spawn_bundle(VehicleBundle {
            physical: Physical {
                mass,
                max_speed,
                max_force,
                max_turn_rate,
                ..Default::default()
            },
            agent: Agent::default(),
            obstacle: Obstacle::default(),
            obstacle_avoidance: ObstacleAvoidance {
                box_length: obstacle_avoidance_box_length,
            },
            wall_avoidance: WallAvoidance::default(),
            vehicle: Vehicle::default(),
        });

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: VEHICLE_RADIUS,
            },
            name: Name::new(name),
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            ..Default::default()
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::RegularPolygon {
                        sides: 3,
                        feature: shapes::RegularPolygonFeature::SideLength(VEHICLE_RADIUS * 2.0),
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

        // debug bounding volume
        /*bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: VEHICLE_RADIUS,
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

        // debug avoidance volume
        /*bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents: Vec2::new(VEHICLE_RADIUS, obstacle_avoidance_box_length),
                        origin: RectangleOrigin::Center,
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::WHITE,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Avoidance Volume"))
                .insert(ObstacleAvoidanceDebug);
        });*/

        bundle.id()
    }
}
