use mlua::prelude::*;

use crate::entity::Entity;
use crate::state::ScriptedStateMachine;

pub struct Miner {
    entity: Entity,
    state_machine: ScriptedStateMachine,
}

impl Miner {
    pub fn new(name: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            entity: Entity::new(name),
            state_machine: ScriptedStateMachine::new("State_GoHome")?,
        })
    }

    pub fn update(&self, lua: &Lua) -> anyhow::Result<()> {
        self.state_machine.update(lua, &self.entity)?;

        Ok(())
    }
}
