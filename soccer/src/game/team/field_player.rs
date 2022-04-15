use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

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
    pub fn execute_global(name: impl AsRef<str>) {
        debug!("executing global state for player {}", name.as_ref());
    }
}
