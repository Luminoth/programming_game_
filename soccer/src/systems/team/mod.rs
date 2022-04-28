#![allow(non_snake_case)]

pub mod field_player;
pub mod goal_keeper;

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::SimulationParams;

pub fn PrepareForKickOff_enter<T>(
    mut commands: Commands,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut goal_keeper_message_dispatcher: ResMut<GoalKeeperMessageDispatcher>,
    query: Query<SoccerTeamQuery<T>, With<SoccerTeamStatePrepareForKickOffEnter>>,
    receiving: Query<Entity, (With<T>, With<ReceivingPlayer>)>,
    closest: Query<Entity, (With<T>, With<ClosestPlayer>)>,
    controlling: Query<Entity, (With<T>, With<ControllingPlayer>)>,
    supporting: Query<Entity, (With<T>, With<SupportingPlayer>)>,
    players: Query<Entity, (With<FieldPlayer>, With<T>)>,
    goal_keepers: Query<Entity, (With<GoalKeeper>, With<T>)>,
) where
    T: TeamColorMarker,
{
    if let Ok(team) = query.get_single() {
        info!("{:?} team preparing for kick off", team.color.team_color());

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
        for entity in players.iter() {
            player_message_dispatcher.dispatch_message(Some(entity), FieldPlayerMessage::GoHome);
        }

        for entity in goal_keepers.iter() {
            goal_keeper_message_dispatcher
                .dispatch_message(Some(entity), GoalKeeperMessage::GoHome);
        }
    }
}

pub fn PrepareForKickOff_execute<T>(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut query: Query<
        (Entity, SoccerTeamQueryMut<T>),
        With<SoccerTeamStatePrepareForKickOffExecute>,
    >,
    players: Query<(FieldPlayerQuery<T>, &Transform)>,
    goal_keepers: Query<(GoalKeeperQuery<T>, &Transform)>,
) where
    T: TeamColorMarker,
{
    if query.is_empty() {
        return;
    }

    info!("Waiting for teams ready ...");

    for (player, transform) in players.iter() {
        if !player.player.is_in_home_region(transform, &pitch) {
            return;
        }
    }

    for (goal_keeper, transform) in goal_keepers.iter() {
        if !goal_keeper.goal_keeper.is_in_home_region(transform, &pitch) {
            return;
        }
    }

    for (entity, mut team) in query.iter_mut() {
        team.state_machine
            .change_state(&mut commands, entity, SoccerTeamState::Defending);
    }
}

pub fn Defending_enter<T>(
    pitch: Res<Pitch>,
    query: Query<SoccerTeamQuery<T>, With<SoccerTeamStateDefendingEnter>>,
    mut players: Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
    mut goal_keepers: Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
) where
    T: TeamColorMarker,
{
    for team in query.iter() {
        info!("{:?} team defending", team.color.team_color());

        let home_regions = match team.color.team_color() {
            TeamColor::Red => RED_TEAM_DEFENDING_HOME_REGIONS,
            TeamColor::Blue => BLUE_TEAM_DEFENDING_HOME_REGIONS,
        };

        team.team
            .reset_player_home_regions(&mut players, &mut goal_keepers, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut players, &mut goal_keepers);
    }
}

pub fn Defending_execute<T>(
    mut commands: Commands,
    mut query: Query<(Entity, SoccerTeamQueryMut<T>), With<SoccerTeamStateDefendingExecute>>,
) where
    T: TeamColorMarker,
{
    for (entity, mut team) in query.iter_mut() {
        //debug!("{:?} defender checking for control", team.team.team);

        // TODO: why not use a message / event for this rather than polling?
        if team.team.in_control {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Attacking);
        }
    }
}

pub fn Attacking_enter<T>(
    pitch: Res<Pitch>,
    query: Query<SoccerTeamQuery<T>, With<SoccerTeamStateAttackingEnter>>,
    mut players: Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
    mut goal_keepers: Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
) where
    T: TeamColorMarker,
{
    for team in query.iter() {
        info!("{:?} team attacking", team.color.team_color());

        let home_regions = match team.color.team_color() {
            TeamColor::Red => RED_TEAM_ATTACKING_HOME_REGIONS,
            TeamColor::Blue => BLUE_TEAM_ATTACKING_HOME_REGIONS,
        };

        team.team
            .reset_player_home_regions(&mut players, &mut goal_keepers, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut players, &mut goal_keepers);
    }
}

pub fn Attacking_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut query: Query<
        (Entity, SoccerTeamQueryMut<T>, &mut SupportSpotCalculator),
        With<SoccerTeamStateAttackingExecute>,
    >,
    players: Query<(AnyTeamFieldPlayerQuery, PhysicalQuery)>,
    controller: Query<(FieldPlayerQuery<T>, &Transform), With<ControllingPlayer>>,
    support: Query<(FieldPlayerQuery<T>, &Transform), With<SupportingPlayer>>,
    ball: Query<PhysicalQuery, With<Ball>>,
    goal: Query<GoalQuery<T>>,
) where
    T: TeamColorMarker,
{
    for (entity, mut team, mut support_calculator) in query.iter_mut() {
        //debug!("{:?} attacker checking for control", team.team.team);

        // TODO: why not use a message / event for this rather than polling?
        if !team.team.in_control {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Defending);
            continue;
        }

        let ball = ball.single();
        team.team.determine_best_supporting_position(
            &params,
            team.color,
            &mut support_calculator,
            &players,
            controller.single(),
            support.get_single().ok(),
            &ball.physical,
            goal.single(),
        );
    }
}
