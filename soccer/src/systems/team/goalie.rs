use bevy::prelude::*;

use crate::components::team::Goalie;
use crate::game::team::GoalieState;

pub fn global_state_execute(query: Query<&Name, With<Goalie>>) {
    for name in query.iter() {
        GoalieState::execute_global(name.as_str());
    }
}
