use bevy::prelude::*;

use crate::game::wife::*;

use super::state::StateMachine;

pub type WifeStateMachine = StateMachine<WifeState>;

#[derive(Debug, Default, Component)]
pub struct Wife {
    pub cooking: bool,
}

impl Wife {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) -> Entity {
        let name = name.into();
        info!("spawning wife {}", name);

        commands
            .spawn()
            .insert(Wife::default())
            .insert(WifeStateMachine::default())
            .insert(Name::new(name))
            .id()
    }
}

#[derive(Debug, Component)]
pub struct WifeMiner {
    pub miner_id: Entity,
}
