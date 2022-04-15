use bevy::prelude::*;

use crate::components::team::Goalie;
use crate::game::team::GoalieState;

pub fn global_state_execute(mut query: Query<&Name, With<Goalie>>) {
    for name in query.iter_mut() {
        GoalieState::execute_global(name.as_str());
    }
}
