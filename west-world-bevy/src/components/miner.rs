use bevy::prelude::*;

use crate::game::miner::*;
use crate::game::Location;

use super::state::StateMachine;

pub type MinerStateMachine = StateMachine<MinerState>;

#[derive(Debug, Component)]
pub struct Miner {
    location: Location,
}

impl Default for Miner {
    fn default() -> Self {
        Self {
            location: Location::Shack,
        }
    }
}

impl Miner {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) {
        let name = name.into();
        info!("spawning miner {}", name);

        commands
            .spawn()
            .insert(Miner::default())
            .insert(MinerStateMachine::default())
            .insert(Name::new(name));
    }
}
