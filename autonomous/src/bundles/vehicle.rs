use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::agent::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::vehicle::*;

use super::entity::GameEntityBundle;

const VEHICLE_SIZE: f32 = 20.0;

#[derive(Debug, Default, Bundle)]
pub struct VehicleBundle<T>
where
    T: Steering + Default,
{
    pub agent: Agent,
    pub steering: T,
    pub vehicle: Vehicle,
    pub physical: Physical,
}

impl<T> VehicleBundle<T>
where
    T: Steering + Default,
{
    pub fn spawn(commands: &mut Commands, steering: T, name: impl Into<String>) {
        let name = name.into();

        info!("spawning vehicle {}", name);

        let mut bundle = commands.spawn_bundle(VehicleBundle {
            agent: Agent::default(),
            steering,
            vehicle: Vehicle::default(),
            physical: Physical::default(),
            ..Default::default()
        });

        bundle.insert(Name::new(name));

        bundle.insert_bundle(GameEntityBundle::default());

        bundle.with_children(|parent| {
            parent.spawn_bundle(GeometryBuilder::build_as(
                &shapes::RegularPolygon {
                    sides: 3,
                    feature: shapes::RegularPolygonFeature::SideLength(VEHICLE_SIZE),
                    ..Default::default()
                },
                DrawMode::Fill(FillMode {
                    color: Color::WHITE,
                    options: FillOptions::default(),
                }),
                Transform::default(),
            ));
        });
    }
}
