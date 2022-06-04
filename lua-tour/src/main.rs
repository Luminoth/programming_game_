use mlua::prelude::*;
use mlua::{chunk, Function, UserData, UserDataMethods};

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

    println!("UserData Example:");
    userdata_example()?;
    println!();

    println!("Shared UserData Example:");
    shared_userdata_example()?;
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
        local name = "Bilbo Baggins"

        print("name = " .. name)

        local x,y,z = 1,2
        print(x,y,z)

        local x,y,z = 1,2,3,4,5
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

struct Animal {
    num_legs: usize,
    noise_emitted: String,
}

impl Animal {
    fn new(noise_emitted: impl Into<String>, num_legs: usize) -> Self {
        Self {
            num_legs,
            noise_emitted: noise_emitted.into(),
        }
    }

    fn num_legs(&self) -> usize {
        self.num_legs
    }

    fn speak(&self) {
        println!("[Rust]: {}", self.noise_emitted);
    }
}

impl UserData for Animal {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("speak", |_, this, ()| {
            this.speak();
            Ok(())
        });
        methods.add_method("num_legs", |_, this, ()| Ok(this.num_legs()));
    }
}

fn userdata_example() -> anyhow::Result<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    // constructor
    let animal_new = lua.create_function(|_, (noise_emitted, num_legs): (String, usize)| {
        Ok(Animal::new(noise_emitted, num_legs))
    })?;
    globals.set("Animal", animal_new)?;

    lua.load(chunk! {
        local cat = Animal("Meow", 4);
        print("[Lua]: A cat has " .. cat:num_legs() .. " legs");
        cat:speak();
    })
    .exec()?;

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct SharedUserData {
    val: usize,
}

impl SharedUserData {
    fn get_val(&self) -> usize {
        self.val
    }

    fn incr_val(&mut self) {
        self.val += 1
    }
}

impl UserData for SharedUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_val", |_, this, ()| Ok(this.get_val()));
        methods.add_method_mut("incr_val", |_, this, ()| {
            this.incr_val();
            Ok(())
        });
    }
}

// TODO: this example doesn't actually do what it's supposed to
fn shared_userdata_example() -> anyhow::Result<()> {
    let lua = Lua::new();

    let data = SharedUserData::default();

    lua.load(chunk! {
    function incr(data)
        print("[Lua]: Before = " .. data:get_val());
        data:incr_val();
        print("[Lua]: After = " .. data:get_val());
    end
    })
    .exec()?;

    println!("[Rust]: Before = {}", data.get_val());
    lua.globals()
        .get::<_, Function>("incr")?
        .call::<_, ()>(data.clone())?;
    println!("[Rust]: After = {}", data.get_val());

    Ok(())
}
