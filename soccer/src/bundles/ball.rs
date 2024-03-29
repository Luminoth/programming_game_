use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::ball::*;
use crate::components::physics::*;
use crate::game::BALL_RADIUS;
use crate::resources::*;
use crate::BALL_SORT;

use super::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub physical: Physical,
    pub bounds: BoundingCircle,

    #[bundle]
    pub actor: ActorBundle,
}

impl BallBundle {
    pub fn spawn(commands: &mut Commands, params: &SimulationParams, position: Vec2) -> Entity {
        info!("spawning ball at {}", position);

        let mut bundle = commands.spawn_bundle(BallBundle {
            physical: Physical {
                mass: params.ball_mass,
                //max_speed: params.ball_max_speed,
                //max_force: params.ball_max_force,
                ..Default::default()
            },
            bounds: BoundingCircle::from_radius(BALL_RADIUS),
            actor: ActorBundle {
                name: Name::new("Ball"),
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    position.extend(BALL_SORT),
                )),
                ..Default::default()
            },
            ..Default::default()
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: BALL_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::WHITE,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
