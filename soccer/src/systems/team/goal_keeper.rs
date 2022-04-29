#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::*;

pub fn GlobalState_execute<T>(goal_keeper: Query<GoalKeeperQuery<T>>)
where
    T: TeamColorMarker,
{
    let _goal_keeper = goal_keeper.single();
    /*debug!(
        "executing global state for goal keeper {}",
        goal_keeper.name.as_ref()
    );*/
}
