use std::marker::PhantomData;

use mlua::prelude::*;
use mlua::{Function, Table, UserData, UserDataMethods};

#[derive(Debug, Clone)]
pub struct ScriptedStateMachine<T>
where
    T: 'static + UserData + Clone,
{
    current_state: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T> ScriptedStateMachine<T>
where
    T: 'static + UserData + Clone,
{
    pub fn new(current_state: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            current_state: current_state.into(),
            phantom: PhantomData::default(),
        })
    }

    pub fn update(&self, lua: &Lua, entity: &T) -> anyhow::Result<()> {
        let current_state = lua.globals().get::<_, Table>(self.current_state.as_str())?;
        current_state
            .get::<_, Function>("Execute")?
            .call::<_, ()>((entity.clone(), self.clone()))?;

        Ok(())
    }

    pub fn change_state(
        &mut self,
        lua: &Lua,
        entity: &T,
        new_state: impl Into<String>,
    ) -> mlua::Result<()> {
        println!("[Rust]: Exit");
        let current_state = lua.globals().get::<_, Table>(self.current_state.as_str())?;
        current_state
            .get::<_, Function>("Exit")?
            .call::<_, ()>(entity.clone())?;

        self.current_state = new_state.into();

        println!("[Rust]: Enter");
        let current_state = lua.globals().get::<_, Table>(self.current_state.as_str())?;
        current_state
            .get::<_, Function>("Enter")?
            .call::<_, ()>(entity.clone())?;

        Ok(())
    }
}

impl<T> UserData for ScriptedStateMachine<T>
where
    T: 'static + UserData + Clone,
{
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "change_state",
            |lua, this, (entity, new_state): (T, String)| {
                this.change_state(lua, &entity, new_state)
            },
        );
    }
}
