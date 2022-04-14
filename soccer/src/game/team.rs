use crate::events::state::*;

use super::state::State;

pub type FieldPlayerStateEnterEvent = StateEnterEvent<FieldPlayerState>;
pub type FieldPlayerStateExitEvent = StateExitEvent<FieldPlayerState>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldPlayerState {
    Idle,
}

impl Default for FieldPlayerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for FieldPlayerState {}

pub type GoalieStateEnterEvent = StateEnterEvent<GoalieState>;
pub type GoalieStateExitEvent = StateExitEvent<GoalieState>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GoalieState {
    Idle,
}

impl Default for GoalieState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for GoalieState {}
