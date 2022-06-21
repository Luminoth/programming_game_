use bevy::prelude::*;

use crate::components::trigger::*;

#[derive(Debug, Bundle)]
pub struct TriggerBundle {
    #[bundle]
    pub transform: TransformBundle,

    pub trigger: Trigger,
}

impl TriggerBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
        info!("spawning trigger at {}", position,);

        let bundle = commands.spawn_bundle(TriggerBundle {
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            trigger: Trigger::default(),
        });

        bundle.id()
    }
}
