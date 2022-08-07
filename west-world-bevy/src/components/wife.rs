use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use super::state::impl_state_machine;

impl_state_machine!(Wife, DoHouseWork, VisitBathroom, CookStew);

#[derive(Debug, Default, Component)]
pub struct Wife {
    pub cooking: bool,
}

impl Wife {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) -> Entity {
        let name = name.into();
        info!("spawning wife {}", name);

        let mut entity = commands.spawn();
        entity.insert(Wife::default()).insert(Name::new(name));

        WifeStateMachine::insert(&mut entity, WifeState::DoHouseWork);

        entity.id()
    }
}

// this is a separate component because we have to add it after spawning the entities
#[derive(Debug, Component)]
pub struct WifeMiner {
    pub miner_id: Entity,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct WifeQuery {
    pub wife: &'static mut Wife,
    pub state_machine: &'static mut WifeStateMachine,
    pub name: &'static Name,
}
