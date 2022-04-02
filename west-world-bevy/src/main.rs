mod components;
mod events;
mod game;
mod systems;

use bevy::log::LogPlugin;
use bevy::prelude::*;

use components::miner::Miner;
use components::wife::Wife;
use game::miner::{MinerStateEnterEvent, MinerStateExitEvent};
use game::wife::{WifeStateEnterEvent, WifeStateExitEvent};

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

    // events
    app.add_event::<MinerStateEnterEvent>()
        .add_event::<MinerStateExitEvent>()
        .add_event::<WifeStateEnterEvent>()
        .add_event::<WifeStateExitEvent>();

    // systems
    app.add_system(systems::state::miner_update)
        .add_system(systems::state::miner_state_exit.label("miner_state_exit"))
        .add_system(
            systems::state::miner_state_enter
                .label("miner_state_enter")
                .after("miner_state_exit"),
        )
        .add_system(
            systems::state::miner_global_state_execute
                .label("miner_global_state_execute")
                .after("miner_state_enter"),
        )
        .add_system(
            systems::state::miner_state_execute
                .label("miner_state_execute")
                .after("miner_global_state_execute"),
        )
        .add_system(systems::state::wife_state_exit.label("wife_state_exit"))
        .add_system(
            systems::state::wife_state_enter
                .label("wife_state_enter")
                .after("wife_state_exit"),
        )
        .add_system(
            systems::state::wife_global_state_execute
                .label("wife_global_state_execute")
                .after("wife_state_exit"),
        )
        .add_system(
            systems::state::wife_state_execute
                .label("wife_state_execute")
                .after("wife_global_state_execute"),
        );

    app.run();
}
