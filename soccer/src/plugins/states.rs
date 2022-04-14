use bevy::app::PluginGroupBuilder;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::components::physics::PHYSICS_STEP;
use crate::events::messaging::MessageEvent;
use crate::events::team::*;
use crate::states;
use crate::states::*;
use crate::systems;

pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MainStatePlugin);
    }
}

struct MainStatePlugin;

impl Plugin for MainStatePlugin {
    fn build(&self, app: &mut App) {
        // events
        app.add_event::<(Entity, MessageEvent)>()
            .add_event::<SoccerTeamStateEnterEvent>()
            .add_event::<SoccerTeamStateExitEvent>()
            .add_event::<FieldPlayerStateEnterEvent>()
            .add_event::<FieldPlayerStateExitEvent>()
            .add_event::<GoalieStateEnterEvent>()
            .add_event::<GoalieStateExitEvent>();

        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
            // physics
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    .with_system(
                        systems::steering::update
                            .label("steering_update")
                            .after("steering"),
                    )
                    .with_system(
                        systems::physics::update
                            .label("physics")
                            .after("steering_update"),
                    ),
            )
            // per-frame systems
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    // messaging
                    .with_system(systems::messaging::update)
                    // team states
                    .with_system(
                        systems::team::soccer_team::global_state_execute
                            .label("global_state_execute")
                            .after("state_enter"),
                    )
                    .with_system(
                        systems::team::field_player::global_state_execute
                            .label("global_state_execute")
                            .after("state_enter"),
                    )
                    .with_system(
                        systems::team::goalie::global_state_execute
                            .label("global_state_execute")
                            .after("state_enter"),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Main).with_system(states::main::teardown),
            );
    }
}
