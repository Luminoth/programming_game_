#![allow(clippy::too_many_arguments)]

mod components;
mod events;
mod game;
mod resources;
mod systems;

use bevy::log::LogPlugin;
use bevy::prelude::*;

use components::miner::{Miner, MinerWife};
use components::wife::{Wife, WifeMiner};
use events::messaging::MessageEvent;
use game::miner::{MinerStateEnterEvent, MinerStateExitEvent};
use game::wife::{WifeStateEnterEvent, WifeStateExitEvent};
use resources::messaging::MessageDispatcher;

fn setup(mut commands: Commands) {
    // spawn miner / wife entities
    let miner_id = Miner::spawn(&mut commands, "Bob");
    let wife_id = Wife::spawn(&mut commands, "Elsa");

    // pair miners and wives
    commands.entity(miner_id).insert(MinerWife { wife_id });
    commands.entity(wife_id).insert(WifeMiner { miner_id });

    // add resources
    commands.insert_resource(MessageDispatcher::default());
}

#[bevy_main]
fn main() {
    let mut app = App::new();

    // basic bevy
    app.insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::INFO,
        ..Default::default()
    })
    .add_plugins(MinimalPlugins)
    .add_plugin(LogPlugin);

    // main setup
    app.add_startup_system(setup);

    // events
    app.add_event::<(Entity, MessageEvent)>()
        .add_event::<MinerStateEnterEvent>()
        .add_event::<MinerStateExitEvent>()
        .add_event::<WifeStateEnterEvent>()
        .add_event::<WifeStateExitEvent>();

    // messaging systems
    app.add_system(systems::messaging::update);

    // miner systems
    app.add_system(systems::miner::update)
        .add_system(systems::miner::state_exit.label("state_exit"))
        .add_system(
            systems::miner::state_enter
                .label("state_enter")
                .after("state_exit"),
        )
        .add_system(
            systems::miner::global_state_execute
                .label("global_state_execute")
                .after("state_enter"),
        )
        .add_system(
            systems::miner::state_execute
                .label("state_execute")
                .after("global_state_execute"),
        )
        .add_system(systems::miner::state_on_message.label("state_on_message"));

    // wife systems
    app.add_system(systems::wife::state_exit.label("state_exit"))
        .add_system(
            systems::wife::state_enter
                .label("state_enter")
                .after("state_exit"),
        )
        .add_system(
            systems::wife::global_state_execute
                .label("global_state_execute")
                .after("state_enter"),
        )
        .add_system(
            systems::wife::state_execute
                .label("state_execute")
                .after("global_state_execute"),
        )
        .add_system(systems::wife::global_state_on_message.label("global_state_on_message"))
        .add_system(
            systems::wife::state_on_message
                .label("state_on_message")
                .after("global_state_on_message"),
        );

    app.run();
}
