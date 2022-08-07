#![allow(clippy::too_many_arguments)]

mod components;
mod events;
mod game;
mod resources;
mod states;
mod systems;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

use events::messaging::MessageEvent;
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

    // plugins
    app.add_plugin(components::miner::MinerStateMachinePlugin)
        .add_plugin(components::wife::WifeStateMachinePlugin);

    // initial game state
    app.add_state(GameState::Main);

    // main setup
    app.add_startup_system(setup);

    // events
    app.add_event::<(Entity, MessageEvent)>();

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
                    systems::miner::EnterMineAndDigForNugget_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                .with_system(
                    systems::miner::VisitBankAndDepositGold_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                .with_system(
                    systems::miner::GoHomeAndSleepTilRested_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                .with_system(
                    systems::miner::QuenchThirst_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                .with_system(
                    systems::miner::EatStew_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                // wife systems
                .with_system(systems::wife::GlobalState_execute.label(Systems::GlobalStateExecute))
                .with_system(
                    systems::wife::DoHouseWork_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                )
                .with_system(
                    systems::wife::VisitBathroom_execute
                        .label(Systems::StateExecute)
                        .after(Systems::GlobalStateExecute),
                ),
        )
        .add_system_set(
            // per-frame systems
            SystemSet::on_update(GameState::Main)
                // miner systems
                .with_system(
                    systems::miner::EnterMineAndDigForNugget_enter.label(Systems::StateEnter),
                )
                .with_system(
                    systems::miner::EnterMineAndDigForNugget_exit.label(Systems::StateExit),
                )
                .with_system(
                    systems::miner::VisitBankAndDepositGold_enter.label(Systems::StateEnter),
                )
                .with_system(systems::miner::VisitBankAndDepositGold_exit.label(Systems::StateExit))
                .with_system(
                    systems::miner::GoHomeAndSleepTilRested_enter.label(Systems::StateEnter),
                )
                .with_system(systems::miner::GoHomeAndSleepTilRested_exit.label(Systems::StateExit))
                .with_system(
                    systems::miner::GoHomeAndSleepTilRested_on_message
                        .label(Systems::StateOnMessage)
                        .after(Systems::GlobalStateOnMessage),
                )
                .with_system(systems::miner::QuenchThirst_enter.label(Systems::StateEnter))
                .with_system(systems::miner::QuenchThirst_exit.label(Systems::StateExit))
                .with_system(systems::miner::EatStew_enter.label(Systems::StateEnter))
                .with_system(systems::miner::EatStew_exit.label(Systems::StateExit))
                // wife systems
                .with_system(
                    systems::wife::GlobalState_on_message.label(Systems::GlobalStateOnMessage),
                )
                .with_system(systems::wife::VisitBathroom_enter.label(Systems::StateEnter))
                .with_system(systems::wife::VisitBathroom_exit.label(Systems::StateExit))
                .with_system(systems::wife::CookStew_enter.label(Systems::StateEnter))
                .with_system(
                    systems::wife::CookStew_on_message
                        .label(Systems::StateOnMessage)
                        .after(Systems::GlobalStateOnMessage),
                ),
        )
        .add_system_set(SystemSet::on_exit(GameState::Main).with_system(states::main::teardown));

    app.run();
}
