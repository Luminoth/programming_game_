use super::state::impl_state_machine;

#[derive(Debug, Default, bevy::prelude::Component)]
pub struct StateMachineOwner;

impl_state_machine!(Test, Idle, Walk, Run);
