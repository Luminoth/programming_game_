#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::GoalKeeperQuery;

pub fn GlobalState_execute(query: Query<GoalKeeperQuery>) {
    for goal_keeper in query.iter() {
        debug!(
            "executing global state for goal keeper {}",
            goal_keeper.name.as_ref()
        );
    }
}
