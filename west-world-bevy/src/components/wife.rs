use bevy::prelude::*;

use crate::game::wife::*;

use super::state::StateMachine;

pub type WifeStateMachine = StateMachine<WifeState>;

#[derive(Debug, Default, Component)]
pub struct Wife;

impl Wife {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) {
        let name = name.into();
        info!("spawning wife {}", name);

        commands
            .spawn()
            .insert(Wife::default())
            .insert(WifeStateMachine::default())
            .insert(Name::new(name));
    }
}
