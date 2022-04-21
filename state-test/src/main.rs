mod bundles;
mod components;
mod states;
mod systems;

use bevy::log::LogPlugin;
use bevy::prelude::*;

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

    // plugins
    app.add_plugin(components::state_test::TestStateMachinePlugin);

    // initial game state
    app.add_state(GameState::Main);

    // main setup
    app.add_startup_system(setup);

    app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_system(
                    systems::state_test::state_test_idle_enter.label(systems::Systems::StateEnter),
                )
                .with_system(
                    systems::state_test::state_test_idle_execute
                        .label(systems::Systems::StateExecute),
                )
                .with_system(
                    systems::state_test::state_test_walk_enter.label(systems::Systems::StateEnter),
                )
                .with_system(
                    systems::state_test::state_test_walk_execute
                        .label(systems::Systems::StateExecute),
                )
                .with_system(
                    systems::state_test::state_test_run_execute
                        .label(systems::Systems::StateExecute),
                )
                .with_system(
                    systems::state_test::state_test_run_exit.label(systems::Systems::StateExit),
                )
                // end of frame marker
                .with_system(
                    systems::end_of_frame
                        .label(systems::Systems::EndOfFrame)
                        .after(systems::Systems::StateExecute),
                ),
        )
        .add_system_set(SystemSet::on_exit(GameState::Main).with_system(states::main::teardown));

    app.run();
}
