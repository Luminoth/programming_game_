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
use systems::Systems;

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
            // rate-limited systems
            SystemSet::on_update(GameState::Main)
                .with_run_criteria(FixedTimestep::step(0.8))
                // messaging systems
                .with_system(systems::messaging::update)
                // miner systems
                .with_system(systems::miner::update)
                .with_system(
                    systems::miner::global_state_execute.label(Systems::GlobalStateExecute),
                )
                .with_system(
                    systems::miner::state_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                // wife systems
                .with_system(systems::wife::global_state_execute.label(Systems::GlobalStateExecute))
                .with_system(
                    systems::wife::state_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                ),
        )
        .add_system_set(
            // per-frame systems
            SystemSet::on_update(GameState::Main)
                // miner systems
                .with_system(systems::miner::state_exit.label(Systems::StateExit))
                .with_system(
                    systems::miner::state_enter
                        .label(Systems::StateEnter)
                        .after(Systems::StateExit),
                )
                .with_system(
                    systems::miner::state_on_message
                        .label(Systems::StateOnMessage)
                        .after(Systems::GlobalStateOnMessage),
                )
                // wife systems
                .with_system(
                    systems::wife::global_state_on_message.label(Systems::GlobalStateOnMessage),
                )
                .with_system(systems::wife::state_exit.label(Systems::StateExit))
                .with_system(
                    systems::wife::state_enter
                        .label(Systems::StateEnter)
                        .after(Systems::StateExit),
                )
                .with_system(
                    systems::wife::state_on_message
                        .label(Systems::StateOnMessage)
                        .after(Systems::GlobalStateOnMessage),
                ),
        );

    app.run();
}
