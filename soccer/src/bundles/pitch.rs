use bevy::prelude::*;

use crate::components::pitch::*;

#[derive(Debug, Default, Bundle)]
pub struct PitchBundle {
    pub pitch: Pitch,
}

impl PitchBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
        info!("spawning pitch at {}", position);

        let mut bundle = commands.spawn_bundle(PitchBundle {
            pitch: Pitch::default(),
        });

        bundle.insert(Name::new("Pitch"));

        bundle.id()
    }
}
