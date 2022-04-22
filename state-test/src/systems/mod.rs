pub mod state_test;

use bevy::prelude::*;

use crate::states::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    StateEnter,
    StateExit,
    StateExecute,

    EndOfFrame,
}

pub fn end_of_frame(state: Res<State<GameState>>) {
    info!("{:?}: **mark**", state.current());
}
