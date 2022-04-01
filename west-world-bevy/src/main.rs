mod components;
mod game;
mod systems;

use bevy::log::LogPlugin;
use bevy::prelude::*;

use components::miner::Miner;
use components::wife::Wife;

fn setup(mut commands: Commands) {
    Miner::spawn(&mut commands, "Bob");
    Wife::spawn(&mut commands, "Elsa");
}

#[bevy_main]
fn main() {
    let mut app = App::new();

    // basic bevy
    app.insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    })
    .add_plugins(MinimalPlugins)
    .add_plugin(LogPlugin);

    // main setup
    app.add_startup_system(setup);

    // systems
    app.add_system(systems::state::update_miner_global_state.label("miner_global_state"))
        .add_system(
            systems::state::update_miner_state
                .label("miner_state")
                .after("miner_global_state"),
        )
        .add_system(systems::state::update_wife_global_state.label("wife_global_state"))
        .add_system(
            systems::state::update_wife_state
                .label("wife_state")
                .after("wife_global_state"),
        );

    app.run();
}
