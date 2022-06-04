use bevy::prelude::*;

use crate::components::agent::*;

#[derive(Debug, Default, Bundle)]
pub struct AgentBundle {
    pub agent: Agent,
}
