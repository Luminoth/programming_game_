use bevy::prelude::*;

use crate::components::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct ActorBundle {
    pub actor: Actor,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ActorBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            actor: Actor::default(),
            transform: Transform::from_translation(position.extend(0.0)),
            global_transform: GlobalTransform::default(),
        }
    }
}
