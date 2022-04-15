use bevy::prelude::*;

use crate::components::team::FieldPlayer;
use crate::game::team::FieldPlayerState;

pub fn global_state_execute(mut query: Query<&Name, With<FieldPlayer>>) {
    for name in query.iter_mut() {
        FieldPlayerState::execute_global(name.as_str());
    }
}
