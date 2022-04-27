use bevy::app::PluginGroupBuilder;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::components::physics::PHYSICS_STEP;
use crate::game::team::*;
use crate::states;
use crate::states::*;
use crate::systems;
use crate::systems::Systems;
use crate::AGENT_UPDATE_STEP;

pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MainStatePlugin);
    }
}

struct MainStatePlugin;

impl Plugin for MainStatePlugin {
    fn build(&self, app: &mut App) {
        // plugins
        app.add_plugin(crate::components::team::SoccerTeamStateMachinePlugin)
            .add_plugin(crate::components::team::FieldPlayerStateMachinePlugin)
            .add_plugin(crate::components::team::GoalKeeperStateMachinePlugin);

        // events
        app.add_event::<FieldPlayerDispatchedMessageEvent>()
            .add_event::<GoalKeeperDispatchedMessageEvent>();

        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
            // physics (fixed timestep)
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    // steering
                    .with_system(systems::steering::update_seek.label(Systems::Steering))
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
            // agents (fixed timestep)
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(AGENT_UPDATE_STEP))
                    // messaging
                    .with_system(systems::messaging::update::<FieldPlayerMessage>)
                    .with_system(systems::messaging::update::<GoalKeeperMessage>)
                    // team systems
                    .with_system(
                        systems::team::PrepareForKickOff_execute
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::Defending_execute
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::Attacking_execute
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    // field player systems
                    .with_system(
                        systems::team::field_player::GlobalState_execute
                            .label(Systems::GlobalStateExecute),
                    )
                    // goal keeper systems
                    .with_system(
                        systems::team::goal_keeper::GlobalState_execute
                            .label(Systems::GlobalStateExecute),
                    ),
            )
            // per-frame systems
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    // team event handlers
                    .with_system(
                        systems::team::PrepareForKickOff_enter
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Defending_enter
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Attacking_enter
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    // field player systems
                    .with_system(
                        systems::team::field_player::GlobalState_on_message
                            .label(Systems::GlobalStateOnMessage),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_enter
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Main).with_system(states::main::teardown),
            );
    }
}
