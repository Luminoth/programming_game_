use bevy::prelude::*;

use crate::components::agent::*;
use crate::components::steering::*;

#[derive(Debug, Default, Bundle)]
pub struct AgentBundle {
    pub agent: Agent,
    pub steering: Steering,
}
