use bevy::prelude::*;

use crate::components::team::FieldPlayerQueryMut;
use crate::game::team::FieldPlayerState;

pub fn global_state_execute(mut query: Query<FieldPlayerQueryMut>) {
    for player in query.iter_mut() {
        FieldPlayerState::execute_global(player);
    }
}
