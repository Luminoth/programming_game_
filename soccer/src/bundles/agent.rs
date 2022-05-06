use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::components::agent::*;
use crate::components::steering::*;

#[derive(Debug, Default, Bundle)]
pub struct AgentBundle {
    pub agent: Agent,
    pub steering: Steering,
}

impl AgentBundle {
    pub fn insert_with_separation(commands: &mut EntityCommands) {
        commands.insert_bundle(Self {
            agent: Agent::default(),
            steering: Steering::default(),
        });

        Agent::separation_on(commands);
    }
}
