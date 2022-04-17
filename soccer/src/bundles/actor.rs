use bevy::prelude::*;

use crate::components::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct ActorBundle {
    pub actor: Actor,
    pub name: Name,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
