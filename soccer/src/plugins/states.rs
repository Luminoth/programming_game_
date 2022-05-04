use bevy::app::PluginGroupBuilder;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::components::physics::PHYSICS_STEP;
use crate::components::team::*;
use crate::events::*;
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
        app.add_event::<FindSupportEvent>()
            .add_event::<FieldPlayerDispatchedMessageEvent>()
            .add_event::<GoalKeeperDispatchedMessageEvent>();

        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
            // physics (fixed timestep)
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    // steering
                    .with_system(systems::steering::update_seek.label(Systems::Steering))
                    .with_system(systems::steering::update_arrive.label(Systems::Steering))
                    .with_system(systems::steering::update_pursuit.label(Systems::Steering))
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
                    )
                    .with_system(systems::facing.after(Systems::Physics)),
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
                        systems::team::PrepareForKickOff_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::PrepareForKickOff_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::Defending_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::Defending_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::Attacking_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::Attacking_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::TeamStates)
                            .after(Systems::GlobalStateExecute),
                    )
                    // field player systems
                    .with_system(
                        systems::team::field_player::GlobalState_execute::<RedTeam>
                            .label(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::field_player::GlobalState_execute::<BlueTeam>
                            .label(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::Wait_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::Wait_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReceiveBall_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReceiveBall_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::KickBall_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::KickBall_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReturnToHomeRegion_execute::<RedTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReturnToHomeRegion_execute::<BlueTeam>
                            .label(Systems::StateExecute)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    // goal keeper systems
                    .with_system(
                        systems::team::goal_keeper::GlobalState_execute::<RedTeam>
                            .label(Systems::GlobalStateExecute),
                    )
                    .with_system(
                        systems::team::goal_keeper::GlobalState_execute::<BlueTeam>
                            .label(Systems::GlobalStateExecute),
                    ),
            )
            // per-frame systems
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    // team systems
                    .with_system(systems::team::update::<RedTeam>.label(Systems::TeamUpdate))
                    .with_system(systems::team::update::<BlueTeam>.label(Systems::TeamUpdate))
                    .with_system(
                        systems::team::PrepareForKickOff_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::PrepareForKickOff_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::PrepareForKickOff_exit::<RedTeam>
                            .label(Systems::StateExit)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::PrepareForKickOff_exit::<BlueTeam>
                            .label(Systems::StateExit)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Defending_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Defending_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Attacking_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Attacking_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Attacking_exit::<RedTeam>
                            .label(Systems::StateExit)
                            .label(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::Attacking_exit::<BlueTeam>
                            .label(Systems::StateExit)
                            .label(Systems::TeamStates),
                    )
                    // field player systems
                    .with_system(
                        systems::team::field_player::update::<RedTeam>
                            .label(Systems::PlayerUpdate)
                            .after(Systems::TeamUpdate),
                    )
                    .with_system(
                        systems::team::field_player::update::<BlueTeam>
                            .label(Systems::PlayerUpdate)
                            .after(Systems::TeamUpdate),
                    )
                    .with_system(
                        systems::team::field_player::find_support_event_handler::<RedTeam>
                            .label(Systems::PlayerEvents)
                            .before(Systems::PlayerStates),
                    )
                    .with_system(
                        systems::team::field_player::find_support_event_handler::<BlueTeam>
                            .label(Systems::PlayerEvents)
                            .before(Systems::PlayerStates),
                    )
                    .with_system(
                        systems::team::field_player::GlobalState_on_message::<RedTeam>
                            .label(Systems::GlobalStateOnMessage),
                    )
                    .with_system(
                        systems::team::field_player::GlobalState_on_message::<BlueTeam>
                            .label(Systems::GlobalStateOnMessage),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_exit::<RedTeam>
                            .label(Systems::StateExit)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ChaseBall_exit::<BlueTeam>
                            .label(Systems::StateExit)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::Wait_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::Wait_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReceiveBall_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReceiveBall_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::KickBall_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::KickBall_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReturnToHomeRegion_enter::<RedTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReturnToHomeRegion_enter::<BlueTeam>
                            .label(Systems::StateEnter)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReturnToHomeRegion_exit::<RedTeam>
                            .label(Systems::StateExit)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    )
                    .with_system(
                        systems::team::field_player::ReturnToHomeRegion_exit::<BlueTeam>
                            .label(Systems::StateExit)
                            .label(Systems::PlayerStates)
                            .after(Systems::TeamStates),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Main).with_system(states::main::teardown),
            );
    }
}
