#![allow(clippy::too_many_arguments)]

mod components;
mod events;
mod game;
mod resources;
pub mod states;
mod systems;

use bevy::core::FixedTimestep;
use bevy::log::LogPlugin;
use bevy::prelude::*;

use events::messaging::MessageEvent;
use game::miner::{MinerStateEnterEvent, MinerStateExitEvent};
use game::wife::{WifeStateEnterEvent, WifeStateExitEvent};
use states::GameState;

fn setup(mut _commands: Commands) {}

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

    // initial game state
    app.add_state(GameState::Main);

    // main setup
    app.add_startup_system(setup);

    // events
    app.add_event::<(Entity, MessageEvent)>()
        .add_event::<MinerStateEnterEvent>()
        .add_event::<MinerStateExitEvent>()
        .add_event::<WifeStateEnterEvent>()
        .add_event::<WifeStateExitEvent>();

    app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_run_criteria(FixedTimestep::step(0.8))
                // miner systems
                .with_system(systems::miner::update)
                .with_system(systems::miner::state_exit.label("state_exit"))
                .with_system(
                    systems::miner::state_enter
                        .label("state_enter")
                        .after("state_exit"),
                )
                .with_system(
                    systems::miner::global_state_execute
                        .label("global_state_execute")
                        .after("state_enter"),
                )
                .with_system(
                    systems::miner::state_execute
                        .label("state_execute")
                        .after("global_state_execute"),
                )
                .with_system(systems::miner::state_on_message.label("state_on_message"))
                // wife systems
                .with_system(systems::wife::state_exit.label("state_exit"))
                .with_system(
                    systems::wife::state_enter
                        .label("state_enter")
                        .after("state_exit"),
                )
                .with_system(
                    systems::wife::global_state_execute
                        .label("global_state_execute")
                        .after("state_enter"),
                )
                .with_system(
                    systems::wife::state_execute
                        .label("state_execute")
                        .after("global_state_execute"),
                )
                .with_system(
                    systems::wife::global_state_on_message.label("global_state_on_message"),
                )
                .with_system(
                    systems::wife::state_on_message
                        .label("state_on_message")
                        .after("global_state_on_message"),
                ),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Main)
                // messaging systems
                .with_system(systems::messaging::update),
        );

    app.run();
}
