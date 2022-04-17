use bevy::prelude::*;

use crate::components::team::GoalieQueryMut;
use crate::game::team::GoalieState;

pub fn global_state_execute(mut query: Query<GoalieQueryMut>) {
    for goalie in query.iter_mut() {
        GoalieState::execute_global(goalie);
    }
}
