use bevy::prelude::*;

use crate::components::entity::*;

#[derive(Debug, Default, Bundle)]
pub struct GameEntityBundle {
    pub game_entity: BaseGameEntity,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
