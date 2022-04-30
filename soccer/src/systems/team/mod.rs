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
use crate::resources::*;
use crate::util::*;

pub fn update<T>(
    mut commands: Commands,
    teams: Query<SoccerTeamQuery<T>>,
    players: Query<(Entity, &Transform), (With<SoccerPlayer>, With<T>)>,
    closest: Query<Entity, (With<T>, With<ClosestPlayer>)>,
    ball: Query<PhysicalQuery, With<Ball>>,
) where
    T: TeamColorMarker,
{
    for team in teams.iter() {
        team.team.calculate_closest_player_to_ball(
            &mut commands,
            &ball.single().transform,
            &players,
            closest.optional_single(),
        );
    }
}

pub fn PrepareForKickOff_enter<T>(
    mut commands: Commands,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut goal_keeper_message_dispatcher: ResMut<GoalKeeperMessageDispatcher>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStatePrepareForKickOffEnter>>,
    receiving: Query<Entity, (With<T>, With<ReceivingPlayer>)>,
    closest: Query<Entity, (With<T>, With<ClosestPlayer>)>,
    controlling: Query<Entity, (With<T>, With<ControllingPlayer>)>,
    supporting: Query<Entity, (With<T>, With<SupportingPlayer>)>,
    field_players: Query<Entity, (With<FieldPlayer>, With<T>)>,
    goal_keeper: Query<Entity, (With<GoalKeeper>, With<T>)>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        info!("{:?} team preparing for kick off", team.color.team_color());

        // reset player positions

        if let Some(receiving) = receiving.optional_single() {
            commands.entity(receiving).remove::<ReceivingPlayer>();
        }

        if let Some(closest) = closest.optional_single() {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        if let Some(controlling) = controlling.optional_single() {
            commands.entity(controlling).remove::<ControllingPlayer>();
        }

        if let Some(supporting) = supporting.optional_single() {
            commands.entity(supporting).remove::<SupportingPlayer>();
        }

        // send players home
        for field_player in field_players.iter() {
            player_message_dispatcher
                .dispatch_message(Some(field_player), FieldPlayerMessage::GoHome);
        }

        goal_keeper_message_dispatcher
            .dispatch_message(Some(goal_keeper.single()), GoalKeeperMessage::GoHome);
    }
}

pub fn PrepareForKickOff_execute<T>(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut teams: Query<
        (Entity, SoccerTeamQueryMut<T>),
        With<SoccerTeamStatePrepareForKickOffExecute>,
    >,
    players: Query<(SoccerPlayerQuery<T>, &Transform)>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut team)) = teams.optional_single_mut() {
        info!("waiting for teams ready ...");

        for (player, transform) in players.iter() {
            if !player.player.is_in_home_region(transform, &pitch) {
                return;
            }
        }

        team.state_machine
            .change_state(&mut commands, entity, SoccerTeamState::Defending);
    }
}

pub fn PrepareForKickOff_exit<T>(
    mut game_state: ResMut<GameState>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStatePrepareForKickOffExit>>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        match team.color.team_color() {
            TeamColor::Red => game_state.red_team_ready = true,
            TeamColor::Blue => game_state.blue_team_ready = true,
        }
    }
}

pub fn Defending_enter<T>(
    pitch: Res<Pitch>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStateDefendingEnter>>,
    mut field_players: Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
    mut goal_keeper: Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        info!("{:?} team defending", team.color.team_color());

        let home_regions = match team.color.team_color() {
            TeamColor::Red => RED_TEAM_DEFENDING_HOME_REGIONS,
            TeamColor::Blue => BLUE_TEAM_DEFENDING_HOME_REGIONS,
        };

        let mut goal_keeper = goal_keeper.single_mut();

        team.team
            .reset_player_home_regions(&mut field_players, &mut goal_keeper, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut field_players, &mut goal_keeper);
    }
}

pub fn Defending_execute<T>(
    mut commands: Commands,
    mut teams: Query<(Entity, SoccerTeamQueryMut<T>), With<SoccerTeamStateDefendingExecute>>,
    controller: Query<Entity, (With<T>, With<ControllingPlayer>)>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut team)) = teams.optional_single_mut() {
        if controller.optional_single().is_some() {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Attacking);
        }
    }
}

pub fn Attacking_enter<T>(
    pitch: Res<Pitch>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStateAttackingEnter>>,
    mut field_players: Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
    mut goal_keeper: Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        info!("{:?} team attacking", team.color.team_color());

        let home_regions = match team.color.team_color() {
            TeamColor::Red => RED_TEAM_ATTACKING_HOME_REGIONS,
            TeamColor::Blue => BLUE_TEAM_ATTACKING_HOME_REGIONS,
        };

        let mut goal_keeper = goal_keeper.single_mut();

        team.team
            .reset_player_home_regions(&mut field_players, &mut goal_keeper, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut field_players, &mut goal_keeper);
    }
}

pub fn Attacking_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut teams: Query<
        (Entity, SoccerTeamQueryMut<T>, &mut SupportSpotCalculator),
        With<SoccerTeamStateAttackingExecute>,
    >,
    opponents: Query<PhysicalQuery, (With<SoccerPlayer>, Without<T>)>,
    controller: Query<&Transform, (With<T>, With<ControllingPlayer>)>,
    support: Query<Entity, (With<T>, With<SupportingPlayer>)>,
    ball: Query<PhysicalQuery, With<Ball>>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut team, mut support_calculator)) = teams.optional_single_mut() {
        if let Some(controller) = controller.optional_single() {
            team.team.determine_best_supporting_position(
                &params,
                team.color,
                &mut support_calculator,
                &opponents,
                controller,
                support.optional_single().is_some(),
                &ball.single().physical,
                opponent_goal.single(),
            );
        } else {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Defending);
        }
    }
}
