use bevy::ecs::query::WorldQuery;
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

// this is a separate component because we have to add it after spawning the entities
#[derive(Debug, Component)]
pub struct WifeMiner {
    pub miner_id: Entity,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct WifeQuery<'w> {
    pub wife: &'w mut Wife,
    pub state_machine: &'w mut WifeStateMachine,
    pub name: &'w Name,
}
