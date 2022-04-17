#![allow(non_snake_case)]

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::components::team::GoalieQueryMutItem;

use super::super::state::State;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Inspectable)]
pub enum GoalieState {
    Idle,
}

impl Default for GoalieState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for GoalieState {}

impl GoalieState {
    pub fn execute_global(goalie: GoalieQueryMutItem) {
        debug!("executing global state for goalie {}", goalie.name.as_ref());
    }
}
