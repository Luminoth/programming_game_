pub mod state_test;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    StateEnter,
    StateExit,
    StateExecute,

    EndOfFrame,
}

pub fn end_of_frame() {
    info!("end of frame!");
}
