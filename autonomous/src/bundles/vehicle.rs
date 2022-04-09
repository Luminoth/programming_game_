use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::actor::*;
use crate::components::agent::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::vehicle::*;

use super::actor::ActorBundle;

pub const VEHICLE_RADIUS: f32 = 10.0;

#[derive(Debug, Default, Bundle)]
pub struct VehicleBundle<T>
where
    T: SteeringBehavior,
{
    pub agent: Agent,
    pub obstacle: Obstacle,
    pub obstacle_avoidance: ObstacleAvoidance,
    pub steering: T,
    pub vehicle: Vehicle,
    pub physical: Physical,
}

impl<T> VehicleBundle<T>
where
    T: SteeringBehavior,
{
    pub fn spawn(
        commands: &mut Commands,
        steering: T,
        position: Vec2,
        mass: f32,
        max_speed: f32,
        max_force: f32,
        max_turn_rate: f32,
        name: impl Into<String>,
        color: Color,
    ) -> Entity {
        let name = name.into();

        info!(
            "spawning vehicle {} ({:?}) at {} with steering behavior {:?}",
            name, color, position, steering
        );

        let mut bundle = commands.spawn_bundle(VehicleBundle {
            agent: Agent::default(),
            obstacle: Obstacle::default(),
            obstacle_avoidance: ObstacleAvoidance::default(),
            steering,
            vehicle: Vehicle::default(),
            physical: Physical {
                mass,
                max_speed,
                max_force,
                max_turn_rate,
                ..Default::default()
            },
        });

        bundle.insert(Name::new(name));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: VEHICLE_RADIUS,
            },
            transform: Transform::from_translation(position.extend(0.0)),
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

        bundle.id()
    }
}
