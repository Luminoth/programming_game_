mod entity;
mod miner;
mod state;

use mlua::prelude::*;

// TODO: this whole thing is broken, data being copied everywhere breaks stuff
// not sure how to pass references around, idk, not great

fn main() -> anyhow::Result<()> {
    let lua = Lua::new();

    lua.load(include_str!("../state_machine.lua")).exec()?;

    let bob = miner::Miner::new("Bob")?;

    loop {
        bob.update(&lua)?;
    }
}
