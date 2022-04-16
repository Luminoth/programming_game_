pub mod messaging;
pub mod miner;
pub mod wife;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    // rate limited state machine updates
    GlobalStateExecute,
    StateExecute,

    // per-frame (event) state machine updates
    GlobalStateOnMessage,
    StateOnMessage,
    StateEnter,
    StateExit,
}
