use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::agent::*;
use crate::components::entity::*;
use crate::components::physics::*;
use crate::components::vehicle::*;

const VEHICLE_SIZE: f32 = 20.0;

#[derive(Debug, Default, Bundle)]
pub struct VehicleBundle {
    pub game_entity: BaseGameEntity,
    pub agent: Agent,
    pub vehicle: Vehicle,
    pub physical: Physical,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl VehicleBundle {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) {
        let name = name.into();

        info!("spawning vehicle {}", name);

        let mut bundle = commands.spawn_bundle(VehicleBundle {
            game_entity: BaseGameEntity::default(),
            agent: Agent::default(),
            vehicle: Vehicle::default(),
            physical: Physical::default(),
            ..Default::default()
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
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
                ))
                .insert(Name::new(name));
        });
    }
}
