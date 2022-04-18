use bevy::app::PluginGroupBuilder;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::components::physics::PHYSICS_STEP;
use crate::events::messaging::DispatchedMessageEvent;
use crate::events::team::*;
use crate::states;
use crate::states::*;
use crate::systems;
use crate::systems::Systems;
use crate::{AGENT_UPDATE_STEP, SUPPORT_UPDATE_STEP};

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
        app.add_event::<DispatchedMessageEvent>()
            .add_event::<SoccerTeamStateEnterEvent>()
            .add_event::<SoccerTeamStateExitEvent>()
            .add_event::<FieldPlayerStateEnterEvent>()
            .add_event::<FieldPlayerStateExitEvent>()
            .add_event::<GoalieStateEnterEvent>()
            .add_event::<GoalieStateExitEvent>();

        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
            // physics (fixed timestep)
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    // steering
                    .with_system(
                        systems::steering::update
                            .label(Systems::SteeringUpdatePhysics)
                            .after(Systems::Steering),
                    )
                    // physics
                    .with_system(
                        systems::physics::update
                            .label(Systems::Physics)
                            .after(Systems::SteeringUpdatePhysics),
                    ),
            )
            // supporting player update (fixed timestep)
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(SUPPORT_UPDATE_STEP))
                    .with_system(systems::team::update_support_spot),
            )
            // agents (fixed timestep)
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(AGENT_UPDATE_STEP))
                    // messaging
                    .with_system(systems::messaging::update)
                    // team systems
                    .with_system(
                        systems::team::global_state_execute.label(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::state_execute
                            .label(Systems::StateExecute)
                            .after(Systems::GlobalStateExecute),
                    )
                    // field player systems
                    .with_system(
                        systems::team::field_player::global_state_execute
                            .label(Systems::GlobalStateExecute),
                    )
                    // goalie systems
                    .with_system(
                        systems::team::goalie::global_state_execute
                            .label(Systems::GlobalStateExecute),
                    ),
            )
            // per-frame systems
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    // team systems
                    .with_system(
                        systems::team::state_enter
                            .label(Systems::StateEnter)
                            .after(Systems::StateExit),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Main).with_system(states::main::teardown),
            );
    }
}
