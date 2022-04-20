use bevy::prelude::*;

use super::state::impl_state_machine;

#[derive(Debug, Default, Component)]
pub struct StateMachineOwner;

impl_state_machine!(Test, Idle, Walk, Run);
