use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::agent::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::vehicle::*;

use super::actor::ActorBundle;

const VEHICLE_SIZE: f32 = 20.0;

#[derive(Debug, Default, Bundle)]
pub struct VehicleBundle<T>
where
    T: SteeringBehavior,
{
    pub agent: Agent,
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
            steering,
            vehicle: Vehicle::default(),
            physical: Physical::new(mass, max_speed, max_force, max_turn_rate),
        });

        bundle.insert(Name::new(name));

        bundle.insert_bundle(ActorBundle::new(position));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::RegularPolygon {
                        sides: 3,
                        feature: shapes::RegularPolygonFeature::SideLength(VEHICLE_SIZE),
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
