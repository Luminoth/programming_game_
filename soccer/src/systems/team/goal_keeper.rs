#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::*;

pub fn GlobalState_execute<T>(query: Query<GoalKeeperQuery<T>>)
where
    T: TeamColorMarker,
{
    for _goal_keeper in query.iter() {
        /*debug!(
            "executing global state for goal keeper {}",
            goal_keeper.name.as_ref()
        );*/
    }
}
