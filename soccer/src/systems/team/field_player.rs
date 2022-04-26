#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::FieldPlayerQuery;

pub fn GlobalState_execute(query: Query<FieldPlayerQuery>) {
    for _player in query.iter() {
        //debug!("executing global state for player {}", player.name.as_ref());
    }
}
