use mlua::prelude::*;
use mlua::{Function, Table, UserData, UserDataMethods};

use crate::entity::Entity;

// https://github.com/khvzak/mlua/discussions/30 ??

pub struct ScriptedStateMachine<'lua> {
    current_state: Table<'lua>,
}

impl<'lua> ScriptedStateMachine<'lua> {
    pub fn new(lua: &'lua Lua, current_state: impl AsRef<str>) -> anyhow::Result<Self> {
        let current_state = lua.globals().get::<_, Table>(current_state.as_ref())?;

        Ok(Self { current_state })
    }

    pub fn update(&self, entity: &Entity) -> anyhow::Result<()> {
        self.current_state
            .get::<_, Function>("Execute")?
            .call::<_, ()>(entity.clone())?;

        Ok(())
    }

    pub fn change_state(
        &mut self,
        lua: &'lua Lua,
        entity: Entity,
        new_state: impl AsRef<str>,
    ) -> mlua::Result<()> {
        let new_state = lua.globals().get::<_, Table>(new_state.as_ref())?;

        self.current_state
            .get::<_, Function>("Exit")?
            .call::<_, ()>(entity.clone())?;

        self.current_state = new_state;

        self.current_state
            .get::<_, Function>("Enter")?
            .call::<_, ()>(entity.clone())?;

        Ok(())
    }
}

impl<'lua> UserData for ScriptedStateMachine<'lua> {
    fn add_methods<'luam, M: UserDataMethods<'luam, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "change_state",
            |lua, this, (entity, new_state): (Entity, String)| {
                this.change_state(lua, entity, &new_state)
            },
        );
    }
}
