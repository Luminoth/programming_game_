use mlua::prelude::*;

use crate::entity::Entity;
use crate::state::ScriptedStateMachine;

pub struct Miner<'lua> {
    entity: Entity,
    state_machine: ScriptedStateMachine<'lua>,
}

impl<'lua> Miner<'lua> {
    pub fn new(name: impl Into<String>, lua: &'lua Lua) -> anyhow::Result<Self> {
        Ok(Self {
            entity: Entity::new(name),
            state_machine: ScriptedStateMachine::new(lua, "State_GoHome")?,
        })
    }

    pub fn update(&self) -> anyhow::Result<()> {
        self.state_machine.update(&self.entity)?;

        Ok(())
    }
}
