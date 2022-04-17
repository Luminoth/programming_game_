use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::components::team::FieldPlayerQueryMutItem;

use super::super::state::State;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Inspectable)]
pub enum FieldPlayerState {
    Idle,
}

impl Default for FieldPlayerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for FieldPlayerState {}

impl FieldPlayerState {
    pub fn execute_global(player: FieldPlayerQueryMutItem) {
        debug!("executing global state for player {}", player.name.as_ref());
    }
}
