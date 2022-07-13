use mlua::prelude::*;
use mlua::{UserData, UserDataMethods};

use crate::entity::Entity;
use crate::state::ScriptedStateMachine;

const TIREDNESS_THRESHOLD: u64 = 2;

// TODO: components could be a UserData exposed as a field

#[derive(Debug, Default, Clone)]
struct Stats {
    gold_carried: u64,
    fatigue: u64,
}

#[derive(Debug, Default, Clone)]
struct MinerComponents {
    stats: Stats,
}

impl MinerComponents {
    fn mine_gold(&mut self, amount: u64) {
        self.stats.gold_carried += amount;

        self.stats.fatigue += 1;
    }

    fn rest(&mut self) {
        self.stats.fatigue -= 1;
    }

    fn gold_carried(&self) -> u64 {
        self.stats.gold_carried
    }

    fn is_fatigued(&self) -> bool {
        self.stats.fatigue > TIREDNESS_THRESHOLD
    }
}

#[derive(Debug, Clone)]
pub struct Miner {
    entity: Entity,
    state_machine: ScriptedStateMachine<Miner>,
    components: MinerComponents,
}

impl Miner {
    pub fn new(name: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            entity: Entity::new(name),
            state_machine: ScriptedStateMachine::new("State_GoHome")?,
            components: MinerComponents::default(),
        })
    }

    pub fn update(&self, lua: &Lua) -> anyhow::Result<()> {
        self.state_machine.update(lua, self)?;

        Ok(())
    }
}

impl UserData for Miner {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| Ok(this.entity.name().to_owned()));
        methods.add_method("gold_carried", |_, this, ()| {
            this.components.gold_carried();
            Ok(())
        });
        methods.add_method_mut("mine_gold", |_, this, amount: u64| {
            this.components.mine_gold(amount);
            Ok(())
        });
        methods.add_method("is_fatigued", |_, this, ()| {
            this.components.is_fatigued();
            Ok(())
        });
        methods.add_method_mut("rest", |_, this, ()| {
            this.components.rest();
            Ok(())
        });
    }
}
