pub mod debug;
pub mod messaging;
pub mod physics;
pub mod steering;
pub mod team;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Physics,

    // steering
    Steering,
    SteeringUpdatePhysics,

    // state machines
    GlobalStateExecute,
    StateExecute,
    GlobalStateOnMessage,
    StateOnMessage,
    StateEnter,
    StateExit,

    TeamStates,
    PlayerStates,
}
