use bevy::prelude::*;

use crate::components::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct ActorBundle {
    pub actor: Actor,
    pub name: Name,

    #[bundle]
    pub transform: TransformBundle,
}
