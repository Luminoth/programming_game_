#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::*;

pub fn GlobalState_execute<T>(query: Query<GoalKeeperQuery<T>>)
where
    T: TeamColorMarker,
{
    let _goal_keeper = query.single();
    /*debug!(
        "executing global state for goal keeper {}",
        goal_keeper.name.as_ref()
    );*/
}
