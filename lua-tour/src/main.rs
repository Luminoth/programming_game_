use mlua::chunk;
use mlua::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Basic Example:");
    basic_example()?;
    println!();

    println!("Book Example:");
    book_example()?;
    println!();

    println!("File Example:");
    file_example()?;
    println!();

    Ok(())
}

fn basic_example() -> anyhow::Result<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    let globals = lua.globals();
    globals.set("map_table", map_table)?;

    lua.load(chunk! {
        for k,v in pairs(map_table) do print(k,v) end
    })
    .exec()?;

    Ok(())
}

fn book_example() -> anyhow::Result<()> {
    let lua = Lua::new();

    lua.load(chunk! {
        name = "Bilbo Baggins"

        print("name = "..name)

        x,y,z = 1,2
        print(x,y,z)

        x,y,z = 1,2,3,4,5
        print(x,y,z)
    })
    .exec()?;

    Ok(())
}

fn file_example() -> anyhow::Result<()> {
    let lua = Lua::new();

    lua.load(include_str!("../test.lua")).exec()?;

    Ok(())
}
