#![allow(non_snake_case)]

pub mod field_player;
pub mod goal_keeper;

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::messaging::MessageEvent;
use crate::game::team::*;
use crate::resources::messaging::MessageDispatcher;
use crate::resources::pitch::*;
use crate::resources::SimulationParams;

pub fn PrepareForKickOff_enter(
    mut commands: Commands,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    query: Query<SoccerTeamQuery, With<SoccerTeamStatePrepareForKickOffEnter>>,
    receiving: Query<Entity, With<ReceivingPlayer>>,
    closest: Query<Entity, With<ClosestPlayer>>,
    controlling: Query<Entity, With<ControllingPlayer>>,
    supporting: Query<Entity, With<SupportingPlayer>>,
) {
    for team in query.iter() {
        info!("{:?} team preparing for kick off", team.team.team);

        // reset player positions

        if let Ok(receiving) = receiving.get_single() {
            commands.entity(receiving).remove::<ReceivingPlayer>();
        }

        if let Ok(closest) = closest.get_single() {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        if let Ok(controlling) = controlling.get_single() {
            commands.entity(controlling).remove::<ControllingPlayer>();
        }

        if let Ok(supporting) = supporting.get_single() {
            commands.entity(supporting).remove::<SupportingPlayer>();
        }

        // send players home
        message_dispatcher.dispatch_message(None, MessageEvent::GoHome(team.team.team));
    }
}

pub fn PrepareForKickOff_execute(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut query: Query<(Entity, SoccerTeamQueryMut), With<SoccerTeamStatePrepareForKickOffExecute>>,
    players: Query<(&FieldPlayer, &Transform)>,
    goal_keepers: Query<(&GoalKeeper, &Transform)>,
) {
    if query.is_empty() {
        return;
    }

    info!("Waiting for teams ready ...");

    for (player, transform) in players.iter() {
        if !player.is_in_home_region(transform, &pitch) {
            return;
        }
    }

    for (goal_keeper, transform) in goal_keepers.iter() {
        if !goal_keeper.is_in_home_region(transform, &pitch) {
            return;
        }
    }

    for (entity, mut team) in query.iter_mut() {
        team.state_machine
            .change_state(&mut commands, entity, SoccerTeamState::Defending);
    }
}

pub fn Defending_enter(
    query: Query<SoccerTeamQuery, With<SoccerTeamStateDefendingEnter>>,
    pitch: Res<Pitch>,
    mut players: Query<FieldPlayerQueryMut>,
    mut goal_keepers: Query<&mut GoalKeeper>,
) {
    for team in query.iter() {
        info!("{:?} team defending", team.team.team);

        let home_regions = match team.team.team {
            Team::Red => RED_TEAM_DEFENDING_HOME_REGIONS,
            Team::Blue => BLUE_TEAM_DEFENDING_HOME_REGIONS,
        };

        team.team
            .reset_player_home_regions(&mut players, &mut goal_keepers, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut players);
    }
}

pub fn Defending_execute(
    mut commands: Commands,
    mut query: Query<(Entity, SoccerTeamQueryMut), With<SoccerTeamStateDefendingExecute>>,
) {
    for (entity, mut team) in query.iter_mut() {
        //debug!("{:?} defender checking for control", team.team.team);

        // TODO: why not use a message / event for this rather than polling?
        if team.team.in_control {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Attacking);
        }
    }
}

pub fn Attacking_enter(
    query: Query<SoccerTeamQuery, With<SoccerTeamStateAttackingEnter>>,
    pitch: Res<Pitch>,
    mut players: Query<FieldPlayerQueryMut>,
    mut goal_keepers: Query<&mut GoalKeeper>,
) {
    for team in query.iter() {
        info!("{:?} team attacking", team.team.team);

        let home_regions = match team.team.team {
            Team::Red => RED_TEAM_ATTACKING_HOME_REGIONS,
            Team::Blue => BLUE_TEAM_ATTACKING_HOME_REGIONS,
        };

        team.team
            .reset_player_home_regions(&mut players, &mut goal_keepers, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut players);
    }
}

pub fn Attacking_execute(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut query: Query<
        (Entity, SoccerTeamQueryMut, &mut SupportSpotCalculator),
        With<SoccerTeamStateAttackingExecute>,
    >,
    players: Query<(&FieldPlayer, PhysicalQuery)>,
    controller: Query<(&FieldPlayer, &Transform), With<ControllingPlayer>>,
    support: Query<(&FieldPlayer, &Transform), With<SupportingPlayer>>,
    ball: Query<BallQuery>,
    goals: Query<GoalQuery>,
) {
    for (entity, mut team, mut support_calculator) in query.iter_mut() {
        //debug!("{:?} attacker checking for control", team.team.team);

        // TODO: why not use a message / event for this rather than polling?
        if !team.team.in_control {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Defending);
            continue;
        }

        let controller = controller.single();
        let ball = ball.single();

        for goal in goals.iter() {
            if goal.goal.team == team.team.team {
                continue;
            }

            team.team.determine_best_supporting_position(
                &params,
                &mut support_calculator,
                &players,
                controller,
                support.get_single().ok(),
                &ball,
                &goal,
            );
        }
    }
}
