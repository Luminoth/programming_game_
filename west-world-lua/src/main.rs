mod entity;
mod miner;
mod state;

use mlua::prelude::*;

fn main() -> anyhow::Result<()> {
    let lua = Lua::new();

    lua.load(include_str!("../state_machine.lua")).exec()?;

    let bob = miner::Miner::new("Bob")?;

    loop {
        bob.update(&lua)?;
    }
}
