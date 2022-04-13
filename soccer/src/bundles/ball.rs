use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::actor::*;
use crate::components::actor::*;
use crate::components::ball::*;
use crate::components::physics::*;
use crate::game::BALL_RADIUS;
use crate::BALL_SORT;

#[derive(Debug, Default, Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub physical: Physical,
}

impl BallBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
        info!("spawning ball at {}", position);

        let mut bundle = commands.spawn_bundle(BallBundle::default());

        bundle.insert(Name::new("Ball"));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: BALL_RADIUS,
            },
            transform: Transform::from_translation(position.extend(BALL_SORT)),
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
