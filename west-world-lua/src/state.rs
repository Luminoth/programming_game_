use mlua::prelude::*;
use mlua::{Function, Table, UserData, UserDataMethods};

use crate::entity::Entity;

// https://github.com/khvzak/mlua/discussions/30 ??

pub struct ScriptedStateMachine {
    current_state: String,
}

impl ScriptedStateMachine {
    pub fn new(current_state: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            current_state: current_state.into(),
        })
    }

    pub fn update(&self, lua: &Lua, entity: &Entity) -> anyhow::Result<()> {
        let current_state = lua.globals().get::<_, Table>(self.current_state.clone())?;
        current_state
            .get::<_, Function>("Execute")?
            .call::<_, ()>(entity.clone())?;

        Ok(())
    }

    pub fn change_state(
        &mut self,
        lua: &Lua,
        entity: Entity,
        new_state: impl Into<String>,
    ) -> mlua::Result<()> {
        let current_state = lua.globals().get::<_, Table>(self.current_state.clone())?;
        current_state
            .get::<_, Function>("Exit")?
            .call::<_, ()>(entity.clone())?;

        self.current_state = new_state.into();

        let current_state = lua.globals().get::<_, Table>(self.current_state.clone())?;
        current_state
            .get::<_, Function>("Enter")?
            .call::<_, ()>(entity.clone())?;

        Ok(())
    }
}

impl UserData for ScriptedStateMachine {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "change_state",
            |lua, this, (entity, new_state): (Entity, String)| {
                this.change_state(lua, entity, new_state)
            },
        );
    }
}
