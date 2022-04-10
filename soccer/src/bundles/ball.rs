use bevy::prelude::*;

use crate::components::ball::*;

#[derive(Debug, Default, Bundle)]
pub struct BallBundle {
    pub ball: Ball,
}

impl BallBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
        info!("spawning ball at {}", position);

        let mut bundle = commands.spawn_bundle(BallBundle {
            ball: Ball::default(),
        });

        bundle.insert(Name::new("Ball"));

        bundle.id()
    }
}
