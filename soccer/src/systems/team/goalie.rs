#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::GoalieQuery;

pub fn GlobalState_execute(query: Query<GoalieQuery>) {
    for goalie in query.iter() {
        debug!("executing global state for goalie {}", goalie.name.as_ref());
    }
}
